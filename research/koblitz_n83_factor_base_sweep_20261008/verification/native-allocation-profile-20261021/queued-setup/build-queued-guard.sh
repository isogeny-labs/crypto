#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-allocation-profile-20261021
export CARGO_TARGET_DIR=/var/tmp/n83-profile-guard-target TMPDIR=/var/tmp/n83-allocation-profile-tmp CARGO_BUILD_JOBS=4
cargo test --offline --manifest-path tools/n83-sweep-runner/Cargo.toml > "$work/queued-guard-tests.txt" 2>&1
cargo build --offline --release --manifest-path tools/n83-sweep-runner/Cargo.toml > "$work/queued-guard-build.txt" 2>&1
cp "$CARGO_TARGET_DIR/release/n83-sweep-runner" "$work/bin/runner-v22"
chmod 755 "$work/bin/runner-v22"
sha256sum "$work/bin/runner-v22" > "$work/queued-guard.sha256"
printf '%s\n' PASS_NATIVE_QUEUE_TESTS_RELEASE_BUILD > "$work/queued-guard.status.txt"
