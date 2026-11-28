#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_f4_syzygy_pairs_20261127
export CARGO_HOME=/dev/shm/relation-cargo-home-20261127 CARGO_TARGET_DIR=/dev/shm/relation-root-build-20261127
export RUSTC=/home/agent/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
export CARGO_PROFILE_RELEASE_OPT_LEVEL=0 CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_INCREMENTAL=false
export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
sha256sum -c "$s/NEGATIVE_SOURCE.sha256" > "$s/raw/negative-preflight.stdout"
# The original inline commands and their exits are retained; do not rerun them.
if test ! -e "$s/raw/negative-build.exit"; then
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 120 prlimit --as=6442450944 --fsize=134217728 -- /home/agent/.cargo/bin/cargo build --release --locked --offline --manifest-path "$s/Cargo.toml" --bin negative -j1 > "$s/raw/negative-build.stdout" 2> "$s/raw/negative-build.stderr"
  relation_pair_negative_build_exit=$?
  set -e
  printf '%s\n' "$relation_pair_negative_build_exit" > "$s/raw/negative-build.exit"
fi
test "$(cat "$s/raw/negative-build.exit")" -eq 0
if test ! -e "$s/raw/negative.exit"; then
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 600 prlimit --as=6442450944 --fsize=134217728 -- "$CARGO_TARGET_DIR/release/negative" "$s" "$CARGO_TARGET_DIR/release/relation-f4-syzygy-pairs" /dev/shm/relation-pair-negative-20261127 > "$s/raw/negative.stdout" 2> "$s/raw/negative.stderr"
  relation_pair_negative_exit=$?
  set -e
  printf '%s\n' "$relation_pair_negative_exit" > "$s/raw/negative.exit"
fi
test "$(cat "$s/raw/negative.exit")" -eq 0
sha256sum "$CARGO_TARGET_DIR/release/negative" > "$s/NEGATIVE_EXECUTABLE.sha256"
sha256sum -c "$s/NEGATIVE_SOURCE.sha256" > "$s/raw/negative-postflight.stdout"
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/negative-original-source-postflight.stdout"
sha256sum -c research/relation_f4_consequence_trace_20261127/PAYLOAD.sha256 > "$s/raw/negative-prior-evidence-postflight.stdout"
