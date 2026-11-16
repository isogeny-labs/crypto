#!/usr/bin/env bash
set -u
cd /Volumes/SSD990-2/tmp/n83-native-rank-query-wt || exit 97
printf '%s\n' "$(git rev-parse HEAD)" > /private/tmp/n83-checkpoint-isolated-20261115.commit
RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
CARGO_TARGET_DIR=/private/tmp/n83-checkpoint-isolated-20261115-target \
gtimeout 1800 cargo test --release --lib --locked --offline -j 1 \
  cryptanalysis::koblitz_groebner::checkpoint::tests::exact_pivot_state_survives_binary_roundtrip_at_every_column \
  -- --exact --nocapture > /private/tmp/n83-checkpoint-isolated-20261115.log 2>&1
rc=$?
printf '%s\n' "$rc" > /private/tmp/n83-checkpoint-isolated-20261115.exit
exit "$rc"
