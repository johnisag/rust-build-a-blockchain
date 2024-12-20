You can find the [solution for the previous step here](https://gist.github.com/nomadbitcoin/03914776998cb74eaa0a6adaa7edacec).

# Enable Balance Transfers

[Youtube](https://www.youtube.com/watch?v=cyaS0WvdgKU)

Now that we have initialized and started to use our balances module, let's add probably the most important API: `transfer`.

## Learn

Before we write our function, it is important that we review some of the principles of blockchain and Rust.

### Bad Actors

In a blockchain system, security is paramount. Bad actors may attempt to exploit vulnerabilities, such as insufficient balances during fund transfers, or overflow / underflow issues. Rust's safe math and error handling mechanisms help mitigate these risks.

### Safe Math

Rust's safe math operations prevent overflow and underflow. The `checked_add` and `checked_sub` methods return an `Option` that allows handling potential arithmetic errors safely.

In Rust, the Option type is a fundamental part of the standard library, designed to handle scenarios where a value may or may not be present. It's commonly used in situations where the result of an operation might be undefined or absent.

Methods like `checked_add` and `checked_sub` return `Option` to indicate success or failure due to overflow or underflow.

```rust
let result = a.checked_add(b);
match result {
    Some(sum) => println!("Sum: {}", sum),
    None => println!("Overflow occurred."),
}
```

### Error Handling

In Rust, error handling is an integral part of writing robust and safe code. The Result type is commonly used for functions that may encounter errors during their execution.

The Result type is an enum defined in the standard library. It has two variants: `Ok(value)` for a successful result and `Err(error)` for an error:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` and `E` are generic parameters that allow you to customize the result type for your needs. For the purposes of this tutorial, we will always return `Ok(())` when everything completes okay, and a `Err(&static str)` to describe any errors with a basic string.

You can then define the `Result` type like:

```rust
Result<(), &'static str>
```

### Options and Results

You can use the `Option` type to trigger an `Err`, which is helpful when you only want your function to execute when everything goes as expected.

In this context, we want a function that will return an error whenever some safe math operation returns `None`.

For this, we can chain `ok_or` along with `?` directly after the safe math operation like so:

```rust
let new_from_balance = from_balance
    .checked_sub(amount)
    .ok_or("Not enough funds.")?;
```

If `checked_sub` returns `None`, we will then return an `Err` with the message `"Not enough funds."` that can be displayed to the user. Otherwise, if `checked_sub` returns `Some(value)`, we will assign `new_from_balance` directly to that value.

In this case, we are writing code which completely handles the `Option` type in a safe and ergonomic way.

# Exercises:

1. Create a safe and simple transfer function in your Balances Pallet.
2. Create a test showing that everything is working as expected, including error handling.

On `balances.rs`:
```rust
impl Pallet {
    /// ... previous code.

    /// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: String,
		to: String,
		amount: u128,
	) -> Result<(), &'static str> {
		/* TODO:
			- Get the balance of account `caller`.
			- Get the balance of account `to`.

			- Use safe math to calculate a `new_caller_balance`.
			- Use safe math to calculate a `new_to_balance`.

			- Insert the new balance of `caller`.
			- Insert the new balance of `to`.
		*/

		Ok(())
	}
}
```
Also on `balances.rs`:
```rust
mod tests {
    /// ... previous code.

    #[test]
	fn transfer_balance() {
		/* TODO: Create a test that checks the following:
			- That `alice` cannot transfer funds she does not have.
			- That `alice` can successfully transfer funds to `bob`.
			- That the balance of `alice` and `bob` is correctly updated.
		*/
	}
}
```

And remember, we're available to answer any questions on [Discord](https://discord.com/channels/898706705779687435/980904325763186788).

### 🌱 Growing Strong
YOU'RE GETTING THERE.

Section 1 is done! You're building a solid foundation. Well done :).