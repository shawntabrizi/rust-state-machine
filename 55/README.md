# Proof of Existence Pallet

We have gone a long way since we built our very first Balances Pallet.

The structure of our Runtime and Pallets have evolved quite a bit since.

- Generic Types
- Config Trait
- Nested Dispatch
- and more...

This will be the last pallet we build for this tutorial, but we will build it knowing all of the tips and tricks we have learned so far.

The goal here is for you to ensure that all of the intricacies of Pallet development is well understood and that you are able to navigate all of the Rust code.

## What is Proof of Existence?

The Proof of Existence Pallet uses the blockchain to provide a secure and immutable ledger that can be used to verify the existence of a particular document, file, or piece of data at a specific point in time.

Because the blockchain acts as an immutable ledger whose history cannot be changed, when some data is placed on the blockchain, it can be referenced at a future time to show that some data already existed in the past.

For example, imagine you discovered a cure to cancer, but before you reveal it, you want to make sure that you can prove when you had made the discovery. To do this, you could put some sort of data on the blockchain which represents the cure. At a later date, when you get your research published and reviewed, you would be able to use the blockchain as verifiable evidence of when you first made the discovery.

Normally, you would not put the raw contents of your claim on the blockchain but a [hash](https://en.wikipedia.org/wiki/Cryptographic_hash_function) of the data, which is both smaller and obfuscates the data in your claim before you are ready to reveal it.

However, for the purposes of this tutorial, we won't introduce hash functions yet.

## Pallet Structure

The `BTreeMap` is again the best tool to use for storing data in this Pallet. However, you will notice that the construction of the storage is a bit different than before. Rather than having a map from accounts to some data, we will actually map the content we want to claim to the user who owns it.

This construction of `content -> account` allows an account to be the owner of multiple different claims, but having each claim only be owned by one user.

## Create Your Pallet

Let's start to create this pallet:

1. Create a new file for your Proof of Existence Pallet.

	```bash
	touch src/proof_of_existence.rs
	```

2. Copy the contents from the template into your new file.

3. Complete the `TODO`s to add a storage to your new pallet and allow it to be initialized.

4. In your `main.rs` file, import the `proof_of_existence` module.

Make sure that everything compiles after you complete these steps.

Compiler warnings about "never read/used" are okay.
