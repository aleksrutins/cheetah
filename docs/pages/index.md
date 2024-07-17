<extends template="layouts/index.html" pagetitle="Home"></extends>

# Cheetah
Cheetah is a scalable static site generator. For simple sites, it generates pure HTML. For more complicated sites and apps, you can add JavaScript either through regular scripts or through [components](components.html).

[![FlakeHub](https://img.shields.io/endpoint?url=https://flakehub.com/f/aleksrutins/cheetah/badge)](https://flakehub.com/flake/aleksrutins/cheetah)
![build](https://github.com/aleksrutins/cheetah/actions/workflows/build_nix.yml/badge.svg)


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
      packages =
        let pkgs = (import nixpkgs) { inherit system; };
        in rec {
          default = (cheetah.buildSite.${system} ./. {
            name = "site";
            inherit config;
          });

          container = cheetah.createContainer.${system} {
            inherit pkgs;
            site = default;
            options = {
              name = "site-container";
              inherit config;
            };
          };
        };

      devShells.default = (cheetah.createDevShell.${system} { inherit config; });
    });
}

```

To build your site, just use `nix build .` - see `workflows/docs.yml` [on GitHub](https://github.com/aleksrutins/cheetah) for an example of how to use this in CI.

> **Important Note:** If you choose this route (which I hope you do), I recommend adding `_build` and `cheetah.toml` to your `.gitignore` if you plan to use the provided dev shell. I also recommend setting up [direnv](https://direnv.net/) with `use flake` as general advice for any Nix Flake project.

### Normal Usage
Alternatively, you can use it as a normal binary.

| Repository | Package |
|------------|---------|
| Nix Flakes | `github:aleksrutins/cheetah` |
| [AUR](https://aur.archlinux.org/packages/cheetah) | `cheetah` |
| [Crates.io](https://crates.io/crates/cheetah) | `cheetah` |

Now, move on to [Getting Started](/getting-started.html).
