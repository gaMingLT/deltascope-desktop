use std::path::{PathBuf};
use crate::{OutputDir, tools::mactime::MacTimeLine};
use crate::db::file_db;
use crate::methods::delta;
use std::{fs, vec};
use chrono::prelude::*;


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
pub async fn initiate_delta(images: Vec<String>, directory_name: String) -> Result<DeltaResponse, ErrorResponse> {
  // let id = Uuid::new_v4();
  let date = Local::now().format("%Y-%m-%d-%H-%M-%S");
  let base_path = format!("output/{date}");

  fs::create_dir_all(base_path.clone()).unwrap();

  let res = delta::delta_images(base_path.clone(), images.clone(), directory_name).await;

  Ok(DeltaResponse { images: images, directory_path: base_path })
}


#[tauri::command]
pub fn remote_image_from_selection(name_image: String) {

}

#[tauri::command]
pub async fn get_events_images(images: Vec<String>, directoryPath: String) -> Result<EventsResponse, ErrorResponse> {
  println!("Retrieving events images!: {:?} & {:?} ", images, directoryPath);

  if images.is_empty() && directoryPath.is_empty() {
    return Err(ErrorResponse { message: "Supplied values are empty".to_string() })
  }

  let res = delta::get_events_images(images, directoryPath).await.unwrap();

  Ok(EventsResponse { base: res.0, next: res.1, delta: res.2 })
}


pub fn set_output_dir(new_path: PathBuf, output_dir_state: OutputDir) {

  // let path = 

}
