{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in rec {
        cheetah = naersk-lib.buildPackage ./.;
        defaultPackage = cheetah;
        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };

        buildSite = src: options: 
          let
            configFormat = pkgs.formats.toml { };
            configFile = configFormat.generate "cheetah.toml" options.config;
          in pkgs.stdenv.mkDerivation {
            inherit (options) name;
            inherit src;
            nativeBuildInputs = [cheetah];
            configurePhase = "cp ${configFile} ./cheetah.toml";
            buildPhase = "cheetah";
            installPhase = "mkdir -p $out && mv _build/pages/* $out/";
          };
      });
}
