#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use stylus_sdk::alloy_primitives::{Address, U256};
use stylus_sdk::prelude::*;

pub trait LendingAggregatorParams {
    const PROTOCOL_FEE_PERCENT: U256;
}

pub struct DefaultParams;
impl LendingAggregatorParams for DefaultParams {
    const PROTOCOL_FEE_PERCENT: U256 = U256::from_limbs([20, 0, 0, 0]);
}

pub trait AggregatorAdapter {
    fn get_rates(&self, asset: Address) -> (U256, U256);
    fn supply(&mut self, asset: Address, amount: U256, user: Address);
    fn withdraw(&mut self, asset: Address, amount: U256, user: Address);
    fn borrow(&mut self, asset: Address, amount: U256, user: Address);
    fn get_protocol_address(&self) -> Address;
}

sol_storage! {
    #[entrypoint]
    pub struct LendingAggregator {
        address treasury;
        uint256 adapter_count;
        mapping(address => uint256) adapters;
        mapping(uint256 => address) adapter_addresses;
        mapping(uint256 => string) adapters_name;
    }
}

#[public]
impl LendingAggregator {
    pub fn init(&mut self, treasury: Address) {
        self.treasury.set(treasury);
        self.adapter_count.set(U256::ZERO);
    }

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
        self.adapter_addresses
            .insert(current_adapter_count, address);
        let next_protocol_count = current_adapter_count + U256::from(1);
        self.set_adapter_name(next_protocol_count, name);
        self.set_adapter_count(next_protocol_count);
    }

    pub fn get_protocol_fee_percent(&self) -> U256 {
        DefaultParams::PROTOCOL_FEE_PERCENT
    }

    pub fn calculate_fee(amount: U256) -> (U256, U256) {
        let fee = (amount * DefaultParams::PROTOCOL_FEE_PERCENT) / U256::from(10000);
        let net_amount = amount - fee;
        (fee, net_amount)
    }

    pub fn get_best_deposit_rate(&self, asset: Address) -> (U256, U256) {
        let mut best_rate = U256::ZERO;
        let mut best_protocol_id = U256::ZERO;

        for i in 0..self.adapter_count.get().as_limbs()[0] {
            let adapter_address = self.adapter_addresses.get(U256::from(i));
            let adapter = Self::get_adapter_instance_private(self, adapter_address);
            let (deposit_rate, _) = adapter.get_rates(asset);
            if deposit_rate > best_rate {
                best_rate = deposit_rate;
                best_protocol_id = U256::from(i);
            }
        }
        (best_rate, best_protocol_id)
    }

    pub fn get_best_borrow_rate(&self, asset: Address) -> (U256, U256) {
        let mut best_rate = U256::MAX;
        let mut best_protocol_id = U256::ZERO;

        for i in 0..self.adapter_count.get().as_limbs()[0] {
            let adapter_address = self.adapter_addresses.get(U256::from(i));
            let adapter = Self::get_adapter_instance_private(self, adapter_address);
            let (_, borrow_rate) = adapter.get_rates(asset);
            if borrow_rate < best_rate {
                best_rate = borrow_rate;
                best_protocol_id = U256::from(i);
            }
        }
        (best_rate, best_protocol_id)
    }
}

// Internal helper functions (not part of the ABI)
impl LendingAggregator {
    fn get_adapter_instance_private(&self, adapter_address: Address) -> ConcreteAdapter {
        let adapter_index = self.adapters.get(adapter_address);
        let protocol_address = self.adapter_addresses.get(adapter_index);
        ConcreteAdapter::new(protocol_address)
    }
}

#[derive(Clone)]
struct ConcreteAdapter {
    protocol_address: Address,
}

impl ConcreteAdapter {
    fn new(protocol_address: Address) -> Self {
        Self { protocol_address }
    }
}

impl AggregatorAdapter for ConcreteAdapter {
    fn get_rates(&self, _asset: Address) -> (U256, U256) {
        (U256::from(100), U256::from(200))
    }
    fn supply(&mut self, _asset: Address, _amount: U256, _user: Address) {}
    fn withdraw(&mut self, _asset: Address, _amount: U256, _user: Address) {}
    fn borrow(&mut self, _asset: Address, _amount: U256, _user: Address) {}
    fn get_protocol_address(&self) -> Address {
        self.protocol_address
    }
}
