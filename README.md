# Cheetah

> A static site generator written in Rust.

[![FlakeHub](https://img.shields.io/endpoint?url=https://flakehub.com/f/aleksrutins/cheetah/badge)](https://flakehub.com/flake/aleksrutins/cheetah)

## Installation

### Sites as Flakes
The recommended way to install Cheetah is by creating a `flake.nix` to build your site.

Here's a simple one to get you started (based on the one used for Cheetah's docs):

```nix
{
  inputs = {
    utils.url = "github:numtide/flake-utils";

    # This pulls the latest tag from FlakeHub; if you want to live on the edge, you can
    # also use "github:aleksrutins/cheetah", but be warned that it is not necessarily stable.
    cheetah.url = "https://flakehub.com/f/aleksrutins/cheetah/*.tar.gz";
  };

  outputs = { self, utils, cheetah }:
    utils.lib.eachDefaultSystem (system: {
      packages.site = (cheetah.buildSite.${system} ./. {
        name = "site"; # The name of the resulting Nix derivation for the site; this doesn't really matter.

        config = {
          # Put Cheetah configuration options here; see the website for more details.
        };
      });
    });
}

```

To build your site, just use `nix build .#site` - see <workflows/docs.yml> for an example of how to use this in CI.

### Normal Usage
Alternatively, you can use it as a normal binary.

Either install it as a flake using Nix (recommended):
```sh
# using a tagged release:
nix profile install 'https://flakehub.com/f/aleksrutins/cheetah/*.tar.gz'
# or, to live on the edge:
nix profile install github:aleksrutins/cheetah
```

Or install it from [Cargo](https://crates.io/crates/cheetah):
```shell
cargo install cheetah
```

## Usage
See [the website](https://cheetah.farthergate.com).
