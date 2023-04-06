use std::path::PathBuf;

use sqlx::{Pool, Sqlite, Row};


pub async fn update_path_output_dir(path: PathBuf, conn: Pool<Sqlite>) {
  println!("Updating outhpath directory: {:?}", path);

  let delete_query = format!("DELETE FROM output_dir");
  let res = sqlx::query(delete_query.as_str())
    .execute(&conn)
    .await;

  let query = format!("INSERT INTO output_dir VALUES(?)");
    let res = sqlx::query(query.as_str())
        .bind(path.into_os_string().into_string().unwrap())
        .execute(&conn)
        .await;

  // Ok(())
}

pub async fn get_output_path(conn: Pool<Sqlite>) -> Result<String, ()> {
  let query = format!("SELECT name FROM output_dir");
  let res = sqlx::query(query.as_str())
      .fetch_one(&conn)
      .await;

  let data = res.unwrap();
  let path = data.try_get::<String, usize>(0).unwrap();
  
  Ok(path) 
}
