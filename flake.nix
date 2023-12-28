{
  description = "A fast and flexible SSG.";
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    packsnap.url = "github:aleksrutins/packsnap";
  };

  outputs = { self, nixpkgs, utils, naersk, packsnap }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };

        genConfig = pkgs: options:
          let configFormat = pkgs.formats.toml { };
          in configFormat.generate "cheetah.toml" options;
        
      in rec {
        cheetah = naersk-lib.buildPackage ./.;
        defaultPackage = cheetah;
        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };

        buildSite = src: options:
          let configFile = genConfig pkgs options.config;
          in pkgs.stdenv.mkDerivation {
            inherit (options) name;
            inherit src;
            nativeBuildInputs = [cheetah];
            configurePhase = "cp ${configFile} ./cheetah.toml";
            buildPhase = "cheetah";
            installPhase = "mkdir -p $out && mv _build/pages/* $out/";
          };
        
        createDevShell = options:
          let configFile = genConfig pkgs options.config;
          in pkgs.mkShell {
            nativeBuildInputs = [cheetah];
            shellHook = "cp ${configFile} ./cheetah.toml";
          };
        
        createContainer = {pkgs, site, options}:
          packsnap.lib.${system}.buildCustomPlan {
            inherit (options) name;
            contents = [
              pkgs.caddy
              site
            ];
            start = "caddy file-server --root ${site}";
          };
      });
}
