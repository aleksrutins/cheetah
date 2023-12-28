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
    cheetah.url = "github:aleksrutins/cheetah";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, utils, cheetah }:
    let config = {
      # Pass your configuration options here.
    };
    in utils.lib.eachDefaultSystem (system: {
      packages.default = (cheetah.buildSite.${system} ./. {
        name = "site";
        inherit config;
      });

      devShells.default = (cheetah.createDevShell.${system} { inherit config; });
    });
}

```

To build your site, just use `nix build .` - see <workflows/docs.yml> for an example of how to use this in CI.

### Normal Usage
Alternatively, you can use it as a normal binary.

Either install it as a flake using Nix (recommended):
```sh
nix profile install github:aleksrutins/cheetah
```

Or install it from [Cargo](https://crates.io/crates/cheetah):
```shell
cargo install cheetah
```

## Usage
See [the website](https://cheetah.farthergate.com).
