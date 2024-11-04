use std::collections::BTreeMap;
use num::traits::{Zero, One};

/// Combine all generic types and their trait bounds into a single `pub trait Config`.
/// When you are done, your `Pallet` can simply be defined with `Pallet<T: Config>`.
pub trait Config {
    /// The type of account identifier.
    type AccountId: Ord + Clone;
    /// The type of block number.
    type BlockNumber: Zero + One + Copy + std::ops::AddAssign;
    /// The type of nonce.
    type Nonce: One + Copy + std::ops::AddAssign;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
    block_number: T::BlockNumber,

	/// A map from an account to their nonce.
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
        Self {
            block_number: Zero::zero(),
            nonce: BTreeMap::new(),
        }
	}

    /// Get the current block number.
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += One::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		self.nonce.entry(who.clone()).and_modify(|e| *e += One::one()).or_insert(One::one());
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig {}

    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let mut system = Pallet::<TestConfig>::new();
        
        // Increment the current block number.
        system.inc_block_number();

        // Increment the nonce of `alice`.
        system.inc_nonce(&"alice".to_string());

        // Check the block number is what we expect.
        assert_eq!(system.block_number(), 1);

        // Check the nonce of `alice` is what we expect.
        assert_eq!(system.nonce.get(&"alice".to_string()), Some(&1));
    }
}