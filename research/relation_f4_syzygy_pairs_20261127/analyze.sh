#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_f4_syzygy_pairs_20261127
export CARGO_HOME=/dev/shm/relation-cargo-home-20261127 CARGO_TARGET_DIR=/dev/shm/relation-root-build-20261127
export RUSTC=/home/agent/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
export CARGO_PROFILE_RELEASE_OPT_LEVEL=0 CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_INCREMENTAL=false
export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
sha256sum -c "$s/ANALYSIS_SOURCE.sha256" > "$s/raw/analysis-preflight.stdout"
# Initial execution used these exact settings; retained exit files prevent it
# from being repeated as a hidden build or numerical follow-up.
if test ! -e "$s/raw/review-build.exit"; then
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 120 prlimit --as=6442450944 --fsize=134217728 -- /home/agent/.cargo/bin/cargo build --release --locked --offline --manifest-path "$s/Cargo.toml" --bin review -j1 > "$s/raw/review-build.stdout" 2> "$s/raw/review-build.stderr"
  relation_pair_review_build_exit=$?
  set -e
  printf '%s\n' "$relation_pair_review_build_exit" > "$s/raw/review-build.exit"
fi
test "$(cat "$s/raw/review-build.exit")" -eq 0
if test ! -e "$s/raw/review.exit"; then
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 60 prlimit --as=6442450944 --fsize=134217728 -- "$CARGO_TARGET_DIR/release/review" "$s/OUTCOME.json" "$s/REPORT_DATA.json" > "$s/raw/review.stdout" 2> "$s/raw/review.stderr"
  relation_pair_review_exit=$?
  set -e
  printf '%s\n' "$relation_pair_review_exit" > "$s/raw/review.exit"
fi
test "$(cat "$s/raw/review.exit")" -eq 0
sha256sum "$CARGO_TARGET_DIR/release/review" > "$s/REVIEW_EXECUTABLE.sha256"
sha256sum -c "$s/ANALYSIS_SOURCE.sha256" > "$s/raw/analysis-postflight.stdout"
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/analysis-original-source-postflight.stdout"
