// main.rs

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// Custom command that handles form submission data and the corresponding file operations
// labs: array of 5 bools that represents if the corresponding lab is being worked on
// src_path: string representing the file path to copy from
// dst_path: string representing the file path to copy to
// Note: there's no return type needed
#[tauri::command]
fn handle_form_submit(arr_labs: [bool; 5], src_path: &str, dst_path: &str) {
  //TODO: fill this in
}

// A handler must be generated for every custom command
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![handle_form_submit])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
