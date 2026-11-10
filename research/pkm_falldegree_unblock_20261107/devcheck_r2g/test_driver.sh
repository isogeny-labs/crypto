#!/bin/bash
# Local dev-check of stageb_units_runner.sh against the REAL frozen pilot binary. Usage: test_driver.sh T1|T2|T3|T4|T5|T6
# The runner is run exactly as in a job: /bin/bash -c "<script>" stageb <PREV...>.
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
RUNNER=$HERE/../stageb_units_runner.sh
SP=${SP:-/tmp/claude-0/-home-user/354dbe26-6a93-506a-a404-2fc6ef137fdc/scratchpad}
L=${LAYER:-$SP/r2g_layer/usr/local/bin}
W=$SP/r2g_test/${1:-x}; export FAKE_S3=$W/s3
SEED="--seed 777"          # NOT the study seed
# runner <jobname> <extra-env...> -- <PREV...>   (job results go to s3://b/<jobname>)
runner() {
  local job=$1; shift; local envs=(); while [ "$1" != -- ]; do envs+=("$1"); shift; done; shift
  rm -rf "$W/w-$job"
  env ISOLAB_RESULTS_URI=s3://b/$job STUDY_DIR=$W/w-$job BIN=${BIN:-$L/pkm_tower_pilot} G1BIN=$L/hky_fp_tower_check \
    S3CP="$HERE/s3shim.sh cp" S3SYNC="$HERE/s3shim.sh sync" S3LS="$HERE/s3shim.sh ls" RAYON_NUM_THREADS=2 "${envs[@]}" \
    bash -c "$(cat "$RUNNER")" stageb "$@"
}
# row fields without wall time: ms, wall_s, closures[].ms
strip() { python3 -I -c '
import sys,json
def clean(o):
    if isinstance(o,dict): return {k:clean(v) for k,v in o.items() if k not in ("ms","wall_s")}
    if isinstance(o,list): return [clean(x) for x in o]
    return o
for l in sys.stdin:
    l=l.strip()
    if l: print(json.dumps(clean(json.loads(l)),sort_keys=True))
'; }

HKY="--p 2013265921 --ladder-t none --planted 0 --random 2 --engine hky --hky-cmax 7 --budget 1800 --hky-max-cols 2000000 --hky-max-entries 300000000"
F4="--p 2013265921 --ladder-t none --planted 0 --random 2 --engine tower --budget 1800 --max-nnz 2000000000"
rows() { grep -v '^{"summary"' "$@"; }
waits3() { # wait until s3 progress of job $1 contains "start $2"
  for _ in $(seq 1 600); do grep -q "start $2\$" "$W/s3/b/$1/progress.txt" 2>/dev/null && return 0; sleep 0.05; done; return 1; }
killjob() { # SIGKILL the runner's process group (runner, syncer, pilot) = a pod loss
  local pg; pg=$(cat "$W/pid-$1"); kill -9 -- "-$pg" 2>/dev/null; }
bgrunner() { # bgrunner <job> <env...> -- <PREV...>: runner in its own session (pgid = $$ of the job script), as a pod would be
  local job=$1
  { echo "echo \$\$ > $W/pid-$job"; declare -f runner; echo "W=$W HERE=$HERE RUNNER=$RUNNER L=$L; export FAKE_S3=$FAKE_S3"
    echo "runner $(printf '%q ' "$@")"; } > "$W/job-$job.sh"
  setsid bash "$W/job-$job.sh" > "$W/out-$job.txt" 2>&1 < /dev/null &
  sleep 0.3; }
case ${1:-} in
T1) bash -c "$(cat "$RUNNER")" stageb --list > $SP/r2g_test/list.txt; echo rc=$?
    wc -l < $SP/r2g_test/list.txt; awk '{print $2}' $SP/r2g_test/list.txt | uniq | wc -l
    awk '{print $2}' $SP/r2g_test/list.txt | uniq | tr '\n' ' '; echo
    grep -c '^B-' $SP/r2g_test/list.txt; grep -c '^DB-' $SP/r2g_test/list.txt
    awk '$2 ~ /^B-/ {print $2}' $SP/r2g_test/list.txt | uniq | wc -l
    awk '{print $2, $3}' $SP/r2g_test/list.txt | awk '{m[$1]=$2} END{for(k in m) print k, "max-t="m[k]}' | sort ;;
