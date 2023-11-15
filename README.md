# Proof Of Existence Functions

The Proof of Existence Pallet is quite simple, so let's build out the logic needed.

## Get Claim

Our Pallet has a simple storage map from some claim content to the owner of that claim.

The `get_claim` function should act as a simple read function returning the `T::AccountId` of the owner, if there is any. In the case we query a claim which has no owner, we should return `None`.

This is not a function that a user would call from an extrinsic, but is useful for other parts of your state machine to access the data in this Pallet.

## Create Claim

Any user can add a new claim to the Proof of Existence Pallet.

The only thing that is important is that we check that the claim has not already been made by another user.

Each claim should only have one owner, and whoever makes the claim first gets priority.

You can check if some claim is already in the `claims` storage using the `contains_key` api:

```rust
if self.claims.contains_key(&claim) {
	return Err(&"this content is already claimed");
}
```

## Revoke Claim

Data on the blockchain is not free, and in fact is very expensive to maintain. Giving users the ability to clean up their data is not only good, but encouraged. If a user no longer has a need to store their claim on chain, they should clean it up.

Furthermore, the history of the blockchain is immutable. Even if the data about a claim does not exist in the "current state", it can be shown to have existed in the past.

Keeping things in the current state just makes querying for information easier.

To revoke a claim, we need to check two things:

1. The the claim exists.
2. That the person who wants to revoke the claim is the owner of that claim.

You should be able to handle all of this logic by calling the `get_claim` function and using `ok_or` to return an error when the claim does not exist. If the claim does exist, you should be able to directly extract the owner from the state query.

## Build Your Functions

Complete the `TODO`s outlined in the template.

Afterward, create a `basic_proof_of_existence` test to check that all your functions are working as expected.

This includes both the success and possible error conditions of your Pallet.
