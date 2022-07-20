# Hyena

> A static site generator written in Rust.

## Declarative Shadow DOM polyfill

```javascript
// https://web.dev/declarative-shadow-dom/#polyfill
window.onload = () => (function attachShadowRoots(root) {
  root.querySelectorAll("template[shadowroot]").forEach(template => {
    const mode = template.getAttribute("shadowroot");
    const shadowRoot = template.parentNode.attachShadow({ mode });
    shadowRoot.appendChild(template.content);
    template.remove();
    attachShadowRoots(shadowRoot);
  });
})(document);
```
