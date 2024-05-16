# Pallet Level Dispatch

We want to make our code more modular and extensible.

Currently, all dispatch happens through the `RuntimeCall`, which is hardcoding dispatch logic for each of the Pallets in our system.

What we would prefer is for Pallet level dispatch logic to live in the Pallet itself, and our Runtime taking advantage of that. We have already seen end to end what it takes to set up call dispatch, so let's do it again at the Pallet level.

## Pallet Call

To make our system more extensible, we want to keep all the calls for a pallet defined at the pallet level.

For this, we define an `enum Call` in our Balances pallet, and just like before, we introduce a new enum variant representing the function that we want to call.

Note that this enum needs to be generic over `T: Config` because we need access to the types defined by our configuration trait!

## Pallet Dispatch

You will also notice in the template, we have included the shell for you to implement Pallet level dispatch.

Everything should look the same as the Runtime level dispatch, except the `type Call` is the Pallet level call we just created.

Just like before, you simply need to match the `Call` variant with the appropriate function, and pass the parameters needed by the function.

## Create Your Pallet Level Dispatch

Follow the `TODO`s in the template to complete the logic for Pallet level dispatch.

The "never constructed" warning for the `Transfer` variant is okay.

In the next step, we will use this logic to improve our dispatch logic in our Runtime.
