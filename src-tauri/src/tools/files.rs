use crate::{db::file_db::get_path_image_from_name, methods::delta::MappingFiles};
use serde::Serialize;
use similar::{ChangeTag, TextDiff};
use std::fs::File;
use std::io::Write;
use std::{fs, process::Command};

pub fn retrieve_files_image(images: Vec<String>, out_path: String, files: MappingFiles) {
    // Create diff directory
    fs::create_dir(format!("{out_path}/diff")).unwrap();

    // Get files for each image
    let mut content_retrieved_files: Vec<(String,String, String, String)> = vec![];

    for image in images.iter() {
        for file in files.modified.clone().into_iter() {
            let image_path = get_path_image_from_name(image.clone()).unwrap();
            let inode = file.inode;
            let file_content = retrieve_file_image(
                image_path.clone(),
                file.name,
                inode.clone(),
            )
            .unwrap();

            let mut exists_already = false;
            let mut value_index: usize = 0;
            for (index, content_retrieved_file) in
                content_retrieved_files.clone().iter().enumerate()
            {
                if content_retrieved_file.1 == inode {
                    exists_already = true;
                    value_index = index;
                }
            }

            if exists_already {
                let mut previous_value = content_retrieved_files.get(value_index).unwrap().clone();
                content_retrieved_files.remove(value_index);
                previous_value.3 = file_content;
                content_retrieved_files.push(previous_value.clone());
            } else {
                content_retrieved_files.push((image_path,inode, file_content, String::from("")))
            }
        }
    }

    diff_files(content_retrieved_files, out_path)
}

pub fn retrieve_file_image(
    image_path: String,
    file_name: String,
    inode: String,
) -> Result<String, ()> {
    let split: Vec<&str> = image_path.split("\\").collect();
    let image_name = split.get(split.capacity() - 1).unwrap();
    let mut directory_path = String::from("");
    log::debug!("Retrieving file: {image_name} {file_name} {inode}");

    split.iter().enumerate().for_each(|(i, x)| {
        if i == split.capacity() - 1 {
            return;
        }
        directory_path.push_str(x);
        directory_path.push_str("\\");
    });

    let cmd_output = Command::new("wsl")
        .current_dir(directory_path)
        .arg("icat")
        .arg(image_name)
        .arg(inode.clone())
        .output()
        .expect("Failed to execute command");

    let cmd_stdout = cmd_output.stdout;

    let str = String::from_utf8_lossy(&cmd_stdout).to_string();

    Ok(str)
}

pub fn diff_files(files_to_diff: Vec<(String,String, String, String)>, out_path: String) {
    for file_to_diff in files_to_diff.into_iter() {
        let diff = TextDiff::from_lines(&file_to_diff.2, &file_to_diff.3);

        let mut content = String::from("");
        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            content.push_str(format!("{sign}{change}").as_str())
        }

        let image_path = file_to_diff.0;
        let split: Vec<&str> = image_path.split("\\").collect();
        let image_name = split.get(split.capacity() - 1).unwrap();

        let image_name_split: Vec<&str> = image_name.split('.').collect();
        let name = image_name_split.get(0).unwrap();
        let inode = file_to_diff.1;
        let path =format!("F:/Howest/2022-2023/Semester 5/140 GIT/thesis/deltascope-client/deltascope/src-tauri/{out_path}/diff/{name}-{inode}.txt");

        let mut file = File::create(path).unwrap();
        file.write_all(&content.as_bytes()).unwrap();
    }
}


#[derive(Debug, Serialize)]
pub struct DiffFileInfo {
  name: String,
  content: String
}

pub fn read_diff_files(out_path: String) -> Result<Vec<DiffFileInfo>, ()> {
  let path =format!("F:/Howest/2022-2023/Semester 5/140 GIT/thesis/deltascope-client/deltascope/src-tauri/{out_path}/diff");

  let paths = fs::read_dir(path).unwrap();

  let mut files: Vec<DiffFileInfo> = Vec::new();

  for path in paths {
      let content = fs::read(path.as_ref().unwrap().path()).unwrap();
      let string_content = String::from_utf8(content).unwrap();
      files.push(DiffFileInfo { name: path.unwrap().file_name().to_str().unwrap().to_string(), content: string_content } , );
  }

  Ok(files)
}
