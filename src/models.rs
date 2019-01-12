use super::schema::players;
use serde::ser::{Serialize, SerializeStruct, Serializer};

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

    pub fn from_string(rank: &str) -> Result<Rank, &'static str> {
        match rank {
            "IRON" => Ok(Rank::Iron),
            "BRONZE" => Ok(Rank::Bronze),
            "SILVER" => Ok(Rank::Silver),
            "GOLD" => Ok(Rank::Gold),
            "PLATINUM" => Ok(Rank::Platinum),
            "DIAMOND" => Ok(Rank::Diamond),
            "MASTER" => Ok(Rank::Master),
            "CHALLENGER" => Ok(Rank::Challenger),
            _ => Err("Sorry that is not a rank"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Rank::Iron => String::from("IRON"),
            Rank::Bronze => String::from("BRONZE"),
            Rank::Silver => String::from("SILVER"),
            Rank::Gold => String::from("GOLD"),
            Rank::Platinum => String::from("PLATINUM"),
            Rank::Diamond => String::from("DIAMOND"),
            Rank::Master => String::from("MASTER"),
            Rank::Challenger => String::from("CHALLENGER"),
        }
    }

    pub fn to_int(&self) -> i32 {
        match self {
            Rank::Iron => 0,
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
    pub discord_id : String,
}

#[derive(Insertable)]
#[table_name = "players"]
pub struct NewPlayer<'a> {
    pub username: &'a str,
    pub discord_name: &'a str,
    pub rank: &'a i32,
    pub discord_id : &'a str,
}

impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let rank = Rank::from_int(self.rank);
        let rank_enum = match rank {
            Ok(r) => r,
            Err(why) => panic!(why),
        };
        let mut s = serializer.serialize_struct("Person", 5)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("username", &self.username)?;
        s.serialize_field("discord_name", &self.discord_name)?;
        s.serialize_field("discord_id", &self.discord_id)?;
        s.serialize_field("rank", &rank_enum.to_string())?;
        s.end()
    }
}
