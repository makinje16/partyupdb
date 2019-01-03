extern crate lfgdb;
extern crate diesel;

use self::lfgdb::*;
use self::models::*;
use self::diesel::prelude::*;
use std::env::args;

fn main() {
    use lfgdb::schema::players::dsl::*;

    let connection = establish_connection();
    let rank_query = args().nth(1).expect("Please input a rank as an argument");
    let rank_enum = match Rank::from_string(&rank_query) {
                        Ok(r) => r,
                        Err(why) => panic!(why),
                    };

    let results = players.filter(rank.eq(rank_enum.to_int()))
        .limit(5)
        .load::<Player>(&connection)
        .expect("Error loading players");

    println!("Displaying {} players", results.len());
    println!("----------\n");
    for player in results {
        println!("Username: {}", player.username);
        println!("Discord: {}", player.discord_name);
        println!("Rank: {}", match Rank::from_int(player.rank) {
                                Ok(r) => r.to_string(),
                                Err(why) => String::from(why),
                            });
        println!("----------\n");
    }
}