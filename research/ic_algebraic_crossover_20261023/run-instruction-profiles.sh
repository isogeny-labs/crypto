#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
run=${1:?fresh profile directory}
bin=${IC_PROFILE_BIN:?frozen ecbench executable}
vg=${IC_VALGRIND:?Valgrind executable}
export VALGRIND_LIB=${IC_VALGRIND_LIB:?Valgrind tool libraries}
debug=${IC_VALGRIND_DEBUG:?matching debug files}
timer=${IC_TIMER:-/workspace/.tools/usr/bin/time}
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 DEBUGINFOD_URLS=
export ECBENCH_CALLGRIND_SOLVE=1 ECBENCH_CALLGRIND_TARGET=1
test ! -e "$run"
mkdir -p "$run"
cp "$0" "$run/runner.sh"
git rev-parse HEAD > "$run/working-head.txt"
sha256sum "$bin" "$vg" "$timer" "$0" "$study/PROFILE_PROTOCOL.md" \
  "$study/pipeline-runs/structured-n13-20261025-01/spec.json" > "$run/hashes.txt"
cp /workspace/.cas-cache/valgrind-packages.json "$run/package-manifest.json"
"$vg" --version > "$run/valgrind-version.txt"
uname -a > "$run/host.txt"
lscpu >> "$run/host.txt"
date -u +%FT%TZ > "$run/started.txt"
for seq in {0..9}; do
  cell="$run/seq-$seq"
  mkdir "$cell"
  set +e
  "$bin" profile-input --spec "$study/pipeline-runs/structured-n13-20261025-01/spec.json" --seq "$seq" \
    > "$cell/input.json" 2> "$cell/input-stderr.txt"
  rc=$?
  set -e
  jq -n --argjson rc "$rc" '{exit_code:$rc}' > "$cell/input-status.json"
  if test "$rc" != 0; then continue; fi
  sha256sum "$cell/input.json" > "$cell/input-sha256.txt"
  argv=("$vg" --tool=callgrind --cache-sim=no --branch-sim=no
    "--extra-debuginfo-path=$debug" "--callgrind-out-file=$cell/callgrind.out" "$bin" exec)
  jq -n --args '{argv:$ARGS.positional,timeout_seconds:360,isolation:"L0",threads:1,address_space_limit:null}' \
    -- "${argv[@]}" > "$cell/command.json"
  date -u +%FT%TZ > "$cell/started.txt"
  set +e
  "$timer" -v -o "$cell/rusage.txt" timeout --kill-after=15s 360s "${argv[@]}" \
    < "$cell/input.json" > "$cell/child.json" 2> "$cell/valgrind-stderr.txt"
  rc=$?
  set -e
  jq -n --argjson rc "$rc" '{exit_code:$rc}' > "$cell/status.json"
  set +e
  "$bin" callgrind-ir --prefix "$cell/callgrind.out" > "$cell/solve-ir.json" 2> "$cell/solve-ir-stderr.txt"
  solve_rc=$?
  "$bin" callgrind-online-ir --prefix "$cell/callgrind.out" > "$cell/online-ir.json" 2> "$cell/online-ir-stderr.txt"
  online_rc=$?
  set -e
  jq -n --argjson solve "$solve_rc" --argjson online "$online_rc" '{solve_exit_code:$solve,online_exit_code:$online}' > "$cell/parser-status.json"
  date -u +%FT%TZ > "$cell/ended.txt"
  printf 'profile seq %s exit %s solve-parser %s online-parser %s\n' "$seq" "$rc" "$solve_rc" "$online_rc"
done
date -u +%FT%TZ > "$run/ended.txt"
