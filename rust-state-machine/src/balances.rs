use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

// Combine all generic types and their trait bounds into a single `pub trait Config`.
//When you are done, your `Pallet` can simply be defined with `Pallet<T: Config>`.
pub trait Config {
    /// The type of account identifier.
    type AccountId: Ord + Clone;
    /// The type of balance.
    type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T>{
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

	/// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&Zero::zero())
    }

    /// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
    pub fn transfer(&mut self, from: T::AccountId, to: T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        // get the balance of caller account
        let from_balance = self.balance(&from);

        // get the balance of `to` account
        let to_balance = self.balance(&to);

        // check if the caller has enough balance to transfer
        let new_from_balance = from_balance.checked_sub(&amount).ok_or("sender does not have enough balance!")?;

        // calculate new `to` balance
        let new_to_balance = to_balance.checked_add(&amount).ok_or("error in increasing the recipient balance!")?;

        // set `from` new balance
        self.set_balance(&from, new_from_balance);

        // set `to` new balance
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    pub struct TestConfig {}

    impl super::Config for TestConfig {
        type AccountId = String;
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::<TestConfig>::new();
        
        assert_eq!(balances.balance(&"alice".to_string()), 0);

        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance_valid() {
        // test that alice can transfer funds to bob and that the balances are updated correctly
        let mut balances = super::Pallet::<TestConfig>::new();

        // CASE A: `alice` can successfully transfer funds to `bob`.
        balances.set_balance(&"alice".to_string(), 100);
        balances.set_balance(&"bob".to_string(), 50);

        let res = balances.transfer("alice".to_string(), "bob".to_string(), 50);

        assert_eq!(res, Ok(()));

        assert_eq!(balances.balance(&"alice".to_string()), 50);
        assert_eq!(balances.balance(&"bob".to_string()), 100);

    }

    #[test]
    fn transfer_balance_insufficient_funds() {
        // test that the balances are not updated if the transfer would cause an underflow
        let mut balances = super::Pallet::<TestConfig>::new();

        balances.set_balance(&"alice".to_string(), 50);
        balances.set_balance(&"bob".to_string(), 50);

        let res = balances.transfer("alice".to_string(), "bob".to_string(), 100);

        assert_eq!(res, Err("sender does not have enough balance!"));

        assert_eq!(balances.balance(&"alice".to_string()), 50);
        assert_eq!(balances.balance(&"bob".to_string()), 50);
    }
}
