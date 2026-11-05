#!/usr/bin/env bash
# Amendment A2 (PROTOCOL.md §11): weighted-order probes on the m = 5 quotient system of the
# 4111 cell, interleaved with the baseline chain.
#   run_weighted.sh drive PID   wait for the baseline 4111 solve (qsolve run PID) to exit, run its
#                               frozen check, run the weighted arms, then the remaining baseline
#                               cells (run_cells.sh m5 262231, m5 33554371)
#   run_weighted.sh arm NAME    one weighted arm (W1, W2, W3)
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
R="$HERE/results"
BASE="$R/m5_p4111_planted"
W="$R/m5_p4111_weighted"
BINW="$HERE/bin/qsolve-w"      # same source as bin/qsolve plus the `weight` subcommand
NPROC=$(nproc)
RSS_CAP_GIB=12

declare -A WTS=( [W1]="2,2,2,2,2,1" [W2]="1,2,3,4,5,5" [W3]="1,1,1,1,1,2" )
declare -A ORD=( [W1]="s1,s2,s3,s4,s5,w" [W2]="s5,w,s4,s3,s2,s1" [W3]="w,s1,s2,s3,s4,s5" )

f4_wall() { # F4 elapsed seconds of a finished msolve -v 2 log (first TIMINGS block), else empty
  grep -m1 -E '^overall\(elapsed\)' "$1" 2>/dev/null | grep -o -E '[0-9]+\.[0-9]+' | head -1
}

arm() {
  local a=$1 cap=$2
  local d="$W/$a"
  [ -e "$d" ] && { echo "refusing to overwrite $d"; return 1; }
  mkdir -p "$d"
  "$BINW" weight --in "$BASE/quot.ms" --weights "${WTS[$a]}" --order "${ORD[$a]}" --out "$d/quot_w.ms" 2> "$d/weight.err"
  sha256sum "$BASE/quot.ms" "$d/quot_w.ms" > "$d/inputs.sha256"
  OMP_WAIT_POLICY=PASSIVE "$BINW" run --rss-cap-gib "$RSS_CAP_GIB" --wall-cap-s "$cap" --record "$d/run.json" -- \
    msolve -g 1 -v 2 -t "$NPROC" -f "$d/quot_w.ms" -o "$d/gb.out" > "$d/quot_w.log" 2>&1 || true
  echo "== $a W=${WTS[$a]} order=${ORD[$a]} cap=${cap}s"
  { grep -o 'Dimension of quotient: [0-9]*' "$d/quot_w.log" | tail -1; } || true
  { grep -o '"signal": [0-9a-z]*\|"killed_by_[a-z_]*": [a-z]*\|"wall_seconds": [0-9.]*\|"user_seconds": [0-9.]*\|"maxrss_kib": [0-9]*' "$d/run.json" | paste -sd' '; } || true
}

drive() {
  local pid=$1
  while kill -0 "$pid" 2>/dev/null; do sleep 30; done
  # the baseline cell's frozen check (identical to run_cells.sh solve(); the chain driver was
  # detached by amendment A2 before it could run it)
  if [ -s "$BASE/quot.out" ]; then
    "$HERE/bin/qsolve" check --msolve-out "$BASE/quot.out" --planted "$BASE/quot.planted" --msolve-log "$BASE/quot.log" > "$BASE/quot.check.json" 2>&1 || true
  else
    echo '{"check": "no msolve output (solve did not finish)"}' > "$BASE/quot.check.json"
  fi
  echo "== baseline 4111"; cat "$BASE/quot.check.json" || true
  local fw cap
  fw=$(f4_wall "$BASE/quot.log" || true)
  if [ -n "$fw" ]; then cap=$(python3 -c "print(int(1.25*$fw)+60)"); else cap=14400; fi
  echo "baseline F4 elapsed: ${fw:-not finished}; weighted wall cap ${cap}s"
  mkdir -p "$W"
  for a in W1 W3 W2; do arm "$a" "$cap" || true; done
  "$HERE/run_cells.sh" m5 262231 || true
  "$HERE/run_cells.sh" m5 33554371 || true
  echo "drive finished $(date -u +%FT%TZ)"
}

case "${1:-}" in
  drive) drive "${2:?pid}" ;;
  arm) arm "${2:?name}" "${3:-14400}" ;;
  *) echo "usage: $0 drive PID | arm NAME [CAP_S]"; exit 2 ;;
esac
