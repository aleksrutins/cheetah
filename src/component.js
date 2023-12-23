/*
 * component.js
 * part of the Cheetah static site generator (https://Cheetah.farthergate.com)
 * copyright (C) 2023 Aleks Rūtiņš <aleks@rutins.com> under the MIT License
 */

function parseFragment(content) {
  let template = document.createElement('template');
  template.innerHTML = content;
  return template.content;
}

export function registerComponent(name, template, scripts) {
  let component = class extends HTMLElement {
    constructor() {
      super();
    }
    connectedCallback() {
      if(this.isConnected) {
        if(this.shadowRoot == null) { // Not prerendered or no declarative shadow DOM
          let mode = 'open';
          let prerendered = false;
          let newTemplate = this.querySelector(":scope > template[shadowroot]");
          if(newTemplate != null) { // Prerendered, but no declarative shadow DOM
            template = newTemplate.content;
            mode = newTemplate.getAttribute('shadowroot');
            prerendered = true;
          }
          let shadow = this.attachShadow({ mode });
          shadow.appendChild(template instanceof String? parseFragment(template) : template.cloneNode(true));
          if(!prerendered) {
            /**
             * @param {Node} el 
             */
            let parseRecursive = (el) => {
              if(el.nodeType = el.TEXT_NODE) {
                el.textContent = el.textContent.replace(/\{\{\s*(\w*)\s*\}\}/g, (_, attr) => this.getAttribute(attr));
              } else if(el.nodeType == el.ELEMENT_NODE) {
                for(let attr of el.attributes) if(attr.name.startsWith('bind')) {
                  attr.value = this.getAttribute(attr.value);
                }
              }
              for(let child of el.children) {
                parseRecursive(child);
              }
            };
            parseRecursive(shadow);
          }
        }
        for(let script of scripts) {
          script.bind(this)();
        }
      }
    }
  }
  customElements.define(name, component);
}
