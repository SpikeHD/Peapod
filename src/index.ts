import { waitForElm } from './util/wait'
import { removeBanner } from './util/cleanup'
import { addTopbarEvents, appendTopbar, appendVersion } from './control'
import { applyExtraCss } from './util/extra'

window.addEventListener('DOMContentLoaded', () => {
  waitForElm('#cap').then(removeBanner).catch(console.error)
  waitForElm('.pgAdWrapper').then(elm => {
    // Remove the parent
    if (!elm.parentElement) return

    elm.parentElement.style.display = 'none'
  })

  appendTopbar().then(async () => {
    const { invoke } = window.__TAURI__.core
    await addTopbarEvents()
    await appendVersion()

    // Assuming this has loaded properly, remove the original titlebar
    await invoke('disable_decorations')
  }).catch(console.error)

  // Apply extra CSS
  applyExtraCss().catch(console.error)
})
