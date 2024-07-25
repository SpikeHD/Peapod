use include_flate::flate;

flate!(pub static TOPBAR: str from "../src/html/topbar.html");
flate!(pub static EXTRA_CSS: str from "../src/css/extra.css");

#[tauri::command]
pub fn get_topbar() -> String {
  TOPBAR.to_string()
}

#[tauri::command]
pub fn get_extra_css() -> String {
  EXTRA_CSS.to_string()
}