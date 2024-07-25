#[tauri::command]
pub fn close(win: tauri::WebviewWindow) {
  win.close().unwrap();
}

#[tauri::command]
pub fn minimize(win: tauri::WebviewWindow) {
  win.minimize().unwrap();
}

#[tauri::command]
pub fn toggle_maximize(win: tauri::WebviewWindow) {
  if win.is_maximized().unwrap_or_default() {
    win.unmaximize().unwrap_or_default();
  } else {
    win.maximize().unwrap_or_default();
  }
}

#[tauri::command]
pub fn disable_decorations(win: tauri::WebviewWindow) {
  win.set_decorations(false).unwrap();
}