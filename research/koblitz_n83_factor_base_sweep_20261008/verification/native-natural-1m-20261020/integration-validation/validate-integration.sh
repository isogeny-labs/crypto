#!/usr/bin/env bash
set -uo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-natural-1m-20261020/integration-validation
export CARGO_TARGET_DIR=/var/tmp/n83-1m-validation-target CARGO_BUILD_JOBS=4 TMPDIR=/var/tmp/n83-1m-validation-tmp
mkdir -p "$TMPDIR"
cargo test --offline --release --lib -- --test-threads=1 > "$work/validation-full-lib.log" 2>&1
rc=$?
printf '%s\n' "$rc" > "$work/validation-full-lib.exit"
test "$rc" -eq 0 || exit "$rc"
for example in koblitz_n83_factor_base_export koblitz_n83_free_selector_export_control; do
 cargo test --offline --release --example "$example" > "$work/validation-$example.log" 2>&1
 rc=$?
 printf '%s\n' "$rc" > "$work/validation-$example.exit"
 test "$rc" -eq 0 || exit "$rc"
done
