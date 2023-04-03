use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, Pool};

// Create events tables
// cur.execute("CREATE TABLE {0}_events(date, size, mActivity, aActivity, cActivity, bActivity, fileType, ownerPerm, groupPerm, otherPerm ,uid,guid,inode,name)".format(name.replace('-','_')))
pub async fn create_events_table(name: String, conn: Pool<Sqlite>) -> Result<Pool<Sqlite>, ()> {
  println!("Creating events table: {}: ", name);
  let new_name = name.replace("-","_");
  let query = format!("CREATE TABLE IF NOT EXISTS {new_name}_events(date, size, mActivity, aActivity, cActivity, bActivity, fileType, ownerPerm, groupPerm, otherPerm ,uid,guid,inode,name);");
  let res = sqlx::query(query.as_str()).execute(&conn).await.unwrap();
  // println!("Res: {:?}", res);

  Ok(conn)
}


// Create files tables
//   cur.execute("CREATE TABLE {0}_events(date, size, mActivity, aActivity, cActivity, bActivity, fileType, ownerPerm, groupPerm, otherPerm ,uid,guid,inode,name)".format(name.replace('-','_')))
pub async fn create_files_table(name: String, conn: Pool<Sqlite>) -> Result<Pool<Sqlite>, ()>  {
  println!("Creating files table: {}: ", name);
  let new_name = name.replace("-", "_");
  let query = format!("CREATE TABLE IF NOT EXISTS {new_name}_events(date, size, mActivity, aActivity, cActivity, bActivity, fileType, ownerPerm, groupPerm, otherPerm ,uid,guid,inode,name);");
  println!("{}", query);
  let res = sqlx::query(query.as_str()).execute(&conn).await.unwrap();
  // println!("Res: {:?}", res);

  Ok(conn)
}
