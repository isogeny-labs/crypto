#!/usr/bin/env bash
# Validator replay runner. usage: replay.sh BIN_DIR PLAN OUT_DIR ; up to 3 units concurrently; counted work only.
set -uo pipefail
B=$(cd "$1" && pwd); PLAN=$(realpath "$2"); OUT=$3
mkdir -p "$OUT/units"; OUT=$(cd "$OUT" && pwd)
sha256sum "$B"/ref "$B"/cand "$B"/oracle > "$OUT/binary.sha256"
{ grep -m1 'model name' /proc/cpuinfo; nproc; rustc --version; } > "$OUT/host.txt"
echo "$(date -u +%FT%T.%NZ) start" >> "$OUT/run.log"
one() {
  idx=$1; role=$2; arm=$3; bin=$4; ev=$5; q=$6; s=$7; c=$8; B=$9; OUT=${10}
  f="$OUT/units/${idx}_${role}_${arm}_${q}_${s}_c${c}.jsonl"
  [ -s "$f" ] && return 0
  [ "$ev" = "-" ] && ev=""
  l=$(cat /proc/loadavg); t0=$(date -u +%FT%TZ)
  env -u NRM -u LZ_REUSE $(echo "$ev" | tr ',' ' ') "$B/$bin" ic "$q" "$s" D 3.0 "$c" > "$f.tmp" 2> "$f.err"; rc=$?
  printf '{"rec":"vunit","idx":%s,"role":"%s","arm":"%s","bin":"%s","env":"%s","q":%s,"seed":%s,"c":"%s","rc":%s,"start_utc":"%s","loadavg_before":"%s","cmd":"%s ic %s %s D 3.0 %s"}\n' \
    "$idx" "$role" "$arm" "$bin" "$ev" "$q" "$s" "$c" "$rc" "$t0" "$l" "$bin" "$q" "$s" "$c" >> "$f.tmp"
  mv "$f.tmp" "$f"
}
export -f one
xargs -P 3 -L 1 bash -c 'one "$@"' _ < <(awk -v B="$B" -v O="$OUT" -F'\t' '{print $1,$2,$3,$4,$5,$6,$7,$8,B,O}' "$PLAN")
echo "$(date -u +%FT%T.%NZ) done" >> "$OUT/run.log"
(cd "$OUT" && find . -type f ! -name SHA256SUMS -printf '%P\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS)
