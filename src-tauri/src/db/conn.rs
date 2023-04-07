use std::fmt::format;

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, Pool};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use std::str::FromStr;

pub async fn db_con(path: String) -> Result<Pool<Sqlite>, ()> {
  // const DB_URL: &str = "";
  let db_url = format!("sqlite://{path}/content.db").to_string();
  if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
      println!("Creating database {}", &db_url);
      match Sqlite::create_database(&db_url).await {
          Ok(_) => println!("Create db success"),
          Err(error) => panic!("error: {}", error),
      }
  } else {
      println!("Database already exists");
  }

  let connection_options = SqliteConnectOptions::from_str(&db_url).unwrap().create_if_missing(true).serialized(false);

  let db = SqlitePool::connect_with(connection_options).await.unwrap();
  println!("Connecting to datbase");
  

  Ok(db)
}

pub async fn db_con_app() -> Result<Pool<Sqlite>, ()> {
    let db_url = format!("sqlite://F:/Howest/2022-2023/Semester 5/140 GIT/thesis/deltascope-client/deltascope/src-tauri/deltascope.db").to_string();
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        println!("Creating database {}", &db_url);
        match Sqlite::create_database(&db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
  
    let db = SqlitePool::connect(&db_url).await.unwrap();
    println!("Connecting to datbase");
    
  
    Ok(db)
  }
  