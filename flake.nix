{
  description = "A server for generating podcast feeds from YouTube channels";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    latest.url = "github:nixos/nixpkgs";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    systems.url = "github:nix-systems/default";
  };

  nixConfig = {
    extra-substituters = [
      "https://cache.garnix.io"
      "https://numtide.cachix.org"
    ];
    extra-trusted-public-keys = [
      "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="
      "numtide.cachix.org-1:2ps1kLBUWjxIneOy1Ik6cQjb41X0iXVXeHigGmycPPE="
    ];
  };

  outputs =
    inputs:
    inputs.parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = with inputs; [
        treefmt-nix.flakeModule
        nci.flakeModule
      ];

      perSystem =
        {
          config,
          pkgs,
          lib,
          system,
          self',
          ...
        }:
        let
          pkgsLatest = import inputs.latest { inherit system; };
        in
        {
          nci.projects.vpod = {
            path = ./.;
            profiles.release.runTests = false;
            depsDrvConfig.mkDerivation = {
              buildInputs = [ pkgs.openssl ];
              nativeBuildInputs = [ pkgs.pkg-config ];
            };
            drvConfig.mkDerivation = {
              buildInputs =
                [ pkgs.openssl ]
                ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
                  pkgs.darwin.apple_sdk.frameworks.Security
                  pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
                ];
              nativeBuildInputs = [
                pkgs.makeWrapper
                pkgs.pkg-config
              ];
              postFixup = ''
                wrapProgram $out/bin/vpod \
                  --set PATH ${lib.makeBinPath [ pkgsLatest.yt-dlp ]}
              '';
            };
          };

          packages =
            let
              inherit (self'.packages) vpod;
              inherit (pkgs) dockerTools;
            in
            {
              vpod = config.nci.outputs.vpod.packages.release;
              default = vpod;

              oci-image = dockerTools.buildLayeredImage {
                inherit (vpod) name;
                tag = vpod.version;
                maxLayers = 128;
                contents = [ dockerTools.caCertificates ];
                config.Cmd = [ "${vpod}/bin/vpod" ];
              };
            };

          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              deadnix.enable = true;
              just.enable = true;
              nixfmt-rfc-style.enable = true;
              rustfmt.enable = true;
              taplo.enable = true;
              yamlfmt.enable = true;
            };
          };

          devShells.default = config.nci.outputs.vpod.devShell.overrideAttrs (old: {
            packages =
              (old.packages or [ ])
              ++ (with pkgs; [
                flyctl
                just
                pkgsLatest.yt-dlp
              ]);
            shellHook = ''
              ${old.shellHook or ""}
              export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
            '';
          });
        };
    };
}
