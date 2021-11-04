use cosmwasm_std::{Addr, Order, StdResult, Storage, Uint128};
use cosmwasm_storage::{
    bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton,
};
use cw_storage_plus::{Map, PrimaryKey};
use localterra_protocol::currencies::FiatCurrency;
use localterra_protocol::offer::{Config, Offer, OfferMsg, OfferState, State, OFFERS};

pub static CONFIG_KEY: &[u8] = b"config";
pub static STATE_KEY: &[u8] = b"state";
pub static OFFERS_KEY: &[u8] = b"offers";
pub const TRADES: Map<&[u8], Vec<Addr>> = Map::new("trades");

pub struct OfferModel<'a> {
    pub data: Offer,
    pub storage: &'a mut dyn Storage,
}

impl OfferModel<'_> {
    pub fn may_load<'a>(storage: &'a mut dyn Storage, id: &u64) -> OfferModel<'a> {
        let offermodal = OfferModel {
            data: OFFERS
                .may_load(storage, &id.to_be_bytes())
                .unwrap_or_default()
                .unwrap(),
            storage,
        };
        return offermodal;
    }

    pub fn save(&mut self) -> StdResult<()> {
        OFFERS.save(self.storage, &self.data.id.to_be_bytes(), &self.data)
    }

    pub fn activate(&mut self) -> StdResult<()> {
        self.data.state = OfferState::Active;
        self.save()
    }

    pub fn pause(&mut self) -> StdResult<()> {
        self.data.state = OfferState::Paused;
        self.save()
    }

    pub fn update(&mut self, msg: OfferMsg) -> StdResult<()> {
        self.data.offer_type = msg.offer_type;
        self.data.fiat_currency = msg.fiat_currency;
        self.data.min_amount = Uint128::from(msg.min_amount);
        self.data.max_amount = Uint128::from(msg.max_amount);
        self.save()
    }
}

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

pub fn query_all_offers(
    storage: &dyn Storage,
    fiat_currency: FiatCurrency,
) -> StdResult<Vec<Offer>> {
    let result: Vec<Offer> = OFFERS
        .range(storage, None, None, Order::Ascending)
        .flat_map(|item| item.and_then(|(_, offer)| Ok(offer)))
        .filter(|offer| offer.fiat_currency == fiat_currency)
        .collect();

    Ok(result)
}

pub fn query_all_trades(storage: &dyn Storage, maker: Addr) -> StdResult<Vec<Addr>> {
    let result = TRADES.load(storage, maker.as_bytes());
    Ok(result.unwrap_or(vec![]))
}
