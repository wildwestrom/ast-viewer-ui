{
  description = "simple rust flake for an iced app";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      overlays = [ (import rust-overlay) ];

      allSystems = [
        "x86_64-linux" # 64bit AMD/Intel x86
        "aarch64-linux" # 64bit ARM Linux
        "x86_64-darwin" # 64bit AMD/Intel macOS
        "aarch64-darwin" # 64bit ARM macOS
      ];

      forAllSystems =
        fn:
        nixpkgs.lib.genAttrs allSystems (
          system: fn { pkgs = import nixpkgs { inherit system overlays; }; }
        );
    in
    {
      devShells = forAllSystems (
        { pkgs }:
        {
          default = pkgs.mkShell rec {
            name = "nix";
            packages = with pkgs; [
              (rust-bin.stable.latest.default.override {
                extensions = [
                  "rust-src"
                  "rust-analyzer"
                  "rustfmt"
                  "clippy"
                ];
              })

              wayland
              libxkbcommon
              libGL

              glib
              pkg-config
              cairo
              gdk-pixbuf
              pango
              gtk3
            ];
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
          };
        }
      );

      packages = forAllSystems (
        { pkgs }:
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "rust";
            version = "0.0.1";

            src = ./rust;

            cargoLock = {
              lockFile = ./rust/Cargo.lock;
            };

            # buildInputs = with pkgs; [];
            nativeBuildInputs = with pkgs; [
              pkg-config
              glib
            ];
          };
        }
      );
    };
}
