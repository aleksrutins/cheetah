<extends template="layouts/index.html" pagetitle="Deployment"></extends>

# Deployment

> **Note:** This guide assumes that you've configured your site as a flake.

## Managing your site with Pixi
Managing your site with Pixi is an easy way to make sure you have everything you need to build it. Here's a starter `pixi.toml`, based on the one used to build this documentation:
```ini
[workspace]
authors = ["John Doe <doej@company.corp>"]
channels = ["conda-forge", "https://prefix.dev/cheetah"]
name = "site"
platforms = ["linux-64"]
version = "0.1.0"

[tasks]
build = "cheetah"
dev = "cheetah dev"

[dependencies]
cheetah = ">=0.2.3,<0.3"
```

## On SourceHut Pages
Here's a sample `.build.yml` for SourceHut Builds to deploy to Pages:
```yml
image: archlinux
packages:
  - hut
  - pixi

oauth: pages.sr.ht/PAGES:RW
environment:
  repo: mysite
  site: my.site.com

tasks:
  - build: |
      cd $repo
      # If your project is managed with Pixi, replace this with `pixi run dev`.
      pixi x -c https://prefix.dev/cheetah cheetah
  - package: |
      cd $repo/_build/pages
      tar -cvz . > ../../../site.tar.gz
  - upload: |
      hut pages publish -d $site site.tar.gz
```

> *Huh? Isn't it on the AUR? Why are you installing it from Pixi, even on Arch?* On Linux, the Prefix.dev repository provides prebuilt packages - installing from the AUR requires building Cheetah from source, which can take several minutes and a decent bit of CPU. If at all possible, use the Conda package.

## On GitHub Pages
To deploy your site on GitHub Pages, set your settings to deploy from an Action and tweak this workflow to your liking:

```yml
name: Deploy site to Pages

on:
  push:
    branches: ["master"]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: "pages"
  cancel-in-progress: true

jobs:
  deploy:
    environment:
      name: github-pages
      url: $!{{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    steps:
      - uses: prefix-dev/setup-pixi@v0.9.1
        with:
          pixi-version: v0.49.0

          cache: true
          auth-host: prefix.dev
          auth-token: ${{ secrets.PREFIX_DEV_TOKEN }}

      # Replace the next line with just `pixi run build` if you've set up a Pixi configuration
      - run: pixi x -c https://prefix.dev/cheetah cheetah

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v1
        with:
          path: './result'
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v1
```
