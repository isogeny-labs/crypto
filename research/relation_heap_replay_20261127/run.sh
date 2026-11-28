#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_heap_replay_20261127
test ! -e "$s/raw/unit.exit"
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/preflight.stdout"
export CARGO_HOME=/dev/shm/relation-cargo-home-20261127 CARGO_TARGET_DIR=/dev/shm/relation-root-build-20261127
export RUSTC=/home/agent/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
export CARGO_PROFILE_RELEASE_OPT_LEVEL=0 CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_INCREMENTAL=false
export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
for action in unit build; do
  set +e
  if test "$action" = unit; then
    taskset -c 0 timeout --signal=TERM --kill-after=10 600 prlimit --as=2147483648 --fsize=134217728 -- /home/agent/.cargo/bin/cargo test --release --locked --offline --manifest-path "$s/candidate/Cargo.toml" --lib -j1 -- --test-threads=1 > "$s/raw/$action.stdout" 2> "$s/raw/$action.stderr"
  else
    taskset -c 0 timeout --signal=TERM --kill-after=10 600 prlimit --as=2147483648 --fsize=134217728 -- /home/agent/.cargo/bin/cargo build --release --locked --offline --manifest-path "$s/Cargo.toml" --bin relation-heap-replay-probe -j1 > "$s/raw/$action.stdout" 2> "$s/raw/$action.stderr"
  fi
  relation_heap_exit=$?
  set -e
  printf '%s\n' "$relation_heap_exit" > "$s/raw/$action.exit"
  test "$relation_heap_exit" -eq 0
done
sha256sum "$CARGO_TARGET_DIR/release/relation-heap-replay-probe" > "$s/EXECUTABLE.sha256"
set +e
taskset -c 0 timeout --signal=TERM --kill-after=10 600 prlimit --as=2147483648 --fsize=134217728 -- "$CARGO_TARGET_DIR/release/relation-heap-replay-probe" "$s" "$s/OUTCOME.json" > "$s/raw/replay.stdout" 2> "$s/raw/replay.stderr"
relation_heap_exit=$?
set -e
printf '%s\n' "$relation_heap_exit" > "$s/raw/replay.exit"
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/postflight.stdout"
test "$relation_heap_exit" -eq 0
