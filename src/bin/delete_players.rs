extern crate lfgdb;
extern crate diesel;

use self::lfgdb::*;
use self::diesel::prelude::*;
use std::env::args;

fn main() {
    use lfgdb::schema::players::dsl::*;

    let connection = establish_connection();
    let name = args().nth(1).expect("Expected a target to match against");
    let _num_deleted = diesel::delete(players.filter(username.eq(name))).execute(&connection);
}