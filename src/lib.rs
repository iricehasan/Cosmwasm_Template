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

#[cfg(test)]
mod test {
    use crate::{
        execute, instantiate,
        msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ValueResp},
        query,
    };
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    #[test]
    fn proper_value_return() {
        // Create app
        let mut app = App::default();

        // Get contract id
        let contract_id = app.store_code(counting_contract());

        // Instantiate contract
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter_value: 1 },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 });
    }

    #[test]
    fn proper_incrementation() {
        // Create app
        let mut app = App::default();

        // Get contract id
        let contract_id = app.store_code(counting_contract());

        // Instantiate contract
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter_value: 0 },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        // Execute contract
        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecuteMsg::Increment {},
            &[],
        )
        .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 });
    }

    #[test]
    fn proper_decrementation() {
        // Create app
        let mut app = App::default();

        // Get contract id
        let contract_id = app.store_code(counting_contract());

        // Instantiate contract
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter_value: 1 },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        // Execute contract
        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecuteMsg::Decrement {},
            &[],
        )
        .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 0 });
        
    }

    #[test]
    fn zero_case_decrementation() {
         // Create app
         let mut app = App::default();

         // Get contract id
         let contract_id = app.store_code(counting_contract());
 
         // Instantiate contract
         let contract_addr = app
             .instantiate_contract(
                 contract_id,
                 Addr::unchecked("sender"),
                 &InstantiateMsg { counter_value: 0 },
                 &[],
                 "Counting contract",
                 None,
             )
             .unwrap();
 
         // Execute contract
         app.execute_contract(
             Addr::unchecked("sender"),
             contract_addr.clone(),
             &ExecuteMsg::Decrement {},
             &[],
         )
         .unwrap();
 
         let resp: ValueResp = app
             .wrap()
             .query_wasm_smart(contract_addr, &QueryMsg::Value {})
             .unwrap();
 
         assert_eq!(resp, ValueResp { value: 0 }); // it does not decrement below zero
    }

    #[test]
    fn proper_reset() {
        // Create app
        let mut app = App::default();

        // Get contract id
        let contract_id = app.store_code(counting_contract());

        // Instantiate contract
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter_value: 0 },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        // Execute contract
        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecuteMsg::Reset { value: 5 },
            &[],
        )
        .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 5 });
    }

    #[test]
    fn reset_by_non_owner() {
         // Create app
         let mut app = App::default();

         // Get contract id
         let contract_id = app.store_code(counting_contract());
 
         // Instantiate contract
         let contract_addr = app
             .instantiate_contract(
                 contract_id,
                 Addr::unchecked("sender"),
                 &InstantiateMsg { counter_value: 0 },
                 &[],
                 "Counting contract",
                 None,
             )
             .unwrap();
 
         // Execute contract
         app.execute_contract(
             Addr::unchecked("random"),
             contract_addr.clone(),
             &ExecuteMsg::Reset { value: 5 },
             &[],
         )
         .unwrap_err(); // it should result in err 

    }
}