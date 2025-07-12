# Make System Configurable

We have one more step to take to make our Runtime as generic and configurable as possible.

To do it, we will need to take advantage of traits.

## Custom Traits

We have already used traits provided to us in order to make our types generic.

Let's take a quick look at how you can define a custom trait:

```rust
pub trait Config {}
```

Traits can contain within it two things:

1. Functions which must be implemented by the type.
2. Associated types.

### Custom Functions

The more obvious use of traits is to define custom functions.

Let's say we want to expose a function which returns the name of something.

You could create a trait `GetName`:

```rust
pub trait GetName {
	fn name() -> String;
}
```

Then you could implement this trait for any object.

```rust
struct Shawn;
impl GetName for Shawn {
	fn name() -> String {
		"shawn".to_string()
	}
}
```

And then call that function on the object which implements it.

```rust
fn main() {
	let name = Shawn::name();
	println!("{name}");
}
```

We won't actually use this feature of traits in our simple blockchain, but there are plenty of use cases for this when developing more complex blockchain systems.

### Associated Types

The other thing you can do with traits is define Associated Types.

This is covered in [chapter 19 of the Rust Book](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html) under "Advanced Traits".

Let's learn this concept by first looking at the problem we are trying to solve.

So far our simple blockchain code looks perfectly fine with generic types. However, let's imagine that our blockchain becomes more and more complex, requiring more and more generic types.

For example:

```rust
pub struct Pallet<AccountId, BlockNumber, BlockLength, BlockWeight, Hash, Nonce, Runtime, Version, ...> {
	// a bunch of stuff
}
```

Imagine every time you wanted to instantiate this struct, you would need to fill out each and every one of those types. Well systems do get this complex, and more, and the ability to abstract these types one level further can really simplify your code and make it much more readable.

For this we will use a trait with a bunch of associated types:

```rust
pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: Zero + One + Copy;
	// and more if needed
}
```

Then we can define our generic type using a single generic parameter!

```rust
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}
```

and implement functions using:

```rust
impl<T: Config> Pallet<T> {
	// functions using types from T here
}
```

Let's try to understand this syntax real quick.

1. There is a generic type `T`. `T` has no meaningful name because it represents a bunch of stuff, and this is the convention most commonly used in Rust.
2. `T` is required to implement the trait `Config`, which we previously defined.
3. Because `T` implements `Config`, and `Config` has the associated types `AccountId`, `BlockNumber`, and `Nonce`, we can access those types like so:
	- `T::AccountId`
	- `T::BlockNumber`
	- `T::Nonce`

While this may seem like a purely stylistic change, it enforces a powerful constraint: for any given type implementing `Config` (like our `Runtime`), there can only be one corresponding `AccountId`, `BlockNumber`, and `Nonce`. This guarantees type consistency across the pallet.

In this context, we call the trait `Config` because it is used to configure all the types for our Pallet.

### Implementing the Config Trait

Let's round this out by showing how you can actually implement and use the `Config` trait.

Just like before, we need some object which will implement this trait. In our case, we can use the `Runtime` struct itself.

```rust
impl system::Config for Runtime {
	type AccountId = String;
	type BlockNumber = u32;
	type Nonce = u32;
}
```

Then, when defining the `system::Pallet` within the `Runtime`, we can use the following syntax:

```rust
pub struct Runtime {
	system: system::Pallet<Self>,
}
```

Here we are basically saying that `Pallet` will use `Runtime` as its generic type, but this is defined within the `Runtime`, so we refer to it as `Self`.

## The Power of Associated Types

Using traits with associated types does more than just make the code less verbose; it fundamentally changes how we can configure our blockchain system.

1. It creates a single place where all types that our blockchain system requires are defined: the implementation of the `Config` trait. As long as you use this single implementation, you ensure type consistency across your entire runtime.

2. It allows us to avoid hardcoding specific types, like `String` or `u32`, and instead use generic data types, like `T::AccountId` and `T::BlockNumber`. This allows us to swap out or change the final configured types in the runtime without changing any of the pallet code.

3. It allows us to create a single spot for us to configure those final types. You can see that we use the `mod types {}` section in `main.rs` to define all the concrete types in one place, so they can be consistently referenced. Without this, you might accidentally use different concrete types in different pallets (like `u32` in one place and `u64` in another), causing compilation errors or, worse, subtle bugs.

4. Finally, it allows us to configure different runtimes with completely different concrete types. This is an extremely powerful and often used feature in Substrate / the Polkadot-SDK.

	For example, we can create two runtime configurations, one for production and one for testing, and configure them completely differently:

	```rust
	// In a production runtime
	struct Runtime;
	impl my_pallet::Config for Runtime {
		type AccountId = AccountId32;
		type BlockNumber = u32;
		type Nonce = u64;
		// etc...
	}

	// In a test runtime
	struct TestConfig;
	impl my_pallet::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u16;
		type Nonce = u8;
		// etc...
	}
	```

### The Runtime is a Configuration Hub

You should be able to see a bigger idea forming based on how we have designed our blockchain system.

You don't *program* the runtime, you **configure** it.

- We have designed our blockchain system with reusable pieces of application logic: the pallets.
- Each of these pallets are written generically, and are entirely configurable.
- Our runtime includes all of the pallets and pieces of logic it wants to use.
- Our runtime aggregates and propagates all the expected types for our blockchain system.

This pattern allows pallets to be completely independent and reusable. A pallet doesn't need to know which other pallets exist in the runtime, it only needs its `Config` trait implemented. This means you can mix and match different pallets in different runtimes (production, test, development) without modifying the pallet code itself.

This is the difference between building a single purpose blockchain and building a blockchain SDK, allowing for reusable, modular components, and ultimately bootstrapping a powerful ecosystem of blockchain developers.

## Make Your System Configurable

Phew. That was a lot.

Let's practice all you have learned to create a `Config` trait for your System Pallet, and then configure the pallet for the `Runtime` in `main.rs`.

1. Define the `Config` trait which will have your 3 associated types `AccountId`, `BlockNumber`, and `Nonce`.
2. Make sure these types have their trait constraints defined in `Config`.
3. Update your `struct Pallet` to use `T: Config` and reference your types using the `T::` syntax.
4. Update all of your functions to use the `T::` syntax.
5. Update your test, creating a struct `TestConfig`, and implementing `Config` for it, and using it to instantiate your `Pallet` struct.
6. Go to your `main.rs` file, and implement `system::Config` for the `Runtime` struct.
7. Update your `Runtime` definition to instantiate `system::Pallet` with `Self`.

Again, this is a big step for new Rust developers, and a common place that people can get very confused.

You will have the opportunity to do this whole process again for the Balances Pallet, so don't be afraid to peek at the solution this time around if you cannot get your code working.

Really take time to understand this step, what is happening, and what all of this syntax means to Rust.

Remember that Rust is a language which is completely type safe, so at the end of the day, all of these generic types and configurations need to make sense to the Rust compiler.
