#!/bin/bash
# entry_m2c.sh (Amendment 31): one exact HKY closure cell of the m = 2 Kummer ladder at P1, one target per job.
# Usage: entry_m2c.sh m2c:T:K:CMAX:ENTRIES:COLS:BUDGET
# The pilot flags are those of run.sh's A-m2-tower-big line (stage A) with --hky-cmax CMAX, the closure limits ENTRIES and
# COLS, the per-closure wall budget BUDGET, and one target (--targets K), as entry_r2.sh's m4: stage does for m = 4.
# Then pfd_grid_zeros certifies inconsistency (Amendment 7 C). Rows: /work/study/runs/M2C-tT-kK-cCMAX.jsonl.
set -u
S=/work/study
mkdir -p "$S/runs" "$S/gates" && cd "$S" || exit 2
BIN=/usr/local/bin/pkm_tower_pilot
P1=2013265921
log() { echo "$(date -u +%FT%TZ) $*" >> "$S/progress.txt"; }
sha256sum /usr/local/bin/pkm_tower_pilot /usr/local/bin/pfd_grid_zeros "$0" > "$S/binary.sha256"
{ echo "argv: $*"; echo "nproc: $(nproc)"; echo "rayon: ${RAYON_NUM_THREADS:-unset}"
  grep -m1 'model name' /proc/cpuinfo; grep MemTotal /proc/meminfo; uname -r; } > "$S/host.txt"
fail=0
for s in "$@"; do
  case "$s" in
    m2c:*) IFS=: read -r _ t k cmax ents cols budget <<< "$s"; name="M2C-t$t-k$k-c$cmax"
      log "start $name"
      "$BIN" --p $P1 --ladder-t none --planted 0 --random 2 --engine hky --kinds kummer --m 2 --controls tower \
        --hky-cmax "$cmax" --hky-max-entries "$ents" --hky-max-cols "$cols" --budget "$budget" \
        --t-min "$t" --max-t "$t" --targets "$k" --out "runs/$name.jsonl" > "runs/$name.out" 2> "runs/$name.log"
      rc=$?; log "end $name exit $rc"; [ "$rc" = 0 ] || fail=1
      /usr/local/bin/pfd_grid_zeros "runs/$name.jsonl" > "runs/$name.gridzeros.jsonl" 2> "runs/$name.gridzeros.log"
      log "end $name.gridzeros exit $?" ;;
    *) log "unknown stage $s"; fail=1 ;;
  esac
done
[ "$fail" = 0 ]
