use bodyfile::Bodyfile3Line;
use sqlx::{Pool, Sqlite, query};



  // cur.executemany("INSERT INTO {0}_files VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)".format(name.replace('-','_')), values)
pub async fn input_values_files(name: String, values: Vec<Bodyfile3Line>, conn: Pool<Sqlite>) -> Result<Pool<Sqlite>, ()> {
  println!("Adding files value into datbase for: {}", name);
  let new_name = name.replace("-","_");
  // let query_str = format!("INSERT INTO {new_name}_files VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)").as_str();
  // let res = sqlx::query!(query_str, values).execute(&conn).await.unwrap();
  // println!("Res: {:?}", res);

  let line = values.get(0).unwrap();

  // let query = format!("INSERT INTO {new_name}_files VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)");
  let query = format!("INSERT INTO {new_name}_files VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");

  for line in values.into_iter() {
    let res = sqlx::query(query.as_str())
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
        .execute(&conn)
        .await;    
  }



  Ok(conn)
}

pub fn get_files_values() {
  // res = cur.execute("SELECT * FROM {0}_files LIMIT 100".format(name))
}

pub fn get_files_values_path() {
  // res = cur.execute("SELECT * FROM {0}_files where name like '{1}%' ORDER BY mtime DESC".format(name, path))
}
