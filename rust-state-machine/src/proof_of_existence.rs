use core::fmt::Debug;
use std::collections::BTreeMap;
use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		Self {
            claims: BTreeMap::new(),
        }
	}

	/// Get the owner (if any) of a claim.
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		// `get` the `claim` from the `claims` map.
		self.claims.get(claim)
	}

	/// Create a new claim on behalf of the `caller`.
	/// This function will return an error if someone already has claimed that content.
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		// Check that a `claim` does not already exist. If so, return an error. 
		// TODO: `insert` the claim on behalf of `caller`. 
		match self.claims.get(&claim) {
			Some(_) => Err("Claim already exists"),
			None => {
				self.claims.insert(claim, caller);
				Ok(())
			}
		}
	}

	/// Revoke an existing claim on some content.
	/// This function should only succeed if the caller is the owner of an existing claim.
	/// It will return an error if the claim does not exist, or if the caller is not the owner.
	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let claim_owner = self.get_claim(&claim).ok_or("Claim does not exist")?;

		if claim_owner != &caller {
			return Err("Caller is not the owner of the claim");
		}

		self.claims.remove(&claim);

		Ok(())
	}
}

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum Call<T: Config> {
	// Create variants for create and revoke claims.
	RevokeClaim { claim: T::Content },
	CreateClaim { claim: T::Content },
}

/// Implementation of the dispatch logic, mapping from `POECall` to the appropriate underlying
/// function we want to execute.
impl<T: Config> crate::support::Dispatch for Pallet<T> {

	type Caller = T::AccountId;
	type Call = Call<T>;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
		match call {
			// Match on the `call` variant and call the appropriate function.
			Call::CreateClaim { claim } => self.create_claim(caller, claim),
			Call::RevokeClaim { claim } => self.revoke_claim(caller, claim),
		}
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		// craete a new instance of the Proof of Existence Module.
		let mut poe = super::Pallet::<TestConfig>::new();

		// Create a claim on behalf of `alice`.
		poe.create_claim("alice", "Hello, World").unwrap();

		// Check that the claim is what we expect.
		assert_eq!(poe.get_claim(&"Hello, World"), Some(&"alice"));

		// Revoke the claim on behalf of `alice`.
		poe.revoke_claim("alice", "Hello, World").unwrap();

		// Check that the claim is no longer in the system.
		assert_eq!(poe.get_claim(&"Hello, World"), None);

		// Check that revoking a claim that does not exist fails.
		assert_eq!(poe.revoke_claim("alice", "Hello, World"), Err("Claim does not exist"));
	}
}