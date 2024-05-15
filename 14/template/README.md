# Derive Debug

In Rust, `derive` macros provide a convenient way to automatically implement trait functionality for custom data structures.

## Macros

In the most simple terms, Macros are rust code that write more rust code.

Macros can make your code easier to read, help avoid repetition, and even let you create your own special rules for coding in Rust.

We will be using (but not writing) macros heavily near the end of this tutorial, and you will see how powerful they can be.

For now, treat them as "magic".

## Traits

Think of traits in Rust as shared rules for different types. They allow you to define a set of things that types must be able to do. This way, you can make sure different parts of your code follow the same rules.

Take a look at [this example](https://doc.rust-lang.org/rust-by-example/trait.html) or re-read [the Rust Book](https://doc.rust-lang.org/book/ch10-02-traits.html) if you need a refresher on Traits.

We will make and use custom traits later in this tutorial, but know for this step that `#[derive(Debug)]` is a macro which implements the `Debug` trait for your custom types.

### Debug Trait

The Debug trait in Rust is part of the standard library and is used to print and format values for debugging purposes. It provides a default implementation through the `#[derive(Debug)] annotation.

For example:

```rust
#[derive(Debug)]
pub struct MyStruct {
    field1: i32,
    field2: String,
}
```

With the `Debug` trait derived, you can now print the `struct` to console:

```rust
let my_instance = MyStruct { field1: 42, field2: "Hello".to_string() };
println!("{:#?}", my_instance);
```

The characters `:#?` help [format](https://doc.rust-lang.org/std/fmt/) the output to make it more readable.

## Derive the Debug Trait for Your Runtime

This is a very simple, but helpful step!

We want to be able to print out the current state of our `Runtime` at the end of our `main` to allow us to easily inspect what it looks like and that everything is functioning as we expect.

To do this, we need to add `#[derive(Debug)]` to the `struct Runtime`.

However... `struct Runtime` is composed of `system::Pallet` and `balances::Pallet`, so these structs ALSO need to implement the Debug trait.

Complete the `TODO`s across the different files in your project and print out your final runtime at the end of the `main` function.

You can use `cargo run` to see the output of your `println`. Everything should compile and run without warnings.
