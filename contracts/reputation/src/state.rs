use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use cosmwasm_storage::{bucket, bucket_read};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use cw_storage_plus::{Item, Map};
use localterra_protocol::reputation::{Config, Score};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::Div;

pub const STATE: Item<Config> = Item::new("score_rules");
pub const SCORES: Map<String, Score> = Map::new("scores");
