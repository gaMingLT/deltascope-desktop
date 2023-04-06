use std::io::{BufRead, Write, BufReader };
use std::path::{PathBuf, self, Path};
use std::{fs, vec};
use std::fs::File;
use std::fs::OpenOptions;

pub fn get_stored_paths_file() -> Result<Vec<String>, ()>  {
  if !Path::new("paths.txt").exists() {
    let empty: Vec<String> = vec![];
    return Ok(empty)
  }
  let mut f = File::open("paths.txt").unwrap();
  let lines = BufReader::new(f).lines();

  let stored_paths = lines
    .into_iter()
    .map(|l| l.unwrap() )
    .collect::<Vec<String>>();

  Ok(stored_paths)
}

pub fn store_paths_file(image_path: PathBuf) -> Result<(),()> {
  // let mut stored_paths = get_stored_paths_file().unwrap();
  let mut f = OpenOptions::new().create(true).append(true).open("paths.txt");

  let line = format!("{}", image_path.into_os_string().into_string().unwrap());
  writeln!(f.as_ref().unwrap(), "{}", line).unwrap();

  Ok(())
}

pub fn get_path_image_from_name(name: String) -> Result<String, ()> {
  let stored_paths = get_stored_paths_file().unwrap();

  let image_path = stored_paths.into_iter().filter(|f| f.contains(&name)).collect::<String>();
 
  Ok(image_path)
}


// TODO: Add way to delete selecting file paths!

pub fn delete_images(images: Vec<String>) {
  let stored_paths = get_stored_paths_file().unwrap();
  let mut new_stored_paths = vec![];

  for path in stored_paths {
    for image in images.clone() {
      if path != image {
        new_stored_paths.push(path.clone());
      }
    }
  }

  fs::remove_file("paths.txt").unwrap();
  for path in new_stored_paths {
    store_paths_file(PathBuf::from(path)).unwrap();
  }

}
