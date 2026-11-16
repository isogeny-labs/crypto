#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-allocation-profile-20261021
export CARGO_BUILD_JOBS=4 TMPDIR=/var/tmp/n83-1m-validation-tmp
CARGO_TARGET_DIR=/var/tmp/n83-profile-guard-target cargo test --offline --manifest-path tools/n83-sweep-runner/Cargo.toml > "$work/build-v2/guard-tests.txt" 2>&1
CARGO_TARGET_DIR=/var/tmp/n83-profile-guard-target cargo build --offline --release --manifest-path tools/n83-sweep-runner/Cargo.toml > "$work/build-v2/guard-build.txt" 2>&1
cp /var/tmp/n83-profile-guard-target/release/n83-sweep-runner "$work/bin/runner-v21"
export CARGO_TARGET_DIR=/var/tmp/n83-1m-validation-target
cargo test --offline --release --lib cryptanalysis::sat::tests -- --test-threads=1 > "$work/build-v2/sat-tests.txt" 2>&1
cargo build --offline --release --example koblitz_n83_factor_base_export > "$work/build-v2/worker-build.txt" 2>&1
cp "$CARGO_TARGET_DIR/release/examples/koblitz_n83_factor_base_export" "$work/bin/native-storage-baseline"
for example in koblitz_n83_factor_base_export koblitz_n83_free_selector_export_control; do
 cargo test --offline --release --example "$example" > "$work/build-v2/$example-tests.txt" 2>&1
done
sha256sum "$work/bin/runner-v21" "$work/bin/native-storage-baseline" > "$work/binaries.sha256"
