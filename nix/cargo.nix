{ sources ? import ./sources.nix }:

let
  cargo2nixOverlay = sources.cargo2nix + "/overlay";
  pkgs = import sources.nixpkgs { overlays = [ (import cargo2nixOverlay) ]; };
  registry = "registry+https://github.com/rust-lang/crates-io-index";
  cargoPkgs = pkgs.rustPkgs."${registry}";
in cargoPkgs
