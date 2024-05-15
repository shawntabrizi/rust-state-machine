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

1. functions which must be implemented by the type
2. associated types

### Custom Functions

The more obvious use of traits is to define custom functions

Let's say we want to expose a function which returns the name of something.

You could a trait `GetName`:

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
		return "shawn".to_string();
	}
}
```

And then call that function on the object which implements it.

```rust
fn main() {
	println!("{}", Shawn::name());
}
```

We won't actually use this feature of traits in our simple blockchain, but there are plenty of use cases for this when developing more complex blockchain systems.

### Associated Types

The other thing you can do with traits is define Associated Types.

This is covered in [chapter 19 of the Rust Book](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html) under "Advance Traits".

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
	type AccountId: Ord;
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

There is no meaningful difference between what we had before with 3 generic parameters, and a single generic parameter represented by a `Config` trait, but it certainly makes everything more scalable, easy to read, and easy to configure.

In this context, we call the trait `Config` because it is used to configure all the types for our Pallet.

### Implementing the Config Trait

Let's round this out with showing how you can actually implement and use the `Config` trait.

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

## Make Your System Configurable

Phew. That was a lot.

Let's practice all you have learned to create `Config` trait for your System Pallet, and then configure the pallet for the `Runtime` in `main.rs`.

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

Remember that Rust is a language which is completely type safe, so end of the day, all of these generic types and configurations need to make sense to the Rust compiler.
