use sqlx::{Pool, Sqlite, query};



  // cur.executemany("INSERT INTO {0}_files VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)".format(name.replace('-','_')), values)
pub async fn input_values_files(name: String, values: Vec<String>, conn: Pool<Sqlite>) -> Result<Pool<Sqlite>, ()> {
  println!("Adding files value into datbase for: {}", name);
  let new_name = name.replace("-","_");
  // let query = format!("CREATE TABLE IF NOT EXISTS {new_name}_events(date, size, mActivity, aActivity, cActivity, bActivity, fileType, ownerPerm, groupPerm, otherPerm ,uid,guid,inode,name);");
  let query_str = format!("INSERT INTO {new_name}_files VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)").as_str();
  // let res = sqlx::query!(query_str, values).execute(&conn).await.unwrap();
  // println!("Res: {:?}", res);

  Ok(conn)
}

pub fn get_files_values() {
  // res = cur.execute("SELECT * FROM {0}_files LIMIT 100".format(name))
}

pub fn get_files_values_path() {
  // res = cur.execute("SELECT * FROM {0}_files where name like '{1}%' ORDER BY mtime DESC".format(name, path))
}
