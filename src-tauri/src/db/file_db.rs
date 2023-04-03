use std::io::{BufRead, Write, BufReader };
use std::path::{PathBuf};
use std::fs::File;
use std::fs::OpenOptions;

pub fn get_stored_paths_file() -> Result<Vec<String>, ()>  {
  let mut f = File::open("paths.txt").unwrap();
  let lines = BufReader::new(f).lines();

  let stored_paths = lines
    .into_iter()
    .map(|l| l.unwrap() )
    .collect::<Vec<String>>();

  Ok(stored_paths)
}

pub fn store_paths_file(path: PathBuf) -> Result<(),()> {
  // let mut stored_paths = get_stored_paths_file().unwrap();
  let mut f = OpenOptions::new().append(true).open("paths.txt");

  let line = format!("{}", path.into_os_string().into_string().unwrap());
  // stored_paths.push(line);

  // for path in stored_paths.into_iter() {
  writeln!(f.as_ref().unwrap(), "{}", line).unwrap();
  // }

  Ok(())
}

pub fn get_path_image_from_name(name: String) -> Result<String, ()> {
  let stored_paths = get_stored_paths_file().unwrap();

  let image_path = stored_paths.into_iter().filter(|f| f.contains(&name)).collect::<String>();
 
  Ok(image_path)
}


// TODO: Add way to delete selecting file paths!
