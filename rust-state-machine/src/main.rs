mod balances;
mod proof_of_existence;
mod system;
mod support;

use crate::support::Dispatch;

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
    use crate::support;

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
    pub type Content = String;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[macros::runtime]
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

// Implement the `balances::Config` trait you created on your `Runtime`.
// Use `Self` to satisfy the generic parameter required for `balances::Pallet`.
impl balances::Config for Runtime {
    type Balance = types::Balance;
}

// Implement the `proof_of_existence::Config` trait you created on your `Runtime`.
// Use `Self` to satisfy the generic parameter required for `proof_of_existence::Pallet`.
impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

fn main() {
	// Create a new instance of the Runtime.
	// It will instantiate with it all the modules it uses.
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();

	// Initialize the system with some initial balance.
	runtime.balances.set_balance(&alice, 100);

    // Create a new block with the extrinsics.
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer { to: bob.clone(), amount: 20 }),
            }
        ],
    };

    // create a new block with the extrinsics for the proof of existence module.
    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim { claim: "Hello, World!".to_string() }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("invalid block");

    runtime.execute_block(block_2).expect("invalid block");

	// Simply print the debug format of our runtime state.
	println!("{:#?}", runtime);
}