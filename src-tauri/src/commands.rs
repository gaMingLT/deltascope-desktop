use crate::db::app;
use crate::db::conn;
use crate::db::file_db;
use crate::db::tables::create_output_dir_table;
use crate::methods::delta;
use crate::tools::files::{read_diff_files, DiffFileInfo};
use crate::tools::mactime::MacTimeLine;
use crate::Settings;
use chrono::prelude::*;
use std::fs;
use std::path::PathBuf;
use tauri::{Manager, State, Window};

#[derive(Clone, serde::Serialize)]
struct OutputDirectoryPayload {
    path: String,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[derive(serde::Serialize)]
pub struct DeltaResponse {
    images: Vec<String>,
    directory_path: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    message: String,
}
#[derive(serde::Serialize)]
pub struct EventsResponse {
    base: Vec<MacTimeLine>,
    next: Vec<MacTimeLine>,
    delta: Vec<MacTimeLine>,
}

/// Adding available images path - using tauir file dialog
pub fn set_path_image(path: PathBuf) -> Result<(), ()> {
    log::info!("Adding new path image.");
    file_db::store_paths_file(path).unwrap();

    Ok(())
}

// Store used paths somewhere!
// Use in react
#[tauri::command]
pub fn get_stored_paths() -> Result<Vec<String>, ()> {
    log::info!("Getting stored paths.");
    Ok(file_db::get_stored_paths_file().unwrap())
}

// Initiate Delta - on selected paths
#[tauri::command]
pub async fn initiate_delta(
    images: Vec<String>,
    directory_name: String,
    window: Window,
    settings: State<'_, Settings>,
) -> Result<DeltaResponse, ErrorResponse> {
    log::info!("Formatting delta images!");
    let date = Local::now().format("%Y-%m-%d-%H-%M-%S");
    let base_path = format!("output/{date}");
    let use_wls = settings.1.lock().unwrap().clone();
    log::debug!("Use WLS {}", use_wls);

    fs::create_dir_all(base_path.clone()).unwrap();

    delta::delta_images(base_path.clone(), images.clone(), directory_name, use_wls)
        .await
        .unwrap();
    window
        .emit_all(
            "delta_finished",
            Payload {
                message: "Tauri is awesome!".into(),
            },
        )
        .unwrap();

    Ok(DeltaResponse {
        images: images,
        directory_path: base_path,
    })
}

// #[tauri::command]
// pub fn remote_image_from_selection(name_image: String) {

// }

#[tauri::command]
pub async fn get_events_images(
    images: Vec<String>,
    directoryPath: String,
) -> Result<EventsResponse, ErrorResponse> {
    log::info!("Getting evens from images");

    if images.is_empty() || directoryPath.is_empty() {
        return Err(ErrorResponse {
            message: "Supplied values are empty".to_string(),
        });
    }

    let res = delta::get_events_images(images, directoryPath)
        .await
        .unwrap();

    Ok(EventsResponse {
        base: res.0,
        next: res.1,
        delta: res.2,
    })
}

#[tauri::command]
pub async fn delete_available_images(images: Vec<String>) -> Result<(), ()> {
    log::info!("Deleting image path from available images.");
    file_db::delete_images(images);

    Ok(())
}

// #[tauri::command]
// pub fn get_output_dir(settings: State<'_, Settings>)  -> Result<String, ()> {
//   log::info!("Retrieving output directory path.");

//   Ok(settings.0.lock().unwrap().to_string())
// }

pub fn set_output_dir(
    new_path: PathBuf,
    window: Window,
    settings: State<'_, Settings>,
) -> Result<(), ()> {
    log::info!("Setting output directory.");

    log::debug!("Output directory state: {}", settings.0.lock().unwrap());
    log::debug!("Selecte path: {:?}", new_path);

    *settings.0.lock().unwrap() = new_path.to_str().unwrap().to_string();

    window.emit_all(
        "output-directory-set",
        OutputDirectoryPayload {
            path: settings.0.lock().unwrap().to_string(),
        },
    );

    Ok(())
}

// pub async fn set_use_wls(settings: State<'_, Settings>) {
//   let use_wls = settings.1;

//   settings.1 = !settings.1

// }

#[tauri::command]
pub fn get_different_files(directoryPath: String) -> Result<Vec<DiffFileInfo>, ()> {
    log::info!("Getting different files content");

    match read_diff_files(directoryPath) {
        Ok(e) => {
          log::debug!("Different files: {:?}", e.len());
          Ok(e)
        },
        Err(_) => Err(()),
    }
}
