{
  description = "A server for generating podcast feeds from YouTube channels";

  inputs = {
    dream2nix.url = "github:nix-community/dream2nix";
    nixpkgs.follows = "dream2nix/nixpkgs";
    latest.url = "github:nixos/nixpkgs";
    treefmt-nix.url = "github:numtide/treefmt-nix";
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
    inputs@{
      self,
      nixpkgs,
      dream2nix,
      latest,
      treefmt-nix,
      systems,
      ...
    }:
    let
      system = "x86_64-linux";

      name = "vpod";
      version = "0.0.3";

      eachSystem = f: nixpkgs.lib.genAttrs (import systems) (system: f nixpkgs.legacyPackages.${system});
      treefmtEval = eachSystem (pkgs: treefmt-nix.lib.evalModule pkgs ./treefmt.nix);
    in
    {
      packages.${system} = {
        default = dream2nix.lib.evalModules {
          packageSets = {
            nixpkgs = inputs.dream2nix.inputs.nixpkgs.legacyPackages.${system};
            latest = import latest { inherit system; };
          };

          modules = [
            ./default.nix
            {
              inherit name version;
              paths = {
                projectRoot = ./.;
                projectRootFile = "flake.nix";
                package = ./.;
              };
            }
          ];
        };

        oci-image =
          let
            inherit (nixpkgs.legacyPackages.${system}) dockerTools;
          in
          dockerTools.buildLayeredImage {
            inherit name;
            tag = version;
            maxLayers = 128;
            contents = [ dockerTools.caCertificates ];
            config.Cmd = [ "${self.packages.${system}.default}/bin/vpod" ];
          };
      };

      formatter = eachSystem (pkgs: treefmtEval.${pkgs.system}.config.build.wrapper);

      checks = eachSystem (pkgs: {
        formatting = treefmtEval.${pkgs.system}.config.build.check self;
      });
    };
}
