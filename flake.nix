{
  description = "A server for generating podcast feeds from YouTube channels";

  inputs = {
    dream2nix.url = "github:nix-community/dream2nix";
    nixpkgs.follows = "dream2nix/nixpkgs";
    latest.url = "github:nixos/nixpkgs";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    dream2nix,
    latest,
    ...
  }: let
    system = "x86_64-linux";

    name = "vpod";
    version = "0.1.0";
  in {
    packages.${system} = {
      default = dream2nix.lib.evalModules {
        packageSets = {
          nixpkgs = inputs.dream2nix.inputs.nixpkgs.legacyPackages.${system};
          latest = import latest {inherit system;};
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

      oci-image = let
        inherit (nixpkgs.legacyPackages.${system}) dockerTools;
      in
        dockerTools.buildLayeredImage {
          inherit name;
          tag = version;
          maxLayers = 128;
          contents = [dockerTools.caCertificates];
          config.Cmd = [
            "${self.packages.${system}.default}/bin/vpod"
          ];
        };
    };
  };
}
