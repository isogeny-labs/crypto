#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-allocation-profile-20261021
export CARGO_TARGET_DIR=/var/tmp/n83-profile-guard-target CARGO_BUILD_JOBS=4
cargo test --offline --manifest-path tools/n83-sweep-runner/Cargo.toml > "$work/guard-final-tests.txt" 2>&1
cargo build --offline --release --manifest-path tools/n83-sweep-runner/Cargo.toml > "$work/guard-final-build.txt" 2>&1
cp "$work/bin/runner-v21" "$work/build-v2/runner-v21-before-strict-profile-gates"
cp "$CARGO_TARGET_DIR/release/n83-sweep-runner" "$work/bin/runner-v21"
sha256sum "$work/bin/runner-v21" > "$work/guard-final.sha256"
