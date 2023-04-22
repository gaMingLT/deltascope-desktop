use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use sqlx::Row;
use sqlx::sqlite::SqliteRow;


// #[derive(Clone)]
pub struct MacTimeLine {
    pub date: String,
    pub size: u64,
    pub m_activity: String,
    pub a_activity: String,
    pub c_activity: String,
    pub b_activity: String,
    pub file_type: String,
    pub owner_perm: String,
    pub group_perm: String,
    pub other_perm: String,
    pub uid: u64,
    pub gid: u64,
    pub inode: String,
    pub name: String,
}

impl Serialize for MacTimeLine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Event", 14)?;
        s.serialize_field("date", &self.date)?;
        s.serialize_field("size", &self.size)?;

        s.serialize_field("m_activity", &self.m_activity)?;
        s.serialize_field("a_activity", &self.a_activity)?;
        s.serialize_field("c_activity", &self.c_activity)?;
        s.serialize_field("b_activity", &self.b_activity)?;

        s.serialize_field("file_type", &self.file_type)?; 
        s.serialize_field("owner_perm", &self.owner_perm)?;
        s.serialize_field("group_perm", &self.group_perm)?;
        s.serialize_field("other_perm", &self.other_perm)?;

        s.serialize_field("uid", &self.uid)?; 
        s.serialize_field("gid", &self.gid)?;
        s.serialize_field("inode", &self.inode)?;
        s.serialize_field("name", &self.name)?; 

        s.end()
    }
}

impl TryFrom<SqliteRow> for MacTimeLine{
    /// Value should be a sqlite
    fn try_from(row: SqliteRow) -> Result<Self, String> {
        let res = MacTimeLine {
            date: row.try_get::<String, usize>(0).unwrap(),  
            size: row.try_get::<u32, usize>(1).unwrap() as u64,  
            // size: row.try_get::<String, usize>(1).unwrap(),  

            m_activity: row.try_get::<String, usize>(2).unwrap(), 
            a_activity: row.try_get::<String, usize>(3).unwrap(), 
            c_activity: row.try_get::<String, usize>(4).unwrap(), 
            b_activity: row.try_get::<String, usize>(5).unwrap(), 
            
            file_type: row.try_get::<String, usize>(6).unwrap(),
            owner_perm: row.try_get::<String, usize>(7).unwrap(),
            group_perm: row.try_get::<String, usize>(8).unwrap(), 
            other_perm: row.try_get::<String, usize>(9).unwrap(),

            uid: row.try_get::<u32, usize>(10).unwrap() as u64, 
            gid: row.try_get::<u32, usize>(11).unwrap() as u64,
            inode: row.try_get::<String, usize>(12).unwrap(),
            name: row.try_get::<String, usize>(13).unwrap(),
        };

        Ok(res)
    }

    type Error = String;
}

// impl Error for MacTimeLine {}

const TOOL_PATH: &str = "tools";

pub fn execute_mactime(out_path: String, name: String) -> Result<Vec<String>, ()> {
    println!("Path: {}", out_path);

    let cmd_output = Command::new("pwsh")
        .current_dir(TOOL_PATH)
        .arg("/C")
        .arg("./mactime2.exe")
        .arg("-b")
        .arg(format!("../{out_path}/{name}.txt"))
        .arg("-F")
        .arg("csv")
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

    let mut file = File::create(format!("{out_path}/tl.{name}.txt")).unwrap();
    file.write_all(&cmd_stdout).unwrap();

    Ok(lines)
}

pub fn parse_mactime_lines(lines: Vec<String>) -> Result<Vec<MacTimeLine>, ()> {
    let parsed_lines = lines
        .into_iter()
        .filter(|l| l.split(',').collect::<Vec<&str>>()[0] != "Date")
        .map(|l| {
            let parts: Vec<&str> = l.split(',').collect();
            let activities: Vec<char> = parts[2].chars().collect();
            let permissions: Vec<char> = parts[3].chars().collect();

            MacTimeLine {
                date: parts[0].to_string(),
                size: str::parse::<u64>(parts[1]).unwrap(),
                m_activity: activities[0].to_string(),
                a_activity: activities[1].to_string(),
                c_activity: activities[2].to_string(),
                b_activity: activities[3].to_string(),

                file_type: permissions[..3].iter().collect(),
                owner_perm: permissions[3..6].iter().collect(),
                group_perm: permissions[6..9].iter().collect(),
                other_perm: permissions[9..12].iter().collect(),

                uid: str::parse::<u64>(parts[4]).unwrap(),
                gid: str::parse::<u64>(parts[5]).unwrap(),
                inode: parts[6].to_string(),
                name: parts[7].to_string(),
            }
        })
        .collect::<Vec<MacTimeLine>>();

    Ok(parsed_lines)
}


// cmd = "mactime -b {1}/{0}.txt -d > {1}/{2}/tl.{0}.txt".format(name.replace('_','-'), out, 'timelines')
// res = system(cmd)

pub async fn execute_mactime_wls(out_path: String, name: String) -> Result<Vec<String>, ()> {
    log::info!("Executing mactime (WLS): {}", out_path);

    let new_outh_path = out_path.clone().to_string();

    let mut base_path = String::from("F:\\Howest\\2022-2023\\Semester 5\\140 GIT\\thesis\\deltascope-client\\deltascope\\src-tauri\\");
    base_path.push_str(out_path.clone().replace("/", "\\") .as_str());

    let cmd_output = Command::new("wsl")
        .current_dir(out_path)
        .arg("mactime")
        .arg("-b")
        .arg(format!("{name}.txt"))
        .arg("-d")
        .arg("-y")
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

    let mut file = File::create(format!("{new_outh_path}/tl.{name}-wls.txt")).unwrap();
    file.write_all(&cmd_stdout).unwrap();

    Ok(lines)
}
