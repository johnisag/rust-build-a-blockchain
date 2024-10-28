use std::collections::BTreeMap;


/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
	/// The current block number.
    block_number: u32,

	/// A map from an account to their nonce.
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
	}

    /// Get the current block number.
    pub fn block_number(&self) -> u32 {
        self.block_number
    }

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &String) {
		self.nonce.entry(who.clone()).and_modify(|e| *e += 1).or_insert(1);
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_system() {
        let mut system = Pallet::new();
        
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