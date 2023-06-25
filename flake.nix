{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat = {
      url = "github:inclyc/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustAttrs = import ./nix/rust.nix { inherit pkgs; };
        dioxus-cli = import ./nix/dioxus-cli.nix {
          inherit pkgs;
          rustPlatform = rustAttrs.rustPlatform;
        };
      in {
        formatter = pkgs.nixpkgs-fmt;
        devShells = {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              rustAttrs.rust-shell
              dioxus-cli
              # common
              just
            ];
          };
        };

      });
}
