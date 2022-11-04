{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    naersk,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust-env = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = ["rust-src"];
          });

        naersk-lib = naersk.lib."${system}".override {
          cargo = pkgs.cargo;
          rustc = pkgs.rustc;
        };
      in rec
      {
        packages.vpod = naersk-lib.buildPackage {
          pname = "vpod";
          root = ./.;
          nativeBuildInputs = with pkgs; [pkg-config];
          buildInputs = with pkgs; [openssl];
        };

        defaultPackage = packages.vpod;

        apps.vpod = flake-utils.lib.mkApp {
          drv = packages.vpod;
        };

        packages.vpod-docker = pkgs.dockerTools.buildImage {
          name = "vpod";
          tag = "latest";
          contents = [pkgs.which pkgs.yt-dlp pkgs.bash];
          config = {
            Env = ["PATH=/bin/:${pkgs.which}/bin/:${packages.vpod}/bin"];
            Cmd = ["${packages.vpod}/bin/vpod"];
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            openssl
            pkgconfig

            rust-env

            yt-dlp
            sqlite
            yq

            flyctl
          ];
        };
      }
    );
}
