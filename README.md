# Creating Our Runtime

We have now established two different Pallets for our blockchain: the System and Balances Pallet.

How do these pallets work together to create a unified blockchain system?

For that, we will need to create a `Runtime`.

## What is the Runtime?

Remember that there is a separation between the blockchain client and the state transition function of our blockchain.

You can think of the runtime as the accumulation of all logic which composes your state transition function. It will combine all of your pallets into a single object, and then expose that single object as the entry point for your users to interact with.

Certainly this sounds pretty abstract, but it will make more sense as we complete this tutorial.

## Create the Runtime

Just like our Pallets, our Runtime will be represented with a simple `struct`, however in this case, the fields of our `struct` will be our Pallets!

Complete the instructions for creating a new runtime which includes our System and Balances pallets. For this, you will need to take advantage of the `new()` functions we exposed for each of the Pallets.

Make sure your code is formatted and everything is still compiling. Compiler warnings about "never read/used" are okay.
