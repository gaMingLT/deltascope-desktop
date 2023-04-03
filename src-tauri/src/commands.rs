use std::path::{PathBuf};
use crate::db::file_db;
use crate::methods::delta;
use std::fs;
use chrono::prelude::*;


#[derive(serde::Serialize)]
pub struct Response {
  data: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
  message: String,
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
pub async fn initiate_delta(images: Vec<String>, directoryName: String) -> Result<Response, ErrorResponse> {
  // let id = Uuid::new_v4();
  let date = Local::now().format("%Y-%m-%d-%H-%M-%S");
  let base_path = format!("output/{date}");

  fs::create_dir_all(base_path.clone()).unwrap();

  let res = delta::delta_images(base_path,images, directoryName).await;

  Ok(Response { data: String::from("") })
}
