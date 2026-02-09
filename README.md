# Making Your System Functional

We have again established the basis of a new Pallet.

Let's add functions which make it useful.

## Block Number

Your blockchain's blocknumber is stored in the System Pallet, and the System Pallet needs to expose functions which allow us to access and modify the block number.

For this we need two simple functions:

- `fn block_number` - a function that returns the currently stored blocknumber.
- `fn inc_block_number` - a function that increments the current block number by one.

This should be everything that a basic blockchain needs to function.

## Nonce

The `nonce` represents "a number used once".

In this context, each user on your blockchain has a `nonce` which gives a unique value to each transaction the user submits to the blockchain.

Remember that blockchains are decentralized and distributed systems, and transactions do not inherently have a deterministic order. For a user, we can assign an order to different transactions by using this nonce to keep track of how many transactions the user has executed on the blockchain.

For this, we again use a `BTreeMap` to give each user their own `nonce` counter.

Our simple blockchain won't use this value, but for the sake of example, we will keep track of it by creating an `inc_nonce` function. If you were creating a more complex blockchain, the user `nonce` would become an important part of your system.

## Safe Math?

We just explained the importance of using safe math when writing the Balances Pallet.

In that context, it is easy to see how a user could provide malicious inputs, and cause simple underflows or overflows if our system did not check the math.

However, you will see in the templates provided, that these new functions in the System Pallet do not return a result, and thus do not provide error handling.

Is this okay?

As you will notice, the `blocknumber` and `nonce` storage items only provide APIs to increment by one. In our System, both of these numbers are represented by `u32`, which means that over 4.2 billion calls to those functions need to occur before an overflow would happen.

Assuming a user does one transaction every block, and a new block is generated every 6 seconds, it would take over 800 years for an overflow to occur. So in this situation, we are preferring an API which requires no error handling rather than one which does.

End of the day, this is a design decision and a preference which is left to the developer. This tutorial chooses this API because this is exactly the API exposed by Substrate and the Polkadot SDK. There is nothing wrong with making these functions handle errors, so feel free to do this if you choose.

## Build Your System Pallet

Follow the instructions in the template to complete:

1. `fn block_number`
2. `fn inc_block_number`
3. `fn inc_nonce`

Then write tests which verify that these functions work as expected, and that your state is correctly updated. There should be no compiler warnings after this step!
