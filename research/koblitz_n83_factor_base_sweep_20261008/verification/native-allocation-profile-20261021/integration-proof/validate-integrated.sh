#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-allocation-profile-20261021
export CARGO_TARGET_DIR=/var/tmp/n83-1m-validation-target CARGO_BUILD_JOBS=4 TMPDIR=/var/tmp/n83-allocation-profile-tmp
mkdir -p "$TMPDIR" "$work/integrated-validation"
rustc --version > "$work/integrated-validation/rustc.txt"
cargo --version > "$work/integrated-validation/cargo.txt"
git rev-parse HEAD HEAD^{tree} > "$work/integrated-validation/source.txt"
cargo test --offline --release --lib > "$work/integrated-validation/library.txt" 2>&1
cargo test --offline --release --example koblitz_n83_factor_base_export > "$work/integrated-validation/exporter.txt" 2>&1
cargo test --offline --release --example koblitz_n83_free_selector_export_control > "$work/integrated-validation/model-checker.txt" 2>&1
CARGO_TARGET_DIR=/var/tmp/n83-profile-guard-target cargo test --offline --release --manifest-path tools/n83-sweep-runner/Cargo.toml > "$work/integrated-validation/guard.txt" 2>&1
printf '%s\n' PASS_RELEASE_LIBRARY_EXPORTER_MODEL_CHECKER_NATIVE_GUARD > "$work/integrated-validation/status.txt"
