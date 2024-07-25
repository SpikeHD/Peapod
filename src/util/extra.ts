export async function applyExtraCss() {
  const { invoke } = window.__TAURI__.core
  const css = await invoke('get_extra_css')

  const style = document.createElement('style')
  style.textContent = css
  style.id = 'extra-css'
  document.head.appendChild(style)
}