{ pkgs ? import <nixpkgs> {
  overlays = [
    (import (builtins.fetchTarball
      "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
  ];
} }:
with pkgs;
let rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in pkgs.mkShell { packages = [ 
    rust rust-analyzer cargo-generate 
    nodejs_latest wasm-pack nodePackages_latest.pnpm 
    simple-http-server  
]; }
#cargo b --manifest-path=logic/Cargo.toml
