#!/bin/bash
# Compiler resource supervision only. Never invokes an application timing mode.
set -eu
relation_opt_study=$(cd "$(dirname "$0")" && pwd)
relation_opt_version=$1
relation_opt_source=$2
relation_opt_manifest=$3
relation_opt_step=$4
shift 4
relation_opt_target=/dev/shm/relation-opt-build-target-20261127
relation_opt_log="$relation_opt_study/raw/$relation_opt_version-$relation_opt_step"
test ! -e "$relation_opt_log.exit"
mkdir -p "$relation_opt_target/tmp"
export CARGO_HOME=/dev/shm/relation-cargo-home-20261127
export CARGO_TARGET_DIR="$relation_opt_target" TMPDIR="$relation_opt_target/tmp"
export RUSTC=/home/agent/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc RUSTUP_TOOLCHAIN=stable
export CARGO_PROFILE_RELEASE_OPT_LEVEL=3 CARGO_PROFILE_RELEASE_LTO=off CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16
export CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_INCREMENTAL=false
export CARGO_BUILD_JOBS=1 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 RUST_TEST_THREADS=1
cd "$relation_opt_source"
sha256sum -c "$relation_opt_study/SOURCE_${relation_opt_version^^}.sha256" > "$relation_opt_log.source-before"
relation_opt_used=0
if test -e "$relation_opt_study/raw/compile-used-seconds"; then
  relation_opt_used=$(cat "$relation_opt_study/raw/compile-used-seconds")
fi
test "$relation_opt_used" -lt 3600
relation_opt_start=$(date +%s)
printf 'target_bytes\tcgroup_memory_bytes\ttmpfs_free_bytes\n' > "$relation_opt_log.resource.tsv"
setsid taskset -c 0 timeout --signal=TERM --kill-after=10 1200 prlimit --as=10737418240 --fsize=1073741824 -- /home/agent/.cargo/bin/cargo "$@" --release --locked --offline --manifest-path "$relation_opt_manifest" -j1 > "$relation_opt_log.stdout" 2> "$relation_opt_log.stderr" &
relation_opt_pid=$!
relation_opt_cleanup() {
  if kill -0 "$relation_opt_pid" 2>/dev/null; then
    kill -TERM -- "-$relation_opt_pid" 2>/dev/null || true
    sleep 1
    kill -KILL -- "-$relation_opt_pid" 2>/dev/null || true
  fi
}
trap relation_opt_cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM HUP
while kill -0 "$relation_opt_pid" 2>/dev/null; do
  relation_opt_bytes=$({ du -sb "$relation_opt_target" 2>> "$relation_opt_log.resource-probe.stderr" || true; } | cut -f1)
  relation_opt_memory=$(cat /sys/fs/cgroup/memory.current)
  relation_opt_free=$(df -B1 --output=avail /dev/shm | tail -1 | tr -d ' ')
  relation_opt_elapsed=$(( $(date +%s) - relation_opt_start ))
  printf '%s\t%s\t%s\n' "$relation_opt_bytes" "$relation_opt_memory" "$relation_opt_free" >> "$relation_opt_log.resource.tsv"
  if test "$relation_opt_bytes" -gt 1610612736 || test "$relation_opt_memory" -gt 16106127360 || test "$relation_opt_free" -lt 536870912 || test "$(( relation_opt_used + relation_opt_elapsed ))" -ge 3600; then
    printf '{"resource_refusal":true,"target_bytes":%s,"cgroup_memory_bytes":%s,"tmpfs_free_bytes":%s,"aggregate_compile_seconds":%s}\n' "$relation_opt_bytes" "$relation_opt_memory" "$relation_opt_free" "$(( relation_opt_used + relation_opt_elapsed ))" > "$relation_opt_log.refusal.json"
    kill -TERM -- "-$relation_opt_pid" 2>/dev/null || true
    sleep 1
    kill -KILL -- "-$relation_opt_pid" 2>/dev/null || true
    break
  fi
  sleep 2
done
set +e
wait "$relation_opt_pid"
relation_opt_exit=$?
set -e
printf '%s\n' "$relation_opt_exit" > "$relation_opt_log.exit"
relation_opt_elapsed=$(( $(date +%s) - relation_opt_start ))
printf '%s\n' "$(( relation_opt_used + relation_opt_elapsed ))" > "$relation_opt_study/raw/compile-used-seconds"
set +e
sha256sum -c "$relation_opt_study/SOURCE_${relation_opt_version^^}.sha256" > "$relation_opt_log.source-after"
relation_opt_source_exit=$?
set -e
printf '%s\n' "$relation_opt_source_exit" > "$relation_opt_log.source-after.exit"
test "$relation_opt_source_exit" -eq 0
exit "$relation_opt_exit"
