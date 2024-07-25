import { waitForElm } from './util/wait'
import { removeBanner } from './util/cleanup'

window.addEventListener('DOMContentLoaded', () => {
  waitForElm('#cap').then(removeBanner).catch(console.error)
  waitForElm('.pgAdWrapper').then(elm => {
    // Remove the parent
    elm.parentElement?.remove()
  })
})