// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}


#[tauri::command]
fn get_all_env() -> Vec<(String, String)> {
    env::vars().collect()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, get_all_env])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
