You can find the [solution for the previous step here](https://gist.github.com/nomadbitcoin/6950ef0c080da83e41eaf2aaa5dde20b).

# Use the Runtime Macro

Finally, let's add the `#[macros::runtime]` macro to our `main.rs` file, and really clean up a ton of boilerplate code.

## Runtime Macro

[Youtube](https://www.youtube.com/watch?v=VTg4SCSgfsI&t=1s)

The purpose of the `#[macros::runtime]` macro is to get rid of all of the boilerplate function we implemented for the `Runtime`, including `fn new()` and `fn execute_block()`. Similar to the `Call` macro, it also generates the `enum RuntimeCall` and all the `dispatch` logic for re-dispatching to pallets.

We apply the `#[macros::runtime]` attribute on top of the main `struct Runtime` object.

### Parse

In order to generate the code we want, we need to keep track of:

1. The name of the `struct` representing our Runtime. Usually this is `Runtime`, but we provide flexibility to the developer.
2. The list of Pallets included in our `Runtime`
	1. Their name, as specified by the user.
	2. The specific type for their `Pallet`, for example `balances::Pallet` vs `proof_of_existence::Pallet`.

All of this information is tracked in the `RuntimeDef` struct.

We are also checking that our `Runtime` definition always contains the System Pallet, and does so as the first pallet in our `Runtime` definition. We will explain more about the assumption of the macros below.

### Expand

Once we have parsed all the data we need, we just need to generate the code that we expect.

Starting with `let runtime_impl = quote!`, you will see the entire `impl Runtime` code block has been swallowed into the macro. Since we know all the pallets in your `Runtime`, we can automatically implement functions like `new()`. The `execute_block` function does not take advantage of any of the parsed data, but the code is completely boilerplate, so we hide it away.

Then we have another code block being generated with `let dispatch_impl = quote!` which is the `enum RuntimeCall` and the implementation of `Dispatch for Runtime`.

Again, due to the quirks of using macros, our `RuntimeCall` enum will have `snake_case` variants which exactly match the name of the fields in the `Runtime` struct.

## Macro Assumptions

One of the assumptions programmed into these macros is the existence of the System Pallet. For example, in the `execute_block` logic, we need access to both `system.inc_block_number` and `system.inc_nonce`.

Some macro level assumptions are intentional, and actually define the architectural decisions of the framework designing those macros. This is the case with the System Pallet, since so much of a blockchain framework depends on a consistent meta-layer.

Other assumptions exist just because it is easier to write the macro if the assumption is made.

The main takeaway here is that macros can almost always continue to improve, providing better and better user experiences for developers. It just needs someone to identify what improvements need to be made, and someone else to program those improvements into the low level macro code.

# Exercises:

### Add the Runtime Macro

Let's finally go through the steps to add the `#[macros::runtime]` attribute to your `Runtime`.

1. In `main.rs`, add `#[macros::runtime]` on top of your `pub struct Runtime`.
2. Remove the entire `impl Runtime` code block.
3. Remove the entire `enum RuntimeCall`.
4. Remove the entire implementation of `Dispatch for Runtime`.
5. Update instances of the `RuntimeCall` enum to use `snake_case`:
	- Change `RuntimeCall::Balances` to `RuntimeCall::balances`.
	- Change `RuntimeCall::ProofOfExistence` to `RuntimeCall::proof_of_existence`.

And that's it! You have now completed the full tutorial for building a simple rust state machine. 🎉

On `main.rs`:

```rust
mod balances;
mod proof_of_existence;
mod support;
mod system;

use crate::support::Dispatch;

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
	pub type Content = &'static str;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
/* TODO: Remove the `RuntimeCall`. This is now generated by the `#[macros::runtime]`. */
pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
	ProofOfExistence(proof_of_existence::Call<Runtime>),
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
/* TODO: Add the `#[macros::runtime]` attribute here and remove duplicate code listed by TODOs. */
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

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

/* TODO: Remove all this. It is now generated by the `#[macros::runtime]` attribute. */
impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
			proof_of_existence: proof_of_existence::Pallet::new(),
		}
	}

	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number();
		if block.header.block_number != self.system.block_number() {
			return Err(&"block number does not match what is expected");
		}
		// An extrinsic error is not enough to trigger the block to be invalid. We capture the
		// result, and emit an error message if one is emitted.
		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}
		Ok(())
	}
}

/* TODO: Remove all this too. Dispatch logic is auto-generated. */
impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		// This match statement will allow us to correctly route `RuntimeCall`s
		// to the appropriate pallet level function.
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

/* TODO: Update the extrinsics to match the automatically generated `RuntimeCall`. */
fn main() {
	// Create a new instance of the Runtime.
	// It will instantiate with it all the modules it uses.
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	// Initialize the system with some initial balance.
	runtime.balances.set_balance(&alice, 100);

	// Here are the extrinsics in our block.
	// You can add or remove these based on the modules and calls you have set up.
	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::transfer {
					to: bob.clone(),
					amount: 20,
				}),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::transfer { to: charlie, amount: 20 }),
			},
		],
	};

	let block_2 = types::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
					claim: &"Hello, world!",
				}),
			},
			support::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
					claim: &"Hello, world!",
				}),
			},
		],
	};

	let block_3 = types::Block {
		header: support::Header { block_number: 3 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice,
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::revoke_claim {
					claim: &"Hello, world!",
				}),
			},
			support::Extrinsic {
				caller: bob,
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
					claim: &"Hello, world!",
				}),
			},
		],
	};

	// Execute the extrinsics which make up our blocks.
	// If there are any errors, our system panics, since we should not execute invalid blocks.
	runtime.execute_block(block_1).expect("invalid block");
	runtime.execute_block(block_2).expect("invalid block");
	runtime.execute_block(block_3).expect("invalid block");

	// Simply print the debug format of our runtime state.
	println!("{:#?}", runtime);
}
```

Adding the #[macros::runtime] attribute is the final step to tidy up and improve your runtime code. Well done on reaching the end of this tutorial!

If you have questions or need assistance, feel free to ask in the [#🆘・section-6](https://discord.com/channels/898706705779687435/1261079463731662959) channel on Discord. 

And remember, helping others with their questions can strengthen your own understanding too. Sharing knowledge is a great way to solidify what you've learned! 🚀