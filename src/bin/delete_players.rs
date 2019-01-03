extern crate diesel;
extern crate lfgdb;

use self::diesel::prelude::*;
use self::lfgdb::*;
use std::env::args;

fn main() {
    use lfgdb::models::Rank;
    use lfgdb::schema::players::dsl::*;
    let connection = establish_connection();
    let name = args().nth(1).expect("Expected a target to match against");
    let _num_deleted =
        diesel::delete(players.filter(rank.eq(Rank::Gold.to_int()))).execute(&connection);
}
