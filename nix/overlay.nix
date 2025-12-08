flakeInputs: final: prev: rec {
  # Set up my toolchains the way I would like them
  toolchain = flakeInputs.fenix.packages.${final.system}.stable.toolchain;
  toolchainDev = flakeInputs.fenix.packages.${final.system}.complete.withComponents [
    "cargo"
    "clippy"
    "rustc"
    "rustfmt"
    "rust-analyzer"
    "rust-docs"
    "rust-src"
    "rust-std"
  ];
}
