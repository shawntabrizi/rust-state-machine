# Integrate PoE Into Your Runtime

The Proof of Existence pallet is done, but we still need to integrate it into your Runtime.

Let's take a look at that process.

## Integration Steps

1. The first place to start is adding the `proof_of_existence` field to your `struct Runtime`.
2. Next you need to update your `fn new()` to also initialize `proof_of_existence`.
3. After, create a new concrete `type Content` which is a `&'static str`. As mentioned, normally this would be a hash, but for simplicity we are once again a simple static string.

	> If you want to use a hash now or in the future, it would be as simple as updating this one line to change all the types in your Runtime and Pallet. That is the kind of flexibility we have been working toward!

4. Then, implement `proof_of_existence::Config` for `Runtime`, using your `types::Content`.
5. At this point, things should already compile successfully, so use this as a checkpoint.
6. Introduce a new variant `ProofOfExistence` for the `RuntimeCall`.
7. Finally, update your `fn dispatch` logic to handle re-dispatching `ProofOfExistence` calls to the `proof_of_existence::Pallet`.

Hopefully from this process, you can see how all of the abstractions we have introduced has made integrating new Pallets into your runtime quite easy.

We will make this process even easier in the near future using macros!

By the end of this step, everything should compile without warnings.
