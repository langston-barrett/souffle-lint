{ pkgs ? import <nixpkgs> { }
, unstable ? import <unstable> { }
}:

pkgs.mkShell {
  hardeningDisable = [ "format" ];
  nativeBuildInputs = [
    pkgs.lit
    pkgs.rustup
  ];
}
