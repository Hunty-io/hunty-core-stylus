use stylus_sdk::alloy_primitives::{Address, U256};
use stylus_sdk::prelude::*;

/// The adapter interface that all lending protocol adapters must implement.
pub trait AggregatorAdapter {
    fn get_rates(&self, asset: Address) -> (U256, U256);
    fn supply(&mut self, asset: Address, amount: U256, user: Address);
    fn withdraw(&mut self, asset: Address, amount: U256, user: Address);
    fn borrow(&mut self, asset: Address, amount: U256, user: Address);
    fn get_protocol_address(&self) -> Address;
}

/// A concrete adapter implementation (example) that implements the AggregatorAdapter trait.
#[derive(Clone)]
pub struct ConcreteAdapter {
    pub protocol_address: Address,
}

impl ConcreteAdapter {
    pub fn new(protocol_address: Address) -> Self {
        Self { protocol_address }
    }
}

impl AggregatorAdapter for ConcreteAdapter {
    fn get_rates(&self, _asset: Address) -> (U256, U256) {
        // Example rates; in a real adapter, this would query the underlying protocol.
        (U256::from(100), U256::from(200))
    }

    fn supply(&mut self, _asset: Address, _amount: U256, _user: Address) {
        // Implementation to supply funds.
    }

    fn withdraw(&mut self, _asset: Address, _amount: U256, _user: Address) {
        // Implementation to withdraw funds.
    }

    fn borrow(&mut self, _asset: Address, _amount: U256, _user: Address) {
        // Implementation to borrow funds.
    }

    fn get_protocol_address(&self) -> Address {
        self.protocol_address
    }
}
