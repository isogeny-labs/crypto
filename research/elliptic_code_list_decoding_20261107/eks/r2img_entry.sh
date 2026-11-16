#!/bin/bash
# r2img entry for elliptic_code_list_decoding_20261107 (round 2, 2026-11-08). Runs the 64 frozen argv lines of
# ecld_jobs.txt SEQUENTIALLY (PROTOCOL.md section 5: one thread, no co-running panel jobs on the pod), one output file
# per line, uploads after every line and at exit. A failed line is recorded and the loop continues (the line is re-run
# whole later). Exit status is non-zero if the source hash check fails or any line exits non-zero.
set -u
: "${ISOLAB_RESULTS_URI:?}" "${ISOLAB_OUTPUT_DIR:?}"
D=${R2IMG_DIR:-/opt/ecld}; OUT=$ISOLAB_OUTPUT_DIR; mkdir -p "$OUT/lines"; cd "$OUT" || exit 2
FROZEN_SRC=de901e230d2558cb488dee653e8bdfe44ff44b928a611322a014670c5fa8769d
up() { aws s3 sync "$OUT" "$ISOLAB_RESULTS_URI" --only-show-errors; }
{ echo "start: $(date -u +%FT%TZ)"; echo "nproc: $(nproc)"; grep -m1 'model name' /proc/cpuinfo; grep MemTotal /proc/meminfo; uname -r
  cat "$D/build-info.json"; } > host.txt
got=$(sha256sum "$D/ecld_toy.rs" | cut -d' ' -f1)
echo "source sha256 in image: $got (frozen $FROZEN_SRC)" >> host.txt
sha256sum "$D/ecld_toy" "$D/ecld_toy.rs" "$D/ecld_jobs.txt" > binary.sha256
if [ "$got" != "$FROZEN_SRC" ]; then echo "SOURCE HASH MISMATCH: freeze voided" >&2; up; exit 3; fi
fail=0; n=0
while IFS= read -r line; do
  [ -z "$line" ] && continue
  n=$((n+1)); id=$(printf '%02d' "$n")
  set -- $line; [ "$1" = ecld_toy ] || { echo "unexpected argv0 $1" >&2; exit 4; }; shift
  echo "line $id: ecld_toy $*" >> progress.txt
  s=$(date +%s)
  "$D/ecld_toy" "$@" > "lines/line_$id.jsonl" 2> "lines/line_$id.err" < /dev/null; rc=$?
  echo "line $id exit $rc wall $(( $(date +%s) - s )) s" >> progress.txt
  [ "$rc" = 0 ] || fail=1
  up
done < "$D/ecld_jobs.txt"
cat lines/line_*.jsonl > panel.jsonl
echo "end: $(date -u +%FT%TZ) lines $n fail $fail" >> progress.txt
find . -type f ! -name SHA256SUMS | sort | sed 's|^\./||' | xargs sha256sum > SHA256SUMS
up
[ "$n" = 64 ] && [ "$fail" = 0 ]
