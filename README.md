# Rust State Machine

Build a simple blockchain state machine from scratch using pure Rust.

## About

This tutorial walks you through building a minimal blockchain state machine step by step. It is heavily inspired by the [Polkadot SDK](https://github.com/paritytech/polkadot-sdk) and [FRAME](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame), but everything is written from scratch without using those libraries.

By the end, you will understand what is happening behind the scenes when you use the Polkadot SDK, and the macros and abstractions it uses will no longer be a mystery.

## Who Is This For?

This tutorial is for developers who:

- Want to learn Rust through a real, practical project
- Are curious about how blockchains work under the hood
- Plan to build with the Polkadot SDK and want to understand its foundations
- Prefer learning by doing over reading documentation

### Prerequisites

You should be familiar with the first 11 chapters of the [Rust Book](https://doc.rust-lang.org/book/). You don't need to be an expert, but you should have exposure to:

- Ownership and borrowing
- Structs, enums, and pattern matching
- Error handling with `Result`
- Traits and generic types
- Writing tests
- Using crates and modules

## What You Will Build

You will build a working state machine with multiple modules (called "pallets"), a runtime that ties them together, and a block execution pipeline, all in vanilla Rust. You start with basic storage and transfers, then progressively add generics, dispatch logic, more pallets, and finally macros to reduce boilerplate.

## What You Will Learn

**Rust**

- Structs, enums, and impl blocks
- Generic types and trait bounds
- Configurable traits and associated types
- Error handling with `Result`
- Safe math (checked arithmetic)
- Writing and running tests
- Procedural macros
- Module organization with crates

**Blockchain**

- State machines and state transitions
- Pallets and modular runtime architecture
- Block structure and extrinsics
- Dispatch and call routing
- Token balances and transfers
- Proof of existence
- Runtime composition

## How It Works

The tutorial has 42 steps across 7 sections. Each step has:

- A README explaining the concepts and what to do
- A template with `TODO` comments showing exactly what code to write
- A solution you can check if you get stuck

Each step builds on the previous one with minimal changes, so you always know exactly what is new.

## Getting Started

1. Make sure you have [Rust installed](https://rustup.rs/).
2. Open the tutorial: **[Start the Tutorial](https://www.shawntabrizi.com/rust-state-machine/)**
3. Create a new Rust project locally and follow along with each step, writing the code in your own project, compiling, and testing as you go.

The website provides the instructions, code templates, diffs, and solutions for each step. Your local project is where you do the actual work.

## Contributing

If you find a bug, typo, or have ideas to improve the tutorial, please [open an issue](https://github.com/shawntabrizi/rust-state-machine/issues). Pull requests are also welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the [MIT License](LICENSE).
