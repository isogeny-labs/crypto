#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_canonical_scan_20261127
test ! -e "$s/raw/candidate-build.exit"
sha256sum -c "$s/CANDIDATE_SOURCE.sha256" > "$s/raw/candidate-build-preflight.stdout"
export CARGO_HOME=/dev/shm/relation-cargo-home-20261127 CARGO_TARGET_DIR=/dev/shm/relation-root-build-20261127
export RUSTC=/home/agent/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
export CARGO_PROFILE_RELEASE_OPT_LEVEL=0 CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_INCREMENTAL=false
export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 1800 prlimit --as=12884901888 --fsize=1073741824 -- /home/agent/.cargo/bin/cargo build --release --locked --offline --manifest-path "$s/harness/Cargo.toml" -j1 > "$s/raw/candidate-build.stdout" 2> "$s/raw/candidate-build.stderr"
relation_scan_build_exit=$?
set -e
printf '%s\n' "$relation_scan_build_exit" > "$s/raw/candidate-build.exit"
test "$relation_scan_build_exit" -eq 0
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 1800 prlimit --as=12884901888 --fsize=1073741824 -- /home/agent/.cargo/bin/cargo test --release --locked --offline --manifest-path "$s/harness/Cargo.toml" --test row_scan --no-run -j1 > "$s/raw/candidate-unit-build.stdout" 2> "$s/raw/candidate-unit-build.stderr"
relation_scan_build_exit=$?
set -e
printf '%s\n' "$relation_scan_build_exit" > "$s/raw/candidate-unit-build.exit"
sha256sum -c "$s/CANDIDATE_SOURCE.sha256" > "$s/raw/candidate-build-postflight.stdout"
test "$relation_scan_build_exit" -eq 0

set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 1800 prlimit --as=12884901888 --fsize=1073741824 -- /home/agent/.cargo/bin/cargo test --release --locked --offline --manifest-path research/relation_root_runtime_20261127/harness/Cargo.toml --tests --no-run -j1 > "$s/raw/native-build.stdout" 2> "$s/raw/native-build.stderr"
relation_scan_build_exit=$?
set -e
printf '%s\n' "$relation_scan_build_exit" > "$s/raw/native-build.exit"
sha256sum -c "$s/CANDIDATE_SOURCE.sha256" > "$s/raw/native-build-postflight.stdout"
test "$relation_scan_build_exit" -eq 0
