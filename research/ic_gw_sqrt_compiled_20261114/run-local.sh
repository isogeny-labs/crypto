#!/usr/bin/env bash
# Runs units.tsv one at a time, each under the host lock. usage: run-local.sh BIN_DIR OUT_DIR LOCKFILE
# BIN_DIR holds v3 (reference) and v3c (compiled chain).
set -uo pipefail
B=$1; OUT=$2; LK=$3; here=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$OUT/units"
sha256sum "$B/v3" "$B/v3c" > "$OUT/binary.sha256"
{ grep -m1 'model name' /proc/cpuinfo; nproc; } > "$OUT/host.txt"
echo "$(date -u +%FT%TZ) start" >> "$OUT/run.log"
while read -r label arm bin q s pass; do
  f="$OUT/units/${label}_${q}_${s}.jsonl"
  [ -s "$f" ] && continue
  flock "$LK" sh -c '
    f=$1; B=$2; bin=$3; q=$4; s=$5; arm=$6; pass=$7; label=$8; OUT=$9
    load=$(cut -d" " -f1 /proc/loadavg)
    t0=$(date +%s%N)
    "$B/$bin" ic "$q" "$s" D 3.0 1.7 > "$f.tmp" 2> "${f%.jsonl}.err"; rc=$?
    printf "{\"rec\":\"unit\",\"label\":\"%s\",\"arm\":\"%s\",\"bin\":\"%s\",\"q\":%s,\"seed\":%s,\"pass\":%s,\"c\":1.7,\"rc\":%s,\"wall_ms\":%s,\"load1\":\"%s\"}\n" \
      "$label" "$arm" "$bin" "$q" "$s" "$pass" "$rc" "$(( ($(date +%s%N) - t0) / 1000000 ))" "$load" >> "$f.tmp"
    mv "$f.tmp" "$f"' sh "$f" "$B" "$bin" "$q" "$s" "$arm" "$pass" "$label" "$OUT"
done < "$here/units.tsv"
echo "$(date -u +%FT%TZ) done" >> "$OUT/run.log"
(cd "$OUT" && find . -type f ! -name SHA256SUMS -printf '%P\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS)
