// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cbz;
mod util;

use cbz::{InfoYAML, CBZ};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cbz_info,
            cbz_pages,
            cbz_page,
            cbz_page_name,
            cbz_page_images,
            cbz_thumbnail
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_fs_extra::init())
        .plugin(tauri_plugin_fs_watch::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn cbz_info(file_location: String) -> InfoYAML {
    let cbz = CBZ::new(file_location);
    cbz.info
}

#[tauri::command]
async fn cbz_pages(file_location: String) -> Vec<String> {
    let cbz = CBZ::new(file_location);
    cbz.pages
}

#[tauri::command]
async fn cbz_page(file_location: String, page: usize) -> String {
    let mut cbz = CBZ::new(file_location);
    cbz.get_image_by_page(page)
}

#[tauri::command]
async fn cbz_page_name(file_location: String, page: String) -> String {
    let mut cbz = CBZ::new(file_location);
    cbz.get_image_by_page_name(page)
}

#[tauri::command]
async fn cbz_page_images(file_location: String) -> Vec<String> {
    let mut cbz = CBZ::new(file_location);
    cbz.get_pages_images()
}

#[tauri::command]
async fn cbz_thumbnail(file_location: String) -> String {
    let mut cbz = CBZ::new(file_location);
    cbz.get_thumbnail()
}
