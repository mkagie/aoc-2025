{
  description = "Advent of Code -- 2025";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    flakeInputs@{
      self,
      nixpkgs,
      flake-utils,
      treefmt-nix,
      ...
    }:
    {
      overlays.default = import ./nix/overlay.nix flakeInputs;
    }
    // flake-utils.lib.eachDefaultSystem (
      system:

      let
        aoc-overlay = self.overlays.default;

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ aoc-overlay ];
        };

        treefmtEval = treefmt-nix.lib.evalModule pkgs ./nix/treefmt.nix;
      in
      {
        packages = {
          treefmt = treefmtEval.config.build.wrapper;
        };

        devShells = {
          default = pkgs.mkShell {
            name = "rust shell";
            packages = with pkgs; [
              toolchainDev
              gdb
              # cargo-generate
            ];
          };
        };

        formatter = treefmtEval.config.build.wrapper;

        legacyPackages = pkgs;

        checks = {
          treefmt = treefmtEval.config.build.check self;
        };

        apps = {
        };
      }
    );
}
