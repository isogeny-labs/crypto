#!/usr/bin/env bash
# Native simulation/CSV contracts and deterministic replay, not EC verification.
set -euo pipefail
cd "$(dirname "$0")/.."

build/simulate --self-test
build/simulate --self-test --depth 1
build/simulate --self-test --depth 5
check_dir=$(mktemp -d)
trap 'rm -rf "$check_dir"' EXIT
build/simulate --seed 20261018 --out "$check_dir/first"
build/audit "$check_dir/first"
build/simulate --seed 20261018 --out "$check_dir/replay"
diff -qr "$check_dir/first" "$check_dir/replay"
if build/simulate --out "$check_dir/first" > "$check_dir/overwrite.log" 2>&1; then
  echo 'FAIL: an existing run directory was overwritten' >&2
  exit 1
fi
grep -q 'output already exists' "$check_dir/overwrite.log"

# Read the original, unmodified October 18 records without running a solver.
sha256sum --check runs.tar.xz.sha256
tar -xJf runs.tar.xz -C "$check_dir"
build/audit "$check_dir/run-final"
echo 'PASS: synthetic model contracts, ten-target summaries, frozen-record audit and replay'
