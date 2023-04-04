
use sqlx::{Pool, Sqlite, query};

use crate::tools::mactime::MacTimeLine;

// cur.executemany("INSERT INTO {0}_events VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)".format(name.replace('-','_')), values)
pub async fn input_values_events(name: String, values: Vec<MacTimeLine>, conn: Pool<Sqlite>) -> Result<Pool<Sqlite>, ()> {
  println!("Adding events value into datbase for: {}", name);
  let new_name = name.replace("-","_");
  let line = values.get(0).unwrap();

  let query = format!("INSERT INTO {new_name}_events VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
  for line in values.into_iter() {
    let res = sqlx::query(query.as_str())
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
        .execute(&conn)
        .await;
  }

  Ok(conn)

}

pub fn input_values_events_2() {
  // cur.executemany("INSERT INTO {0}_events VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)".format(name.replace('-','_')), values)
}

pub fn get_events_image_values_neariest_date() {
  //  res = cur.execute("SELECT date FROM {0}_events order by date desc LIMIT 1".format(name))
}
