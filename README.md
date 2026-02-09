# Using Named Types

Up till now, we have just been hardcoding raw types into our structs and function definitions.

There are already examples where this can be confusing, for example if you see a function accept a `u32` parameter, is it a `blocknumber` or a `nonce`?

To make our code more clear, let's extract all of our raw types and define custom named types for our structs and functions.

Across the Balances and System Pallet, we need to define the following types:

1. `type AccountId = String;`
2. `type Balance = u128;`
3. `type Nonce = u32;`
4. `type BlockNumber = u32;`

Note that extracting these types into common type definitions also allows us to update the types more easily if we choose to.

As we go further into this tutorial, we will show you how we can make these type definitions even more flexible and customizable in the context of building a blockchain SDK for developers like yourself.

## Create Custom Types

Follow the `TODO`s in the template to add these type definitions to each of your Pallets, and update all of your structs and functions to use these types.
