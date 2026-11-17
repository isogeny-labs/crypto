#!/usr/bin/env bash
# Runs units.tsv one at a time with the published v2 binary. usage: run-local.sh V2_BINARY OUT_DIR
set -uo pipefail
V2=$1; OUT=$2; here=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$OUT/units"
sha256sum "$V2" > "$OUT/binary.sha256"
{ grep -m1 'model name' /proc/cpuinfo; nproc; } > "$OUT/host.txt"
echo "$(date -u +%FT%TZ) start" >> "$OUT/run.log"
while read -r arm c q s; do
  f="$OUT/units/${arm}_${q}_${s}.jsonl"
  [ -s "$f" ] && continue
  t0=$(date +%s%N)
  "$V2" ic "$q" "$s" D 3.0 "$c" > "$f.tmp" 2> "$OUT/units/${arm}_${q}_${s}.err"
  rc=$?
  printf '{"rec":"unit","arm":"%s","q":%s,"seed":%s,"c":%s,"rc":%s,"wall_ms":%s,"load1":"%s"}\n' \
    "$arm" "$q" "$s" "$c" "$rc" "$(( ($(date +%s%N) - t0) / 1000000 ))" "$(cut -d' ' -f1 /proc/loadavg)" >> "$f.tmp"
  mv "$f.tmp" "$f"
done < "$here/units.tsv"
echo "$(date -u +%FT%TZ) done" >> "$OUT/run.log"
(cd "$OUT" && find . -type f ! -name SHA256SUMS -printf '%P\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS)
