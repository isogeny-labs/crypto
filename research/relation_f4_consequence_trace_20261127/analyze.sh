#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_f4_consequence_trace_20261127
export CARGO_HOME=/dev/shm/relation-cargo-home-20261127 CARGO_TARGET_DIR=/dev/shm/relation-root-build-20261127
export RUSTC=/home/agent/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
export CARGO_PROFILE_RELEASE_OPT_LEVEL=0 CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_INCREMENTAL=false
export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
sha256sum -c "$s/ANALYSIS_SOURCE.sha256" > "$s/raw/analysis-preflight.stdout"
# The first build was executed with exactly these settings and retained. This
# conditional lets a clean reconstruction build it, without repeating that run.
if test ! -e "$s/raw/summary-build.exit"; then
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 120 prlimit --as=6442450944 --fsize=134217728 -- /home/agent/.cargo/bin/cargo build --release --locked --offline --manifest-path "$s/Cargo.toml" --bin summarize -j1 > "$s/raw/summary-build.stdout" 2> "$s/raw/summary-build.stderr"
  relation_summary_build_exit=$?
  set -e
  printf '%s\n' "$relation_summary_build_exit" > "$s/raw/summary-build.exit"
fi
test "$(cat "$s/raw/summary-build.exit")" -eq 0
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 120 prlimit --as=6442450944 --fsize=134217728 -- /home/agent/.cargo/bin/cargo test --release --locked --offline --manifest-path "$s/Cargo.toml" --bin summarize -j1 -- --test-threads=1 > "$s/raw/summary-unit.stdout" 2> "$s/raw/summary-unit.stderr"
relation_summary_unit_exit=$?
set -e
printf '%s\n' "$relation_summary_unit_exit" > "$s/raw/summary-unit.exit"
test "$relation_summary_unit_exit" -eq 0
sha256sum "$CARGO_TARGET_DIR/release/summarize" > "$s/SUMMARY_EXECUTABLE.sha256"
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 60 prlimit --as=6442450944 --fsize=134217728 -- "$CARGO_TARGET_DIR/release/summarize" "$s" "$s/OUTCOME.json" > "$s/raw/summary.stdout" 2> "$s/raw/summary.stderr"
relation_summary_exit=$?
set -e
printf '%s\n' "$relation_summary_exit" > "$s/raw/summary.exit"
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/analysis-original-source-postflight.stdout"
sha256sum -c "$s/PREPARED.sha256" > "$s/raw/analysis-prepared-postflight.stdout"
sha256sum -c "$s/ANALYSIS_SOURCE.sha256" > "$s/raw/analysis-postflight.stdout"
test "$relation_summary_exit" -eq 0
