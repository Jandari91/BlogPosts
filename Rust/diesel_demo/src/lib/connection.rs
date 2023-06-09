use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection(db_url:&String) -> PgConnection {
    PgConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}