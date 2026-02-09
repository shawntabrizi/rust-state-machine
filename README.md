# Add Proof of Existence Dispatch

We have already established the nested dispatch pipeline for Pallets in the `Runtime`.

Let's build Pallet level dispatch logic for the Proof of Existence to take advantage of that.

## Create Pallet Level Dispatch

There is nothing new here, but we have left more for you to fill out than before.

1. Create the variants for `CreateClaim` and `RevokeClaim` for your `Call` enum.
2. Implement the `Dispatch` trait for your `Pallet`.

If you get stuck, try not to look at the solution provided here, but instead look at what you did in the Balances Pallet. Everything we have done here, we have already done in the past. This is an opportunity to catch where you may have outstanding questions or misunderstandings.

Don't worry about compiler warnings like "never used/constructed".
