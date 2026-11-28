#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_root_runtime_20261127
test ! -e "$s/raw/root-unit-retry.exit"
sha256sum -c "$s/ROOT_UNIT_SOURCE.sha256" > "$s/raw/root-unit-retry-preflight.stdout"
export CARGO_HOME=/dev/shm/relation-cargo-home-20261127 CARGO_TARGET_DIR=/dev/shm/relation-root-build-20261127
export RUSTC=/home/agent/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
export CARGO_PROFILE_RELEASE_OPT_LEVEL=0 CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_INCREMENTAL=false
export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 1800 prlimit --as=15032385536 --fsize=1073741824 -- /home/agent/.cargo/bin/cargo --config 'profile.release.package.crypto.codegen-units=256' test --release --locked --offline --lib --no-run -j1 > "$s/raw/root-unit-retry.stdout" 2> "$s/raw/root-unit-retry.stderr"
relation_unit_retry_exit=$?
set -e
printf '%s\n' "$relation_unit_retry_exit" > "$s/raw/root-unit-retry.exit"
sha256sum -c "$s/ROOT_UNIT_SOURCE.sha256" > "$s/raw/root-unit-retry-postflight.stdout"
test "$relation_unit_retry_exit" -eq 0
