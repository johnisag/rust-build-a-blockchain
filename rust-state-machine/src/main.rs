mod balances;
mod system;
mod support;

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
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    // TODO: Not implemented yet.
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
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

impl Runtime {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let carlie = "carlie".to_string();

	// Create a mutable variable `runtime`, which is a new instance of `Runtime`.
    let mut runtime = Runtime::new();

	//Set the balance of `alice` to 100, allowing us to execute other transactions. 
    runtime.balances.set_balance(&alice, 100);

	// start emulating a block
	runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(), 1);

	// first transaction
    runtime.system.inc_nonce(&alice);
    let _res = runtime.balances
    .transfer(alice.clone(), bob.clone(), 30)
    .map_err(|e| println!("{}", e));

	// second transaction
    runtime.system.inc_nonce(&alice);
    let _res = runtime
    .balances
    .transfer(alice.clone(), carlie.clone(), 20)
    .map_err(|e| println!("{}", e));

    print!("Alice's balance: {}\n", runtime.balances.balance(&alice));
    print!("Bob's balance: {}\n", runtime.balances.balance(&bob));
    print!("Carlie's balance: {}\n", runtime.balances.balance(&carlie));

    println!("{:#?}", runtime);
}