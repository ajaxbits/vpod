{
  config,
  dream2nix,
  ...
}: {
  imports = [
    dream2nix.modules.dream2nix.rust-cargo-lock
    dream2nix.modules.dream2nix.rust-crane
  ];

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
      lib
      makeWrapper
      ;
    inherit (latest) yt-dlp;
  };

  mkDerivation = with config.deps; {
    src = ./.;
    buildInputs = [openssl rustc];
    nativeBuildInputs = [pkg-config makeWrapper];
    postFixup = let
      inherit (config.deps) lib yt-dlp;
    in ''
      wrapProgram $out/bin/vpod \
        --set PATH ${lib.makeBinPath [yt-dlp]}
    '';
  };

  rust-crane = {
    depsDrv.mkDerivation = with config.deps; {
      buildInputs = [openssl rustc];
      nativeBuildInputs = [pkg-config];
    };
    runTests = false;
  };
}
