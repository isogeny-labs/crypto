#!/usr/bin/env bash
set -u
cd /Volumes/SSD990-2/tmp/n83-native-rank-query-wt || exit 97
if test -n "$(git status --porcelain)"; then echo 'source not clean' >&2; exit 98; fi
source_sha=$(git rev-parse HEAD)
printf '%s\n' "$source_sha" > /private/tmp/n83-root-lib-warm-20261115.commit
target=/Volumes/SSD990-2/tmp/n83-checkpoint-extended-20261115-target
scratch=/Volumes/SSD990-2/tmp/n83-root-lib-warm-20261115-scratch
if ! test -d "$target" || test -e "$scratch"; then echo 'target missing or scratch exists' >&2; exit 99; fi
mkdir -p "$scratch"
wrapper="$scratch/run-with-receipt.sh"
git show "$source_sha":research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-validation-wrapper-20261115/run-with-receipt.sh > "$wrapper" || exit 96
if test "$(shasum -a 256 "$wrapper" | awk '{print $1}')" != 090a3020c0ec2862735af0b1c0ab10258199fd4bcefc11c1b55daa0113c48bb3; then
  echo 'wrapper hash mismatch' >&2
  exit 95
fi
bash "$wrapper" /private/tmp/n83-root-lib-warm-20261115.log /private/tmp/n83-root-lib-warm-20261115.receipt -- \
  /usr/bin/env RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
  CARGO_TARGET_DIR="$target" TMPDIR="$scratch" SPOT_NATIVE_TEST_PARENT="$scratch" \
  gtimeout 7200 cargo test --release --lib --locked --offline -j 1 -- --test-threads=2
exit $?
