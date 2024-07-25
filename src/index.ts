import { waitForElm } from './util/wait'
import { removeBanner } from './util/cleanup'
import { addTopbarEvents, appendTopbar, appendVersion, setMaximizeIcon } from './control'
import { applyExtraCss } from './util/extra'

window.addEventListener('DOMContentLoaded', async () => {
  const { invoke } = window.__TAURI__.core

  // If we are in an iframe, abort!
  if (window !== window.parent) return

  waitForElm('#cap').then(removeBanner).catch(console.error)

  if (await invoke('get_platform') !== 'macos') {
    appendTopbar().then(async () => {
      const { invoke } = window.__TAURI__.core
      await addTopbarEvents()
      await appendVersion()
  
      // Assuming this has loaded properly, remove the original titlebar
      await invoke('disable_decorations')
    }).catch(console.error)
  }

  // Apply extra CSS
  applyExtraCss().catch(console.error)

  setMaximizeIcon()
})
