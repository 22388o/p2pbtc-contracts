use cosmwasm_std::{Addr, StdResult, Storage, Uint128};
use cosmwasm_storage::{bucket, bucket_read};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::ops::Div;

pub static SCORE_RULES_KEY: &[u8] = b"score_rules";
pub static SCORES_KEY: &[u8] = b"scores";

pub fn score_rules_storage(storage: &mut dyn Storage) -> Singleton<ScoreRules> {
    singleton(storage, SCORE_RULES_KEY)
}

pub fn score_rules_read(storage: &dyn Storage) -> ReadonlySingleton<ScoreRules> {
    singleton_read(storage, SCORE_RULES_KEY)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ScoreRules {
    pub trade_completed_with_like_weight: u128,
    pub trade_completed_with_dislike_weight: u128,
    pub trade_completed_weight: u128,
    pub trade_refunded_weight: u128,
    pub trade_refunded_with_dislike_weight: u128,
    pub closed_dispute_against_weight: u128,
}

fn score_read(storage: &dyn Storage, maker_addr: Addr) -> StdResult<Score> {
    bucket_read(storage, SCORES_KEY).load(&maker_addr.as_bytes())
}

fn score_storage(storage: &mut dyn Storage, maker_addr: Addr, score: &Score) -> StdResult<()> {
    bucket(storage, SCORES_KEY).save(&maker_addr.as_bytes(), score)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Score {
    pub trade_completed: u128,
    pub trade_completed_with_like: u128,
    pub trade_completed_with_dislike: u128,
    pub trade_refunded: u128,
    pub trade_refunded_with_dislike: u128,
    pub closed_dispute_against: u128,
}

impl Score {
    fn total_trades(&self) -> u128 {
        self.trade_completed
            + self.trade_completed_with_like
            + self.trade_completed_with_dislike
            + self.trade_refunded
            + self.trade_refunded_with_dislike
    }

    fn calculate_reputation(&self, rules: ScoreRules) -> f64 {
        let trade_completed_rating = self.trade_completed * rules.trade_completed_weight;
        let trade_completed_with_like_rating =
            self.trade_completed_with_like * rules.trade_completed_with_like_weight;
        let trade_completed_with_dislike_rating =
            self.trade_completed_with_dislike * rules.trade_completed_with_dislike_weight;
        let trade_refunded_rating = self.trade_refunded * rules.trade_refunded_weight;
        let trade_refunded_with_dislike_rating =
            self.trade_refunded_with_dislike * rules.trade_refunded_with_dislike_weight;

        (trade_completed_rating
            + trade_completed_with_like_rating
            + trade_completed_with_dislike_rating
            + trade_refunded_rating
            + trade_refunded_with_dislike_rating)
            .div(self.total_trades()) as f64
    }
}
