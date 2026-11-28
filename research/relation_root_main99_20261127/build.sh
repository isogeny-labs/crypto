#!/bin/bash
# Bounded production-library correctness build; never full root libtest.
set -eu
cd "$(dirname "$0")/../.."
s=research/relation_root_main99_20261127
test -e "$s/CACHE_RELEASE.json"
test ! -e "$s/raw/build.exit"
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/build-preflight.stdout"
export CARGO_HOME=/dev/shm/relation-cargo-home-20261127 CARGO_TARGET_DIR=/dev/shm/relation-root-build-20261127
export RUSTC=/home/agent/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
export CARGO_PROFILE_RELEASE_OPT_LEVEL=0 CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_INCREMENTAL=false
export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
relation_main99_initial_bytes=$(du -sb "$CARGO_TARGET_DIR" | cut -f1)
printf 'target_bytes\tcgroup_memory_bytes\ttmpfs_free_bytes\n' > "$s/raw/build.resource.tsv"
setsid taskset -c 0 timeout --signal=TERM --kill-after=10 900 prlimit --as=12884901888 --fsize=1073741824 -- /home/agent/.cargo/bin/cargo test --release --locked --offline --manifest-path research/relation_root_runtime_20261127/harness/Cargo.toml --tests --no-run -j1 > "$s/raw/build.stdout" 2> "$s/raw/build.stderr" &
relation_main99_pid=$!
while kill -0 "$relation_main99_pid" 2>/dev/null; do
  relation_main99_bytes=$(du -sb "$CARGO_TARGET_DIR" | cut -f1)
  relation_main99_memory=$(cat /sys/fs/cgroup/memory.current)
  relation_main99_free=$(df -B1 --output=avail /dev/shm | tail -1 | tr -d ' ')
  printf '%s\t%s\t%s\n' "$relation_main99_bytes" "$relation_main99_memory" "$relation_main99_free" >> "$s/raw/build.resource.tsv"
  if test "$(( relation_main99_bytes - relation_main99_initial_bytes ))" -gt 2684354560 || test "$relation_main99_memory" -gt 15032385536 || test "$relation_main99_free" -lt 536870912; then
    printf '{"refused":true,"target_bytes":%s,"cgroup_memory_bytes":%s,"tmpfs_free_bytes":%s}\n' "$relation_main99_bytes" "$relation_main99_memory" "$relation_main99_free" > "$s/raw/build.refusal.json"
    kill -TERM -- "-$relation_main99_pid" 2>/dev/null || true
    sleep 1
    kill -KILL -- "-$relation_main99_pid" 2>/dev/null || true
    break
  fi
  sleep 2
done
set +e
wait "$relation_main99_pid"
relation_main99_exit=$?
set -e
printf '%s\n' "$relation_main99_exit" > "$s/raw/build.exit"
sha256sum -c "$s/SOURCE.sha256" > "$s/raw/build-postflight.stdout"
test "$relation_main99_exit" -eq 0
