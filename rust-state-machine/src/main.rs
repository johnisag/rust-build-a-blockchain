mod balances;
mod system;

fn main() {
    let mut pallet = balances::Pallet::new();

    pallet.set_balance(&"alice".to_string(), 100);

    println!("Alice's balance: {}", pallet.balance(&"alice".to_string()));

}

