use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

// Contract Logic
use contract::{exec, query};

// State variables
use state::{COUNTER, OWNER};

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
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    COUNTER.save(deps.storage, &msg.counter_value)?;
    OWNER.save(deps.storage, &info.sender)?;

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
        ExecuteMsg::Increment {} => exec::increment(deps, info),
        ExecuteMsg::Reset { value } => exec::reset(deps, info, value),
        ExecuteMsg::Decrement {} => exec::decrement(deps, info),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Value {} => to_json_binary(&query::value(deps)?),
    }
}
