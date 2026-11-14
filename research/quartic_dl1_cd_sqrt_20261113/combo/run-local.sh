#!/usr/bin/env bash
# Runs units.tsv one at a time. usage: run-local.sh BIN_DIR OUT_DIR (BIN_DIR holds v2 and v3)
set -uo pipefail
B=$1; OUT=$2; here=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$OUT/units"
sha256sum "$B/v2" "$B/v3" > "$OUT/binary.sha256"
{ grep -m1 'model name' /proc/cpuinfo; nproc; } > "$OUT/host.txt"
echo "$(date -u +%FT%TZ) start" >> "$OUT/run.log"
while read -r arm bin c q s; do
  f="$OUT/units/${arm}_${q}_${s}.jsonl"
  [ -s "$f" ] && continue
  t0=$(date +%s%N)
  "$B/$bin" ic "$q" "$s" D 3.0 "$c" > "$f.tmp" 2> "$OUT/units/${arm}_${q}_${s}.err"
  rc=$?
  printf '{"rec":"unit","arm":"%s","bin":"%s","q":%s,"seed":%s,"c":%s,"rc":%s,"wall_ms":%s,"load1":"%s"}\n' \
    "$arm" "$bin" "$q" "$s" "$c" "$rc" "$(( ($(date +%s%N) - t0) / 1000000 ))" "$(cut -d' ' -f1 /proc/loadavg)" >> "$f.tmp"
  mv "$f.tmp" "$f"
done < "$here/units.tsv"
echo "$(date -u +%FT%TZ) done" >> "$OUT/run.log"
(cd "$OUT" && find . -type f ! -name SHA256SUMS -printf '%P\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS)
