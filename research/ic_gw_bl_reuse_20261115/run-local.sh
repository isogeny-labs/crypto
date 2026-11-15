#!/usr/bin/env bash
# Runs units.tsv, up to 3 units concurrently (counted work only). usage: run-local.sh BIN_DIR OUT_DIR
# BIN_DIR holds ref and blr.
set -uo pipefail
B=$(cd "$1" && pwd); OUT=$2; here=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$OUT/units"; OUT=$(cd "$OUT" && pwd)
sha256sum "$B/ref" "$B/blr" > "$OUT/binary.sha256"
{ grep -m1 'model name' /proc/cpuinfo; nproc; } > "$OUT/host.txt"
echo "$(date -u +%FT%TZ) start" >> "$OUT/run.log"
one() {
  arm=$1; bin=$2; q=$3; s=$4; B=$5; OUT=$6
  f="$OUT/units/${arm}_${q}_${s}.jsonl"
  [ -s "$f" ] && return 0
  l=$(cut -d" " -f1 /proc/loadavg); t0=$(date +%s%N)
  "$B/$bin" ic "$q" "$s" D 3.0 1.7 > "$f.tmp" 2> "$f.err"; rc=$?
  printf '{"rec":"unit","arm":"%s","bin":"%s","q":%s,"seed":%s,"rc":%s,"wall_ms":%s,"load1":"%s"}\n' \
    "$arm" "$bin" "$q" "$s" "$rc" "$(( ($(date +%s%N) - t0) / 1000000 ))" "$l" >> "$f.tmp"
  mv "$f.tmp" "$f"
}
export -f one
xargs -P 3 -L 1 bash -c 'one "$@"' _ < <(awk -v B="$B" -v O="$OUT" '{print $1,$2,$3,$4,B,O}' "$here/units.tsv")
echo "$(date -u +%FT%TZ) done" >> "$OUT/run.log"
(cd "$OUT" && find . -type f ! -name SHA256SUMS -printf '%P\n' | LC_ALL=C sort | xargs -r sha256sum > SHA256SUMS)
