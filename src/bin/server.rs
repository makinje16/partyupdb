#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_json;
extern crate lfgdb;

use lfgdb::insert_player;
use lfgdb::lfgresponses;
use lfgdb::lfgresponses::{LfgResponse, PlayerList, STATUS_OK, STATUS_FAILURE, SUCC_MSG};
use lfgdb::look_for_by_rank;
use lfgdb::models::Rank;

use rocket_contrib::json::Json;
use rocket::config::{Config, Environment};
use rocket::custom;

#[get("/insert/<username>/<discord_name>/<rank>")]
fn new_player(username: String, discord_name: String, rank: String) -> Json<LfgResponse> {
    let rank_enum = Rank::from_string(rank.as_ref());
    let response;
    match rank_enum {
        Ok(rank_enum) => {
            insert_player(&username, &discord_name, &rank_enum);
            response = LfgResponse {
                status: STATUS_OK,
                body: SUCC_MSG,
            };
        }
        Err(why) => {
            response = LfgResponse {
                status: STATUS_FAILURE,
                body: why,
            };
        }
    }
    Json(response)
}

#[get("/delete/<discord_name>")]
fn remove_player(discord_name: String) -> Json<LfgResponse> {
    lfgdb::delete_player(&discord_name.as_ref());
    Json(LfgResponse {
        status: STATUS_OK,
        body: SUCC_MSG,
    })
}

#[get("/get/<rank_str>")]
fn find_by_rank(rank_str: String) -> Json<PlayerList> {
    let rank_enum = match Rank::from_string(rank_str.as_ref()) {
                                Ok(r) => r,
                                _ => return Json(lfgresponses::bad_rank_player_list()),
                    };
    let results = look_for_by_rank(rank_enum);
    Json(PlayerList {
        status: STATUS_OK,
        body: SUCC_MSG,
        players: results,
    })
}

fn main() {
    let config = Config::build(Environment::Staging)
    .address("0.0.0.0")
    .port(8080)
    .workers(2)
    .unwrap();

    let app = rocket::custom(config);
    app.mount("/", routes![new_player, remove_player, find_by_rank]).launch();
}
