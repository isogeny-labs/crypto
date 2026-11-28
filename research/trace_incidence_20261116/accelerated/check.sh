#!/usr/bin/env bash
# Development correctness only: no scientific timings are collected.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
check_dir=$(mktemp -d "${TMPDIR:-/tmp}/trace-accelerated-check.XXXXXX")
printf '%s\n' "$check_dir"
cp "$here/build/audit" "$check_dir/qualifier"
cp "$here/build/benchmark" "$check_dir/benchmark.qualifier"
chmod 0555 "$check_dir/qualifier" "$check_dir/benchmark.qualifier"
sources=("$here/accelerated.cpp" "$here/accelerated.hpp" "$here/audit.cpp" "$here/benchmark.cpp" "$here/rust/src/lib.rs" "$here/rust/Cargo.toml" "$here/rust/Cargo.lock" "$check_dir/qualifier" "$check_dir/benchmark.qualifier")
sha256sum "${sources[@]}" > "$check_dir/build.sha256"
set -o noclobber
cp "$check_dir/build.sha256" "$check_dir/pre-call.sha256"
"$check_dir/qualifier" "$here/.." > "$check_dir/checks.json"
for generator in affine straus window; do
  for norm in horner compose support support-bloom first first-bloom frozen-pairs; do
    "$check_dir/benchmark.qualifier" --check-only "$generator" "$norm" "$here/../runpod/nonlinear-control.json" 20261116 >> "$check_dir/driver-checks.jsonl"
  done
done
sha256sum "${sources[@]}" > "$check_dir/post-call.sha256"
cmp "$check_dir/pre-call.sha256" "$check_dir/post-call.sha256"
cat "$check_dir/checks.json"
