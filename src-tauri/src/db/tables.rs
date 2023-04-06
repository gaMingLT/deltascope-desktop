use sqlx::{Sqlite, Pool};

// Create events tables
// cur.execute("CREATE TABLE {0}_events(date, size, mActivity, aActivity, cActivity, bActivity, fileType, ownerPerm, groupPerm, otherPerm ,uid,guid,inode,name)".format(name.replace('-','_')))
pub async fn create_events_table(name: String, conn: Pool<Sqlite>) -> Result<Pool<Sqlite>, ()> {
  println!("Creating events table: {}: ", name);
  let new_name = name.replace("-","_");
  let query = format!("CREATE TABLE IF NOT EXISTS {new_name}_events(date, size INTEGER, mActivity, aActivity, cActivity, bActivity, fileType, ownerPerm, groupPerm, otherPerm ,uid INTEGER,guid INTEGER,inode,name);");
  let res = sqlx::query(query.as_str()).execute(&conn).await.unwrap();

  Ok(conn)
}


// Create files tables
// cur.execute("CREATE TABLE {0}_files(md5,name,inode,mode_as_string,uid,gid,size,atime,mtime,ctime,crtime)".format(name.replace('-','_')))
pub async fn create_files_table(name: String, conn: Pool<Sqlite>) -> Result<Pool<Sqlite>, ()>  {
  println!("Creating files table: {}: ", name);
  let new_name = name.replace("-", "_");
  let query = format!("CREATE TABLE IF NOT EXISTS {new_name}_files(md5,name,inode,mode_as_string,uid,gid,size,atime,mtime,ctime,crtime);");
  let res = sqlx::query(query.as_str()).execute(&conn).await.unwrap();

  Ok(conn)
}


pub async fn create_output_dir_table(conn: Pool<Sqlite>) -> Result<Pool<Sqlite>, ()> {
  println!("Creating output dir path table");
  let query = format!("CREATE TABLE IF NOT EXISTS (id PRIMARY, path);");
  let res = sqlx::query(query.as_str()).execute(&conn).await.unwrap();

  Ok(conn)
}
