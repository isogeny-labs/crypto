#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto
export CARGO_HOME=/workspace/scratch/rust/cargo RUSTUP_HOME=/workspace/scratch/rust/toolchain CARGO_BUILD_JOBS=1 VALGRIND_INCLUDE=/workspace/scratch/profile-tools/root/usr/include
export CARGO_TARGET_DIR=/workspace/crypto/research/trace_zero_halftrace_elimination_20261123/native/target
trace_study=research/trace_zero_cold50x_20261124
for trace_variant in baseline incumbent; do
 trace_feature=legacy-halftrace
 if [ "$trace_variant" = incumbent ]; then trace_feature=incumbent; fi
 taskset -c 0 /workspace/scratch/rust/cargo/bin/cargo build --locked --release --bins --manifest-path "$trace_study/native/Cargo.toml" --features "$trace_feature" > "$trace_study/checks/build-$trace_variant.txt" 2>&1
 cp "$CARGO_TARGET_DIR/release/trace-zero-halftrace" "$trace_study/bin/$trace_variant"
 chmod a-w "$trace_study/bin/$trace_variant"
done
cp "$CARGO_TARGET_DIR/release/audit_halftrace" "$trace_study/bin/audit_halftrace"
cp "$CARGO_TARGET_DIR/release/check_polynomial" "$trace_study/bin/check_polynomial"
chmod a-w "$trace_study/bin/audit_halftrace" "$trace_study/bin/check_polynomial"
export TRACE_BASELINE_EXE="$PWD/$trace_study/bin/baseline" TRACE_INCUMBENT_EXE="$PWD/$trace_study/bin/incumbent"
taskset -c 0 timeout 120 "$trace_study/bin/elimination" equivalence /workspace/scratch/linearized-workload-from-git.json /workspace/scratch/cold50x-dense-equivalence "$trace_study/dense-equivalence.jsonl" > "$trace_study/checks/dense-equivalence.stdout" 2> "$trace_study/checks/dense-equivalence.stderr"
echo $? > "$trace_study/checks/dense-equivalence.exit"
