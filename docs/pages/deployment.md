<extends template="layouts/index.html"></extends>

# Deployment

> **Note:** This guide assumes that you've configured your site as a flake.

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
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: DeterminateSystems/nix-installer-action@v9
      - uses: DeterminateSystems/magic-nix-cache-action@v2
      - uses: actions/configure-pages@v2
      - name: Build Site
        run: 'nix build .'
      - name: Upload artifact
        uses: actions/upload-pages-artifact@v1
        with:
          path: './result'
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v1
```

## In a Container
The [starting flake configuration](/) contains a `container` output. This uses [Packsnap](https://github.com/aleksrutins/packsnap)'s custom plan builder to build a very small container with only your site and the [Caddy](https://caddyserver.com/) web server. To build it and make Docker aware of it, just run:

```sh
nix build '.#container'
docker load < result
```

> **Note:** The image will only work if built on Linux.

This built image can then be pushed to a registry and deployed anywhere. The webserver runs on container port 80.

### Deploying on Railway
To deploy a Hyena site on [Railway](https://railway.app), as an example, you can [use a GitHub Action to build a container and publish it to GHCR](https://docs.github.com/en/packages/managing-github-packages-using-github-actions-workflows/publishing-and-installing-a-package-with-github-actions), and then you can use Railway's [OCI image deployment](https://docs.railway.app/guides/services#deploying-from-a-docker-image) functionality to deploy it.

If you need help, ping me (@aleks) on the Railway Discord server.