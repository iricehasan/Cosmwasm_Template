use cw_storage_plus::Item;
use cosmwasm_std::Addr;

pub const OWNER: Item<Addr> = Item::new("owner");
pub const COUNTER: Item<u64> = Item::new("counter");