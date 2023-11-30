library;

use std::bytes::Bytes;
use std::hash::{Hash, Hasher};

pub enum GuildAction {
    Joined: (),
    Owner: (),
    Admin: (),
}

impl GuildAction {
    pub fn into_byte(self) -> u8 {
        match self {
            GuildAction::Joined => 0,
            GuildAction::Owner => 1,
            GuildAction::Admin => 2,
        }
    }

    pub fn to_str(self) -> str {
        match self {
            GuildAction::Joined => "Joined",
            GuildAction::Owner => "Owner of",
            GuildAction::Admin => "Admin of",
        }
    }

    pub fn to_description(self) -> str {
        match self {
            GuildAction::Joined => " joined ",
            GuildAction::Owner => "'re the owner of ",
            GuildAction::Admin => "'re the admin of ",
        }
    }
}

impl Hash for GuildAction {
    fn hash(self, ref mut state: Hasher) {
        let mut bytes = Bytes::with_capacity(1);
        bytes.push(self.into_byte());
        state.write(bytes);
    }
}
