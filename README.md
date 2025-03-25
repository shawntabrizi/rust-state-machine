# Interacting with Balances

Now that we have established the basics of our balances module, let's add ways to interact with it.

To do this, we will continue to create more functions implemented on `Pallet` which grants access to read, write, and update the `balances: BTreeMap` we created.

Finally, we will see what it looks like to actually start interacting with our balances pallet from the `main.rs` file.

## Rust Prerequisite Knowledge

Before we continue, let's take a moment to go over some Rust which we will be using in this next section.

### Option and Option Handling

One of the key principals of Rust is to remove undefined behavior from your code.

One way undefined behavior can happen is by allowing states like `null` to exist. Rust prevents this by having the user explicitly handle all cases, and this is where the creation of the `Option` type comes in. Spend a moment to re-review [the section on `Option`](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html?highlight=option#the-option-enum-and-its-advantages-over-null-values) from the Rust book if needed.

The `BTreeMap` api uses an `Option` when reading values from the map, since it could be that you ask to read the value of some key that you did not set. For example:

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert("alice", 100);
assert_eq!(map.get(&"alice"), Some(&100));
assert_eq!(map.get(&"bob"), None);
```

Once we have an `Option` type, there are lots of different ways we can interact with it using Rust.

The most verbose way is using a match statement:

```rust
let maybe_value = map.get(&"alice");
match maybe_value {
	Some(value) => {
		// do something with the `value`
	},
	None => {
		// perhaps return an error since there was no value there
	}
}
```


> IMPORTANT NOTE!

What you SHOULD NOT do is blindly `unwrap()` options. This will result in a `panic` in your code, which is exactly the kind of thing Rust was designed to prevent! Instead, you should always explicitly handle all of your different logical cases, and if you let Rust do it's job, your code will be super safe.

In the context of what we are designing for with the balances module, we have a map which has an arbitrary number of user keys, and their balance values.

What should we do when we read the balance of a user which does not exist in our map?

Well, the trick here is that in the context of blockchains, a user having `None` balance, and a user having `0` balance is the same. Of course, there is some finer details to be expressed between a user who exists in our state with value 0 and a user which does not exist at all, but for the purposes of our APIs, we can treat them the same.

What does this look like?

Well, we can use `unwrap_or(...)` to safely handle this condition, and make our future APIs more ergonomic to use. For example:

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert("alice", 100);
assert_eq!(*map.get(&"alice").unwrap_or(&0), 100);
assert_eq!(*map.get(&"bob").unwrap_or(&0), 0);
```

As you can see, by using `unwrap_or(&0)` after reading from our map, we are able to turn our `Option` into a basic integer, where users with some value have their value exposed, and users with `None` get turned into `0`.

Let's see how that can be used next.

## Setting and Reading User Balances

As you can see, our initial state machine starts that everyone has no balance.

To make our module useful, we need to at least have some functions which will allow us to mint new balances for users, and to read those balances.

1. Create a new function inside `impl Pallet` called `fn set_balance`:

	```rust
	impl Pallet {
		pub fn set_balance(&mut self, who: &String, amount: u128) {
			self.balances.insert(who.clone(), amount);
		}

		// -- snip --
	}
	```

	As you can see, this function simply takes input about which user we want to set the balance of, and what balance we want to set. This then pushes that information into our `BTreeMap`, and that is all.

2. Create a new function inside `impl Pallet` called `fn balance`:

	```rust
	pub fn balance(&self, who: &String) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}
	```

	As you can see, this function allows us to read the balance of users in our map. The function allows you to input some user, and we will return their balance.

	> Important Detail!

	Note that we do our little trick here! Rather than exposing an API which forces the user downstream to handle an `Option`, we instead are able to have our API always return a `u128` by converting any user with `None` value into `0`.

As always, confirm everything is still compiling. Warnings are okay.

Next we will write our first test and actually interact with our balances module.
