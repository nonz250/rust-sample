use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    MysqlConnection::establish(&database_url).expect(&format!("ERROR conncting to {}", database_url))
}
