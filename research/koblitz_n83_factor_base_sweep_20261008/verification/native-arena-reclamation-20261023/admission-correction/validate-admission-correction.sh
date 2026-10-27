#!/usr/bin/env bash
set -euo pipefail
cd /home/ec2-user/crypto-n83-resume
work=/home/ec2-user/scratch/n83-arena-reclamation-20261022
log="$work/admission-correction-validation"
mkdir "$log"
export CARGO_TARGET_DIR=/var/tmp/n83-arena-target CARGO_BUILD_JOBS=4 TMPDIR=/var/tmp/n83-arena-tmp
export PATH=/home/ec2-user/.cargo/bin:/home/ec2-user/.local/bin:/usr/local/bin:/usr/bin:/bin
exec 9>/tmp/crypto-bench.lock
flock -x 9
git rev-parse HEAD 'HEAD^{tree}' > "$log/base-source.txt"
git diff -- research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs tools/n83-sweep-runner > "$log/source.patch"
cp tools/n83-sweep-runner/src/admission_refusal.rs "$log/admission_refusal.rs"
rustc --version > "$log/rustc.txt"
cargo fmt --check --manifest-path tools/n83-sweep-runner/Cargo.toml > "$log/format.txt" 2>&1
cargo test --offline --release --example koblitz_n83_factor_base_export > "$log/exporter.txt" 2>&1
cargo build --offline --release --example koblitz_n83_factor_base_export > "$log/worker-build.txt" 2>&1
test ! -e "$work/bin/native-arena-admission-v2"
cp "$CARGO_TARGET_DIR/release/examples/koblitz_n83_factor_base_export" "$work/bin/native-arena-admission-v2"
export CARGO_TARGET_DIR=/var/tmp/n83-arena-guard-target
cargo test --offline --release --manifest-path tools/n83-sweep-runner/Cargo.toml > "$log/guard.txt" 2>&1
cargo build --offline --release --manifest-path tools/n83-sweep-runner/Cargo.toml > "$log/guard-build.txt" 2>&1
test ! -e "$work/bin/runner-arena-continuation-v4"
cp "$CARGO_TARGET_DIR/release/n83-sweep-runner" "$work/bin/runner-arena-continuation-v4"
sha256sum "$work/bin/native-arena-admission-v2" "$work/bin/runner-arena-continuation-v4" > "$log/binaries.sha256"
printf '%s\n' PASS_ADMISSION_CORRECTION_NATIVE_TESTS_AND_BUILDS > "$log/status.txt"
