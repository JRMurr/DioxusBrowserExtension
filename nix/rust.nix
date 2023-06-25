{ pkgs, ... }:

let
  rustVersion = (pkgs.rust-bin.fromRustupToolchainFile
    ../rust-toolchain.toml); # rust-bin.stable.latest.default
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rustVersion;
    rustc = rustVersion;
  };
in {
  inherit rustPlatform;
  rust-shell = (rustVersion.override {
    extensions = [ "rust-src" "rust-analyzer" ];
    targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" ];
  });
}
