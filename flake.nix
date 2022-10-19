{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rust-env = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
        
        server = pkgs.writeShellScriptBin "server" ''
          ${pkgs.python3}/bin/python -m http.server 3000
        '';
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkgconfig
            
            rust-env
            server
            
            yt-dlp
            yq
          ];
          

        };
      }
    );
}