{ pkgs ? import <nixpkgs> { }
, lib ? pkgs.lib
, ...
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    cargo-watch
  ];
}
