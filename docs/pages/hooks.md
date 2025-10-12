<extends template="layouts/index.html" pagetitle="Hooks"></extends>

# Hooks

Hooks are Cheetah's implementation of extensibility. They allow you to define commands in `cheetah.toml` to be run before or after the build.

A hook to run TailwindCSS whenever HTML or Markdown files are changed, for instance, might look like this:
```toml
[[hooks]]
name = "tailwind"
during = "PreBuild"
dev = { Watch = ["**/*.{html,md}"] }
command = "tailwindcss -i tailwind.css -o assets/tailwind.css"
```

- `name` is the name of the hook, printed when it's run.
- `during` can be either `PreBuild` or `PostBuild`.
- `dev` can be either `"Disabled"` or (as above) an object with a `Watch` property containing a list of globs to watch during development. If `Disabled`, this hook will not run in development.
- `command` is the command to run.
