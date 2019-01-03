#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use self::models::{Player, NewPlayer, Rank};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABAE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}

pub fn create_player<'a>(conn: &PgConnection, username: &'a str, discord_name: &'a str, rank: &'a Rank) -> Player {
    let new_player = NewPlayer {
        username: username,
        discord_name: discord_name,
        rank: &rank.to_int(),
    };
    diesel::insert_into(schema::players::table)
        .values(&new_player)
        .get_result(conn)
        .expect("Error saving new Player")
}