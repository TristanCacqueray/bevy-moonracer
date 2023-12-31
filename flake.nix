# Build release with: nix -L build .#release
{
  description = "bevy-moonracer";

  inputs = {
    # nixpkgs is tracking nixpkgs-unstable
    nixpkgs.url = "github:NixOS/nixpkgs";

    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, crane, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "x86_64-unknown-linux-musl" "wasm32-unknown-unknown" ];
        };
        linker = with pkgs; {
          "x86_64-linux" = mold;
          "x86_64-darwin" = zld;
          "aarch64-darwin" = zld;
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = pkgs.lib.cleanSourceWith {
          src = ./.; # The original, unfiltered source
          filter = path: type:
            # prevent "error: linker `clang` not found" issue
            !(pkgs.lib.hasSuffix "config.toml" path)
            && ((pkgs.lib.hasSuffix ".html" path)
              || (pkgs.lib.hasSuffix ".config.js" path)
              || (pkgs.lib.hasSuffix ".css" path)
              || (pkgs.lib.hasSuffix ".md" path)
              || (pkgs.lib.hasSuffix ".svg" path)
              || (pkgs.lib.hasSuffix ".txt" path) ||
              # Default filter from crane (allow .rs files)
              (craneLib.filterCargoSources path type));
        };
        web-info = {
          src = src;
        } // craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };
        cargoArtifactsWasm = craneLib.buildDepsOnly (web-info // {
          doCheck = false;
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        });
        web = craneLib.buildTrunkPackage (web-info // {
          cargoArtifacts = cargoArtifactsWasm;
          trunkIndexPath = "./index.html";
          # Fixup the dist output for a publishable package.
          postInstall = ''
            chmod 644 $out/index.html
            # grab the generated hash
            JSFILE=$(basename $out/*.js)
            # Fix path url, remove the auto start and fix the custom start
            sed -e 's/href="\//href=".\//' -e "s/^ *<script.*init('.*$//" -e "s/JSFILE/$JSFILE/" -i $out/index.html
            chmod 444 $out/index.html
          '';
        });

      in {
        packages.web = web;
        devShell = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            rustToolchain
            rust-analyzer
            clang

            udev
            alsa-lib
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr # To use the x11 feature
            libxkbcommon
            wayland # To use the wayland feature

            pipewire

            trunk
            cargo-watch
          ];

          nativeBuildInputs = with pkgs;
            [ pkg-config ] ++ pkgs.lib.catAttrs system [ linker ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          XCURSOR_PATH = "/usr/share/icons/";
          ALSA_PLUGIN_DIR = "${pkgs.pipewire.lib}/lib/alsa-lib/";
        };
      });
}
