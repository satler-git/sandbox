{

  pkgs ? import <nixpkgs> { },
}:
let
  lib = pkgs.lib;
in
pkgs.rustPlatform.buildRustPackage {
  pname = "hello-gpui-tokio";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = with pkgs; [
    pkg-config
    rustPlatform.bindgenHook
  ];

  buildInputs = with pkgs; [
    xorg.libxcb
    wayland
  ];
}
