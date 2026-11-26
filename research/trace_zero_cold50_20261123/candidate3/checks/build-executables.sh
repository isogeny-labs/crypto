#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto
export CARGO_HOME=/workspace/scratch/rust/cargo RUSTUP_HOME=/workspace/scratch/rust/toolchain
export VALGRIND_INCLUDE=/workspace/scratch/profile-tools/root/usr/include CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=/workspace/crypto/research/trace_zero_halftrace_elimination_20261123/native/target
trace_root=research/trace_zero_cold50_20261123/candidate3
trace_workspace=research/trace_zero_cold50_20261123/native
mkdir -p "$trace_root/bin"
for trace_variant in baseline incumbent elimination;do
 trace_flags=()
 case "$trace_variant" in baseline) trace_flags=(--features legacy-halftrace);; incumbent) trace_flags=(--features incumbent);; esac
 taskset -c 0 /workspace/scratch/rust/cargo/bin/cargo build --locked --release --bins --manifest-path "$trace_workspace/Cargo.toml" "${trace_flags[@]}" > "$trace_root/checks/build-$trace_variant.txt" 2>&1
 cp "$CARGO_TARGET_DIR/release/trace-zero-halftrace" "$trace_root/bin/$trace_variant"
 chmod a-w "$trace_root/bin/$trace_variant"
done
cp "$CARGO_TARGET_DIR/release/audit_halftrace" "$trace_root/bin/audit_halftrace"
cp "$CARGO_TARGET_DIR/release/check_polynomial" "$trace_root/bin/check_polynomial"
chmod a-w "$trace_root/bin/audit_halftrace" "$trace_root/bin/check_polynomial"
