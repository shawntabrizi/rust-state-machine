# Rust State Machine

This repository is the basis for a tutorial teaching how to develop a simple state machine using Rust.

## Goal

The goal of this tutorial is to **teach by experience** various entry level concepts around Rust and blockchain development.

This tutorial is opinionated, and heavily influenced by the [Polkadot SDK](https://github.com/paritytech/polkadot-sdk), [Substrate](https://github.com/paritytech/polkadot-sdk/tree/master/substrate), and [FRAME](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame). However, everything in this tutorial is written from scratch and built without using these libraries.

We designed this tutorial with the hope that readers who complete it will have a better understanding of what is happening behind FRAME and will feel more capable of using the Polkadot-SDK successfully.

This tutorial will teach and review many Rust concepts, however it is not a replacement for a primary Rust education course. It is recommended, before you even start this tutorial, that you are already familiar with the concepts taught in the first 11 chapters of the [Rust Book](https://doc.rust-lang.org/book/).


## How To Use

This repository is not meant to be used directly, but as the source for generating an interactive tutorial using the source code and readme files included at each commit.

If you browse the [commit history](https://github.com/shawntabrizi/rust-state-machine/commits/master) of this tutorial, you will see that each commit is designed to be a single step in the tutorial.

Some commits are prefixed with `template`, and you will see these commits leave `TODO` comments throughout the code for what the reader should do to advance to the next step of the tutorial. Immediately following a `template` commit is a `solution` commit that completes all of the `TODO`s. The `solution` should always compile and run successfully (but there might be some compiler warnings).

Some commits do not have a `template` or `solution` step, in which case the user is not expected to write any code for that step, but instead do other actions like import new crates, or review code provided by the tutorial.

Each commit includes changes to the top level README.md file, which includes documentation and background information relevant for that step.

## Exporting the Tutorial

Included with this repo is an `export_tutorial.sh` script.

This will generate an `output` folder where each commit is placed in an independent folder with the step number. For example:

```text
output/
├─ 1/
│  ├─ src/
│  │  ├─ main.rs
│  ├─ Cargo.toml
│  ├─ README.md
├─ 2/
│  ├─ src/
│  │  ├─ main.rs
│  │  ├─ balances.rs
│  ├─ Cargo.toml
│  ├─ README.md
├─ 3/
│  ├─ ...
```

This should provide a very basic example of how to turn this repository into step by step documentation.

More specific tooling can be created to customize this process further for your specific needs and output.

## Maintenance

This tutorial depends on having structured git history. To accomplish this, we use git in ways it was not intended. I apologize in advance to those offended by what they are going to read below.

This tutorial is updated and maintained by modifying its git history and force pushing changes back to master.

This allows us to maintain all the different snapshots of code needed by the reader, without having to individually maintain many copies of the same file. Making breaking changes to a step simply requires you to fix merge conflicts as you rebase those changes.

As a result, you should expect that the entire git history of this tutorial could change at any time.
