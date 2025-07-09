# Introducing Macros

If you have made it this far, then you have finished designing your simple state machine.

At this point, our goal is to see if we can use the power of Rust macros to make future development even easier.

All of this is in preparation for you to work with the Polkadot SDK, which heavily relies on macros like the ones you will see here.

## Auto Generated Code

As mentioned earlier, Rust macros are basically code which can generate more code.

As you can see from our simple state machine, there is a lot of boiler plate code that we could generate, following the simple patterns and structures we have designed.

For example:

- We expect that each Pallet will expose some callable functions with `Call`.
- We know that each `Call` will have all the same parameters of the underlying Pallet function, except the `caller`.
- We know that each Pallet will implement `Dispatch` logic on the `Pallet` struct.
- We know that the `Runtime` will accumulate all the `pallet::Call`s into the `RuntimeCall` outer enum.
- We know that the `Runtime` will have logic to re-dispatch runtime level calls to the pallet level.
- and so on...

The more we abstract our Pallet and Runtime into consistent and and extensible pieces, the more we can automate, and ultimately this can provide a better developer experience.

## Navigating the Macros

This tutorial is not attempting to teach you how to write these macros. That information would take a whole tutorial itself.

Instead, we are providing you with macros which should work directly with your existing code, and replace a lot of code that you have already written.

Macros in general are "magical". If you have not written the macro yourself, there can be very little insight into what is happening underneath. In this context, the macros we are providing to you will directly replace code you have already written, so you should completely understand what is being generated, and how they work.

The `macros` folder contains a `lib.rs`, which exposes the two attribute macros built for this tutorial:

1. `#[macros::call]`
2. `#[macros::runtime]`

You can find the code for these two macros in their respective `call` and `runtime` folders.

In each of these folders there are 3 files:

1. `mod.rs` - The entry point for the macro, where code is parsed, and then generated.
2. `parse.rs` - The parsing logic for the macro, extracting the information we need to generate code.
3. `expand.rs` - The expansion / generation code, which will write new code for us with the data provided.

We will go through each of these more deeply as we include the macros into our code.

## Adding the Macros to Our Project

All of the macros are contained within their own crate which will be a folder in your project.

Download the folder contents for the macros here: [download](https://download-directory.github.io?url=https://github.com/shawntabrizi/rust-state-machine/tree/gitorial/macros)

> If that link does not work, you can extract the `macros` folder however is best for you from the source repository for this tutorial: https://github.com/shawntabrizi/rust-state-machine/tree/gitorial/

Once you have the contents of the macros folder:

1. Copy the contents into a `macros` folder in the root of your project.
2. Update your `cargo.toml` file to include this crate into your project:

   ```toml
   [dependencies]
   num = "0.4.1"
   macros = { path = "./macros/" }
   ```

Recompile your project, and you should see this new create and its sub-dependencies being compiled.

In the next step we will actually start integrating these macros into your simple state machine.
