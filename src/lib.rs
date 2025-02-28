#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::Address, prelude::*};

// Define some persistent storage using the Solidity ABI.
// `LendingAggregator` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct LendingAggregator {
        address treasury;
    }
}

/// Declare that `LendingAggregator` is a contract with the following external methods.
#[public]
impl LendingAggregator {
    /// Gets the treasury address from storage.
    pub fn treasury(&self) -> Address {
        self.treasury.get()
    }

    /// Sets the treasury address in storage to a user-specified value.
    pub fn set_treasury(&mut self, new_treasury: Address) {
        self.treasury.set(new_treasury);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use stylus_sdk::testing::*;

    #[test]
    fn test_lending_aggregator() {
        let vm = TestVM::default();
        let mut contract = LendingAggregator::from(&vm);

        // Initialize treasury address to zero address
        assert_eq!(Address::ZERO, contract.treasury());

        // Set a new treasury address
        let new_treasury_address = Address::from_slice(&[1u8; 20]);
        contract.set_treasury(new_treasury_address);
        assert_eq!(new_treasury_address, contract.treasury());
    }
}
