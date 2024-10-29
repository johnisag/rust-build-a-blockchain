use std::collections::BTreeMap;

type  AccountId = String;
type Balance = u128;

// Pallet struct acting as the state and entry point for this module
#[derive(Debug)]
pub struct Pallet {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    balances: BTreeMap<AccountId, Balance>,
}

impl Pallet {
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

	/// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&0)
    }

    /// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
    pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<(), &'static str> {
        // get the balance of caller account
        let from_balance = self.balance(&from);

        // get the balance of `to` account
        let to_balance = self.balance(&to);

        // check if the caller has enough balance to transfer
        let new_from_balance = from_balance.checked_sub(amount).ok_or("sender does not have enough balance!")?;

        // calculate new `to` balance
        let new_to_balance = to_balance.checked_add(amount).ok_or("error in increasing the recipient balance!")?;

        // set `from` new balance
        self.set_balance(&from, new_from_balance);

        // set `to` new balance
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_balances() {
        let mut balances = Pallet::new();
        
        assert_eq!(balances.balance(&"alice".to_string()), 0);

        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance_valid() {
        // test that alice can transfer funds to bob and that the balances are updated correctly
        let mut balances = Pallet::new();

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
        let mut balances = Pallet::new();

        balances.set_balance(&"alice".to_string(), 50);
        balances.set_balance(&"bob".to_string(), 50);

        let res = balances.transfer("alice".to_string(), "bob".to_string(), 100);

        assert_eq!(res, Err("sender does not have enough balance!"));

        assert_eq!(balances.balance(&"alice".to_string()), 50);
        assert_eq!(balances.balance(&"bob".to_string()), 50);
    }
}
