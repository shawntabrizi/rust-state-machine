# Import the Num Crate

Rust is designed to be very lightweight and provides very little right out of the box.

Within the ecosystem, many functions and features which you might expect to be included into Rust `std` or `core` are actually delegated to small, well-known, and widely used crates.

For our next step, we want to access traits for basic numerical operations like:

- `CheckedAdd` - A type which supports `checked_add`
- `CheckedSub` - A type which supports `checked_sub`
- `Zero` - A type which can return the value zero when calling `zero()`
- `One` - A type which can return the value one when calling `one()`

To access these traits, we will need to import a new crate into our project.

## Cargo.toml

When we first initialized our project, a `Cargo.toml` file was generated for us.

As mentioned before, it is very similar to a `package.json` file you would expect to find in a Node.js project.

Already in your `Cargo.toml` is metadata like the `name` of your project, the `version` of the crate you are building, and the `edition` of Rust you are using.

What you can see is that you can also add `dependencies` to your crate which will allow you to use other external crates and libraries in your project.

You can add the dependency by hand by editing your `Cargo.toml` file or you can run `cargo add num`.

## Crates.io

Where is this crate coming from?

The Rust community has a large registry of available crates on [crates.io](https://crates.io/). When you import a crate, it will use `crates.io` by default.

You can also import crates directly from github by specifying the repo where the source code can be found.

That would look something like:

```toml
[dependencies]
pallet-balances = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v1.0.0" }
```

## Add the Num Crate to Your Project

This step is short and simple.

Run `cargo add num` in your project directory and make sure your project compiles afterward.

You should see something like:

```bash
➜  rust-state-machine git:(master) ✗ cargo add num
    Updating crates.io index
      Adding num v0.4.1 to dependencies.
             Features:
             + std
             - alloc
             - libm
             - num-bigint
             - rand
             - serde
    Updating crates.io index
➜  rust-state-machine git:(master) ✗ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rust-state-machine`
```
