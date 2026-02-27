#!/bin/bash

set -e

# Default to 'check' mode if no argument is provided
MODE=${1:-check}
# Max parallel jobs (default: number of CPU cores)
MAX_JOBS=${2:-$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)}

# Check if the mode is valid
if [[ "$MODE" != "check" && "$MODE" != "fix" ]]; then
  echo "Invalid mode: $MODE. Use 'check' or 'fix'."
  echo "Usage: $0 [check|fix] [max_jobs]"
  exit 1
fi

# Check if the 'src' directory exists
if [ ! -d "src" ]; then
  echo "Directory 'src' does not exist."
  exit 1
fi

# Enable sccache if available
if command -v sccache &>/dev/null; then
  export RUSTC_WRAPPER=sccache
  echo "Using sccache for compilation caching"
else
  echo "Warning: sccache not found, running without compilation cache"
fi

ROOT_DIR=$(pwd)
FAILED_FILE=$(mktemp)
echo -n "" > "$FAILED_FILE"

# Process a single directory: takes the absolute dir path and step name
check_dir() {
  local dir="$1"
  local step_name="$2"
  local mode="$3"
  local label="$dir"

  # Run cargo fmt and cargo clippy based on the mode
  if [ "$mode" == "check" ]; then

    echo "[$label] Checking cargo fmt"
    if ! (cd "$dir" && cargo +nightly fmt -- --check) 2>&1; then
      echo "FAIL: $label (fmt)" >> "$FAILED_FILE"
      return 1
    fi

    echo "[$label] Checking cargo clippy"
    if [[ "$step_name" =~ ^([1-9]|1[0-9]|2[0-6])$ ]]; then
      # We specifically allow `clippy::ptr-arg` in the first 26 steps, because
      # it helps minimize the transition to generic types.
      if ! (cd "$dir" && RUSTFLAGS="-A unused -A clippy::ptr-arg" cargo +nightly clippy -- -D warnings) 2>&1; then
        echo "FAIL: $label (clippy)" >> "$FAILED_FILE"
        return 1
      fi
    else
      if ! (cd "$dir" && RUSTFLAGS="-A unused" cargo +nightly clippy -- -D warnings) 2>&1; then
        echo "FAIL: $label (clippy)" >> "$FAILED_FILE"
        return 1
      fi
    fi

    echo "[$label] Checking cargo test"
    if ! (cd "$dir" && RUSTFLAGS="-A unused -D warnings" cargo test) 2>&1; then
      echo "FAIL: $label (test)" >> "$FAILED_FILE"
      return 1
    fi

  elif [ "$mode" == "fix" ]; then

    echo "[$label] Running cargo fmt"
    if ! (cd "$dir" && RUSTFLAGS="-A unused" cargo +nightly fmt) 2>&1; then
      echo "FAIL: $label (fmt)" >> "$FAILED_FILE"
      return 1
    fi

    echo "[$label] Running cargo clippy"
    if [[ "$step_name" =~ ^([1-9]|1[0-9]|2[0-6])$ ]]; then
      # We specifically allow `clippy::ptr-arg` in the first 26 steps, because
      # it helps minimize the transition to generic types.
      if ! (cd "$dir" && RUSTFLAGS="-A unused -A clippy::ptr-arg" cargo +nightly clippy --fix --allow-dirty) 2>&1; then
        echo "FAIL: $label (clippy)" >> "$FAILED_FILE"
        return 1
      fi
    else
      if ! (cd "$dir" && RUSTFLAGS="-A unused" cargo +nightly clippy --fix --allow-dirty) 2>&1; then
        echo "FAIL: $label (clippy)" >> "$FAILED_FILE"
        return 1
      fi
    fi

    echo "[$label] Running cargo test"
    if ! (cd "$dir" && RUSTFLAGS="-A unused -D warnings" cargo test) 2>&1; then
      echo "FAIL: $label (test)" >> "$FAILED_FILE"
      return 1
    fi

  fi

  echo "[$label] OK"
}

export -f check_dir
export FAILED_FILE
export RUSTC_WRAPPER

echo "Running in '$MODE' mode with up to $MAX_JOBS parallel jobs"
echo "---"

# Collect all directories to process
DIRS=()
STEPS=()
for step_dir in src/*/; do
  step_name=$(basename "$step_dir")

  # Skip non-numeric directories (e.g. _gitorial)
  if ! [[ "$step_name" =~ ^[0-9]+$ ]]; then
    continue
  fi

  for dir in "$step_dir"*/; do
    if [ ! -f "$dir/Cargo.toml" ]; then
      echo "Skipping directory (no Cargo.toml found): $dir"
      continue
    fi

    if [ -d "$dir" ]; then
      DIRS+=("$ROOT_DIR/$dir")
      STEPS+=("$step_name")
    fi
  done
done

echo "Found ${#DIRS[@]} directories to process"
echo "---"

# Run jobs in parallel with a job slot limiter
PIDS=()
JOB_COUNT=0

for i in "${!DIRS[@]}"; do
  # Wait for a slot if we've hit the max
  while (( JOB_COUNT >= MAX_JOBS )); do
    # Wait for any child to finish
    wait -n 2>/dev/null || true
    JOB_COUNT=$(jobs -rp | wc -l | tr -d ' ')
  done

  check_dir "${DIRS[$i]}" "${STEPS[$i]}" "$MODE" &
  PIDS+=($!)
  JOB_COUNT=$(jobs -rp | wc -l | tr -d ' ')
done

# Wait for all remaining jobs
EXIT_CODE=0
for pid in "${PIDS[@]}"; do
  wait "$pid" || EXIT_CODE=1
done

echo "---"

# Report results
if [ -s "$FAILED_FILE" ]; then
  echo "FAILURES:"
  cat "$FAILED_FILE"
  rm -f "$FAILED_FILE"
  exit 1
else
  rm -f "$FAILED_FILE"
  echo "All operations completed successfully."
  exit 0
fi
