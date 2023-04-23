use sqlx::{Pool, Row, Sqlite};

use crate::tools::mactime::MacTimeLine;

pub async fn input_values_events(
    name: String,
    values: Vec<MacTimeLine>,
    conn: &Pool<Sqlite>,
) -> Result<Pool<Sqlite>, ()> {
    log::info!("Adding events value into database for: {}", name);
    let new_name = name.replace("-", "_");

    let mut tx = conn.begin().await.unwrap();

    let query =
        format!("INSERT INTO {new_name}_events VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
    for line in values.into_iter() {
        let _res = sqlx::query(query.as_str())
            .bind(line.date)
            .bind(line.size.to_string())
            .bind(line.m_activity.to_string())
            .bind(line.a_activity)
            .bind(line.c_activity)
            .bind(line.b_activity)
            .bind(line.file_type)
            .bind(line.owner_perm)
            .bind(line.group_perm)
            .bind(line.other_perm)
            .bind(line.uid.to_string())
            .bind(line.gid.to_string())
            .bind(line.inode)
            .bind(line.name)
            .execute(&mut *tx)
            .await;
    }

    tx.commit().await.unwrap();

    Ok(conn.clone())
}

pub async fn get_events_image_values_neariest_date(
    name: String,
    conn: Pool<Sqlite>,
) -> Result<String, ()> {
    log::info!("Adding events value into database for: {}", name);
    let new_name = name.replace("-", "_");

    let query = format!("SELECT date FROM {new_name}_events order by date desc limit 1");
    let res = sqlx::query(query.as_str()).fetch_one(&conn).await;

    let data = res.unwrap();
    let date = data.try_get::<String, usize>(0).unwrap();

    Ok(date)
}
