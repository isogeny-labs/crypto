#!/usr/bin/env bash
# Frozen grid of PROTOCOL.md. Sequential, trace=0 then trace=1 for each (t, G).
# One JSON line per ok cell in f4trace.jsonl (with the uptime before the cell),
# one status line per cell in f4trace.jsonl.status. Usage: run_cells.sh [binary]
set -u
BIN=${1:-/home/user/frozen-bin/curve25519_family_leadC}
D=$(cd "$(dirname "$0")" && pwd)
OUT=$D/f4trace.jsonl; ST=$OUT.status; ERR=$D/stderr
mkdir -p "$ERR"; : > "$OUT"; : > "$ST"
for t in 6 7 8; do for G in 0 2 4; do for tr in 0 1; do
  seed=$((2026111400 + t)); tmp=$(mktemp)
  up=$(uptime | sed 's/.*load average: //'); s=$(date +%s)
  ( ulimit -v 3500000; timeout 900 "$BIN" f4cost 20 isogeny "$t" "$G" 100 "$seed" 600 2 full 0 "$tr" ) >"$tmp" 2>"$ERR/iso-t$t-g$G-tr$tr.err"
  rc=$?; e=$(( $(date +%s) - s ))
  if [ $rc -eq 0 ] && jq -e . "$tmp" >/dev/null 2>&1; then
    jq -c --arg load "$up" --argjson e "$e" '. + {load_avg_before: $load, elapsed_s: $e}' "$tmp" >>"$OUT"
    echo "ok t=$t G=$G trace=$tr rc=0 elapsed_s=$e load=$up" >>"$ST"
  else
    echo "failed t=$t G=$G trace=$tr rc=$rc elapsed_s=$e load=$up" >>"$ST"
  fi
  rm -f "$tmp"
done; done; done
