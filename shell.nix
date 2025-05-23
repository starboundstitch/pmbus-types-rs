{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    rustc
    rust-analyzer
    rustup
    rustfmt
  ];
}
