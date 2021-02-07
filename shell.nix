let
  sources = import ./nix/sources.nix;
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs {};
in
pkgs.mkShell {
  nativeBuildInputs = [
    rust

    pkgs.cargo-edit
    pkgs.pkg-config
  ];

  buildInputs = [
    pkgs.openssl
    pkgs.libgit2
  ];

  # Preconfigure debug tools for backtracing
  RUST_BACKTRACE = 1;
}