T2) rm -rf "$W"; mkdir -p "$W"
    runner j1 PILOT_EXTRA="$SEED" UNIT_FILTER='^(B-m3-tower-t[23]|DB-m3-tower-t2)$' -- ; echo rc=$?
    cat $W/w-j1/progress.txt; wc -l $W/w-j1/runs/*.jsonl; ls $W/s3/b/j1 $W/s3/b/j1/runs; cat $W/w-j1/units_done.txt $W/w-j1/ladders_closed.txt; cat $W/w-j1/host.txt ;;
T3) rm -rf "$W"; mkdir -p "$W"
    for B in 0 2; do
      echo "=== T3 budget $B: runner, ladder B-m3-tower"
      runner b$B PILOT_EXTRA="$SEED --budget $B" UNIT_FILTER='^B-m3-tower-t' -- ; echo rc=$?
      cat $W/w-b$B/progress.txt; cat $W/w-b$B/ladders_closed.txt
      grep -H 'timed out; stopping' $W/w-b$B/runs/*.log
      echo "--- registered-style single invocation, same extra flags"
      ( cd $W && RAYON_NUM_THREADS=2 $L/pkm_tower_pilot $HKY --kinds kummer --m 3 --controls tower --t-min 1 --max-t-m3 8 --max-t-m4 6 --max-t 6 $SEED --budget $B --out reg-b$B.jsonl ) 2>&1 | grep -E 'timed out|TIMEOUT|^kummer.*hky' | cut -c1-150
    done ;;
T4) rm -rf "$W"; mkdir -p "$W"; cd $W
    echo "=== T4a hky B-m3-tower levels 2..4"
    RAYON_NUM_THREADS=2 $L/pkm_tower_pilot $HKY --kinds kummer --m 3 --controls tower --t-min 1 --max-t-m3 4 --max-t-m4 4 --max-t 4 $SEED --out reg-B.jsonl 2> reg-B.log
    runner e1 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t[234]$' -- ; echo rc=$?
    rows reg-B.jsonl | strip > reg-B.stripped; cat w-e1/runs/B-m3-tower-t2.jsonl w-e1/runs/B-m3-tower-t3.jsonl w-e1/runs/B-m3-tower-t4.jsonl | rows /dev/stdin | strip > units-B.stripped
    wc -l reg-B.stripped units-B.stripped; cmp reg-B.stripped units-B.stripped && echo "IDENTICAL (all fields except ms, wall_s, closures[].ms)"
    echo "=== T4b F4 DB-m3-tower levels 2..3"
    RAYON_NUM_THREADS=2 $L/pkm_tower_pilot $F4 --kinds kummer --m 3 --controls tower --t-min 1 --max-t-m3 3 --max-t-m4 3 --max-t 3 $SEED --out reg-DB.jsonl 2> reg-DB.log
    runner e2 PILOT_EXTRA="$SEED" UNIT_FILTER='^DB-m3-tower-t[23]$' -- ; echo rc=$?
    rows reg-DB.jsonl | strip > reg-DB.stripped; cat w-e2/runs/DB-m3-tower-t2.jsonl w-e2/runs/DB-m3-tower-t3.jsonl | rows /dev/stdin | strip > units-DB.stripped
    wc -l reg-DB.stripped units-DB.stripped; cmp reg-DB.stripped units-DB.stripped && echo "IDENTICAL (all fields except ms, wall_s, closures[].ms)"
    echo "=== T4c P0 isogeny B-m3-tower-p0-isogeny levels 2..3 (extra)"
    RAYON_NUM_THREADS=2 $L/pkm_tower_pilot --p 786433 --ladder-t none --planted 0 --random 2 --engine hky --hky-cmax 7 --budget 1800 --hky-max-cols 2000000 --hky-max-entries 300000000 --kinds isogeny --m 3 --controls tower --t-min 1 --max-t-m3 3 --max-t-m4 3 --max-t 3 $SEED --out reg-P0.jsonl 2> reg-P0.log
    runner e3 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-p0-isogeny-t[23]$' -- ; echo rc=$?
    rows reg-P0.jsonl | strip > reg-P0.stripped; cat w-e3/runs/B-m3-tower-p0-isogeny-t2.jsonl w-e3/runs/B-m3-tower-p0-isogeny-t3.jsonl | rows /dev/stdin | strip > units-P0.stripped
    wc -l reg-P0.stripped units-P0.stripped; cmp reg-P0.stripped units-P0.stripped && echo "IDENTICAL"
    echo "=== T4d null chain B-m3-null levels 2..3 (extra)"
    RAYON_NUM_THREADS=2 $L/pkm_tower_pilot $HKY --kinds kummer --m 3 --controls null --null-chain --t-min 1 --max-t-m3 3 --max-t-m4 3 --max-t 3 $SEED --out reg-N.jsonl 2> reg-N.log
    runner e4 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-null-t[23]$' -- ; echo rc=$?
    rows reg-N.jsonl | strip > reg-N.stripped; cat w-e4/runs/B-m3-null-t2.jsonl w-e4/runs/B-m3-null-t3.jsonl | rows /dev/stdin | strip > units-N.stripped
    wc -l reg-N.stripped units-N.stripped; cmp reg-N.stripped units-N.stripped && echo "IDENTICAL" ;;
T5) rm -rf "$W"; mkdir -p "$W"; F='^(B-m3-tower-t[2-4]|DB-m3-tower-t2)$'
    echo "=== job A (PREV none), killed with SIGKILL while B-m3-tower-t4 is in flight"
    bgrunner A PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- ; waits3 A B-m3-tower-t4 && killjob A; sleep 1
    # a killed child can linger as a defunct zombie (no reaper in this sandbox): count live pilots only
    ps -eo stat,cmd | grep pkm_tower_pilot | grep -v grep | grep -qv '^Z' && echo "pilot still running (BAD)" || echo "no live pilot left after the kill (defunct zombie entries ignored)"
    echo "S3 progress of A:"; cat $W/s3/b/A/progress.txt; ls $W/s3/b/A/runs
    echo "=== job D (PREV = A): resume to the end"
    runner D PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A; echo rc=$?; cat $W/w-D/progress.txt; cat $W/w-D/units_done.txt; grep prev $W/w-D/host.txt
    echo "=== job B (PREV = A), killed again while B-m3-tower-t4 is in flight"
    bgrunner B PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A; waits3 B B-m3-tower-t4 && killjob B; sleep 1
    cat $W/s3/b/B/progress.txt
    echo "=== job C (PREV = A, B): t4 in flight in two prevs"
    runner C PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A s3://b/B; echo rc=$?; cat $W/w-C/progress.txt; echo "lost_units.txt:"; cat $W/w-C/lost_units.txt; ls $W/w-C/runs; ls $W/s3/b/C ;;
T6) rm -rf "$W"; mkdir -p "$W"
    cat > $W/failwrap.sh <<EOF
#!/bin/bash
case "\$*" in *"--out runs/B-m3-tower-t3.jsonl"*) echo "wrapper: forced failure" >&2; exit 3;; esac
exec $L/pkm_tower_pilot "\$@"
EOF
    chmod +x $W/failwrap.sh
    BIN=$W/failwrap.sh runner f1 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t[234]$' -- ; echo rc=$?
    cat $W/w-f1/progress.txt; grep -c "end B-m3-tower-t3 exit 0" $W/w-f1/progress.txt; cat $W/s3/b/f1/progress.txt | tail -3; ls $W/s3/b/f1/runs; cat $W/w-f1/units_done.txt ;;
T5k) rm -rf "$W"; mkdir -p "$W"   # kill check only: is the pilot gone after the SIGKILL of the runner's process group?
    bgrunner A PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t4$' -- ; waits3 A B-m3-tower-t4 && { ps -eo pid,pgid,cmd | grep -E "pkm_tower|job-A" | grep -v grep | cut -c1-90; echo "pgid file: $(cat $W/pid-A)"; killjob A; }
    sleep 1; echo "after kill:"; ps -eo pid,pgid,cmd | grep -E "pkm_tower|job-A|stageb" | grep -v grep | cut -c1-90 ;;
*) echo "unknown test"; exit 2 ;;
esac
