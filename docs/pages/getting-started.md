<extends template="layouts/index.html" pagetitle="Getting Started"></extends>

# Getting Started

Let's make a page!

First, though, let's make a layout for it. In your project directory, add a `layouts/index.html` file:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My Amazing Website</title>

    <style>
        :root {
            font-family: system-ui, sans-serif;
        }
    </style>
</head>
<body>
    <header>
        <h1>My Website</h1>
    </header>
    <main>
        <slot></slot>
    </main>
</body>
</html>
```

That `<slot></slot>` is where our content will end up. The style in the `<head>` is just to make the website palatable; we won't be focusing much on styles in this tutorial.

Now, let's add the actual page. Add a `pages/index.md` file (Cheetah supports both HTML and Markdown for everything &mdash; layouts, pages, and components; Markdown support is provided by [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)):

```md
<extends template="layouts/index.html"></extends>

## Hello, World!
This is a page.
```
The `<extends>` tag tells Cheetah to plug the rendered page into our `layouts/index.html` template that we just created. Template paths are relative to the root of the project.

Now, let's try the dev server. If you chose to build your site as a flake, you're going to want to be in the dev shell for this (if you're not sure, go re-read the [introduction](/)):

```sh
cheetah dev
```

Now, open your browser to `localhost:3000`, and you should see about what you would expect.

## Adding Interactivity

What sets Cheetah apart from Zola, Hugo, or other static site generators (excluding Astro; Astro is in a class of its own, and you should absolutely use it for larger projects) is its support for prerendered interactive components. These components are pure vanilla JavaScript, and don't depend on any bloated UI libraries, making them extremely lightweight. To make one, just create an HTML file in `components`; for instance, a component at `components/x-counter.html` could be rendered as `<x-counter></x-counter>` in any other template in your site. 

Let's add a simple button to greet the user. Make a `components/x-greeter.html` file:

```html
<button id="greeter">Greet Me</button>

<script>
    this.shadowRoot.querySelector("#greeter").addEventListener("click", () => alert("Hello!"));
</script>
```

Now, add an `<x-greeter></x-greeter>` to your home page, and you should find an interactive button!

For a more in-depth tutorial, and some details on how components work, take a look at the [Components](/components.html) page.

## Deployment
You've finished the tutorial! Now it's time to [put your site on the Internet](/deployment.html).