# Executing Blocks

We will now start the process to replace the simple block simulation in our `main` function with a proper block execution pipeline.

## Execute Block

We have introduced a new function to our `Runtime` called `fn execute_block`.

The steps of this function is exactly the same as our current `main` function, but using the concrete `Block` type we defined to extract details like the expected block number and the extrinsics that we want to execute.

### Iterating Over a Vector

In order to build our `execute_block` function, we will need to iterate over all the extrinsics in our block, and dispatch those calls. In rust, the common way to access the elements of a vector is to turn it into an iterator.

There are two functions used for turning a vector into an interator, `iter` and `into_iter`, and their difference lies in ownership:

- `iter`: This method creates an iterator that borrows each element from the vector, allowing you to read the values without taking ownership. It's useful when you want to iterate over the vector while keeping it intact.

- `into_iter`: This method consumes the vector, transferring ownership of each element to the iterator. It's handy when you want to move or transfer ownership of the vector's elements to another part of your code. After using `into_iter`, the original vector can't be used anymore, as ownership has been transferred.

In our context, we want to use `into_iter()`, so you will get something that looks like:

```rust
for support::Extrinsic { caller, call } in block.extrinsics.into_iter() {
	// do stuff with `caller` and `call`
}
```

Here you can see we also do a trick to separate out the fields of the `Extrinsic` in a single line, since ultimately we want to work with `caller` and `call`. You can of course break this process up into multiple lines if you want.

### Dispatching a Call

Once we have the `call` and `caller`, what should we do with them?

This is where the `Dispatch` trait starts to come into play. You will see in our template, we included the shell of an `unimplemented()` `fn dispatch`. We will write this logic in the next step, but we need to already use the `dispatch` function in our `execute_block` logic.

Once we have the `call` and `caller`, we want to pass them to the `dispatch` logic, which you see is implemented on the `Runtime`.

That will look something like:

```rust
let _res = self.dispatch(caller, call).map_err(|e| eprintln!("{}", e));
```

Note that in Rust, if you want to access a function within a trait, like we do here with `dispatch`, you need to explicitly import that trait into your project.

We left a `TODO` at the top of `main.rs` where we ask you to import `crate::support::Dispatch`, which will allow you access to calling `dispatch` on `Runtime`.

### Better Error Messages

Since this is a more permanent function of our project, it also makes sense to expand the message being printed when there are extrinsic errors. For example:

```rust
eprintln!(
	"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
	block.header.block_number, i, e
)
```

This allows you to see the block number, extrinsic number, and the error message whenever there is an extrinsic error. This can be very helpful when you have many blocks being imported each with potentially many extrinsics.

To get the extrinsic number `i`, use you chain the [`enumerate()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate) function after the `into_iter()`.

## Build Your Execute Block Function

You should now have all the tools and information needed to successfully write your `execute_block` function.

Follow the `TODO`s provided by the template, and make sure to include the `impl crate::support::Dispatch for Runtime` that we provided for you, and that we will implement in the next steps.

Your code should still compile with some "never constructed/used" warnings.
