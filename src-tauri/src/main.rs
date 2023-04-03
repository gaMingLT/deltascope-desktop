#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use tauri::api::dialog;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

mod commands;
mod db;
mod methods;
mod tools;


fn main() {
    // TODO: Add different action - for example:
    // - select image
    // - select fls body?
    // - select mactimeline file?

    let open = CustomMenuItem::new("select".to_string(), "Select Image Path");
    let fileMenu = Submenu::new("File", Menu::new().add_item(open));
    let menu = Menu::new()
        .add_submenu(fileMenu)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit);

    let mut stored_paths: Vec<PathBuf> = Vec::new();


    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "select" => {
                dialog::FileDialogBuilder::default()
                    .pick_file(|path_buf| match path_buf {
                        Some(p) => commands::set_path_image(p).unwrap(),
                        _ => {}
                    });
            }
            _ => {}
        })
          .invoke_handler(tauri::generate_handler![commands::get_stored_paths, commands::initiate_delta])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
