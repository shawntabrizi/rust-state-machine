#!/bin/bash

# Check if the 'steps' directory exists
if [ ! -d "steps" ]; then
  echo "Directory 'steps' does not exist."
  exit 1
fi

# Iterate through each subdirectory in the 'steps' directory
for dir in steps/*/; do
  if [ -d "$dir" ]; then
    echo "Entering directory: $dir"
    cd "$dir"

    # Run cargo fmt and cargo clippy
    echo "Running cargo +nightly fmt"
    cargo +nightly fmt

    echo "Running cargo clippy --fix"
    cargo clippy --fix --allow-dirty

    # Return to the previous directory
    cd - > /dev/null
  fi
done

echo "All operations completed."
