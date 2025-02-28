#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use stylus_sdk::alloy_primitives::{Address, U256};
use stylus_sdk::prelude::*;

sol_storage! {
    #[entrypoint]
    pub struct LendingAggregator {
        address treasury;
        uint256 adapter_count;
        mapping(address => uint256) adapters;
        mapping(uint256 => string) adapters_name;
    }
}

static PROTOCOL_FEE_PERCENT: U256 = U256::ZERO.saturating_add(U256::from_limbs([20, 0, 0, 0]));

#[public]
impl LendingAggregator {
    pub fn treasury(&self) -> Address {
        self.treasury.get()
    }

    pub fn set_treasury(&mut self, new_treasury: Address) {
        self.treasury.set(new_treasury);
    }

    pub fn adapter_count(&self) -> U256 {
        self.adapter_count.get()
    }

    pub fn get_adapter(&self, key: Address) -> U256 {
        self.adapters.get(key)
    }

    fn set_adapter_count(&mut self, new_value: U256) {
        self.adapter_count.set(new_value);
    }

    fn set_adapter_name(&mut self, key: U256, name: String) {
        self.adapters_name.setter(key).set_str(name);
    }

    pub fn set_adapter(&mut self, address: Address, name: String) {
        let current_adapter_count: U256 = self.adapter_count();
        self.adapters.insert(address, current_adapter_count);

        let next_protocol_count = current_adapter_count + U256::from(1);

        self.set_adapter_name(next_protocol_count, name);
        self.set_adapter_count(next_protocol_count);
    }

    pub fn get_protocol_fee_percent(&self) -> U256 {
        PROTOCOL_FEE_PERCENT
    }

    fn calculate_fee(amount: U256) -> (U256, U256) {
        let fee = (amount * PROTOCOL_FEE_PERCENT) / U256::from(10000);
        let net_amount = amount - fee;
        (fee, net_amount)
    }
}
