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
    buildInputs = with config.deps; [openssl yt-dlp rustc];
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
      rustc
      openssl
      ;
    inherit (latest) yt-dlp;
  };

  rust-crane = {
    depsDrv = {
      mkDerivation.buildInputs = with config.deps; [openssl yt-dlp rustc];
      mkDerivation.nativeBuildInputs = [config.deps.pkg-config];
    };
    runTests = false;
  };
}
