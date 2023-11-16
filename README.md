# Adding Call Macro to Balances

Let's start by adding the `#[macros::call]` macro to our Balances Pallet.

## The Call Macro

The purpose of the `#[macros::call]` macro is to automatically generate the `enum Call` from the functions of the pallet and the pallet level `Dispatch` logic found in each Pallet.

We can place the `#[macros::call]` attribute over our `impl<T: Config> Pallet<T>` where the callable functions are implemented. From there, the macro can parse the whole object, and extract the data it needs. Not all of your functions are intended to be callable, so you can isolate the functions which should be in their own `impl<T: Config> Pallet<T>` as the template does.

### Parse

In order to generate the code that we want, we need to keep track of:

1. Each callable function that the developer wants to expose through the Runtime.
	1. The name of that function.
	2. The argument names and types of that function.
2. The name of the `struct` where those functions are implemented. Normally this is `Pallet`, but we can allow the developer flexibility in their naming.

These things are tracked with `CallDef` and `CallVariantDef`.

Also, during the parsing process, we might want to check for certain consistencies in the code being parsed. In this case, we require that every callable function muse have `caller` as their first parameter with type `T::AccountId`. This should make sense to you since you have designed a number of different callable functions, and they all follow this pattern.

This checking logic is handled by `fn check_caller_arg`.

### Expand

Once we have parsed all the data we need, generating the code is pretty straight forward.

If you jump down to `let dispatch_impl = quote!` you will see a bunch of code that looks like the templates we used earlier in the tutorial. We just left markers where the macro generation logic should place all the information to write the code we need.

## Macro Quirks

Macros are often very "quirky" when you use them. Since all of the input going into the macro is other code, sometimes the format of that code might not match what you expect.

For example, the original `Call` enum we have constructed looks like:

```rust
pub enum Call<T: Config> {
	Transfer { to: T::AccountId, amount: T::Balance },
}
```

The variant is called `Transfer` because the function it represents is named `fn transfer`.

However, if we want to generate the `Call` enum, and we only have `fn transfer`, where will we get the specific string `Transfer` with a capital `T`?

It is possible to do string manipulation and adjust everything to make it consistent to what Rust expects, but in this case it is better for our macros to make minimal modifications to user written code.

What does this mean?

When the `#[macros::call]` macro generates our `enum Call`, it will actually look like this:

```rust
#[allow(non_camel_case_types)]
pub enum Call<T: Config> {
	transfer { to: T::AccountId, amount: T::Balance },
}
```

Here you see that `transfer` is exactly the string which comes from the name of the function. Normally all enum variants should be `CamelCase`, but since rust functions are `snake_case`, our enum will have variants which are also `snake_case`. We won't see any warnings about this because we enabled `#[allow(non_camel_case_types)]`.

Ultimately, this has no significant impact on your underlying code. It is just ergonomics and expectations.

Indeed, macros can be quirky, but the amount of time they save you makes them worth it.

## Time to Add Your Call Macro

1. If you haven't, move your `transfer` function into its own `impl<T: Config> Pallet<T>`. We only want to apply the macro to this one function, so we need to isolate it from the other functions which are not meant to be callable.
2. Add the `#[macros::call]` attribute over this new `impl<T: Config> Pallet<T>`.
3. Delete your existing `enum Call`.
4. Delete your existing implementation of `Dispatch for Pallet`.
5. Then, in your `main.rs` file, change instances of `balances::Call::Transfer` to `balances::Call::transfer` with a lowercase `t`.

At this point, everything should compile just like before! We are witnessing the power of macros to generate code for us auto-magically!
