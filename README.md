# Use the Runtime Macro

Finally, let's add the `#[macros::runtime]` macro to our `main.rs` file, and really clean up a ton of boilerplate code.

## Runtime Macro

The purpose of the `#[macros::runtime]` macro is to get rid of all of the boilerplate function we implemented for the `Runtime`, including `fn new()` and `fn execute_block()`. Similar to the `Call` macro, it also generates the `enum RuntimeCall` and all the `dispatch` logic for re-dispatching to pallets.

We apply the `#[macros::runtime]` attribute on top of the main `struct Runtime` object.

### Parse

In order to generate the code we want, we need to keep track of:

1. The name of the `struct` representing our Runtime. Usually this is `Runtime`, but we provide flexibility to the developer.
2. The list of Pallets included in our `Runtime`
	1. Their name, as specified by the user.
	2. The specific type for their `Pallet`, for example `balances::Pallet` vs `proof_of_existence::Pallet`.

All of this information is tracked in the `RuntimeDef` struct.

We are also checking that our `Runtime` definition always contains the System Pallet, and does so as the first pallet in our `Runtime` definition. We will explain more about the assumption of the macros below.

### Expand

Once we have parsed all the data we need, we just need to generate the code that we expect.

Starting with `let runtime_impl = quote!`, you will see the entire `impl Runtime` code block has been swallowed into the macro. Since we know all the pallets in your `Runtime`, we can automatically implement functions like `new()`. The `execute_block` function does not take advantage of any of the parsed data, but the code is completely boilerplate, so we hide it away.

Then we have another code block being generated with `let dispatch_impl = quote!` which is the `enum RuntimeCall` and the implementation of `Dispatch for Runtime`.

Again, due to the quirks of using macros, our `RuntimeCall` enum will have `snake_case` variants which exactly match the name of the fields in the `Runtime` struct.

## Macro Assumptions

One of the assumptions programmed into these macros is the existence of the System Pallet. For example, in the `execute_block` logic, we need access to both `system.inc_block_number` and `system.inc_nonce`.

Some macro level assumptions are intentional, and actually define the architectural decisions of the framework designing those macros. This is the case with the System Pallet, since so much of a blockchain framework depends on a consistent meta-layer.

Other assumptions exist just because it is easier to write the macro if the assumption is made.

The main takeaway here is that macros can almost always continue to improve, providing better and better user experiences for developers. It just needs someone to identify what improvements need to be made, and someone else to program those improvements into the low level macro code.

## Add the Runtime Macro

Let's finally go through the steps to add the `#[macros::runtime]` attribute to your `Runtime`.

1. In `main.rs`, add `#[macros::runtime]` on top of your `pub struct Runtime`.
2. Remove the entire `impl Runtime` code block.
3. Remove the entire `enum RuntimeCall`.
4. Remove the entire implementation of `Dispatch for Runtime`.
5. Update instances of the `RuntimeCall` enum to use `snake_case`:
	- Change `RuntimeCall::Balances` to `RuntimeCall::balances`.
	- Change `RuntimeCall::ProofOfExistence` to `RuntimeCall::proof_of_existence`.

And that's it! You have now completed the full tutorial for building a simple rust state machine. 🎉
