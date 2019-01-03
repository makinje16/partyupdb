#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;
pub mod lfgresponses;

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

pub fn insert_player(username: &str, discord_name: &str, rank: &Rank) {
    let conn = establish_connection();
    create_player(&conn, username, discord_name, rank);
}

pub fn delete_player(discord_name: &str) {
    use self::schema::players::dsl::*;
    let conn = establish_connection();
    match diesel::delete(players.filter(discord_name.eq(discord_name))).execute(&conn) {
        Ok(_ok) => (),
        Err(why) => println!("Error: {}", why),
    }
}

pub fn look_for_by_rank(player_rank: Rank) -> std::vec::Vec<models::Player> {
    use self::schema::players::dsl::*;

    let conn = establish_connection();
    players.filter(rank.eq(player_rank.to_int()))
        .limit(10)
        .load::<Player>(&conn)
        .expect("Error loading players")
}