pub mod exec {
    use crate::state::{COUNTER, OWNER};
    use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult, StdError};

    pub fn increment(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        // load the counter from the storage and increment it by one
        let counter = COUNTER
            .load(deps.storage)?
            .checked_add(1u64)
            .unwrap_or(COUNTER.load(deps.storage)?);

        // This is an alternatives which does not check for overflow
        // let mut counter = COUNTER.load(deps.storage)?;
        // counter += 1;

        // save the counter
        COUNTER.save(deps.storage, &counter)?;

        // return response
        Ok(Response::new()
            .add_attribute("action", "increment")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string()))
    }

    pub fn reset(deps: DepsMut, info: MessageInfo, counter_value: u64) -> StdResult<Response> {

        let owner = OWNER.load(deps.storage)?;

        if owner != info.sender {
            return Err(StdError::generic_err("Only the owner can reset!"));
        }
        // save the counter
        COUNTER.save(deps.storage, &counter_value)?;

        // return response
        Ok(Response::new()
            .add_attribute("action", "reset")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter_value.to_string()))
    }

    pub fn decrement(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        // load the counter from the storage and decrement it by one
        let counter = COUNTER
            .load(deps.storage)?
            .checked_sub(1u64)
            .unwrap_or(COUNTER.load(deps.storage)?);

        // This is an alternative
        // let mut counter = COUNTER.load(deps.storage)?;
        /* 
        if counter > 0 {
            counter -= 1;
        }
        */
    
        // save the decremented counter
        COUNTER.save(deps.storage, &counter)?;

        // return response
        Ok(Response::new()
            .add_attribute("action", "decrement")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string()))
    }  

}

pub mod query {
    use crate::{msg::ValueResp, state::COUNTER};
    use cosmwasm_std::{Deps, StdResult};

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        // Result<ValueResp, StdError>
        let counter = COUNTER.load(deps.storage)?; // If something goes wrong, Err(StdError)
        Ok(ValueResp { value: counter })
    }
}