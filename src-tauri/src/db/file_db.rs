use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::{fs, vec};

pub fn get_stored_paths_file() -> Result<Vec<String>, ()> {
    log::info!("Getting stored paths form file");
    if !Path::new("paths.txt").exists() {
        let empty: Vec<String> = vec![];
        return Ok(empty);
    }
    let f = File::open("paths.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let stored_paths = lines
        .into_iter()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    Ok(stored_paths)
}

pub fn store_paths_file(image_path: PathBuf) -> Result<(), ()> {
    log::info!("Storing new paths in file.");
    let f = OpenOptions::new()
        .create(true)
        .append(true)
        .open("paths.txt");

    let line = format!("{}", image_path.into_os_string().into_string().unwrap());
    writeln!(f.as_ref().unwrap(), "{}", line).unwrap();

    Ok(())
}

pub fn get_path_image_from_name(name: String) -> Result<String, ()> {
    log::info!("Retrieving path from image name.");
    let stored_paths = get_stored_paths_file().unwrap();

    let image_path = stored_paths
        .into_iter()
        .filter(|f| f.contains(&name))
        .collect::<String>();

    Ok(image_path)
}

// TODO: Add way to delete selecting file paths!
pub fn delete_images(images: Vec<String>) {
    log::info!("Deleting images paths from file storage");
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
