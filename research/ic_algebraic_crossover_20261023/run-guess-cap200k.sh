#!/usr/bin/env bash
# Sequential native continuation, preserving every failed/capped branch.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
study=research/ic_algebraic_crossover_20261023
run=${1:?fresh run directory}
bin=${IC_FROZEN_BIN:?absolute path of frozen instrument}
timer=${IC_TIMER:-/workspace/.tools/usr/bin/time}
test -x "$bin"
test -x "$timer"
test ! -e "$run"
mkdir -p "$run"
cp "$0" "$run/runner.sh"
git rev-parse HEAD > "$run/working-head.txt"
printf '%s\n' 'd67f6085 (instrument build; later source parity lint fix excluded)' > "$run/binary-source.txt"
sha256sum "$bin" "$timer" "$0" "$study/GUESS_CAP200K_PROTOCOL.md" \
  "$study/inputs/guess-ell8.jsonl" > "$run/hashes.txt"
uname -a > "$run/host.txt"
lscpu >> "$run/host.txt"
cat /sys/fs/cgroup/memory.max > "$run/cgroup-memory-max.txt"
date -u +%FT%TZ > "$run/started.txt"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=1000000 RAYON_NUM_THREADS=4
jq -n '{F4_F2_MAX_ROWS:50000000,F4_F2_MAX_COLS:1000000,RAYON_NUM_THREADS:4}' > "$run/environment.json"
seqno=0
branch() {
  local draw=$1 k=$2 assignment=$3 vars cell rc
  case "$k" in
    0) vars=none ;;
    1) vars=7 ;;
    2) vars=7,15 ;;
    3) vars=7,15,23 ;;
    4) vars=7,15,23,24 ;;
    *) return 2 ;;
  esac
  cell="$run/cell-$(printf '%04d' "$seqno")-d${draw}-k${k}-b${assignment}"
  seqno=$((seqno+1))
  mkdir "$cell"
  local argv=("$bin" guess --input "$study/inputs/guess-ell8.jsonl" --draw "$draw"
    --vars "$vars" --degree 4 --max-cols 200000 --first-branch "$assignment"
    --end-branch "$((assignment+1))")
  jq -n --args '{argv:$ARGS.positional,timeout_seconds:900,address_space_bytes:12884901888,isolation:"L0"}' \
    -- "${argv[@]}" > "$cell/command.json"
  date -u +%FT%TZ > "$cell/started.txt"
  set +e
  "$timer" -v -o "$cell/rusage.txt" timeout --kill-after=15s 900s \
    prlimit --as=12884901888 -- "${argv[@]}" > "$cell/rows.jsonl" 2> "$cell/stderr.txt"
  rc=$?
  set -e
  jq -n --argjson rc "$rc" --arg ended "$(date -u +%FT%TZ)" \
    '{exit_code:$rc,ended:$ended,status:(if $rc==0 then "finished" elif $rc==124 then "timeout" elif $rc==137 then "killed_cause_unconfirmed" else "failed" end)}' > "$cell/status.json"
  printf '%s %s\n' "$cell" "$rc"
}
branch 6 0 0
for k in 0 1 2 3 4; do
  for draw in 0 3 5 6; do
    if test "$k" = 3 && test "$draw" = 0; then continue; fi
    if test "$k" = 0 && test "$draw" = 6; then continue; fi
    for ((assignment=0; assignment<(1<<k); assignment++)); do
      branch "$draw" "$k" "$assignment"
    done
  done
done
test "$seqno" = 116
date -u +%FT%TZ > "$run/ended.txt"
