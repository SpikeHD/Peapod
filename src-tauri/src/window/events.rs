#[tauri::command]
pub fn close(win: tauri::WebviewWindow) {
  win.close().unwrap();
}

#[tauri::command]
pub fn minimize(win: tauri::WebviewWindow) {
  win.minimize().unwrap();
}

#[tauri::command]
pub fn maximize(win: tauri::WebviewWindow) {
  win.maximize().unwrap();
}

#[tauri::command]
pub fn unmaximize(win: tauri::WebviewWindow) {
  win.unmaximize().unwrap();
}

#[tauri::command]
pub fn disable_decorations(win: tauri::WebviewWindow) {
  win.set_decorations(false).unwrap();
}