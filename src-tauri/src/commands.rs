use std::path::{PathBuf};
use crate::OutputDir;
use crate::db::file_db;
use crate::methods::delta;
use std::{fs, vec};
use chrono::prelude::*;


#[derive(serde::Serialize)]
pub struct Response {
  data: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
  message: String,

}
#[derive(serde::Serialize)]
pub struct EventsResponse {
  base: Vec<String>,
  next: Vec<String>,
  delta: Vec<String>,
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
pub async fn initiate_delta(images: Vec<String>, directory_name: String) -> Result<Response, ErrorResponse> {
  // let id = Uuid::new_v4();
  let date = Local::now().format("%Y-%m-%d-%H-%M-%S");
  let base_path = format!("output/{date}");

  fs::create_dir_all(base_path.clone()).unwrap();

  let res = delta::delta_images(base_path,images, directory_name).await;

  Ok(Response { data: String::from("") })
}


#[tauri::command]
pub fn remote_image_from_selection(name_image: String) {

}

#[tauri::command]
pub fn get_events_images(images: Vec<String>, directory_name: String) -> Result<EventsResponse, ()> {
  println!("Retrieving events images!");

  Ok(EventsResponse { base: vec![], next: vec![], delta: vec![] })
}

pub fn set_output_dir(new_path: PathBuf, output_dir_state: OutputDir) {

  // let path = 

}
