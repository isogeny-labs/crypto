#!/bin/bash
# Amendment R4 (draft): the 2-isogeny arm at P1 = 2013265921. Every job's argv is fixed here (edit E4.1).
# Usage: entry_iso.sh <stage>...
#   g1                gate G1 (frozen hky_fp_tower_check)
#   gaux              G-AUX check 3 on this host: 16 P0 isogeny draws, compared field by field with stage A (must pass)
#   A                 A-m2-iso-p1      closure (hky), cmax 7, t = 1..10
#   A-big             A-m2-iso-p1-big  closure (hky), cmax 5, t = 11..13
#   DA                DA-m2-iso-p1     F4 (tower), t = 1..13
#   g3                gate G3 over the A-m2-iso-p1* rows, through the wrapper (Amendment 11 scope)
#   cert              decomposability certificate of every m = 2 isogeny row in runs/ (certify_m2.py)
#   cap-A             capped F4 at D - 1 for t = 11..13, both draws, D from DA-m2-iso-p1 (run.sh cap-A rule)
#   A-14, DA-14, cap-14   the t = 14 cell of Amendment 19 (closure cmax 5, F4, capped F4 at D - 1)
#   A-null, A-null-big, DA-null, cap-A-null   Amendment 21: the shape-matched isogeny null at P1, t = 1..13
#   g3-b              Amendment 20: stage B gate G3 rerun with --jobs 1 on the frozen Kummer pilot over /work/rows_in/B-*.jsonl
#   x:T:K:C:DENSE     ONE Amendment 7 B bracket run (budget 86400, --max-dense DENSE, --trace), then cert of its row
# Results: $ISOLAB_RESULTS_URI (runs/, gates/, progress.txt, binary.sha256, host.txt), synced every 300 s and at exit.
# A Mestre panic ("Mestre: #E ambiguous") in any log stops the job with exit 3 (edit E4.7).
set -u
: "${ISOLAB_RESULTS_URI:?}"
R=/opt/repo/research/pkm_falldegree_unblock_20261107
S=/work/study
mkdir -p "$S/runs" "$S/gates" && cd "$S" || exit 2
export BIN=$R/iso/pilot_mestre.sh
export RAYON_NUM_THREADS=12          # pinned (G-AUX: F4 muladds depend on the rayon thread count)
P0=786433
P1=2013265921
c1=(--p $P1 --ladder-t none --planted 0 --random 2 --kinds isogeny --m 2 --controls tower)
n1=(--p $P1 --ladder-t none --planted 0 --random 2 --kinds isogeny --m 2 --controls null)   # Amendment 21: isogeny null
hky=(--engine hky --hky-cmax 7 --budget 1800 --hky-max-cols 2000000 --hky-max-entries 300000000)
f4=(--engine tower --budget 1800 --max-nnz 2000000000)
log() { echo "$(date -u +%FT%TZ) $*" >> "$S/progress.txt"; }
up() {
  aws s3 sync "$S/runs" "$ISOLAB_RESULTS_URI/runs" --only-show-errors
  aws s3 sync "$S/gates" "$ISOLAB_RESULTS_URI/gates" --only-show-errors
  aws s3 cp "$S/progress.txt" "$ISOLAB_RESULTS_URI/progress.txt" --only-show-errors 2>/dev/null
}
panicked() { grep -l 'Mestre: #E ambiguous' "$S"/runs/*.log "$S"/gates/*.log "$S"/gates/gaux/*.log 2>/dev/null | head -1; }
run() {  # run NAME ARGS... : one pilot invocation, rows to runs/NAME.jsonl
  name=$1; shift
  log "start $name: $*"
  "$BIN" "$@" --out "runs/$name.jsonl" > "runs/$name.out" 2> "runs/$name.log"
  log "end $name exit $?"
}
sha256sum /usr/local/bin/pkm_tower_pilot_r4 $(ls /usr/local/bin/pkm_tower_pilot 2>/dev/null) /usr/local/bin/prime_fall_degree_tools /usr/local/bin/hky_fp_tower_check \
  "$BIN" "$0" "$R/iso/certify_m2.py" "$R/../pkm_tower_pilot_20260924/verify.py" > "$S/binary.sha256"
{ echo "argv: $*"; echo "nproc: $(nproc)"; echo "rayon: $RAYON_NUM_THREADS"
  grep -m1 'model name' /proc/cpuinfo; grep MemTotal /proc/meminfo; uname -r
  echo "cgroup memory.max: $(cat /sys/fs/cgroup/memory.max 2>/dev/null || echo n/a)"; } > "$S/host.txt"
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
    gaux) log "start gaux"; mkdir -p gates/gaux
        c0=(--p $P0 --ladder-t none --planted 0 --random 2 --kinds isogeny --m 2 --controls tower --t-min 1 --max-t 8)
        "$BIN" "${c0[@]}" "${hky[@]}" --out gates/gaux/A-m2-iso-p0-mestre.jsonl 2> gates/gaux/A.log
        "$BIN" "${c0[@]}" "${f4[@]}" --out gates/gaux/DA-m2-iso-p0-mestre.jsonl 2> gates/gaux/DA.log
        python3 "$R/gaux/compare.py" gates/gaux > gates/gaux.log 2>&1; rc=$?
        grep -q '^pass 16 draws, 0 mismatched' gates/gaux.log || { [ "$rc" = 0 ] && rc=1; }; log "end gaux exit $rc"
        [ "$rc" = 0 ] || { fail=1; break; } ;;
    A) run A-m2-iso-p1 "${c1[@]}" "${hky[@]}" --t-min 1 --max-t 10 ;;
    A-big) run A-m2-iso-p1-big "${c1[@]}" "${hky[@]}" --hky-cmax 5 --t-min 11 --max-t 13 ;;
    DA) run DA-m2-iso-p1 "${c1[@]}" "${f4[@]}" --t-min 1 --max-t 13 ;;
    A-null) run A-m2-iso-p1-null "${n1[@]}" "${hky[@]}" --t-min 1 --max-t 10 ;;                   # Amendment 21
    A-null-big) run A-m2-iso-p1-null-big "${n1[@]}" "${hky[@]}" --hky-cmax 5 --t-min 11 --max-t 13 ;; # Amendment 21
    DA-null) run DA-m2-iso-p1-null "${n1[@]}" "${f4[@]}" --t-min 1 --max-t 13 ;;                  # Amendment 21
    cap-A-null) for t in 11 12 13; do for k in 0 1; do                                             # Amendment 21
          D=$(grep -h "\"t\":$t," runs/DA-m2-iso-p1-null.jsonl | grep "\"target_index\":$k," | sed 's/.*"solving_degree_max":\([0-9]*\).*/\1/' | head -1)
          [ -n "$D" ] || continue
          run CAPN-iso-m2-t$t-k$k-c$((D-1)) "${n1[@]}" "${f4[@]}" --t-min $t --max-t $t --targets $k --cap $((D-1))
        done; done ;;
    A-14) run A-m2-iso-p1-t14 "${c1[@]}" "${hky[@]}" --hky-cmax 5 --t-min 14 --max-t 14 ;;     # Amendment 19
    DA-14) run DA-m2-iso-p1-t14 "${c1[@]}" "${f4[@]}" --t-min 14 --max-t 14 ;;               # Amendment 19
    cap-14) for k in 0 1; do                                                                  # Amendment 19
          D=$(grep -h "\"t\":14," runs/DA-m2-iso-p1-t14.jsonl | grep "\"target_index\":$k," | sed 's/.*"solving_degree_max":\([0-9]*\).*/\1/' | head -1)
          [ -n "$D" ] || continue
          run CAP-iso-m2-t14-k$k-c$((D-1)) "${c1[@]}" "${f4[@]}" --t-min 14 --max-t 14 --targets $k --cap $((D-1))
        done ;;
    g3) log "start g3"; mkdir -p gates/g3dir/runs; cp runs/A-m2-iso-p1*.jsonl gates/g3dir/runs/
        /usr/local/bin/prime_fall_degree_tools gate3 "$S/gates/g3dir" "$BIN" --max-n 20 --jobs 2 --budget 900 \
          > gates/g3.log 2>&1; rc=$?
        grep -q ', 0 disagreements' gates/g3.log || { [ "$rc" = 0 ] && rc=1; }; log "end g3 exit $rc"
        [ "$rc" = 0 ] || { fail=1; break; } ;;
    g3-b) log "start g3-b"; mkdir -p gates/g3b/runs; cp /work/rows_in/B-*.jsonl gates/g3b/runs/   # Amendment 20
        /usr/local/bin/prime_fall_degree_tools gate3 "$S/gates/g3b" /usr/local/bin/pkm_tower_pilot --max-n 20 --jobs 1 \
          --budget 900 > gates/g3b.log 2>&1; rc=$?
        grep -q ', 0 disagreements' gates/g3b.log || { [ "$rc" = 0 ] && rc=1; }; log "end g3-b exit $rc"
        [ "$rc" = 0 ] || { fail=1; break; } ;;
    cert) log "start cert"; python3 "$R/iso/certify_m2.py" runs/CERT-m2-iso-p1.jsonl runs/A-m2-iso-p1*.jsonl runs/DA-m2-iso-p1*.jsonl \
          > runs/CERT.log 2>&1; log "end cert exit $?" ;;
    cap-A) for t in 11 12 13; do for k in 0 1; do
          D=$(grep -h "\"t\":$t," runs/DA-m2-iso-p1.jsonl | grep "\"target_index\":$k," | sed 's/.*"solving_degree_max":\([0-9]*\).*/\1/' | head -1)
          [ -n "$D" ] || continue
          run CAP-iso-m2-t$t-k$k-c$((D-1)) "${c1[@]}" "${f4[@]}" --t-min $t --max-t $t --targets $k --cap $((D-1))
        done; done ;;
    x:*) IFS=: read -r _ t k c dense <<< "$s"; name="X-iso-m2-t$t-k$k-c$c"
        run "$name" "${c1[@]}" --engine tower --budget 86400 --max-dense "$dense" --trace --t-min "$t" --max-t "$t" \
          --targets "$k" --cap "$c"
        python3 "$R/iso/certify_m2.py" "runs/$name.cert.jsonl" "runs/$name.jsonl" > "runs/$name.cert.log" 2>&1
        log "end $name.cert exit $?" ;;
    *) log "unknown stage $s"; fail=1; break ;;
  esac
  if [ -n "$(panicked)" ]; then log "STOP: Mestre panic in $(panicked)"; fail=3; break; fi
done
kill "$syncer" 2>/dev/null; up
[ "$fail" = 0 ] || exit "$fail"
! grep -Eq ' exit [1-9][0-9]*$' "$S/progress.txt"
