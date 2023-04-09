use bodyfile::Bodyfile3Line;
use futures::{
  stream::{self},
  StreamExt,
};
use sqlx::{Pool, Sqlite };

// cur.executemany("INSERT INTO {0}_files VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)".format(name.replace('-','_')), values)
pub async fn input_values_files(name: String, values: Vec<Bodyfile3Line>, conn: &Pool<Sqlite>) -> Result<Pool<Sqlite>, ()> {
  log::info!("Adding files value into datbase for: {}", name);
  let new_name = name.replace("-","_");
  // let line = values.get(0).unwrap();

  // let stream = stream::iter(values);
  // let name_test = new_name.as_bytes().clone();

  // sqlx::query("BEGIN TRANSACTION").execute(conn).await.unwrap();
  
  // stream.for_each_concurrent(100,
  //   |i: Bodyfile3Line| async move {
  //     let line = i;
  //     let new_name = String::from_utf8(name_test.clone().to_vec()).unwrap();
  //     let query = format!("INSERT INTO {new_name}_files VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"); 
  //     sqlx::query(query.as_str())
  //       .bind(line.get_md5())
  //       .bind(line.get_name())
  //       .bind(line.get_inode())
  //       .bind(line.get_mode())
  //       .bind(line.get_uid().to_string())
  //       .bind(line.get_gid().to_string())
  //       .bind(line.get_size().to_string())
  //       .bind(line.get_atime())
  //       .bind(line.get_mtime())
  //       .bind(line.get_ctime())
  //       .bind(line.get_crtime())
  //       .execute(conn)
  //       .await.unwrap();

  //   }
  // ).await;

  // sqlx::query("COMMIT TRANSACTION").execute(conn).await.unwrap();

  let query = format!("INSERT INTO {new_name}_files VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
  for line in values.into_iter() {
    sqlx::query(query.as_str())
        .bind(line.get_md5())
        .bind(line.get_name())
        .bind(line.get_inode())
        .bind(line.get_mode())
        .bind(line.get_uid().to_string())
        .bind(line.get_gid().to_string())
        .bind(line.get_size().to_string())
        .bind(line.get_atime())
        .bind(line.get_mtime())
        .bind(line.get_ctime())
        .bind(line.get_crtime())
        .execute(conn)
        .await.unwrap();    
  }



  Ok(conn.clone())
}

// pub fn get_files_values() {
//   // res = cur.execute("SELECT * FROM {0}_files LIMIT 100".format(name))
// }

// pub fn get_files_values_path() {
//   // res = cur.execute("SELECT * FROM {0}_files where name like '{1}%' ORDER BY mtime DESC".format(name, path))
// }
