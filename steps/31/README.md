# Make System Pallet Generic

Now that you have some practice with the Balances Pallet, let's do the same task for the System Pallet.

1. In this case we need to make System generic over `AccountId`, `BlockNumber`, and `Nonce`.

2. You will also need to figure out the trait constraints needed for these types to be compatible with the logic you have previously written. The compiler is your friend here to help you navigate everything.

3. Update your tests.

4. Finally move your type definitions to your `main.rs` file and update your `Runtime`.

Make sure everything compiles and all tests pass after this step.

If you need help, I recommend to look at your Balances Pallet rather than the solution for this step. All of the patterns are the same as before, so it is better that you start to connect the dots yourself rather than relying on the solution.

If you struggled here, it is a good opportunity to take a pause and re-review generic types from other examples across the Rust ecosystem.
