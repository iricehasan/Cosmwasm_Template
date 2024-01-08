use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

// Contract Logic
use contract::query;

// State variables
use state::COUNTER;

// Messages
use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// Modules
mod contract;
pub mod msg;
mod state;


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    COUNTER.save(deps.storage, &msg.counter_value)?;

    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        // Call functions from the exec module depending on the message received
        ExecuteMsg::Increment {} => todo!(),
        ExecuteMsg::Reset { value } => todo!(),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Value {} => to_json_binary(&query::value(deps)?),
    }
}
