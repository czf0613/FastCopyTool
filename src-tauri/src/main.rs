// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod fs_utils;
mod sys_utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            sys_utils::get_sys_info,
            sys_utils::get_4k_write_speed,
            sys_utils::get_4k_read_speed,
            sys_utils::get_read_delay,
            sys_utils::get_write_delay,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
