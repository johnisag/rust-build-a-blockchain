mod balances;
mod system;

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet,
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
    runtime.balances.set_balance(&"alice".to_string(), 100);

	// start emulating a block
	runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(), 1);

	// first transaction
    runtime.system.inc_nonce(&"alice".to_string());
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
}

