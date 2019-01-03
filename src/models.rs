use super::schema::players;
use diesel::sql_types::*;

#[derive(Debug, Copy, Clone, AsExpression, FromSqlRow)]
pub enum Rank {
    Iron,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Master,
    Challenger,
}

impl Rank {
    pub fn from_int(rank: i32) -> Result<Rank, &'static str> {
        match rank {
            0 => Ok(Rank::Iron),
            1 => Ok(Rank::Bronze),
            2 => Ok(Rank::Silver),
            3 => Ok(Rank::Gold),
            4 => Ok(Rank::Platinum),
            5 => Ok(Rank::Diamond),
            6 => Ok(Rank::Master),
            7 => Ok(Rank::Challenger),
            _ => Err("That int doesn't equate to a rank"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Rank::Iron => String::from("Iron"),
            Rank::Bronze => String::from("Bronze"),
            Rank::Silver => String::from("Silver"),
            Rank::Gold => String::from("Gold"),
            Rank::Platinum => String::from("Platinum"),
            Rank::Diamond => String::from("Diamond"),
            Rank::Master => String::from("Master"),
            Rank::Challenger => String::from("Challenger"),
        }
    }

    pub fn to_int(&self) -> i32 {
        match self {
            Rank::Iron =>  0,
            Rank::Bronze => 1,
            Rank::Silver => 2,
            Rank::Gold => 3,
            Rank::Platinum => 4, 
            Rank::Diamond => 5,
            Rank::Master => 6,
            Rank::Challenger => 7,
        }
    }
}

#[derive(Queryable)]
pub struct Player {
    pub id: i32,
    pub username: String,
    pub discord_name: String,
    pub rank: i32,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="players"]
pub struct NewPlayer<'a> {
    pub username: &'a str,
    pub discord_name: &'a str,
    pub rank: &'a i32,
}