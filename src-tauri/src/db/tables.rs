use sqlx::{Pool, Sqlite};

/// Create events tables
pub async fn create_events_table(name: String, conn: Pool<Sqlite>) -> Result<Pool<Sqlite>, ()> {
    log::info!("Creating events table: {}: ", name);
    let new_name = name.replace("-", "_");
    let query = format!("CREATE TABLE IF NOT EXISTS {new_name}_events(date, size INTEGER, mActivity, aActivity, cActivity, bActivity, fileType, ownerPerm, groupPerm, otherPerm ,uid INTEGER,guid INTEGER,inode,name);");
    sqlx::query(query.as_str()).execute(&conn).await.unwrap();

    Ok(conn)
}

/// Create files tables
pub async fn create_files_table(name: String, conn: Pool<Sqlite>) -> Result<Pool<Sqlite>, ()> {
    log::info!("Creating files table: {}: ", name);
    let new_name = name.replace("-", "_");
    let query = format!("CREATE TABLE IF NOT EXISTS {new_name}_files(md5,name,inode,mode_as_string,uid INTEGER ,gid INTEGER,size INTEGER,atime INTEGER,mtime INTEGER,ctime INTGER,crtime INTEGER);");
    sqlx::query(query.as_str()).execute(&conn).await.unwrap();

    Ok(conn)
}

// Crate output_dir_table
pub async fn create_output_dir_table(conn: Pool<Sqlite>) -> Result<Pool<Sqlite>, ()> {
    log::info!("Creating output dir path table");
    let query = format!("CREATE TABLE IF NOT EXISTS (id PRIMARY, path);");
    sqlx::query(query.as_str()).execute(&conn).await.unwrap();

    Ok(conn)
}
