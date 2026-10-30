#!/bin/bash
# Archive a completed OR partial fixture without changing the retained directory.
set -euo pipefail
root=/home/ec2-user/crypto-ml-algebraic-targets-20261030
scratch=/home/ec2-user/ml-dataset-work-20261028
attempt=${1:-01}
[[ "$attempt" =~ ^[0-9]{2}$ ]]
binary=$scratch/algebraic-export-crypto-$attempt
output=$scratch/algebraic-cli-fixture-run-$attempt
archive=$output.tar.xz
files=()
if [[ -d "$output" ]]; then
  [[ ! -e "$archive" ]]
  tar -cJf "$archive" -C "$scratch" algebraic-cli-fixture-run-$attempt
  files+=("$archive")
fi
if [[ -f "$binary" ]]; then files+=("$binary"); fi
if [[ ${#files[@]} -gt 0 ]]; then
  bash "$root/research/ml_dataset_algebraic_targets_20261030/validation/archive.sh" "${files[@]}"
fi
