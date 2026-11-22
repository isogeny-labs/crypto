#!/usr/bin/env bash
set -u
cd /Volumes/SSD990-2/tmp/n83-native-rank-query-wt || exit 97
if test -n "$(git status --porcelain)"; then echo 'source not clean' >&2; exit 98; fi
printf '%s\n' "$(git rev-parse HEAD)" > /private/tmp/n83-checkpoint-retry-20261115.commit
scratch=/Volumes/SSD990-2/tmp/n83-checkpoint-retry-20261115-scratch
target=/Volumes/SSD990-2/tmp/n83-checkpoint-retry-20261115-target
if test -e "$scratch" || test -e "$target"; then echo 'scratch or target exists' >&2; exit 99; fi
mkdir -p "$scratch"
research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-validation-wrapper-20261115/run-with-receipt.sh \
  /private/tmp/n83-checkpoint-retry-20261115.log \
  /private/tmp/n83-checkpoint-retry-20261115.receipt -- \
  /usr/bin/env RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
  CARGO_TARGET_DIR="$target" TMPDIR="$scratch" SPOT_NATIVE_TEST_PARENT="$scratch" \
  gtimeout 1800 cargo test --release --lib --locked --offline -j 1 \
  cryptanalysis::koblitz_groebner::checkpoint::tests::exact_pivot_state_survives_binary_roundtrip_at_every_column \
  -- --exact --nocapture
exit $?
