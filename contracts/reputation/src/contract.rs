#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use std::ops::Add;

use crate::error::ContractError;
use crate::error::ContractError::Unauthorized;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, SCORES, STATE};
use localterra_protocol::reputation::{Config, Score};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = Config {
        factory_addr: info.sender.clone(),
        completed_with_like_weight: u128::from(msg.completed_with_like_weight),
        completed_with_dislike_weight: u128::from(msg.completed_with_dislike_weight),
        completed_weight: u128::from(msg.completed_weight),
        refunded_weight: u128::from(msg.refunded_weight),
        refunded_with_dislike_weight: u128::from(msg.refunded_with_dislike_weight),
        closed_dispute_against_weight: u128::from(msg.closed_dispute_against_weight),
    };

    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("factory_addr", info.sender)
        .add_attribute("completed_with_like_weight", msg.completed_with_like_weight)
        .add_attribute(
            "completed_with_dislike_weight",
            msg.completed_with_dislike_weight,
        )
        .add_attribute("completed_weight", msg.completed_weight)
        .add_attribute("refunded_weight", msg.refunded_weight)
        .add_attribute(
            "refunded_with_dislike_weight",
            msg.refunded_with_dislike_weight,
        )
        .add_attribute(
            "closed_dispute_against_weight",
            msg.closed_dispute_against_weight,
        ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig { .. } => update_config(deps, info, msg),
        ExecuteMsg::SaveCompleteTradeRating { maker, liked } => {
            save_rating(deps, info, maker, true, liked)
        }
        ExecuteMsg::SaveRefundTradeRating { maker, liked } => {
            save_rating(deps, info, maker, false, liked)
        }
    }
}

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    msg: ExecuteMsg::UpdateConfig,
) -> Result<Response, ContractError> {
    let cfg = STATE.load(deps.storage).unwrap();

    // Just the adm can change the reputation config
    if info.sender != cfg.factory_addr {
        Err(Unauthorized {})
    }

    let config = Config {
        factory_addr: cfg.factory_addr,
        completed_with_like_weight: u128::from(msg.completed_with_like_weight),
        completed_with_dislike_weight: u128::from(msg.completed_with_dislike_weight),
        completed_weight: u128::from(msg.completed_weight),
        refunded_weight: u128::from(msg.refunded_weight),
        refunded_with_dislike_weight: u128::from(msg.refunded_with_dislike_weight),
        closed_dispute_against_weight: u128::from(msg.closed_dispute_against_weight),
    };

    STATE.save(deps.storage, &config).unwrap();

    Ok(Response::new()
        .add_attribute("method", "update_config")
        .add_attribute("completed_with_like_weight", completed_with_like_weight)
        .add_attribute(
            "completed_with_dislike_weight",
            completed_with_dislike_weight,
        )
        .add_attribute("completed_weight", completed_weight)
        .add_attribute("refunded_weight", refunded_weight)
        .add_attribute("refunded_with_dislike_weight", refunded_with_dislike_weight)
        .add_attribute(
            "closed_dispute_against_weight",
            closed_dispute_against_weight,
        ))
}

pub fn save_rating(
    deps: DepsMut,
    info: MessageInfo,
    maker: String,
    completed_trade: bool,
    liked: bool,
) -> Result<Response, ContractError> {
    let score = SCORES.load(deps.storage, maker.clone()).unwrap_or(Score {
        trade_completed: 0,
        trade_completed_with_like: 0,
        trade_completed_with_dislike: 0,
        trade_refunded: 0,
        trade_refunded_with_dislike: 0,
        closed_dispute_against: 0,
    });

    let trade_completed_with_like = if completed_trade && liked {
        score.trade_completed_with_like.add(1)
    } else {
        score.trade_completed_with_like
    };

    let trade_completed_with_dislike = if completed_trade && liked {
        score.trade_completed_with_dislike.add(1)
    } else {
        score.trade_completed_with_dislike
    };

    let trade_refunded = if completed_trade && liked {
        score.trade_refunded.add(1)
    } else {
        score.trade_refunded
    };

    let trade_refunded_with_dislike = if completed_trade && liked {
        score.trade_refunded_with_dislike.add(1)
    } else {
        score.trade_refunded_with_dislike
    };

    let newScore = Score {
        trade_completed: score.trade_completed,
        trade_completed_with_like,
        trade_completed_with_dislike,
        trade_refunded,
        trade_refunded_with_dislike,
        closed_dispute_against: score.closed_dispute_against,
    };

    SCORES.save(deps.storage, maker, &newScore).unwrap();

    Ok(Response::new()
        .add_attribute("method", "save_rating")
        .add_attribute("rating", newScore))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}
