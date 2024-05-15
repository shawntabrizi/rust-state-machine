# Add Our Support Module

In this step, we will introduce a `support` module to help bring in various types and traits that we will use to enhance our simple state machine.

This `support` module parallels something similar to the [`frame_support` crate](https://docs.rs/frame-support/latest/frame_support/) that you would find in the Polkadot SDK.

The reason the `frame_support` crate exists, is to allow multiple other crates use common types and trait, while avoiding [cyclic dependencies](https://users.rust-lang.org/t/how-to-resolve-cyclic-dependency/51387), which is not allowed in Rust.

Our simple state machine will not experience this problem explicitly, since we are building everything in a single crate, but the structure of the project will still follow these best practices.

## Constructing a Block

The first set of primitives provided by the `support` module are a set of structs that we need to construct a simple `Block`.

### The Block

A block is basically broken up into two parts: the header and a vector of extrinsics.

You can see that we keep the `Block` completely generic over the `Header` and `Extrinsic` type. The exact contents and definitions of these sub-types may change, but the generic `Block` struct can always be used.

#### The Header

The block header contains metadata about the block which is used to verify that the block is valid. In our simple state machine, we only store the blocknumber in the header, but real blockchains like Polkadot have:

- Parent Hash
- Block Number
- State Root
- Extrinsics Root
- Consensus Digests / Logs

#### The Extrinsic

In our simple state machine, extrinsics are synonymous with user transactions.

Thus our extrinsic type is composed of a `Call` (the function we will execute) and a `Caller` (the account that wants to execute that function).

The Polkadot SDK supports other kinds of [extrinsics beyond a user transactions](https://docs.rs/sp-runtime/36.0.0/sp_runtime/generic/struct.UncheckedExtrinsic.html), which is why it is called an `Extrinsic`, but that is beyond the scope of this tutorial.

## Dispatching Calls

The next key change we are going to make to our simple state machine is to handle function dispatching. Basically, you can imagine that there could be multiple different pallets in your system, each with different calls they want to expose.

Your runtime, acting as a single entrypoint for your whole state transition function needs to be able to route incoming calls to the appropriate functions. For this, we need the `Dispatchable` trait.

You will see how this is used near the end of this tutorial.

### Dispatch Result

One last thing we added to the support module was a simple definition of the `Result` type that we want all dispatchable calls to return. This is exactly the type we already used for the `fn transfer` function, and allows us to return `Ok(())` if everything went well, or `Err("some error message")` if something went wrong.

## Create the Support Module

Now that you understand what is in the support module, add it to your project.

1. Create the `support.rs` file:

	```bash
	touch src/support.rs
	```

2. Copy and paste the content provided into your file.
3. Import the support module at the top of your `main.rs` file.
4. Finally, replace your `Result<(), &'static str>` with `crate::support::DispatchResult` in the `fn transfer` function in your Balances Pallet.

Introducing this new module will cause your compiler to emit lots of "never constructed" warnings. Everything should still compile, so that is okay. We will use these new types soon.
