use bodyfile::Bodyfile3Line;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

const TOOL_PATH: &str = "tools";

pub async fn execute_fls(
    image_path: String,
    out_path: String,
    name: String,
) -> Result<Vec<String>, ()> {
    log::info!("Path: {}", image_path);

    let cmd_output = Command::new("pwsh")
        .current_dir(TOOL_PATH)
        .arg("/C")
        .arg("./fls-rs.exe")
        .arg("head-all")
        .arg(image_path)
        .output()
        .expect("Failed to execute command");

    let cmd_stdout = cmd_output.stdout;

    let to_str = String::from_utf8(cmd_stdout.clone()).unwrap();
    let lines = to_str
        .split("\n")
        .into_iter()
        .map(|f| f.to_string())
        .filter(|f| f.len() != 0)
        .collect::<Vec<String>>();

    let mut file = File::create(format!("{out_path}/{name}.txt")).unwrap();
    file.write_all(&cmd_stdout).unwrap();

    Ok(lines)
}

pub fn parse_fls_lines(lines: Vec<String>) -> Result<Vec<Bodyfile3Line>, ()> {
    log::info!("Parsing fls lines");

    let parsed_lines = lines
        .into_iter()
        .map(|l| {
            let bf_line = Bodyfile3Line::try_from(l.as_str()).unwrap();
            bf_line
        })
        .collect::<Vec<Bodyfile3Line>>();

    Ok(parsed_lines)
}

pub fn parse_fls_file(out_path: String, name: String) -> Result<Vec<String>, ()> {
    let path = format!("F:/Howest/2022-2023/Semester 5/140 GIT/thesis/deltascope-client/deltascope/src-tauri/{out_path}/{name}.txt");
    let file_path = Path::new(path.as_str());

    let f = File::open(file_path).unwrap();
    let lines = BufReader::new(f).lines();

    let fls_body_file = lines
        .into_iter()
        .map(|l| {
            let bf_line = Bodyfile3Line::try_from(l.unwrap().as_str()).unwrap();

            bf_line.to_string()
        })
        .collect::<Vec<String>>();

    Ok(fls_body_file)
}

pub async fn execute_fls_wsl(
    image_path: String,
    out_path: String,
    name: String,
) -> Result<Vec<String>, ()> {
    log::info!("Path (wsl): {}", image_path);

    let split: Vec<&str> = image_path.split("\\").collect();
    let image_name = split.get(split.capacity() - 1).unwrap();
    let mut directory_path = String::from("");

    split.iter().enumerate().for_each(|(i, x)| {
        if i == split.capacity() - 1 {
            return;
        }
        directory_path.push_str(x);
        directory_path.push_str("\\");
    });

    let cmd_output = Command::new("wsl")
        .current_dir(directory_path)
        .arg("fls")
        .args(["-r", "-h", "-m", "/"])
        .arg(image_name)
        .output()
        .expect("Failed to execute command");

    let cmd_stdout = cmd_output.stdout;

    let to_str = unsafe { String::from_utf8_unchecked(cmd_stdout.clone()) };
    let lines = to_str
        .split("\n")
        .into_iter()
        .map(|f| f.to_string())
        .filter(|f| f.len() != 0)
        .collect::<Vec<String>>();

    let mut file = File::create(format!("{out_path}/{name}.txt")).unwrap();
    file.write_all(&cmd_stdout).unwrap();

    Ok(lines)
}
