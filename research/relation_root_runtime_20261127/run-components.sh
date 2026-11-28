#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_root_runtime_20261127
export CARGO_HOME=/dev/shm/relation-cargo-home-20261127 CARGO_TARGET_DIR=/dev/shm/relation-root-build-20261127
export RUSTC=/home/agent/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
export CARGO_PROFILE_RELEASE_OPT_LEVEL=0 CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_INCREMENTAL=false
export CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
sha256sum -c "$s/CANDIDATE_SOURCE.sha256" > "$s/raw/component-source-preflight.stdout"
for lane in producer producer-counted checker; do
  test ! -e "$s/raw/$lane-build.exit"
  case "$lane" in
    producer) manifest=crates/certified-relations/Cargo.toml; target_name=certified_relations; set -- ;;
    producer-counted) manifest=crates/certified-relations/Cargo.toml; target_name=certified_relations; set -- --features count-field-ops ;;
    checker) manifest=crates/relation-replay-checker/Cargo.toml; target_name=relation_replay_checker; set -- ;;
  esac
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 1200 prlimit --as=2147483648 --fsize=134217728 -- /home/agent/.cargo/bin/cargo test --release --locked --offline --manifest-path "$manifest" --lib --no-run --message-format=json -j1 "$@" > "$s/raw/$lane-build.jsonl" 2> "$s/raw/$lane-build.stderr"
  relation_component_exit=$?
  set -e
  printf '%s\n' "$relation_component_exit" > "$s/raw/$lane-build.exit"
  test "$relation_component_exit" -eq 0
  artifact=$(jq -r --arg name "$target_name" 'select(.reason=="compiler-artifact" and .profile.test==true and .target.name==$name)|.executable' "$s/raw/$lane-build.jsonl")
  test -f "$artifact"
  qualifier="/dev/shm/relation-root-workers-20261127/$lane"
  test ! -e "$qualifier"
  cp "$artifact" "$qualifier"
  chmod a-w "$qualifier"
  sha256sum "$qualifier" > "$s/raw/$lane-precall.sha256"
  taskset -c 0 "$qualifier" --list > "$s/raw/$lane-roster.stdout"
  set +e
  taskset -c 0 timeout --signal=TERM --kill-after=10 1200 prlimit --as=2147483648 --fsize=134217728 -- "$qualifier" --test-threads=1 --nocapture > "$s/raw/$lane.stdout" 2> "$s/raw/$lane.stderr"
  relation_component_exit=$?
  set -e
  printf '%s\n' "$relation_component_exit" > "$s/raw/$lane.exit"
  test "$relation_component_exit" -eq 0
done
sha256sum -c "$s/CANDIDATE_SOURCE.sha256" > "$s/raw/component-source-postflight.stdout"
