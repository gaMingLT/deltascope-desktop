use std::path::{PathBuf};
use crate::{OutputDir, tools::mactime::MacTimeLine};
use crate::db::file_db;
use crate::db::conn;
use crate::db::tables::create_output_dir_table;
use crate::db::app;
use crate::methods::delta;
use std::{fs};
use chrono::prelude::*;
use tauri::{Window, Manager};

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

// Set path of image(s) - need 2 paths - on filesystem (no uploading)
pub fn set_path_image(path: PathBuf) -> Result<(), ()> {
  file_db::store_paths_file(path).unwrap();

  Ok(())
}


// Store used paths somewhere!
// Use in react
#[tauri::command]
pub fn get_stored_paths() -> Result<Vec<String>, ()> {
  Ok(file_db::get_stored_paths_file().unwrap())
}


// Initiate Delta - on selected paths
#[tauri::command]
pub async fn initiate_delta(images: Vec<String>, directory_name: String, window: Window) -> Result<DeltaResponse, ErrorResponse> {
  // let id = Uuid::new_v4();
  let date = Local::now().format("%Y-%m-%d-%H-%M-%S");
  let base_path = format!("output/{date}");

  fs::create_dir_all(base_path.clone()).unwrap();

  let res = delta::delta_images(base_path.clone(), images.clone(), directory_name).await;

window.emit_all("delta_finished", Payload { message: "Tauri is awesome!".into() }).unwrap();

  Ok(DeltaResponse { images: images, directory_path: base_path })
}


#[tauri::command]
pub fn remote_image_from_selection(name_image: String) {

}

#[tauri::command]
pub async fn get_events_images(images: Vec<String>, directoryPath: String) -> Result<EventsResponse, ErrorResponse> {
  println!("Retrieving events images!: {:?} & {:?} ", images, directoryPath);

  if images.is_empty() || directoryPath.is_empty() {
    return Err(ErrorResponse { message: "Supplied values are empty".to_string() })
  }

  let res = delta::get_events_images(images, directoryPath).await.unwrap();

  Ok(EventsResponse { base: res.0, next: res.1, delta: res.2 })
}

#[tauri::command]
pub async fn delete_available_images(images: Vec<String>) -> Result<(), ()> {
  println!("Deleting selected available images: {:?}", images);
  file_db::delete_images(images);

  Ok(())
}

#[tauri::command]
pub async fn get_output_dir() -> Result<(), ()> {
  println!("Getting output directory path");

  let conn = conn::db_con_app().await.unwrap();
  app::get_output_path(conn).await;

  Ok(())
}


pub async fn set_output_dir(new_path: PathBuf) -> Result<(), ()>  {
  println!("Setting output directory");

  let mut conn = conn::db_con_app().await.unwrap();
  conn = create_output_dir_table(conn.clone()).await.unwrap();

  app::update_path_output_dir(new_path.clone(), conn).await;

  Ok(())
}
