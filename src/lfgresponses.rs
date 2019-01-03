use super::models::Player;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct LfgResponse {
    status: i32,
    body: &'static str,
}

pub struct PlayerList {
    players: Vec<Player>,
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
        s.serialize_field("players", &self.players)?;
        s.end()
    }
}
