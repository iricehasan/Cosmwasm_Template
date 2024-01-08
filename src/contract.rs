pub mod query {
    use crate::{msg::ValueResp, state::COUNTER};
    use cosmwasm_std::{Deps, StdResult};

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        // Result<ValueResp, StdError>
        let counter = COUNTER.load(deps.storage)?; // If something goes wrong, Err(StdError)
        Ok(ValueResp { value: counter })
    }
}