import Alpine from 'alpinejs'
import htmx from 'htmx.org';

window.Alpine = Alpine

document.body.addEventListener('htmx:afterSwap', (e) => {
  Alpine.initTree(e.detail.target)
})

Alpine.start()
