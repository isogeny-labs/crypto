#!/usr/bin/env bash
set -euo pipefail
set -C

root="$(git rev-parse --show-toplevel)"
out="$root/research/koblitz_n83_factor_base_sweep_20261008/verification/v5-composed-adapter-20261116/attempt-2"
git -C "$root" diff --quiet HEAD
test ! -e "$out"
mkdir "$out"
date -u +%Y-%m-%dT%H:%M:%SZ > "$out/start.utc"
git -C "$root" rev-parse HEAD > "$out/source-commit.txt"

set +e
(
  cd "$root"
  env RAYON_NUM_THREADS=1 \
    RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
    CARGO_TARGET_DIR=/Volumes/SSD990/tmp/n83-k600-budget-v3-target \
    TMPDIR=/Volumes/SSD990/tmp \
    cargo run --release --locked --offline -j 1 --quiet \
      --example koblitz_n83_factor_base_export -- \
      primary-v5-retained-binding-check 26960859236 \
      research/koblitz_n83_factor_base_sweep_20261008/verification/k600-v3-s3-adoption-20261115/attempt-1
) > "$out/stdout" 2> "$out/stderr"
status=$?
set -e

date -u +%Y-%m-%dT%H:%M:%SZ > "$out/end.utc"
printf '%s\n' "$status" > "$out/exit.txt"
exit "$status"
