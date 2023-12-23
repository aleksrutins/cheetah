<extends template="layouts/index.html"></extends>

<style>
    div.component-sidebyside {
        display: flex;
        gap: 1em;
        width: inherit;
        container-type: size;
        height: 300px;
        margin: 1em 0;
    }

    div.component-sidebyside pre, div.component-sidebyside div {
        flex: 1;
    }

    div.component-sidebyside > pre {
        container-type: size;
        height: 100cqh;
    }

    div.component-sidebyside > pre > code.hljs {
        width: calc(100cqw - 2em);
        overflow: auto auto;
        overflow-x: auto;
        height: calc(100cqh - 3em);
    }

    div.component-sidebyside div {
        height: min-content;
        overflow: auto;
    }
</style>

# Components
Components are the basic building block of a Hyena site, and
enable more complex behaviors through client-side JavaScript.
For instance, here's a counter:

<x-counter></x-counter>

To make a component, make a `component-name.html` file under
your site's `components/` directory. For instance, to build a
site header, you could put it in the `components/site-header.html`
file.

The simplest component contains only HTML, and maybe CSS; for instance, a basic
site header might look like this:

<div class="component-sidebyside">

```html
<!-- components/site-header.html -->
<header>
    <h1>My Site</h1>
    <p>The best content, ever, anywhere.</p>
</header>

<style>
    header {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>

<!-- pages/index.html -->
<site-header></site-header>
```
<div>
<components-tutorial-site-header></components-tutorial-site-header>
</div>

</div>

Hyena uses the magic of [declarative shadow DOM](https://developer.chrome.com/docs/css-ui/declarative-shadow-dom)
to make components work, so styles, scripts, and even IDs
are scoped to the component.

Speaking of scripts, to make more complicated components
(like the counter above), you'll want some JavaScript.
For this, just put a `<script>` tag in your component.
This script will be converted by Hyena into its own separate
file, with your code in a function which is called with the [custom element](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements)
that your component is hydrated with as `this`.

> **Important note:** If your component has no scripts, it will not be hydrated,
> and no scripts will be loaded. No need to worry about bloat!

To make the counter component shown above, just write vanilla JavaScript:

<div class="component-sidebyside">

```html
<!-- components/x-counter.html -->
<button id="increment">+</button>
<span id="count">0</span>
<button id="decrement">-</button>

<style>
    :host {
        display: block;
        box-sizing: content-box;
        padding: 5px;
        padding-right: 8px;
        border-radius: 10px;

        width: min-content;

        background-color: #f6f6f7;
    }

    button, span {
        display: block;
        text-align: center;
        width: 100%;
        margin: 2px;
    }

    button {
        border-radius: 5px;
        border: none;
        background-color: #299c63;
        color: white;
        padding: 5px;

        padding-left: 20px;
        padding-right: 20px;

        font-size: medium;

        &:hover {
            background-color: #299f68;
        }
        &:active {
            background-color: #238554;
        }
    }
</style>

<script>
    const decrement = this.shadowRoot.querySelector("#decrement");
    const increment = this.shadowRoot.querySelector("#increment");

    const count = this.shadowRoot.querySelector("#count");

    increment.addEventListener('click', () => count.textContent = parseInt(count.textContent) + 1);
    decrement.addEventListener('click', () => count.textContent = parseInt(count.textContent) - 1);
</script>

<!-- pages/index.html -->
<x-counter></x-counter>
```

<div><x-counter></x-counter></div>

</div>

As you can see, to write a component script, just write as you usually would in a `componentDidMount()` method.