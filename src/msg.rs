use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub counter_value: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { value: u64 },
    Decrement {},
}

#[cw_serde]
pub enum QueryMsg {
    Value {},
}

#[cw_serde]
pub struct ValueResp {
    pub value: u64,
}