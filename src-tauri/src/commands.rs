use std::path::{PathBuf};
use crate::{ Settings};
use crate::{tools::mactime::MacTimeLine};
use crate::db::file_db;
use crate::db::conn;
use crate::db::tables::create_output_dir_table;
use crate::db::app;
use crate::methods::delta;
use std::{fs};
use chrono::prelude::*;
use tauri::{Window, Manager, State};

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
pub async fn initiate_delta(images: Vec<String>, directory_name: String, window: Window, settings: State<'_, Settings>) -> Result<DeltaResponse, ErrorResponse> {
  log::info!("Formatting delta images!");
  let date = Local::now().format("%Y-%m-%d-%H-%M-%S");
  let base_path = format!("output/{date}");

  fs::create_dir_all(base_path.clone()).unwrap();

  delta::delta_images(base_path.clone(), images.clone(), directory_name,  settings.1.lock().await.clone()).await.unwrap();
  window.emit_all("delta_finished", Payload { message: "Tauri is awesome!".into() }).unwrap();

  Ok(DeltaResponse { images: images, directory_path: base_path })
}


// #[tauri::command]
// pub fn remote_image_from_selection(name_image: String) {

// }

#[tauri::command]
pub async fn get_events_images(images: Vec<String>, directoryPath: String) -> Result<EventsResponse, ErrorResponse> {
  log::info!("Getting evens from images");
  // println!("Retrieving events images!: {:?} & {:?} ", images, directoryPath);

  if images.is_empty() || directoryPath.is_empty() {
    return Err(ErrorResponse { message: "Supplied values are empty".to_string() })
  }

  let res = delta::get_events_images(images, directoryPath).await.unwrap();

  Ok(EventsResponse { base: res.0, next: res.1, delta: res.2 })
}

#[tauri::command]
pub async fn delete_available_images(images: Vec<String>) -> Result<(), ()> {
  log::info!("Deleting image path from available images.");
  // println!("Deleting selected available images: {:?}", images);
  file_db::delete_images(images);

  Ok(())
}

#[tauri::command]
pub async fn get_output_dir() -> Result<(), ()> {
  log::info!("Retrieving output directory path.");
  // println!("Getting output directory path");

  let conn = conn::db_con_app().await.unwrap();
  app::get_output_path(conn).await.unwrap();

  Ok(())
}


pub async fn set_output_dir(new_path: PathBuf) -> Result<(), ()>  {
  // println!("Setting output directory");
  log::info!("Setting output directory.");

  let mut conn = conn::db_con_app().await.unwrap();
  conn = create_output_dir_table(conn.clone()).await.unwrap();

  app::update_path_output_dir(new_path.clone(), conn).await;

  Ok(())
}


// pub async fn set_use_wls(settings: State<'_, Settings>) {
//   let use_wls = settings.1;

//   settings.1 = !settings.1

// }
