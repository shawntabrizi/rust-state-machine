# Contributing

Thanks for your interest in improving the Rust State Machine tutorial. This guide covers the structure of the project and the rules you should follow when making changes.

## Project Structure

Steps live in `src/` and are numbered sequentially. Each step is one of three types:

**Section headers** introduce a new section of the tutorial. They contain only a `README.md` with no code.

**Action steps** have a `source/` directory with the project state after the action and a `README.md` explaining what to do. No template/solution split.

**Template/solution steps** are the most common. They look like this (e.g., `src/8/`):

```
src/N/
├── README.md        # Wrapper that includes template/README.md
├── files.json       # Manifest of files shown in the editor
├── template/        # Starting code with TODO comments
│   └── src/
└── solution/        # Completed code
    └── src/
```

## The Golden Rule: Minimal Diffs

The most important property of this tutorial is that **each step's template must match the previous step's solution exactly, except for the new TODO comments and scaffolding introduced by the current step.**

This means a student finishing step N and moving to step N+1 should see only the new instructions, not unexplained changes to existing code.

### What counts as a bad change

Any difference between step N's solution and step N+1's template that is not explained by step N+1's README is a bug. Common examples:

- Changing code style without mentioning it (e.g., switching from `///` doc comments to `//` regular comments)
- Reformatting a function from single-line to multi-line (or vice versa) when the step doesn't touch that function
- Changing a `match` arm from direct return style to `?`/`Ok(())` style without explanation
- Fixing typos in code from earlier steps (fix them in the step where they were introduced instead)

### How to check

Diff the previous solution against the current template:

```sh
diff -r src/N/solution/src/ src/N+1/template/src/
```

Every difference you see should be either a `TODO` comment or scaffolding that the README explains. If you see anything else, it needs to be fixed.

## Writing Steps

### Template files

- Use `/* TODO: ... */` comments to tell the student what to write.
- TODO text should use the same type names, function names, and casing that the solution uses. If the solution uses `AccountId`, don't write `AccountID` in the TODO.
- Keep TODOs concise but specific enough that the student knows what to do without guessing.

### Solution files

- Every solution must compile, pass clippy, and pass tests. Run `bash check.sh check` to verify.
- The `solution/README.md` should contain exactly:

```md
# Step Title

You can find the solution to the previous step here.
```

Use "to" (not "for"), and always include "You can find" at the start.

### files.json

Each step with code has a `files.json` that tells the editor which files to display:

```json
{
  "template": [
    { "label": "src/balances.rs", "path": "./template/src/balances.rs", "status": "M" }
  ],
  "solution": [
    { "label": "src/balances.rs", "path": "./solution/src/balances.rs", "status": "M" }
  ]
}
```

- `status` is `"A"` for files added in this step and `"M"` for files modified from the previous step.
- Only list files that actually change in this step. If a file is identical between this template and the previous solution, don't include it.
- Labels should use the path relative to the crate root (e.g., `src/balances.rs`, not `balances.rs`).

## Code Style

### Formatting

All code must pass `cargo +nightly fmt`. The project uses `hard_tabs = true` (see `rustfmt.toml` in each step).

Run the formatter on all steps:

```sh
bash check.sh fix
```

### Clippy

All steps must pass clippy with warnings denied, but there are intentional exceptions:

- **`unused` warnings are suppressed in all steps.** Templates contain TODO stubs and `unimplemented!()` calls that naturally produce unused imports, variables, and dead code warnings. These are expected and would be distracting to students, so `check.sh` passes `-A unused` to all steps.

- **`clippy::ptr-arg` is allowed in steps 1-26.** Early steps use `&String` parameters (e.g., `fn balance(&self, who: &String)`) because the tutorial hasn't introduced generics yet. Clippy would normally suggest `&str` instead, but that change happens naturally in steps 18-22 when the code is refactored to use generic types. Allowing `ptr-arg` in early steps keeps the focus on the concepts being taught rather than premature optimization.

- **Steps 27+ enforce full clippy strictness** (minus `unused`). By this point the code uses generics and the `ptr-arg` lint is no longer relevant.

### Comments

- Use `///` doc comments for public functions and structs that students write.
- Use `//` regular comments for implementation notes and explanations.
- Be consistent across steps. If step 5 introduces `/// Create a new instance of the balances module.` on a function, every later step should keep it as `///`, not silently change it to `//`.

## Running Checks

### Validate all steps

```sh
bash check.sh check
```

This runs `cargo +nightly fmt --check`, `cargo +nightly clippy`, and `cargo test` on every template and solution directory in parallel.

### Auto-fix formatting and clippy issues

```sh
bash check.sh fix
```

### Check a single step manually

```sh
cd src/8/solution
cargo +nightly fmt --check
cargo +nightly clippy -- -D warnings
cargo test
```

## Building the Tutorial Book

The tutorial is published as an [mdBook](https://github.com/rust-lang/mdBook). The configuration is in `book.toml` and the table of contents is `src/SUMMARY.md`.

To build and preview locally:

```sh
mdbook serve
```

The step wrapper READMEs use `{{#include ./template/README.md}}` to pull in content, so editing the inner README is what matters. The outer `src/N/README.md` files are boilerplate wrappers that set up the interactive Monaco editor and should not need changes unless the editor UI itself is being updated.

If you add or remove a step, update `src/SUMMARY.md` to match.

## Submitting Changes

1. Make your changes on the `master` branch.
2. Run `bash check.sh check` and confirm everything passes.
3. For any step you changed, diff its template against the previous step's solution to verify the minimal diff rule.
4. Open a PR with a clear description of what you changed and why.

If your change affects multiple steps (e.g., renaming a type that carries forward), make sure you update every step from the point of introduction onward, and verify the solution-to-template chain is clean across all of them.
