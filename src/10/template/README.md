# Introduce the System Pallet

We have basically completed the creation of a basic Balances Pallet. This is the pallet that most users will interact with.

However, your blockchain usually needs to keep track of many other pieces of data to function properly.

For this, we will create a new pallet called the System Pallet.

## What is the System Pallet?

The System Pallet is a "meta"-pallet which stores all the metadata needed for your blockchain to function. For example, the current blocknumber or the nonce of users on your blockchain.

This pallet does not need to expose any functions to end users, but can still play an important role in our overall state transition function.

We will see the importance of the System Pallet evolve as you walk through the steps of building it.

## Create the System Pallet

1. Create a new file `src/system.rs` in your project.
2. Copy the starting template provided, then complete the steps outlined by the template code.
3. Import the `system` module into your `main.rs` file.

You will notice that the instructions here are quite brief. You have already done all of these steps before, so you should already be familiar with everything you need to complete this step.

Confirm everything is compiling. You should expect some "never used/constructed" warnings. That is okay.
