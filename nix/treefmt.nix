{ ... }:
{
  # Formatter configuration
  projectRootFile = "flake.nix"; # marks the root of the repo

  programs = {
    nixfmt.enable = true;

    rustfmt.enable = true;

    prettier.enable = true; # for markdown
  };

  settings.formatter = {
    nixfmt.exclues = [ "target" ];
  };
}
