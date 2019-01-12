#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod lfgresponses;
pub mod models;
pub mod schema;

use self::models::{NewPlayer, Player, Rank};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABAE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connection to {}", database_url))
}

pub fn create_player<'a>(
    conn: &PgConnection,
    username: &'a str,
    discord_name: &'a str,
    discord_id: &'a str,
    rank: &'a Rank,
) -> Player {
    let new_player = NewPlayer {
        username: username,
        discord_name: discord_name,
        discord_id: discord_id,
        rank: &rank.to_int(),
    };
    diesel::insert_into(schema::players::table)
        .values(&new_player)
        .get_result(conn)
        .expect("Error saving new Player")
}

pub fn check_duplicate(name: &str, conn: &PgConnection) -> bool {
    use self::schema::players::dsl::*;
    let results = players
        .filter(discord_name.eq(name))
        .load::<Player>(conn)
        .expect("Error loading players");
    return if results.len() == 0 { false } else { true };
}

pub fn insert_player(username: &str, discord_name: &str, discord_id : &str, rank: &Rank) {
    let conn = establish_connection();
    match check_duplicate(discord_name, &conn) {
        false => {
            create_player(&conn, username, discord_name, discord_id, rank);
        }
        true => (),
    }
}

pub fn delete_player(name: &str) {
    use self::schema::players::dsl::*;
    let conn = establish_connection();
    match diesel::delete(players.filter(discord_name.eq(name))).execute(&conn) {
        Ok(_ok) => (),
        Err(why) => println!("Error: {}", why),
    }
}

pub fn look_for_by_rank(player_rank: Rank) -> Vec<models::Player> {
    use self::schema::players::dsl::*;

    let conn = establish_connection();
    players
        .filter(rank.eq(player_rank.to_int()))
        .limit(10)
        .load::<Player>(&conn)
        .expect("Error loading players")
}

pub fn look_for_by_id(db_id: i32) -> Vec<models::Player> {
    use self::schema::players::dsl::*;
    let conn = establish_connection();
    players
        .filter(id.eq(db_id))
        .limit(1)
        .load::<Player>(&conn)
        .expect("Error loading players")
}
