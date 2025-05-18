#!/bin/bash

# Ensure the script is executable by running: chmod +x run_lox_files.sh

# Loop through all .lox files in the current directory
for file in *.lox; do
  # Check if there are any .lox files
  if [[ -e "$file" ]]; then
    echo "Running: cargo run -p interpreter -- $file"
    cargo run -p interpreter -- "$file"
  else
    echo "No .lox files found in the current directory."
    break
  fi
done
