#!/usr/bin/env bash
# usage: replay.sh BIN_DIR PLAN_TSV OUT_DIR  (BIN_DIR: ref blr audit mut_c2 mut_pm). Up to 3 units concurrently.
# Command: <bin> ic q seed D 3.0 1.7 (block 4, the producer's form) or <bin> ic q seed D 3.0 1.7 4.0 <block>.
set -uo pipefail
B=$(cd "$1" && pwd); PLAN=$2; OUT=$3; mkdir -p "$OUT/units"; OUT=$(cd "$OUT" && pwd)
sha256sum "$B"/ref "$B"/blr "$B"/audit "$B"/mut_c2 "$B"/mut_pm > "$OUT/binary.sha256"
one() {
  idx=$1; role=$2; arm=$3; q=$4; s=$5; blk=$6; B=$7; OUT=$8
  case $arm in REF) bin=ref;; BLR) bin=blr;; AUD) bin=audit;; MUTC2) bin=mut_c2;; MUTPM) bin=mut_pm;; esac
  f="$OUT/units/${idx}_${role}_${arm}_${q}_${s}_b${blk}.jsonl"
  [ -s "$f" ] && return 0
  if [ "$blk" = 4 ]; then args="ic $q $s D 3.0 1.7"; else args="ic $q $s D 3.0 1.7 4.0 $blk"; fi
  l=$(cat /proc/loadavg); st=$(date -u +%FT%TZ); t0=$(date +%s%N)
  "$B/$bin" $args > "$f.tmp" 2> "$f.err"; rc=$?
  printf '{"rec":"vunit","idx":%s,"role":"%s","arm":"%s","bin":"%s","cmd":"%s","q":%s,"seed":%s,"block":%s,"rc":%s,"start_utc":"%s","wall_ms":%s,"loadavg_before":"%s"}\n' \
    "$idx" "$role" "$arm" "$bin" "<$arm> $args" "$q" "$s" "$blk" "$rc" "$st" "$(( ($(date +%s%N) - t0) / 1000000 ))" "$l" >> "$f.tmp"
  mv "$f.tmp" "$f"
}
export -f one
echo "$(date -u +%FT%TZ) start" >> "$OUT/run.log"
awk -v B="$B" -v O="$OUT" '{print $1,$2,$3,$4,$5,$6,B,O}' "$PLAN" | xargs -P 3 -L 1 bash -c 'one "$@"' _
echo "$(date -u +%FT%TZ) done" >> "$OUT/run.log"
