#!/usr/bin/env bash
set -euo pipefail
cd /workspace/crypto
trace_study="$(cd -- "$(dirname -- "$0")/.." && pwd)"
export CARGO_HOME=/workspace/scratch/rust/cargo RUSTUP_HOME=/workspace/scratch/rust/toolchain CARGO_BUILD_JOBS=1 VALGRIND_INCLUDE=/workspace/scratch/profile-tools/root/usr/include CARGO_TARGET_DIR=/workspace/scratch/cold10m-isolated-target
mkdir -p "$trace_study/bin"
for trace_variant in elimination baseline incumbent; do
  trace_flags=()
  if [ "$trace_variant" = baseline ]; then trace_flags=(--features legacy-halftrace); fi
  if [ "$trace_variant" = incumbent ]; then trace_flags=(--features incumbent); fi
  taskset -c 0 /workspace/scratch/rust/cargo/bin/cargo build --locked --release --bins --manifest-path "$trace_study/native/Cargo.toml" "${trace_flags[@]}" > "$trace_study/checks/build-$trace_variant.txt" 2>&1
  test ! -e "$trace_study/bin/$trace_variant"
  cp "$CARGO_TARGET_DIR/release/trace-zero-halftrace" "$trace_study/bin/$trace_variant"
  chmod a-w "$trace_study/bin/$trace_variant"
  if [ "$trace_variant" = elimination ]; then
    cp "$CARGO_TARGET_DIR/release/audit_halftrace" "$trace_study/bin/audit_halftrace"
    cp "$CARGO_TARGET_DIR/release/check_polynomial" "$trace_study/bin/check_polynomial"
    cp "$CARGO_TARGET_DIR/release/preflight_guard" "$trace_study/bin/preflight_guard"
  fi
done
export TRACE_BASELINE_EXE="$trace_study/bin/baseline" TRACE_INCUMBENT_EXE="$trace_study/bin/incumbent"
test -x "$TRACE_BASELINE_EXE"
test -x "$TRACE_INCUMBENT_EXE"
taskset -c 0 timeout 120 "$trace_study/bin/elimination" equivalence /workspace/scratch/linearized-workload-from-git.json /workspace/scratch/cold10m-equivalence "$trace_study/equivalence.jsonl" > "$trace_study/checks/equivalence.stdout" 2> "$trace_study/checks/equivalence.stderr"
printf '%s\n' "$?" > "$trace_study/checks/equivalence.exit"
