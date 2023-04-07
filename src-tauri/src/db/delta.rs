use sqlx::{Pool, Sqlite};
use crate::tools::mactime::MacTimeLine;

//   res = cur.execute("SELECT json_group_array(json_object('Date', date,  'Size', size, 'mActivity',  mActivity, 'aActivity',  aActivity,'cActivity',  cActivity,'bActivity',  bActivity,'FileType',  fileType,'OwnerPerm', ownerPerm,'GroupPerm', groupPerm,'OtherPerm', otherPerm,'UUID', uid,'GUID', guid,'Inode', inode, 'Path', name)) FROM (SELECT * From {0}_events where date like '%{1}%' order by date desc limit 200) ".format(name, year))
pub async fn get_events_json(name: String, conn: Pool<Sqlite>) -> Result<Vec<MacTimeLine>, ()> {
  log::info!("Retrieving events values from: {}", name);
  let new_name = name.replace("-","_");

  let query = format!("SELECT * FROM {new_name}_events LIMIT 1000");
  let res = sqlx::query(&query.as_str()).fetch_all(&conn).await.unwrap();

  let mut parsed_rows = Vec::new();
  for row in res.into_iter() {
    parsed_rows.push(MacTimeLine::try_from(row).unwrap());
  }

  Ok(parsed_rows)
}

  //   res = cur.execute("SELECT json_group_array(json_object('Date', date,  'Size', size, 'mActivity',  mActivity, 'aActivity',  aActivity,'cActivity',  cActivity,'bActivity',  bActivity,'FileType',  fileType,'OwnerPerm', ownerPerm,'GroupPerm', groupPerm,'OtherPerm', otherPerm,'UUID', uid,'GUID', guid,'Inode', inode, 'Path', name)) FROM (SELECT * FROM {0}_events where date not in (SELECT date from {1}_events) order by date desc limit 200)".format(next, base))
pub async fn get_events_delta(base_name: String, next_name: String, conn: Pool<Sqlite>) -> Result<Vec<MacTimeLine>, ()> {
  log::info!("Retrieving delta's between images: {} & {}", base_name, next_name);
  let new_base_name = base_name.replace("-","_");
  let new_next_name = next_name.replace("-", "_");

  let query = format!("SELECT * FROM (SELECT * FROM {new_next_name}_events where date not in (SELECT date from {new_base_name}_events) order by date desc limit 200)");
  let res = sqlx::query(&query.as_str()).fetch_all(&conn).await.unwrap();

  let mut parsed_rows = Vec::new();
  for row in res.into_iter() {
    parsed_rows.push(MacTimeLine::try_from(row).unwrap());
  }

  Ok(parsed_rows)
}


// def get_events_json(name: str, year: int ,con):
//   cur = con.cursor()
//   main_logger.debug("Retrieving values from database: {0}_events".format(name))
//   res = cur.execute("SELECT json_group_array(json_object('Date', date,  'Size', size, 'mActivity',  mActivity, 'aActivity',  aActivity,'cActivity',  cActivity,'bActivity',  bActivity,'FileType',  fileType,'OwnerPerm', ownerPerm,'GroupPerm', groupPerm,'OtherPerm', otherPerm,'UUID', uid,'GUID', guid,'Inode', inode, 'Path', name)) FROM (SELECT * From {0}_events where date like '%{1}%' order by date desc limit 200) ".format(name, year))
//   return res.fetchmany(size=500)

// def get_events_delta(base: str, next: str, year: int ,con):
//   cur = con.cursor()
//   main_logger.debug('Retrieving delta values from database: {0} - {1}'.format(base, next))
//   res = cur.execute("SELECT json_group_array(json_object('Date', date,  'Size', size, 'mActivity',  mActivity, 'aActivity',  aActivity,'cActivity',  cActivity,'bActivity',  bActivity,'FileType',  fileType,'OwnerPerm', ownerPerm,'GroupPerm', groupPerm,'OtherPerm', otherPerm,'UUID', uid,'GUID', guid,'Inode', inode, 'Path', name)) FROM (SELECT * FROM {0}_events where date not in (SELECT date from {1}_events) order by date desc limit 200)".format(next, base))
//   return res.fetchall()
