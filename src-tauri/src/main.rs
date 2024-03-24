// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod events;
mod global_vars;
mod services;
mod startup;
mod utils;

use services::theme_service::get_infos_service;
use startup::startup;
use tauri::{generate_handler, App};

fn main() {
    tauri::Builder::default()
        .setup(|app: &mut App| {
            startup(app);
            Ok(())
        })
        .invoke_handler(generate_handler![get_infos_service])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
