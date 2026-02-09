# Make Balances Configurable

There is nothing new to learn in this step, just repeating the same process we did for our System Pallet for the Balances pallet.

In this case, our `Config` trait will only have two associated types: `AccountId` and `Balance`.

For this step, try to avoid looking at the solution, and instead refer to the changes you made to get the System Pallet configurable.

1. Define the `Config` trait which will have your associated types.
2. Make sure these types have their trait constraints defined in `Config`.
3. Update your `struct Pallet` to use `T: Config` and reference your types using the `T::` syntax.
4. Update all of your functions to use the `T::` syntax.
5. Update your test, creating a struct `TestConfig`, and implementing `Config` for it, and using it to instantiate your `Pallet` struct.
6. Go to your `main.rs` file, and implement `balances::Config` for the `Runtime` struct.
7. Update your `Runtime` definition to instantiate `balances::Pallet` with `Self`.

If you have made it this far, I think it is fair to say you have made it over the hardest part of this tutorial, and the hardest part of using Rust in Substrate.

It is important to take a step back and remember that while these abstractions make your code a bit more complicated to fully understand, it also makes your code extremely flexible, at zero cost to performance and safety thanks to the Rust compiler.
