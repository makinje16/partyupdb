extern crate diesel;
extern crate lfgdb;

use self::diesel::prelude::*;
use self::lfgdb::*;

fn main() {
    use lfgdb::schema::players::dsl::*;
    let connection = establish_connection();
    let _num_deleted =
        diesel::delete(players.filter(discord_name.eq(discord_name))).execute(&connection);
}
