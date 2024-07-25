// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use include_flate::flate;
use tauri::{WebviewUrl, WebviewWindowBuilder};
use utils::paths::get_webdata_dir;

mod utils;
mod window;

// Injected JS
flate!(pub static INJECTION: str from "../src/inject.min.js");

fn main() {
  let context = tauri::generate_context!("tauri.conf.json");
  let parsed_url = reqwest::Url::parse(&"https://www.photopea.com".to_string()).unwrap();
  let url_ext = WebviewUrl::External(parsed_url);

  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![
      window::events::close,
      window::events::minimize,
      window::events::toggle_maximize,
      window::events::disable_decorations,
      window::html::get_topbar,
      window::html::get_extra_css,
      utils::platform::get_platform,
    ])
    .on_window_event(|_window, event| match event {
      tauri::WindowEvent::Resized { .. } => {
        // Sleep for a millisecond (blocks the thread but it doesn't really matter)
        // https://github.com/tauri-apps/tauri/issues/6322#issuecomment-1448141495
        std::thread::sleep(Duration::from_millis(1));
      }
      _ => {}
    })
    .setup(move |app: &mut tauri::App| {
      // First, grab preload plugins
      let title = format!("Peapod - v{}", app.package_info().version);
      let win = WebviewWindowBuilder::new(app, "main", url_ext)
        .title(title.as_str())
        .initialization_script(&INJECTION)
        .resizable(true)
        .min_inner_size(100.0, 100.0)
        .disable_drag_drop_handler()
        .data_directory(get_webdata_dir())
        // Prevent flickering by starting hidden, and show later
        .visible(false)
        .decorations(true)
        .shadow(true)
        .build()?;

      win.show()?;
      win.maximize()?;

      Ok(())
    })
    .run(context)
    .expect("error while running tauri application");
}
