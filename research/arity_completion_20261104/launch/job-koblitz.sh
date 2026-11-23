#!/usr/bin/env bash
# Koblitz m = 5, 6 study (protocol koblitz_m5_m6, frozen 4fa25efc0e): the m = 2
# shard (negative controls) first, then the m2 gate of CARD-arity-koblitz-m5/m6
# (no process_error record), then shards m = 5 and m = 6 concurrently, one
# single-threaded process per cell as run_shard.sh does. Records go to
# raw/shard<m>/, the directories the frozen analysis plan reads.
# Usage: job-koblitz.sh [RESUME_S3_PREFIX]
source /opt/study/launch/job-lib.sh
preamble
resume_from "${1:-}"
BIN=/usr/local/bin/n83_msweep_cell
RS=/opt/study/koblitz_m5_m6/run_shard.sh
sha256sum "$RS" /opt/study/koblitz_m5_m6/grid.tsv > "$OUT/frozen-files.sha256"
export RAYON_NUM_THREADS=1
log "shard m2 start"
bash "$RS" "$BIN" "$OUT/raw/shard2" 2 > "$OUT/shard2.stdout" 2> "$OUT/shard2.stderr"
rc2=$?
log "shard m2 exit $rc2"
if [ "$rc2" -ne 0 ] || grep -lq '"status":"process_error"' "$OUT"/raw/shard2/*.json 2>/dev/null; then
  printf '{"gate":"m2","passed":false,"run_shard_exit":%s}\n' "$rc2" > "$OUT/gate-m2.json"
  log "m2 gate failed; shards m5 and m6 not run"
  exit 30
fi
printf '{"gate":"m2","passed":true,"run_shard_exit":0}\n' > "$OUT/gate-m2.json"
log "shards m5 and m6 start (concurrent, 1 thread each)"
bash "$RS" "$BIN" "$OUT/raw/shard5" 5 > "$OUT/shard5.stdout" 2> "$OUT/shard5.stderr" &
p5=$!
bash "$RS" "$BIN" "$OUT/raw/shard6" 6 > "$OUT/shard6.stdout" 2> "$OUT/shard6.stderr" &
p6=$!
wait "$p5"; rc5=$?
wait "$p6"; rc6=$?
log "shard m5 exit $rc5; shard m6 exit $rc6"
[ "$rc5" -eq 0 ] && [ "$rc6" -eq 0 ]
