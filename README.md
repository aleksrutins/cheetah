# Cheetah

> A static site generator written in Rust.

[![FlakeHub](https://img.shields.io/endpoint?url=https://flakehub.com/f/aleksrutins/cheetah/badge)](https://flakehub.com/flake/aleksrutins/cheetah)

## Installation

Install cheetah as a flake:
```
$ nix profile install github:aleksrutins/cheetah
```

Add cheetah to your `flake.nix`:

```nix
{
  inputs.cheetah.url = "https://flakehub.com/f/aleksrutins/cheetah/*.tar.gz";

  outputs = { self, cheetah }: {
    # Use in your outputs
  };
}

```

Or, install it from [Cargo](https://crates.io/crates/cheetah):
```shell
cargo install cheetah
```

## Usage
See [the website](https://cheetah.farthergate.com).