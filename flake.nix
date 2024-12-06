{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    slint-viewer.url = "github:kamwithk/slint-viewer-flake";
  };

  outputs =
    {
      nixpkgs,
      utils,
      slint-viewer,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        defaultPackage =
          with pkgs;
          rustPlatform.buildRustPackage {
            name = "funkyduck";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [
              pkg-config
              autoPatchelfHook
              qt6Packages.wrapQtAppsHook
            ];
            buildInputs = [
              (lib.getLib stdenv.cc.cc)
              qt6Packages.qtsvg
              qt6Packages.qtbase
              qt6Packages.qtwayland
            ];
            runtimeDependencies = [
              wayland
              libxkbcommon
              libGL
              libGL.dev
              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
              xorg.libX11
              fontconfig.lib
            ];
          };
        devShell =
          with pkgs;
          mkShell {
            nativeBuildInputs = [
              pkg-config
              autoPatchelfHook
            ];
            buildInputs = [
              cargo
              rustc
              rustfmt
              taplo
              rustPackages.clippy
              slint-lsp
              slint-viewer.defaultPackage.${system}
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            SLINT_STYLE = "cosmic-dark";
          };
      }
    );
}
