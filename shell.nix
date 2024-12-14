{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "rust-wasm-env";
  nativeBuildInputs = [
    # rust
    pkgs.rustup
    pkgs.rustc
    pkgs.cargo
    pkgs.clippy
    pkgs.rustfmt
    pkgs.rust-analyzer
    pkgs.cargo-binutils
    pkgs.cargo-binstall
    pkgs.cargo-generate

    # wasm & yew
    pkgs.wasm-pack 
    pkgs.trunk
    pkgs.lld_18

    # c
    pkgs.pkg-config
  ];
  buildInputs = [
    pkgs.openssl
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
}