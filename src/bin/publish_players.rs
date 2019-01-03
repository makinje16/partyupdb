extern crate lfgdb;
extern crate diesel;

use self::diesel::prelude::*;
use self::lfgdb::*;
use self::models::Player;
use std::env::args;

fn main() {
    use lfgdb::schema::players::dsl::{players, published};

    let id = args().nth(1).expect("publish_post requires a post id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let player = diesel::update(players.find(id))
        .set(published.eq(true))
        .get_result::<Player>(&connection)
        .expect(&format!("Unable to find player {}", id));
    println!("Published player {}", player.username);
}