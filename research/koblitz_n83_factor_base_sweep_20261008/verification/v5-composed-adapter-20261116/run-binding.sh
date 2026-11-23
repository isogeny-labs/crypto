#!/usr/bin/env bash
set -euo pipefail
set -C

root="$(git rev-parse --show-toplevel)"
out="$root/research/koblitz_n83_factor_base_sweep_20261008/verification/v5-composed-adapter-20261116"
test "$(git -C "$root" status --porcelain)" = ""
for name in binding.start.utc binding.end.utc binding.exit.txt binding.stdout binding.stderr binding.source-commit.txt; do
  test ! -e "$out/$name"
done
date -u +%Y-%m-%dT%H:%M:%SZ > "$out/binding.start.utc"
git -C "$root" rev-parse HEAD > "$out/binding.source-commit.txt"

set +e
(
  cd "$root"
  env RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
    CARGO_TARGET_DIR=/Volumes/SSD990/tmp/n83-k600-budget-v3-target \
    TMPDIR=/Volumes/SSD990/tmp \
    cargo run --release --locked --offline -j 1 --quiet \
      --example koblitz_n83_factor_base_export -- \
      primary-v5-retained-binding-check 26960859236 \
      research/koblitz_n83_factor_base_sweep_20261008/verification/k600-v3-s3-adoption-20261115/attempt-1
) > "$out/binding.stdout" 2> "$out/binding.stderr"
status=$?
set -e

date -u +%Y-%m-%dT%H:%M:%SZ > "$out/binding.end.utc"
printf '%s\n' "$status" > "$out/binding.exit.txt"
exit "$status"
