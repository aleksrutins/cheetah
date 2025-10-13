# Cheetah
[![builds.sr.ht status](https://builds.sr.ht/~aleksrutins/cheetah.svg)](https://builds.sr.ht/~aleksrutins/cheetah?)
```
$ pixi x -c https://prefix.dev/cheetah cheetah
```

A very fast and simple static site generator. See [the website](https://cheetah.farthergate.com) for more info.

## Organization of this repository
This repository is organized into a number of crates:
- `cheetah` is the main SSG.
- `velociraptor` is a pure-HTML template engine.
- `expresso` is a crate for parsing and evaluating simple expressions.
