# Cheetah

> A static site generator written in Rust.

## Declarative Shadow DOM polyfill

```javascript
// https://web.dev/declarative-shadow-dom/#polyfill
(function attachShadowRoots(root) {
  root.querySelectorAll("template[shadowrootmode]").forEach(template => {
    const mode = template.getAttribute("shadowrootmode");
    const shadowRoot = template.parentNode.attachShadow({ mode });
    shadowRoot.appendChild(template.content);
    template.remove();
    attachShadowRoots(shadowRoot);
  });
})(document);
```
