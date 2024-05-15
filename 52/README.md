# Nested Dispatch

Now that we have defined Pallet level dispatch logic in the Pallet, we should update our Runtime to take advantage of that logic.

After this, whenever the Pallet logic is updated, the Runtime dispatch logic will also automatically get updated and route calls directly. This makes our code easier to manage, and prevent potential errors or maintenance in the future.

## Nested Calls

The Balances Pallet now exposes its own list of calls in `balances::Call`. Rather than list them all again in the Runtime, we can use a nested enum to route our calls correctly.

Imagine the following construction:

```rust
pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
}
```

In this case, we have a variant `RuntimeCall::Balances`, which itself contains a type `balances::Call`. This means we can access all the calls exposed by `balances:Call` under this variant. As we create more pallets or extend our calls, this nested structure will scale very well.

We call the `RuntimeCall` an "outer enum", and the `balances::Call` an "inter enum". This construction of using outer and inter enums is very common in the Polkadot SDK.

## Re-Dispatching to Pallet

Our current `dispatch` logic directly calls the functions in the Pallet. As we mentioned, having this logic live outside of the Pallet can increase the burden of maintenance or errors.

But now that we have defined Pallet level dispatch logic in the Pallet itself, we can use this to make the Runtime dispatch more extensible.

To do this, rather than calling the Pallet function directly, we can extract the inner call from the `RuntimeCall`, and then use the `balances::Pallet` to dispatch that call to the appropriate logic.

That would look something like:

```rust
match runtime_call {
	RuntimeCall::Balances(call) => {
		self.balances.dispatch(caller, call)?;
	},
}
```

Here you can see that the first thing we do is check that the call is a `Balances` variant, then we extract from it the `call` which is a `balances::Call` type, and then we use `self.balances` which is a `balances::Pallet` to dispatch the `balances::Call`.

## Updating Your Block

Since we have updated the construction of the `RuntimeCall` enum, we will also need to update our `Block` construction in `fn main`. Nothing magical here, just needing to construct a nested enum using both `RuntimeCall::Balances` and `balances::Call::Transfer`.

## Enable Nested Dispatch

Now is the time to complete this step and glue together Pallet level dispatch with the Runtime level dispatch logic.

Follow the `TODO`s provided in the template to get your full end to end dispatch logic running. By the end of this step there should be no compiler warnings.
