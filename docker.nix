{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  owlmaker = import ./default.nix { inherit sources pkgs; };

  name = "ghcr.io/moosingin3space/owlmaker/owlmaker";
  tag = "0.1.0";

in pkgs.dockerTools.buildLayeredImage {
  inherit name tag;
  contents = [ owlmaker ];

  extraCommands = ''
    mkdir -p /work
  '';

  config = {
    Cmd = [ "/bin/owlmaker" ];
    WorkingDir = "/work";
    Volumes = {
      "/work" = {};
    };
  };
}
