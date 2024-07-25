export async function appendTopbar() {
  const { invoke } = window.__TAURI__.core;
  const topbar = await invoke('get_topbar')

  // Append directly to the top of the body
  document.body.insertAdjacentHTML('afterbegin', topbar)
}

export function addTopbarEvents() {
  const { invoke } = window.__TAURI__.core;
  const close = document.querySelector('#topclose');
  const minimize = document.querySelector('#topmin');
  const maximize = document.querySelector('#topmax');

  close?.addEventListener('click', () => {
    invoke('close');
  });

  minimize?.addEventListener('click', () => {
    invoke('minimize');
  });

  maximize?.addEventListener('click', () => {
    invoke('maximize');
  });
}

export async function appendVersion() {
  const { getVersion } = window.__TAURI__.app
  const version = await getVersion()
  const versionElm = document.querySelector('#peapod-version')

  if (!versionElm) return

  versionElm.textContent = `Peapod v${version}`
}