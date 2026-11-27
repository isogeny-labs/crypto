#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto
export CARGO_HOME=/workspace/scratch/rust/cargo RUSTUP_HOME=/workspace/scratch/rust/toolchain CARGO_BUILD_JOBS=1 VALGRIND_INCLUDE=/workspace/scratch/profile-tools/root/usr/include
export CARGO_TARGET_DIR=/workspace/scratch/cold50x-isolated-target
trace_study=research/trace_zero_cold50x_20261124
mkdir -p "$trace_study/bin-v2"
for trace_variant in elimination baseline incumbent; do
 trace_flags=()
 case "$trace_variant" in baseline) trace_flags=(--features legacy-halftrace);; incumbent) trace_flags=(--features incumbent);; esac
 taskset -c 0 /workspace/scratch/rust/cargo/bin/cargo build --locked --release --bins --manifest-path "$trace_study/native/Cargo.toml" "${trace_flags[@]}" > "$trace_study/checks/isolated-build-$trace_variant.txt" 2>&1
 cp "$CARGO_TARGET_DIR/release/trace-zero-halftrace" "$trace_study/bin-v2/$trace_variant"
 chmod a-w "$trace_study/bin-v2/$trace_variant"
done
cp "$CARGO_TARGET_DIR/release/audit_halftrace" "$trace_study/bin-v2/audit_halftrace"
cp "$CARGO_TARGET_DIR/release/check_polynomial" "$trace_study/bin-v2/check_polynomial"
export TRACE_BASELINE_EXE="$PWD/$trace_study/bin-v2/baseline" TRACE_INCUMBENT_EXE="$PWD/$trace_study/bin-v2/incumbent"
taskset -c 0 timeout 90 "$trace_study/bin-v2/elimination" equivalence /workspace/scratch/linearized-workload-from-git.json /workspace/scratch/cold50x-isolated-equivalence "$trace_study/isolated-equivalence.jsonl" > "$trace_study/checks/isolated-equivalence.stdout" 2> "$trace_study/checks/isolated-equivalence.stderr"
echo $? > "$trace_study/checks/isolated-equivalence.exit"
