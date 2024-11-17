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

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
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

impl Runtime {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
        }
    }

	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        // Increment the system's block number.
        self.system.inc_block_number();

        // Check that the block number of the incoming block matches the current block number,
        // or return an error.
        if block.header.block_number != self.system.block_number() {
            return Err("Block number does not match current block number.");
        }

        // Iterate over the extrinsics in the block...
        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            // Increment the nonce of the caller.
            self.system.inc_nonce(&caller);

            // Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
            let _ = self.dispatch(caller, call).map_err(|e| 
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            );
        }

        // Return `Ok(())` if everything is successful.
		Ok(())
	}
}


impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            },
            RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            },
        }

        Ok(())
	}
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
                call: RuntimeCall::Balances(balances::Call::transfer { to: bob.clone(), amount: 20 }),
            }
        ],
    };

    // create a new block with the extrinsics for the proof of existence module.
    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim { claim: "Hello, World!".to_string() }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("invalid block");

    runtime.execute_block(block_2).expect("invalid block");

	// Simply print the debug format of our runtime state.
	println!("{:#?}", runtime);
}