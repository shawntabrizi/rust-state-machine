# Using Execute Block

We have now successfully implemented the `execute_block` and `dispatch` logic needed to build and execute real `Blocks`.

Let's bring that logic into our `main` function.

## Creating a Block

You can create a new `Block` by filling out all the fields of the struct and assigning it to a variable.

For example:

```rust
let block_1 = types::Block {
	header: support::Header { block_number: 1 },
	extrinsics: vec![
		support::Extrinsic {
			caller: &"alice",
			call: RuntimeCall::BalancesTransfer { to: &"bob", amount: 69 },
		},
	],
};
```

It is important that you set the block number correctly since we verify this in our `execute_block` function. The first block in our state machine will have the number `1`.

Also remember that you can add multiple extrinsics in a single block by extending the vector.

## Executing a Block

Once you have constructed your `Block`, you can pass it to the `execute_block` function implemented on your `runtime`.

```rust
runtime.execute_block(block_1).expect("invalid block");
```

Note how we panic with the message `"invalid block"` if the `execute_block` function returns an error. This should only happen when something is seriously wrong with your block, for example the block number is incorrect for what we expect.

This panic will NOT be triggered if there is an error in an extrinsic, as we "swallow" those errors in the `execute_block` function. This is the behavior we want.

## Update Your Main Function

Go ahead and use the `Block` type and `execute_block` function to update the logic of your `main` function.

Follow the `TODO`s provided in the template to complete this step

By the end of this step, your code should compile, test, and run successfully, all without compiler warnings!
