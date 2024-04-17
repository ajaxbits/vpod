{
  config,
  dream2nix,
  ...
}: {
  imports = [
    dream2nix.modules.dream2nix.rust-cargo-lock
    dream2nix.modules.dream2nix.rust-crane
  ];

  mkDerivation = {
    src = ./.;
    buildInputs = [config.deps.openssl config.deps.yt-dlp];
    nativeBuildInputs = [config.deps.pkg-config];
  };

  deps = {
    nixpkgs,
    latest,
    ...
  }: {
    inherit
      (nixpkgs)
      stdenv
      pkg-config
      openssl
      ;
    inherit (latest) yt-dlp;
  };

  name = "vpod";
  version = "0.1.0";

  rust-crane = {
    depsDrv = {
      mkDerivation.buildInputs = [config.deps.openssl config.deps.yt-dlp];
      mkDerivation.nativeBuildInputs = [config.deps.pkg-config];
    };
  };
}
