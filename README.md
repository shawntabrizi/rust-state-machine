# Create Your Block Type

The support module provided for us a bunch of generic types which can be customized for our simple state machine. To actually start using them, we need to define concrete versions of these types using our other concrete types.

## Runtime Call

You will see the template provides an empty `enum RuntimeCall` which we will expand later. This is an object which is supposed to represent all the various calls exposed by your blockchain to users and the outside world. We need to mock this enum at this step so that it can be used to build a concrete `Extrinsic` type.

For now, there is just the `transfer` function exposed by the Balances Pallet, but we will add more before this tutorial is complete, and figure out ways to automate the creation of our `RuntimeCall`.

You can access this type within `mod types` with `crate::RuntimeCall`.

## Building the Block Type

It's time to define the concrete `Block` type that we will use to enhance our simple state machine.

1. Using the `RuntimeCall` enum and the `AccountId` type, you can define a concrete `Extrinsic` type.
2. Using the `BlockNumber` type, you can define a concrete `Header` type.
3. Using the concrete `Header` and `Extrinsic` types, you can define a concrete `Block` type.

As you can see, the `Block` is composed of layers of generic types, allowing the whole structure to be flexible and customizable to our needs.

Pay attention to the generic type definitions to ensure that you use all the correct generic parameters in all the right places.

Your code should still compile with some "never constructed/used" warnings.
