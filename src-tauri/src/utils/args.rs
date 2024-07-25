#[tauri::command]
pub fn is_gpu_disabled() -> bool {
  let args: Vec<String> = std::env::args().collect();
  args.iter().any(|arg| arg == "--disable-gpu")
}