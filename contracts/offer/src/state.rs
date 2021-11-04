use cosmwasm_std::{Addr, Order, StdResult, Storage};
use cosmwasm_storage::{
    bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton,
};
use cw_storage_plus::Map;
use localterra_protocol::constants::OFFERS_KEY;
use localterra_protocol::currencies::FiatCurrency;
use localterra_protocol::offer::{Config, Offer, State, OFFERS};

pub static CONFIG_KEY: &[u8] = b"config";
pub static STATE_KEY: &[u8] = b"state";
pub const TRADES: Map<&[u8], Vec<Addr>> = Map::new("trades");

pub fn config_storage(storage: &mut dyn Storage) -> Singleton<Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn state_storage(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, STATE_KEY)
}

pub fn state_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, STATE_KEY)
}

pub fn query_all_trades(storage: &dyn Storage, maker: Addr) -> StdResult<Vec<Addr>> {
    let result = TRADES.load(storage, maker.as_bytes());
    Ok(result.unwrap_or(vec![]))
}
