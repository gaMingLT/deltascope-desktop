use sqlx::ConnectOptions;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, Pool};
use sqlx::sqlite::{SqliteConnectOptions};
use std::str::FromStr;

pub async fn db_con(path: String) -> Result<Pool<Sqlite>, ()> {

  let db_url = format!("sqlite://{path}/content.db").to_string();
  if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
    log::info!("Creating database {}", db_url);
      match Sqlite::create_database(&db_url).await {
        Ok(_) => log::info!("Create db success!"),
        Err(error) => {
            log::error!("Error: (to add)");
            panic!("error {}", error)
        }
      }
  } else {
    log::info!("Database already exists {}", db_url);
  }

  let mut connection_options = SqliteConnectOptions::from_str(&db_url).unwrap().create_if_missing(true).serialized(false);
  connection_options.disable_statement_logging();

  let db = SqlitePool::connect_with(connection_options).await.unwrap();
  log::info!("Connecting to database.");
  
  Ok(db)
}

pub async fn db_con_app() -> Result<Pool<Sqlite>, ()> {
    let db_url = format!("sqlite://F:/Howest/2022-2023/Semester 5/140 GIT/thesis/deltascope-client/deltascope/src-tauri/deltascope.db").to_string();
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
      log::info!("Creating database {}", db_url);
        match Sqlite::create_database(&db_url).await {
          Ok(_) => log::info!("Create db success!"),
          Err(error) => {
              log::error!("Error: (to add)");
              panic!("error {}", error)
          }
        }
    } else {
      log::info!("Database already exists {}", db_url);
    }
  
    let db = SqlitePool::connect(&db_url).await.unwrap();
    log::info!("Connecting to database.");
    
    Ok(db)
  }
  