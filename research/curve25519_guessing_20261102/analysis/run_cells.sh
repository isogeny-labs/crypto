#!/usr/bin/env bash
# Frozen grid of PROTOCOL.md. Sequential; one JSON line per ok cell in
# f4cost_guess.jsonl, one status line per cell in f4cost_guess.jsonl.status.
# Usage: run_cells.sh [binary]
set -u
BIN=${1:-/home/user/frozen-bin/curve25519_family_lead4}
D=$(cd "$(dirname "$0")" && pwd)
OUT=$D/f4cost_guess.jsonl; ST=$OUT.status; ERR=$D/stderr
mkdir -p "$ERR"; : > "$OUT"; : > "$ST"
cells() {
  for t in 6 7 8; do for G in 3 4 5 6 7; do [ "$G" -lt "$t" ] && echo "bits $t $G"; done; done
  for t in 6 7 8; do for G in 0 1 2 3 4; do echo "isogeny $t $G"; done; done
}
cells | while read -r form t G; do
  seed=$((2026111400 + t)); tmp=$(mktemp)
  s=$(date +%s)
  ( ulimit -v 3500000; timeout 900 "$BIN" f4cost 20 "$form" "$t" "$G" 100 "$seed" 600 ) >"$tmp" 2>"$ERR/$form-t$t-g$G.err"
  rc=$?; e=$(( $(date +%s) - s ))
  if [ $rc -eq 0 ] && jq -e . "$tmp" >/dev/null 2>&1; then
    jq -c . "$tmp" >>"$OUT"; echo "ok form=$form t=$t G=$G rc=0 elapsed_s=$e" >>"$ST"
  else
    echo "failed form=$form t=$t G=$G rc=$rc elapsed_s=$e" >>"$ST"
  fi
  rm -f "$tmp"
done
