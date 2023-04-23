use crate::tools::mactime::MacTimeLine;
use sqlx::{Pool, Sqlite};

pub async fn get_events_json(name: String, conn: Pool<Sqlite>) -> Result<Vec<MacTimeLine>, ()> {
    log::info!("Retrieving events values from: {}", name);
    let new_name = name.replace("-", "_");

    let query = format!("SELECT * FROM {new_name}_events LIMIT 1000");
    let res = sqlx::query(&query.as_str()).fetch_all(&conn).await.unwrap();

    let mut parsed_rows = Vec::new();
    for row in res.into_iter() {
        parsed_rows.push(MacTimeLine::try_from(row).unwrap());
    }

    Ok(parsed_rows)
}

pub async fn get_events_delta(
    base_name: String,
    next_name: String,
    conn: Pool<Sqlite>,
) -> Result<Vec<MacTimeLine>, ()> {
    log::info!(
        "Retrieving delta's between images: {} & {}",
        base_name,
        next_name
    );
    let new_base_name = base_name.replace("-", "_");
    let new_next_name = next_name.replace("-", "_");

    let query = format!("SELECT * FROM (SELECT * FROM {new_next_name}_events where date not in (SELECT date from {new_base_name}_events) order by date desc limit 200)");
    let res = sqlx::query(&query.as_str()).fetch_all(&conn).await.unwrap();

    let mut parsed_rows = Vec::new();
    for row in res.into_iter() {
        parsed_rows.push(MacTimeLine::try_from(row).unwrap());
    }

    Ok(parsed_rows)
}
