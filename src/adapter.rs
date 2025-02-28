#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use stylus_sdk::alloy_primitives::{Address, U256};
use stylus_sdk::stylus_proc::external;

#[external]
trait IAggregatorAdapterInterface {
    fn get_rates(asset: Address) -> (U256, U256);
    fn supply(asset: Address, amount: U256, user: Address);
    fn withdraw(asset: Address, amount: U256, user: Address);
    fn borrow(asset: Address, amount: U256, user: Address);
    fn get_protocol_address() -> Address;
}
