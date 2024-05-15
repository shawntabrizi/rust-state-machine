# Basic Balance Test

Now that we have the basics of our `Pallet` set up, let's actually interact with it.

For that, we will go back to the `main.rs` file, and create our first `#[test]` which will play with the code we have written so far.

1. In your `src/balances.rs` file, add a new `#[test]` named `fn init_balances()`:

	```rust
	#[test]
	fn init_balances() { }
	```

2. To begin our test, we need to initialize a new instance of our `Pallet`:

	```rust
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::new();
	}
	```

	Note that we make this variable `mut` since we plan to mutate our state using our newly created API.

3. Finally, let's check that our read and write APIs are working as expected:

	```rust
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}
	```

4. We can run our tests using `cargo test`, where hopefully you should see that it passes. **There should be no compiler warnings now!**

I hope at this point you can start to see the beginnings of your simple blockchain state machine.
