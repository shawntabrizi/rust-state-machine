# Rust State Machine

This repository is the basis for a tutorial teaching how to develop a simple state machine using Rust.

## Goal

The goal of this tutorial is to **teach by experience** various entry level concepts around Rust and blockchain development.

This tutorial is opinionated, and heavily influenced by the [Polkadot SDK](https://github.com/paritytech/polkadot-sdk), [Substrate](https://github.com/paritytech/polkadot-sdk/tree/master/substrate), and [FRAME](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame). However, everything in this tutorial is written from scratch and built without using these libraries.

The tutorial is designed with the hope that readers who complete it will have a better understanding of what is happening behind FRAME and will feel more capable of using the Polkadot-SDK successfully.

This tutorial will teach and review many Rust concepts, however it is not a replacement for a primary Rust education course. It is recommended, before you even start this tutorial, that you are already familiar with the concepts taught in the first 11 chapters of the [Rust Book](https://doc.rust-lang.org/book/).

## How To Use

This repository is not meant to be used directly, but as the source for generating an interactive tutorial using the source code and readme files included at each commit.

If you browse the [commit history](https://github.com/shawntabrizi/rust-state-machine/commits/master) of this tutorial, you will see that each commit is designed to be a single step in the tutorial.

All commits are prefixed with one of:

- `section`: This denotes the beginning of a new set of steps which will have a specific goal. These commits will only have changes to the `README.md` file which can be used to introduce the new section of the tutorial.
- `template`: This is the commit has a `README.md` that teaches the reader any information needed to complete the step. It will also include files with `TODO` comments, telling the user what specifically needs to be done. A `template` will always be followed by a `solution`.
- `solution`: These commits will always come after a `template` commit. These commits will have the final state of all files in the project at the end of a step. Commits prefixed with `solution` should always compile, run and test successfully (compiler warning are okay). The `template` and `solution` commits should be presented together so reads can compare their work to a working solution. These commits can also be used to generate a `diff` of the step. The `README.md` file in this commit does not need to be presented to the user.
- `action`: This denotes a step in the tutorial where the user needs to complete some action, not necessarily write any code. For example, the user might need to import a new crate. In this case, it does not make sense to have a `template` and `solution`, but just the final outcome after the action was taken. The previous commit can be used for generating a `diff`. The `README.md` file should contain any information the user needs to complete the action successfully.
- `readme`: This is only applied to the last commit in this repo, and denotes that this commit was specifically for make a `README.md` for users that browse this repository on github. This step should not be used in the tutorial generation.

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
