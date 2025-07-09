# Adding Call Macro to PoE

We have already seen the `#[macros::call]` macro help clean up the Balances Pallet.

Let's also add it to the Proof of Existence Pallet, where there is even more code that can be eliminated.

## Add Your Call Macro

We basically need to repeat the steps that we did for the Balances Pallet here:

1. Move your `create_claim` and `revoke_claim` functions into its own `impl<T: Config> Pallet<T>`.
2. Add the `#[macros::call]` attribute over this new `impl<T: Config> Pallet<T>`.
3. Delete your existing `enum Call`.
4. Delete your existing implementation of `Dispatch for Pallet`.
5. Then, in your `main.rs` file, change instances of:
	- `proof_of_existence::Call::CreateClaim` to `proof_of_existence::Call::create_claim` using `snake_case`.
	- `proof_of_existence::Call::RevokeClaim` to `proof_of_existence::Call::revoke_claim` using `snake_case`.

Check that everything is compiling and running just as before.

## Expand your Rust Code

Let's take the opportunity to show you how you can peek deeper into what the macros are doing.

Rust provides the command `cargo expand` which allows you to output the generated rust code after all macros have been applied to your project.

To install `cargo expand`:

```bash
cargo install cargo-expand
```

Then, run the following command:

```bash
cargo expand > out.rs
```

This will output your project's generated code into a file `out.rs`.

Then take a look at that file.

Here are some things you should notice:

- All of your different `mod` files have been combined together into a single file with your `main.rs`.
- You will see that our final Pallet code has all of the `Call` and `Dispatch` logic generated!
- You might notice that the very first `#[derive(Debug)]` macro has generated code

	```rust
    #[automatically_derived]
    impl<T: ::core::fmt::Debug + Config> ::core::fmt::Debug for Pallet<T>
    where
        T::Content: ::core::fmt::Debug,
        T::AccountId: ::core::fmt::Debug,
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "Pallet", "claims", &&self.claims)
        }
    }
	```
- You might even notice that other smaller macros like `vec![]` have changed:

	```rust
	extrinsics: <[_]>::into_vec(
		#[rustc_box]
		::alloc::boxed::Box::new([
			// stuff
		])
	)
	```
- And `println!()` :

	```rust
	{
		::std::io::_print(format_args!("{0:#?}\n", runtime));
	};
	```
- etc...

There are two main takeaways for you:

1. Macros ultimately follow all the same rules as regular Rust code, because it does generate regular Rust code. They feel magical, but there is really nothing magic about them.
2. Macros are an important part of the Rust ecosystem, and heavily used to improve developer experience and code quality.

If you ever use externally developed macros, and you want to look closer at what is going on, `cargo expand` can be a useful tool for you to better understand some of the hidden architectural details of a project. As you jump into the Polkadot SDK, I recommend you continue to use this tool to enhance your learning and understanding.
