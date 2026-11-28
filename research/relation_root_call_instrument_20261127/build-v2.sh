#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_root_call_instrument_20261127
test ! -e "$s/raw/v2-build.exit"
sha256sum -c "$s/SOURCE_V2.sha256" > "$s/raw/v2-build-preflight.stdout"
export CARGO_HOME=/dev/shm/relation-cargo-home-20261127 CARGO_TARGET_DIR=/dev/shm/relation-root-build-20261127
export RUSTC=/home/agent/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
export CARGO_PROFILE_RELEASE_OPT_LEVEL=0 CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_INCREMENTAL=false
export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
for action in v2-build v2-unit-build; do
  set +e
  if test "$action" = v2-build; then
    taskset -c 0 timeout --signal=TERM --kill-after=10 1800 prlimit --as=12884901888 --fsize=1073741824 -- /home/agent/.cargo/bin/cargo build --release --locked --offline --manifest-path "$s/Cargo.toml" --bins -j1 > "$s/raw/$action.stdout" 2> "$s/raw/$action.stderr"
  else
    taskset -c 0 timeout --signal=TERM --kill-after=10 1800 prlimit --as=12884901888 --fsize=1073741824 -- /home/agent/.cargo/bin/cargo test --release --locked --offline --manifest-path "$s/Cargo.toml" --bins --no-run -j1 > "$s/raw/$action.stdout" 2> "$s/raw/$action.stderr"
  fi
  relation_instrument_exit=$?
  set -e
  printf '%s\n' "$relation_instrument_exit" > "$s/raw/$action.exit"
  test "$relation_instrument_exit" -eq 0
done
sha256sum -c "$s/SOURCE_V2.sha256" > "$s/raw/v2-build-postflight.stdout"
