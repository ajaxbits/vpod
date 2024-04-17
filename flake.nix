{
  description = "A devShell example";

  inputs = {
    dream2nix.url = "github:nix-community/dream2nix";
    nixpkgs.follows = "dream2nix/nixpkgs";
    latest.url = "github:nixos/nixpkgs";
  };

  outputs = inputs @ {
    nixpkgs,
    dream2nix,
    latest,
    ...
  }: let
    system = "x86_64-linux";
  in {
    packages.${system}.default = dream2nix.lib.evalModules {
      packageSets = {
        nixpkgs = inputs.dream2nix.inputs.nixpkgs.legacyPackages.${system};
        latest = import latest {inherit system;};
      };

      modules = [
        ./default.nix
        {
          paths = {
            projectRoot = ./.;
            projectRootFile = "flake.nix";
            package = ./.;
          };
        }
      ];
    };
  };
}
