use bodyfile::Bodyfile3Line;
use sqlx::{Pool, Row, Sqlite};

pub async fn input_values_files(
    name: String,
    values: Vec<Bodyfile3Line>,
    conn: &Pool<Sqlite>,
) -> Result<Pool<Sqlite>, ()> {
    log::info!("Adding files value into database for: {}", name);
    let new_name = name.replace("-", "_");

    let query = format!("INSERT INTO {new_name}_files VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");

    let mut tx = conn.begin().await.unwrap();

    for line in values.into_iter() {
        sqlx::query(&query.as_str())
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
            .execute(&mut *tx)
            .await
            .unwrap();
    }

    tx.commit().await.unwrap();

    Ok(conn.clone())
}

#[derive(Debug, Clone)]
pub struct Bodyfile3Line2 {
    pub md5: String,
    pub name: String,
    pub inode: String,
    pub mode_as_string: String,
    pub uid: u64,
    pub gid: u64,
    pub size: u64,
    pub atime: i64,
    pub mtime: i64,
    pub ctime: i64,
    pub crtime: i64,
}

pub async fn get_files(name: String, conn: &Pool<Sqlite>) -> Result<Vec<Bodyfile3Line2>, ()> {
    log::info!("Retrieving files values from: {}", name);
    let new_name = name.replace("-", "_");

    let query = format!("SELECT * FROM {new_name}_files");
    let res = sqlx::query(&query.as_str()).fetch_all(conn).await.unwrap();

    let mut parsed_rows = Vec::new();
    for row in res.into_iter() {
        parsed_rows.push(Bodyfile3Line2 {
            md5: row.try_get::<String, usize>(0).unwrap(),
            // name:  String::from_utf8_lossy(row.try_get::<&str, usize>(1).unwrap().as_bytes()).to_string(),
            name: {
                match row.try_get::<String, usize>(1) {
                    Ok(e) => e,
                    Err(e) => {
                        log::error!("{}", e);
                        String::from("error")
                    }
                }
            },
            inode: row.try_get::<String, usize>(2).unwrap(),
            mode_as_string: row.try_get::<String, usize>(3).unwrap(),

            uid: row.try_get::<u32, usize>(4).unwrap() as u64,
            gid: row.try_get::<u32, usize>(5).unwrap() as u64,

            size: row.try_get::<u32, usize>(6).unwrap() as u64,

            atime: row.try_get::<i64, usize>(7).unwrap(),
            mtime: row.try_get::<i64, usize>(8).unwrap(),
            ctime: row.try_get::<i64, usize>(8).unwrap(),
            crtime: row.try_get::<i64, usize>(9).unwrap(),
        });
    }

    Ok(parsed_rows)
}
