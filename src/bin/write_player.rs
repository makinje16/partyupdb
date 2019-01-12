extern crate diesel;
extern crate lfgdb;

use self::lfgdb::*;
use lfgdb::models::Rank;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("Username: ");
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = &username[..(username.len() - 1)]; // Drop the newline character
    println!("Discord Name: ");
    let mut discord_name = String::new();
    stdin().read_line(&mut discord_name).unwrap();
    let discord_name = &discord_name[..(discord_name.len() - 1)];
    println!("Discord_id: ");
    let mut discord_id_str = String::new();
    stdin().read_line(&mut discord_id_str).unwrap();
    let discord_id: i32 = discord_id_str.parse().unwrap();
    let rank: Rank;

    loop {
        println!("Rank:");
        let mut rank_str = String::new();
        stdin().read_line(&mut rank_str).unwrap();
        let rank_str = &rank_str[..(rank_str.len() - 1)];
        match rank_str.as_ref() {
            "Iron" => {
                rank = Rank::Iron;
                break;
            }
            "Bronze" => {
                rank = Rank::Bronze;
                break;
            }
            "Silver" => {
                rank = Rank::Silver;
                break;
            }
            "Gold" => {
                rank = Rank::Gold;
                break;
            }
            "Platinum" => {
                rank = Rank::Platinum;
                break;
            }
            "Diamond" => {
                rank = Rank::Diamond;
                break;
            }
            "Master" => {
                rank = Rank::Master;
                break;
            }
            "Challenger" => {
                rank = Rank::Challenger;
                break;
            }
            _ => {
                println!("Sorry {} isn't a rank", rank_str);
                continue;
            }
        }
    }

    let player = create_player(&connection, &username, &discord_name, &discord_id, &rank);
    println!("\nSaved draft {} with id {}", username, player.id);
}
