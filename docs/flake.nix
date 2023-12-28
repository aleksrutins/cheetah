{
  inputs = {
    cheetah.url = "../.";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, utils, cheetah }:
    utils.lib.eachDefaultSystem (system: {
      packages.site = (cheetah.buildSite.${system} ./. {
        name = "cheetah-docs";

        config = {
          always_hydrate = true;
        };
      });
    });
}