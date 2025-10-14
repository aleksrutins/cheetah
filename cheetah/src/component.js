/*
 * component.js
 * part of the Cheetah static site generator (https://cheetah.farthergate.com)
 * copyright (C) 2025 Aleks Rūtiņš <aleks@rutins.com> under the MIT License
 */

export function registerComponent(name, scripts) {
  let component = class extends HTMLElement {
    constructor() {
      super();
    }
    connectedCallback() {
      if (this.isConnected) {
        for (let script of scripts) {
          script.bind(this)();
        }
      }
    }
  };
  customElements.define(name, component);
}
