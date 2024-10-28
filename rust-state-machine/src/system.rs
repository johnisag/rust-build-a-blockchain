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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_system() {
        let system = Pallet::new();
        assert_eq!(system.block_number, 0);
        assert_eq!(system.nonce.len(), 0);
    }
}