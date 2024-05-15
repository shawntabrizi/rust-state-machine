# Adding State to Our Pallet

So let's add some simple state to our `balances.rs` module.

We can do this by adding fields into our `Pallet` struct.

For a balance system, we really only need to keep track of one thing: how much balance each user has in our system.

For this we will use a `BTreeMap`, which we can import from the Rust `std` library.

Maps are simple `key -> value` objects, allowing us to define an arbitrary sized storage where we can map some user identifier (`key`) to their account balance (`value`).

1. Import the `BTreeMap` object.
	```rust
	use std::collections::BTreeMap;
	```

2. Create a `balances` field in `Pallet` using the `BTreeMap`.

	For the `key`, we are using a simple static string for now. This way we can access users like `"alice"`, `"bob"`, etc... This will be changed in the future.

	For the `value`, we will use a `u128`, which is the largest natively supported type in Rust. This will allow our users to have very, very large balances if we want.

	In the end, that looks like:

	```rust
	pub struct Pallet {
		balances: BTreeMap<String, u128>,
	}
	```

3. Finally, we need a way to initialize this object and its state. For this, we will implement a function on the `Pallet` called `fn new()`:

	```rust
	impl Pallet {
		pub fn new() -> Self {
			Self {
				balances: BTreeMap::new()
			}
		}
	}
	```

You can confirm at this point that everything should still be compiling, and that you haven't made any small errors. Warnings are okay.

Next we will actually start to **use** this module.

## Notes

It is important to note that this is NOT how Pallet storage works with the Polkadot SDK, but just a simple emulation of the behaviors.

In the Polkadot SDK, there is a separate storage layer which manages a proper key-value database which holds all the information (past and present) of our blockchain system. There are abstractions which look and behave just like a `BTreeMap` in the Polkadot SDK, but the underlying logic which maintains that data is much more complex.

Using simple fields in a struct keeps this project simple, and illustrates that each Pallet really is meant to manage it's own storage. However, this simplification also leads to issues if you design more complex systems where multiple pallets interact with one another.

We won't have any cross pallet interactions in this workshop, however, this is definitely doable with the Polkadot SDK and a proper database.
