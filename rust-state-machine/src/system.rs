use std::collections::BTreeMap;
use num::traits::{Zero, One};

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
	/// The current block number.
    block_number: BlockNumber,

	/// A map from an account to their nonce.
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
    AccountId: Ord + Clone,
    BlockNumber: Zero + One + Copy + std::ops::AddAssign,
    Nonce: One + Copy + std::ops::AddAssign,
    {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
        Self {
            block_number: Zero::zero(),
            nonce: BTreeMap::new(),
        }
	}

    /// Get the current block number.
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += One::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &AccountId) {
		self.nonce.entry(who.clone()).and_modify(|e| *e += One::one()).or_insert(One::one());
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_system() {
        let mut system = Pallet::<String, u32, u32>::new();
        
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