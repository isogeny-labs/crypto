#!/usr/bin/env bash
# Amendments A2 + A3 (PROTOCOL.md §11, §12): solver probes on the m = 5 quotient system of the
# 4111 cell, then the remaining §7 baseline cells. Supersedes run_weighted.sh's `drive`, which
# was stopped while it was still waiting (it had run nothing).
#   run_probes.sh drive PID   wait for the baseline 4111 solve (qsolve run PID) to exit, run its
#                             frozen check, the signature-based arms F5a, F5b, the weighted
#                             arms W1, W3, W2, then run_cells.sh m5 262231 and m5 33554371
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
R="$HERE/results"
BASE="$R/m5_p4111_planted"
BINW="$HERE/bin/qsolve-w"
RSS_CAP_GIB=12

f4_field() { # first value of a TIMINGS line of msolve's F4 block: overall(elapsed) or overall(cpu)
  grep -m1 -E "^overall\($2\)" "$1" 2>/dev/null | grep -o -E '[0-9]+\.[0-9]+' | head -1
}

sba_arm() { # name sbaOrder rewrite cap_s
  local a=$1 o=$2 rw=$3 cap=$4
  local d="$R/m5_p4111_sba/$a"
  [ -e "$d" ] && { echo "refusing to overwrite $d"; return 1; }
  mkdir -p "$d"
  "$HERE/ms2sing.sh" "$BASE/quot.ms" sba "$o" "$rw" > "$d/quot.sing"
  sha256sum "$BASE/quot.ms" "$d/quot.sing" > "$d/inputs.sha256"
  # Singular's protocol output (one character per reduction) is capped at 4 MB on disk
  "$BINW" run --rss-cap-gib "$RSS_CAP_GIB" --wall-cap-s "$cap" --record "$d/run.json" -- \
    Singular -q "$d/quot.sing" 2>&1 | tail -c 4000000 > "$d/sing.log" || true
  echo "== $a sba(i, $o, $rw) cap=${cap}s"
  { grep -o 'RESULT.*' "$d/sing.log"; grep -o 'syz criterion:[0-9]* rew criterion:[0-9]*' "$d/sing.log" | tail -1; } || true
  { grep -o '"signal": [0-9a-z]*\|"killed_by_[a-z_]*": [a-z]*\|"wall_seconds": [0-9.]*\|"user_seconds": [0-9.]*\|"maxrss_kib": [0-9]*' "$d/run.json" | paste -sd' '; } || true
}

drive() {
  local pid=$1
  while kill -0 "$pid" 2>/dev/null; do sleep 30; done
  if [ -s "$BASE/quot.out" ]; then
    "$HERE/bin/qsolve" check --msolve-out "$BASE/quot.out" --planted "$BASE/quot.planted" --msolve-log "$BASE/quot.log" > "$BASE/quot.check.json" 2>&1 || true
  else
    echo '{"check": "no msolve output (solve did not finish)"}' > "$BASE/quot.check.json"
  fi
  echo "== baseline 4111"; cat "$BASE/quot.check.json" || true
  local fw fc capw capc
  fw=$(f4_field "$BASE/quot.log" elapsed || true)
  fc=$(f4_field "$BASE/quot.log" cpu || true)
  if [ -n "$fw" ]; then capw=$(python3 -c "print(int(1.25*$fw)+60)"); else capw=14400; fi
  if [ -n "$fc" ]; then capc=$(python3 -c "print(int(1.25*$fc)+60)"); else capc=14400; fi
  echo "baseline F4 elapsed ${fw:-not finished} s, cpu ${fc:-not finished} s; caps: weighted wall ${capw}s, sba ${capc}s"
  mkdir -p "$R/m5_p4111_sba"
  sba_arm F5a 0 1 "$capc" || true
  sba_arm F5b 1 0 "$capc" || true
  for a in W1 W3 W2; do "$HERE/run_weighted.sh" arm "$a" "$capw" || true; done
  "$HERE/run_cells.sh" m5 262231 || true
  "$HERE/run_cells.sh" m5 33554371 || true
  echo "drive finished $(date -u +%FT%TZ)"
}

case "${1:-}" in
  drive) drive "${2:?pid}" ;;
  sba) sba_arm "${2:?name}" "${3:?order}" "${4:?rewrite}" "${5:-14400}" ;;
  *) echo "usage: $0 drive PID | sba NAME ORDER REWRITE [CAP_S]"; exit 2 ;;
esac
