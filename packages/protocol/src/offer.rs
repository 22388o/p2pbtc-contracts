use super::constants::OFFERS_KEY;
use crate::currencies::FiatCurrency;
use crate::trade::State as TradeState;
use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{self};

pub static CONFIG_KEY: &[u8] = b"config";
pub const OFFERS: Map<&[u8], Offer> = Map::new(OFFERS_KEY);

///Messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OfferMsg {
    pub offer_type: OfferType,
    pub fiat_currency: FiatCurrency,
    pub min_amount: u64,
    pub max_amount: u64, // TODO change to Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Create {
        offer: OfferMsg,
    },
    Pause {
        id: u64,
    },
    Activate {
        id: u64,
    },
    Update {
        id: u64,
        offer: OfferMsg,
    },
    NewTrade {
        offer_id: u64,
        ust_amount: String,
        counterparty: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {},
    Offers { fiat_currency: FiatCurrency },
    Offer { id: u64 },
    Trades { maker: String },
}

///Data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub factory_addr: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub offers_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Offer {
    pub id: u64,
    pub owner: Addr,
    pub offer_type: OfferType,
    pub fiat_currency: FiatCurrency,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub state: OfferState,
}

pub struct OfferModel<'a> {
    pub offer: Offer,
    pub storage: &'a mut dyn Storage,
}

impl OfferModel {
    pub fn store(storage: &mut dyn Storage, offer: &Offer) -> StdResult<()> {
        OFFERS.save(storage, &offer.id.to_be_bytes(), &offer)
    }

    pub fn fetch(storage: &mut dyn Storage, id: &u64) -> StdResult<Option<Offer>> {
        OFFERS.may_load(storage, &id.to_be_bytes())
    }

    pub fn create(storage: &mut dyn Storage, offer: Offer) -> OfferModel {
        Offer::store(storage, &offer);
        OfferModel { offer, storage }
    }

    pub fn save(self) -> StdResult<Offer> {
        Offer::store(storage, &self);
        Ok(self.offer)
    }

    pub fn may_load<'a>(storage: &'a mut dyn Storage, id: &u64) -> OfferModel<'a> {
        let offer_model = OfferModel {
            offer: Offer
                .fetch(storage, &id.to_be_bytes())
                .unwrap_or_default()
                .unwrap(),
            storage,
        };
        return offer_model;
    }

    pub fn activate(&mut self) -> StdResult<Offer> {
        self.state = OfferState::Active;
        self.save()
    }

    pub fn pause(&mut self) -> StdResult<Offer> {
        self.state = OfferState::Paused;
        self.save()
    }

    pub fn update(&mut self) -> StdResult<Offer> {
        self.offer_type = msg.offer_type;
        self.fiat_currency = msg.fiat_currency;
        self.min_amount = Uint128::from(msg.min_amount);
        self.max_amount = Uint128::from(msg.max_amount);
        self.save()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TradeInfo {
    pub trade: TradeState,
    pub offer: Offer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OfferType {
    Buy,
    Sell,
}
impl fmt::Display for OfferType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OfferState {
    Active,
    Paused,
}
