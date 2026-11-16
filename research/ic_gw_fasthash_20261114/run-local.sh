#!/usr/bin/env bash
# Runs units.tsv one at a time, each under the host lock. usage: run-local.sh BIN_DIR OUT_DIR LOCK
# BIN_DIR holds ref (v3 de500bc1...), hash, combo.
set -uo pipefail
B=$1; OUT=$2; LOCK=$3; here=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$OUT/units"
sha256sum "$B/ref" "$B/hash" "$B/combo" > "$OUT/binary.sha256"
{ grep -m1 'model name' /proc/cpuinfo; nproc; } > "$OUT/host.txt"
echo "$(date -u +%FT%TZ) start" >> "$OUT/run.log"
while read -r pass arm bin q s; do
  f="$OUT/units/p${pass}_${arm}_${q}_${s}.jsonl"
  [ -s "$f" ] && continue
  flock "$LOCK" bash -c '
    l=$(cut -d" " -f1 /proc/loadavg); t0=$(date +%s%N)
    "$1/run-unit.sh" "$2/$3" "$4" "$5" > "$6.tmp" 2> "$6.err"; rc=$?
    printf "{\"rec\":\"unit\",\"pass\":%s,\"arm\":\"%s\",\"bin\":\"%s\",\"q\":%s,\"seed\":%s,\"rc\":%s,\"wall_ms\":%s,\"load1\":\"%s\"}\n" \
      "$7" "$8" "$3" "$4" "$5" "$rc" "$(( ($(date +%s%N) - t0) / 1000000 ))" "$l" >> "$6.tmp"
    mv "$6.tmp" "$6"' _ "$here" "$B" "$bin" "$q" "$s" "$f" "$pass" "$arm"
done < "$here/units.tsv"
echo "$(date -u +%FT%TZ) done" >> "$OUT/run.log"
(cd "$OUT" && find . -type f ! -name SHA256SUMS -printf '%P\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS)
