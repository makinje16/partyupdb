use super::models::Player;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub const STATUS_OK: i32 = 200;
pub const STATUS_FAILURE: i32 = 400;
pub const BAD_RANK_MSG: &'static str = "Bad rank was inputted";
pub const SUCC_MSG: &'static str = "Success!";

pub struct LfgResponse {
    pub status: i32,
    pub body: &'static str,
}

pub struct PlayerList {
    pub status: i32,
    pub body: &'static str,
    pub players: Vec<Player>,
}

impl Serialize for LfgResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Person", 2)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("body", &self.body)?;
        s.end()
    }
}

impl Serialize for PlayerList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("PlayerList", 1)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("body", &self.body)?;
        s.serialize_field("players", &self.players)?;
        s.end()
    }
}

pub fn bad_rank_player_list() -> PlayerList {
    PlayerList {
        status: STATUS_FAILURE,
        body: BAD_RANK_MSG,
        players: Vec::<Player>::new(),
    }
}