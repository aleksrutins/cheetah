{
  inputs = {
    cheetah.url = "../.";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, cheetah }:
    let config = {
      always_hydrate = true;
    };
    in utils.lib.eachDefaultSystem (system: {
      packages =
        let pkgs = (import nixpkgs) { inherit system; };
        in rec {
          default = (cheetah.buildSite.${system} ./. {
            name = "cheetah-docs";
            inherit config;
          });

          container = cheetah.createContainer.${system} {
            inherit pkgs;
            site = default;
            options = {
              name = "cheetah-docs";
              inherit config;
            };
          };
        };

      devShells.default = (cheetah.createDevShell.${system} { inherit config; });
    });
}