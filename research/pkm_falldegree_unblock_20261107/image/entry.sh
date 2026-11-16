#!/bin/bash
# pkm_falldegree_unblock entry script (PROPOSED, development-checked only; derived from
# research/prime_ic_program_20261029/fall_degree_relaunch/proposed/entry.sh).
# Usage: entry.sh <stage>...
#   g1                       gate G1: hky_fp_tower_check must print "G1: 6 cases pass"
#   g3                       gate G3: prime_fall_degree_tools gate3 over this job's rows
#   cap:T:K:C:DENSE          ONE capped F4 run (Amendment 7 B), Kummer m=2, P1, t=T, draw K, cap C,
#                            --budget 86400, --max-dense DENSE, --trace; then pfd_grid_zeros on the row
#                            (Amendment 7 C). The next cap is the director's decision from the row.
#   m4:T:K:CMAX:ENTRIES:COLS ONE Amendment-8 closure: Kummer m=4, P1, t=T, draw K, --hky-cmax CMAX
#   anything else            passed to run.sh (g4 hky-A hky-B hky-C d-A d-B cap-A)
# Results go to $ISOLAB_RESULTS_URI (the pod's /work is temporary): runs/, gates/, progress.txt,
# binary.sha256, host.txt, synced every 300 s and at exit. Exit status is non-zero if any stage
# line in progress.txt ends in a non-zero exit, or a gate fails.
set -u
: "${ISOLAB_RESULTS_URI:?}"
S=/work/study
mkdir -p "$S/runs" "$S/gates" && cp /opt/study/run.sh "$S/run.sh" && cd "$S" || exit 2
export BIN=/usr/local/bin/pkm_tower_pilot
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-14}
P1=2013265921
SEED=1347112276   # 0x504B4D54, the pilot's default; recorded, not passed
log() { echo "$(date -u +%FT%TZ) $*" >> "$S/progress.txt"; }
up() {
  aws s3 sync "$S/runs" "$ISOLAB_RESULTS_URI/runs" --only-show-errors
  aws s3 sync "$S/gates" "$ISOLAB_RESULTS_URI/gates" --only-show-errors
  aws s3 cp "$S/progress.txt" "$ISOLAB_RESULTS_URI/progress.txt" --only-show-errors 2>/dev/null
}
sha256sum /usr/local/bin/pkm_tower_pilot /usr/local/bin/prime_fall_degree_tools \
  /usr/local/bin/hky_fp_tower_check /usr/local/bin/pfd_grid_zeros > "$S/binary.sha256"
{ echo "argv: $*"; echo "nproc: $(nproc)"; echo "rayon: $RAYON_NUM_THREADS"; echo "seed: $SEED"
  grep -m1 'model name' /proc/cpuinfo; grep MemTotal /proc/meminfo; uname -r
  cat /opt/study/build-info.json 2>/dev/null; } > "$S/host.txt"
aws s3 cp "$S/binary.sha256" "$ISOLAB_RESULTS_URI/binary.sha256" --only-show-errors
aws s3 cp "$S/host.txt" "$ISOLAB_RESULTS_URI/host.txt" --only-show-errors
( while sleep 300; do up; done ) &
syncer=$!
fail=0
for s in "$@"; do
  case "$s" in
    g1) log "start g1"; /usr/local/bin/hky_fp_tower_check > gates/g1.log 2>&1; rc=$?
        grep -q 'G1: 6 cases pass' gates/g1.log || { [ "$rc" = 0 ] && rc=1; }; log "end g1 exit $rc"
        [ "$rc" = 0 ] || { fail=1; break; } ;;
    g3) log "start g3"; /usr/local/bin/prime_fall_degree_tools gate3 "$S" "$BIN" --max-n 20 --jobs 2 \
          --budget 900 > gates/g3.log 2>&1; rc=$?
        grep -q ', 0 disagreements' gates/g3.log || { [ "$rc" = 0 ] && rc=1; }; log "end g3 exit $rc"
        [ "$rc" = 0 ] || { fail=1; break; } ;;
    cap:*) IFS=: read -r _ t k c dense <<< "$s"; name="X-m2-t$t-k$k-c$c"
        log "start $name"
        "$BIN" --p $P1 --ladder-t none --planted 0 --random 2 --engine tower --kinds kummer --m 2 --controls tower \
          --budget 86400 --max-dense "$dense" --trace --t-min "$t" --max-t "$t" --targets "$k" --cap "$c" \
          --out "runs/$name.jsonl" > "runs/$name.out" 2> "runs/$name.log"
        log "end $name exit $?"
        /usr/local/bin/pfd_grid_zeros "runs/$name.jsonl" > "runs/$name.gridzeros.jsonl" 2> "runs/$name.gridzeros.log"
        log "end $name.gridzeros exit $?" ;;
    m4:*) IFS=: read -r _ t k cmax ents cols <<< "$s"; name="M4-t$t-k$k-c$cmax"
        log "start $name"
        "$BIN" --p $P1 --ladder-t none --planted 0 --random 2 --engine hky --kinds kummer --m 4 --controls tower \
          --hky-cmax "$cmax" --hky-max-entries "$ents" --hky-max-cols "$cols" --budget 172800 \
          --t-min "$t" --max-t-m4 "$t" --max-t "$t" --targets "$k" --out "runs/$name.jsonl" > "runs/$name.out" 2> "runs/$name.log"
        log "end $name exit $?" ;;
    *) bash ./run.sh "$s" ;;
  esac
done
kill "$syncer" 2>/dev/null; up
[ "$fail" = 0 ] && ! grep -Eq ' exit [1-9][0-9]*$' "$S/progress.txt"
