#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_json;
extern crate lfgdb;

use lfgdb::insert_player;
use lfgdb::lfgresponses;
use lfgdb::look_for_by_rank;
use lfgdb::models::Rank;

const STATUS_OK: i32 = 200;
const STATUS_FAILURE: i32 = 400;

// #[get("/insert/<username>/<discord_name>/<rank>")]
// fn new_player(username: String, discord_name: String, rank: String) -> Json<LfgResponse> {
//     let rank_enum = Rank::from_string(rank.as_ref());
//     let mut response;
//     match rank_enum {
//         Ok(rank_enum) => {
//             insert_player(&username, &discord_name, &rank_enum);
//             response = LfgResponse {
//                 status: STATUS_OK,
//                 body: "Player was added to the db",
//             };
//             },
//         Err(why) => {
//             response = LfgResponse {
//                 status: STATUS_FAILURE,
//                 body: "Rank was not found",
//             };
//         },
//     }
//     json!(response)
// }

#[get("/delete/<discord_name>")]
fn remove_player(discord_name: String) {
    lfgdb::delete_player(&discord_name.as_ref())
}

// #[get("/get/<rank_str>")]
// fn find_by_rank(rank_str: String) -> &'static str {
//     let rank_enum = match Rank::from_string(rank_str.as_ref()) {
//                                 Ok(r) => r,
//                                 _ => return "Rank not found",
//                             };
//     let results = look_for_by_rank(rank_enum);
//     let mut return_str = String::new();

//     for p in results {
//         return_str.push_str("~~~~");
//         return_str.push_str(&p.discord_name);
//     }
//     let return_str = return_str.as_ref();
//     return_str
// }

fn main() {
    // rocket::ignite().mount("/", routes![new_player, remove_player]).launch();
}
