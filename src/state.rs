use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DomainMappingState {
    pub domain: String,
    pub account_address: Addr,
}

pub const CONFIGSTATE: Item<Config> = Item::new("config");
pub const DOMAINSTATE: Map<String, DomainMappingState> = Map::new("domain_state");
pub const ACCOUNTSTATE: Map<String, DomainMappingState> = Map::new("account_state");