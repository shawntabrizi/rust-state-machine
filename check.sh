#!/bin/bash

# Check if the 'steps' directory exists
if [ ! -d "steps" ]; then
  echo "Directory 'steps' does not exist."
  exit 1
fi

# Iterate through each subdirectory in the 'steps' directory
for dir in steps/*/; do
  # Check if the directory contains a Cargo.toml file
  if [ ! -f "$dir/Cargo.toml" ]; then
    echo "Skipping directory (no Cargo.toml found): $dir"
    continue
  fi

  if [ -d "$dir" ]; then
    echo "Entering directory: $dir"
    cd "$dir"

    # Run cargo fmt and cargo clippy
    echo "Running cargo +nightly fmt"
    cargo +nightly fmt

    echo "Running cargo clippy"
    RUSTFLAGS="-A unused" cargo clippy

    echo "Running cargo test"
    RUSTFLAGS="-A unused" cargo test

    # Return to the previous directory
    cd - > /dev/null
  fi
done

echo "All operations completed."
