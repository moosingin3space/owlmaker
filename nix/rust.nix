{ sources ? import ./sources.nix }:

let
  pkgs = import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  channel = "1.49.0";
  date = null;
  targets = [ ];
  chan = pkgs.rustChannelOfTargets channel date targets;
in chan
