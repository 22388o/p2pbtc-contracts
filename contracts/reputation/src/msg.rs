use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub completed_with_like_weight: Uint128,
    pub completed_with_dislike_weight: Uint128,
    pub completed_weight: Uint128,
    pub refunded_weight: Uint128,
    pub refunded_with_dislike_weight: Uint128,
    pub closed_dispute_against_weight: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        completed_with_like_weight: Uint128,
        completed_with_dislike_weight: Uint128,
        completed_weight: Uint128,
        refunded_weight: Uint128,
        refunded_with_dislike_weight: Uint128,
        closed_dispute_against_weight: Uint128,
    },
    SaveCompleteTradeRating {
        maker: String,
        liked: bool,
    },
    SaveRefundTradeRating {
        maker: String,
        liked: bool,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetScore { maker: String },
    GetAllTrades { maker: String },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ScoreResponse {
    pub score: i32,
}
