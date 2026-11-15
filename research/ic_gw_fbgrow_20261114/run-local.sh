#!/usr/bin/env bash
# Runs units.tsv one unit at a time, each under the host lock.
# usage: run-local.sh BIN_DIR OUT_DIR   (BIN_DIR holds v3 = REF and v3g = growth; LOCK env = host lock file)
set -uo pipefail
B=$1; OUT=$2; here=$(cd "$(dirname "$0")" && pwd)
LOCK=${LOCK:-/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/wf/timing.lock}
mkdir -p "$OUT/units"
sha256sum "$B/v3" "$B/v3g" > "$OUT/binary.sha256"
{ grep -m1 'model name' /proc/cpuinfo; nproc; } > "$OUT/host.txt"
echo "$(date -u +%FT%TZ) start" >> "$OUT/run.log"
while read -r arm bin q s rest; do
  f="$OUT/units/${arm}_${q}_${s}.jsonl"
  [ -s "$f" ] && continue
  flock "$LOCK" sh -c '
    load1=$(cut -d" " -f1 /proc/loadavg)
    t0=$(date +%s%N)
    "$1/$2" ic "$3" "$4" $5 > "$6.tmp" 2> "$7"
    rc=$?
    printf "{\"rec\":\"unit\",\"arm\":\"%s\",\"bin\":\"%s\",\"q\":%s,\"seed\":%s,\"args\":\"%s\",\"rc\":%s,\"wall_ms\":%s,\"load1\":\"%s\"}\n" \
      "$8" "$2" "$3" "$4" "$5" "$rc" "$(( ($(date +%s%N) - t0) / 1000000 ))" "$load1" >> "$6.tmp"
    mv "$6.tmp" "$6"
  ' _ "$B" "$bin" "$q" "$s" "$rest" "$f" "$OUT/units/${arm}_${q}_${s}.err" "$arm"
done < "$here/units.tsv"
echo "$(date -u +%FT%TZ) done" >> "$OUT/run.log"
(cd "$OUT" && find . -type f ! -name SHA256SUMS -printf '%P\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS)
