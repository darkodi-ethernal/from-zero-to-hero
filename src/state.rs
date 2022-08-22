use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}
// Derive JSON serialization
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64)>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ballot {
    pub option: String, // vote
}
pub const STATE: Item<State> = Item::new("state");
// A map with a String key and Poll value.
// The key will be a UUID generated clientside
pub const POLLS: Map<String, Poll> = Map::new("polls");
//key: (sender, poll_id),  value: vote option
pub const BALLOTS: Map<(Addr, String), Ballot> = Map::new("ballots");

pub const CONFIG: Item<Config> = Item::new("config");
