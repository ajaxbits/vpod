{ ... }:
{
  projectRootFile = "flake.nix";
  programs = {
    just.enable = true;
    rustfmt.enable = true;
    nixfmt-rfc-style.enable = true;
    deadnix.enable = true;
    taplo.enable = true;
  };
}
