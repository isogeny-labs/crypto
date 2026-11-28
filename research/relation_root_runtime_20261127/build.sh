#!/bin/sh
# Unoptimized correctness compilation only. Outputs and pins are never replaced.
set -eu
cd "$(dirname "$0")/../.."
study=research/relation_root_runtime_20261127
test ! -e "$study/raw/build.exit"
test ! -e "$study/raw/build.stdout"
sha256sum -c "$study/BUILD_SOURCE.sha256" > "$study/raw/build-preflight.stdout"
export CARGO_HOME=/dev/shm/relation-cargo-home-20261127
export CARGO_TARGET_DIR=/dev/shm/relation-root-build-20261127
export RUSTC=/home/agent/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
export RUSTDOC=/home/agent/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustdoc
export CARGO_PROFILE_RELEASE_OPT_LEVEL=0 CARGO_PROFILE_RELEASE_LTO=off
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_PROFILE_RELEASE_DEBUG=0
export CARGO_PROFILE_RELEASE_INCREMENTAL=false CARGO_BUILD_JOBS=1
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 1800 prlimit --as=12884901888 --fsize=1073741824 -- /home/agent/.cargo/bin/cargo test --release --locked --offline --manifest-path "$study/harness/Cargo.toml" --test flat_constructor --no-run -j1 > "$study/raw/build.stdout" 2> "$study/raw/build.stderr"
result=$?
set -e
printf '%s\n' "$result" > "$study/raw/build.exit"
sha256sum -c "$study/BUILD_SOURCE.sha256" > "$study/raw/build-postflight.stdout"
exit "$result"
