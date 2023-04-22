#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use futures::FutureExt;
use tauri::api::dialog;
use std::{collections::HashMap, sync::Mutex};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, Manager};

mod commands;
mod db;
mod methods;
mod tools;

use simple_logger::SimpleLogger;

pub struct Settings(Mutex<String>, Mutex<bool>);

fn main() {
    let select_image_path =
        CustomMenuItem::new("select-image-path".to_string(), "Select Image Path");
    let set_output_dir = CustomMenuItem::new("set-output-dir".to_string(), "Set Output Directory");

    let file_menu = Submenu::new(
        "File",
        Menu::new()
            .add_item(select_image_path)
            .add_item(set_output_dir),
    );
    let menu = Menu::new()
        .add_submenu(file_menu)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit);

    SimpleLogger::new().env().init().unwrap();

    log::warn!("This is an example message.");

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "select-image-path" => {
                dialog::FileDialogBuilder::default().pick_file(|path_buf| match path_buf {
                    Some(p) => commands::set_path_image(p).unwrap(),
                    _ => {}
                });
            },
            "set-output-dir" => {
                dialog::FileDialogBuilder::default().pick_folder(move |path_buf| match path_buf {
                        Some(p) => {
                            let app_handle = event.window().app_handle().clone();

                            commands::set_output_dir(p, event.window().clone(), app_handle.state::<Settings>());
                        } 
                        _ => {}
                    });
            },
            _ => {}
        })
        .manage(Settings(Default::default(), true.into()))
        .invoke_handler(tauri::generate_handler![
            commands::get_stored_paths,
            commands::initiate_delta,
            commands::get_events_images,
            commands::delete_available_images,
            commands::get_different_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
