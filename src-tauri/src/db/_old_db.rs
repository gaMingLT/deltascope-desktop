use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Path {
    pub id: i32,
    pub path: String,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// pub fn get_paths() {
//   let con = &mut establish_connection();

//   let results = paths.load::<Path>(con).expect("Error loading paths");

//   println('');

// }
