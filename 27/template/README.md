# Dispatching Calls

We have built our `execute_block` logic depending on the `dispatch` logic we have not implemented yet.

Let's do that.

## Adding Our Calls

Dispatch logic is all about routing a user's extrinsic to the proper Pallet function. So far, the only user callable function we have created is the `transfer` function in the Balances Pallet.

So let's add that call to our `RuntimeCall` enum.

Our `transfer` function expects 3 inputs:

- `caller`: The account calling the transfer function, and whose balance will be reduced.
- `to`: The account where the funds will be sent.
- `amount`: The amount of funds to transfer.

However, remember that our `dispatch` logic already has information about the `caller` which is coming from the `Extrinsic` in the `Block`. So we do not need this data again in the `RuntimeCall`.

In fact, every `Call` in our runtime should omit the `caller`, and know that it is being provided by our `dispatch` logic.

So when adding a new variant to `RuntimeCall`, it should look something like:

```rust
pub enum RuntimeCall {
	BalancesTransfer { to: types::AccountId, amount: types::Balance },
}
```

A user submitting an extrinsic to our state machine can use this enum variant to specify which function they want to call (`transfer`), and the parameters needed for that call.

## Dispatch Logic

The core logic in the `dispatch` function is a simple `match` statement.

Basically, given some `RuntimeCall`, we need to match on the variant being provided to us, and then pass the appropriate parameters to the correct Pallet function. As mentioned before, `dispatch` already has access to the `caller` information, so the final logic is as simple as:

```rust
match runtime_call {
	RuntimeCall::BalancesTransfer { to, amount } => {
		self.balances.transfer(caller, to, amount)?;
	}
}
```

Dispatch logic really is that simple!

Note that we propagate up any errors returned by our function call with the `?` operator. This is important if you want to see the error messages that we set up in the `execute_block` logic.

## Write Your Dispatch Logic

Follow the `TODO`s provided in the template to build your `RuntimeCall` and complete your `dispatch` logic.

Your code should still compile with some "never constructed/used" warnings. Just one more step and we will get rid of all those warnings!
