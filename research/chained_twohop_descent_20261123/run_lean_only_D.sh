#!/usr/bin/env bash
# Thin native caller. Python handles only file/hash/Git/JSON metadata.
set -euo pipefail
source_commit=${1:?source commit}
out=${2:?fresh absolute output directory}
compiler_temp=${3:?fresh absolute compiler temporary directory}
study_dir=$(cd "$(dirname "$0")" && pwd)
repo_dir=$(cd "$study_dir/../.." && pwd)
study_rel=research/chained_twohop_descent_20261123
export PATH="/home/agent/.cargo/bin:$PATH"
export LEAN4_BIN=${LEAN4_BIN:-/home/agent/.elan/toolchains/leanprover--lean4---v4.35.0/bin/lean}
export RAYON_NUM_THREADS=1 LEAN_NUM_THREADS=1 CARGO_BUILD_JOBS=1
export LEAN_STACK_SIZE_KB=16384 LEAN_MAIN_USE_THREAD=0 MIMALLOC_ARENA_RESERVE=65536
unset LEAN_PATH LEAN_OPTS CLIPPY_ARGS RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER
umask 077
mkdir "$out" "$compiler_temp"
export TMPDIR="$compiler_temp"
metadata="$study_dir/custody_metadata_D.py"
finish() {
  local status=$? guard_status=0
  trap - EXIT
  python3 "$metadata" finish "$out" "$status" > "$out/final-guards.stdout" 2> "$out/final-guards.stderr" || guard_status=$?
  if [[ "$status" -ne 0 ]]; then exit "$status"; fi
  exit "$guard_status"
}
trap finish EXIT
cd "$repo_dir"
python3 "$metadata" init "$source_commit" "$out" "$compiler_temp" > "$out/init.stdout" 2> "$out/init.stderr"
copied="$out/source/$study_rel"
phase() {
  local name=$1 status=0
  shift
  (
    ulimit -c 0
    ulimit -v 2097152
    timeout --signal=TERM --kill-after=5s 120s "$@"
  ) > "$out/$name.stdout" 2> "$out/$name.stderr" || status=$?
  python3 "$metadata" phase "$out" "$name" "$status"
  if [[ "$status" -ne 0 ]]; then return "$status"; fi
}
python3 "$metadata" pre "$out" > "$out/pre-call.stdout" 2> "$out/pre-call.stderr"
phase lean-version strace -f -e trace=clone,clone3,mmap,munmap -o "$out/lean-startup.trace" "$LEAN4_BIN" -j1 --version
phase lean "$LEAN4_BIN" -j1 --root="$copied/formal" -DwarningAsError=true -o "$out/TwoHopDescent.olean" "$copied/formal/TwoHopDescent.lean"
