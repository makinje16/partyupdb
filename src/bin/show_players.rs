extern crate lfgdb;
extern crate diesel;

use self::lfgdb::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use lfgdb::schema::players::dsl::*;

    let connection = establish_connection();
    let results = players.filter(published.eq(true))
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