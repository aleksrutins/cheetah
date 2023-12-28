<extends template="layouts/index.html"></extends>

# Configuration

You can configure your site by passing a `config` hash to `cheetah.buildSite` (if using a flake) or using a `cheetah.toml` in your site's root.

## Available Options

- `always_hydrate` - Always include JavaScript to hydrate every component, interactive or not. See [Components](/components.html) for more details.