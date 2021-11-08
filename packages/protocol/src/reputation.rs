use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub factory_addr: Addr,
    pub completed_with_like_weight: u128,
    pub completed_with_dislike_weight: u128,
    pub completed_weight: u128,
    pub refunded_weight: u128,
    pub refunded_with_dislike_weight: u128,
    pub closed_dispute_against_weight: u128,
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
