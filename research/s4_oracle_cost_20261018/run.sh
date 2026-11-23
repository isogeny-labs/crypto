#!/usr/bin/env bash
# Registered queue (PREREGISTRATION.md). Thin orchestration only.
set -u
BIN=${1:-target/release/examples/s4_oracle_cost}
HERE=$(cd "$(dirname "$0")" && pwd); REPO=$(cd "$HERE/../.." && pwd); OUT=$HERE/runs
mkdir -p "$OUT"
export RAYON_NUM_THREADS=1 F4_F2_MAX_MATRIX_WORDS=$((1<<26))
declare -A dead=([chained]=0 [eliminated]=0)
one() { # cell draw arm rep
  local cell=$1 d=$2 arm=$3 rep=$4 f="$OUT/cell-${1/:/-}.jsonl" line rc
  line=$(cd "$REPO" && prlimit --as=3000000000 timeout 900 "$BIN" --cell "$cell" --draw "$d" --arm "$arm" --rep "$rep" --seed 20261018 --budget-secs 600 2>>"$OUT/stderr.log")
  rc=$?
  echo "$(date -u +%FT%TZ) exit $rc cell=$cell draw=$d arm=$arm rep=$rep" >> "$OUT/queue.log"
  if [ $rc -ne 0 ] || [ -z "$line" ]; then
    line="{\"arm\":\"$arm\",\"cell\":\"$cell\",\"draw\":$d,\"rep\":$rep,\"finished\":false,\"verdict\":\"process_failed\",\"rc\":$rc}"
  fi
  echo "$line" >> "$f"
}
for cell in 12:3 13:4 16:5 18:6 19:6 21:7 24:8; do
  f="$OUT/cell-${cell/:/-}.jsonl"
  declare -A first2=([chained]=0 [eliminated]=0) ran=([chained]=0 [eliminated]=0)
  for d in 0 1 2 3; do
    if [ $((d%2)) -eq 0 ]; then order="chained eliminated"; else order="eliminated chained"; fi
    for arm in $order; do
      if [ "${dead[$arm]}" = 1 ]; then continue; fi
      if [ "${ran[$arm]}" -ge 2 ] && [ "${first2[$arm]}" -ge 2 ]; then continue; fi
      one $cell $d $arm 1
      ran[$arm]=$((ran[$arm]+1))
      if tail -1 "$f" | grep -q '"finished":true'; then :; else
        if [ "${ran[$arm]}" -le 2 ]; then first2[$arm]=$((first2[$arm]+1)); fi
      fi
    done
  done
  if [ "$cell" = 13:4 ]; then for arm in chained eliminated; do one $cell 0 $arm 2; done; fi
  for arm in chained eliminated; do
    if [ "${first2[$arm]}" -ge 2 ]; then dead[$arm]=1; echo "$(date -u +%FT%TZ) skip rule: $arm unmeasured on first 2 draws at $cell" >> "$OUT/queue.log"; fi
  done
  if [ "${dead[eliminated]}" = 1 ]; then echo "$(date -u +%FT%TZ) stop: eliminated unmeasured at $cell" >> "$OUT/queue.log"; break; fi
done
echo "$(date -u +%FT%TZ) queue done" >> "$OUT/queue.log"
