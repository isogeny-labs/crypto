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
  /usr/bin/env ISOLAB_RESULTS_URI=s3://b/$job STUDY_DIR=$W/w-$job BIN=${BIN:-$L/pkm_tower_pilot} G1BIN=$L/hky_fp_tower_check \
    S3CP="$HERE/s3shim.sh cp" S3SYNC="$HERE/s3shim.sh sync" RAYON_NUM_THREADS=2 "${envs[@]}" \
    /bin/bash -c "$(cat "$RUNNER")" stageb "$@"
}
runner_bg() { # same, in the background without setsid; the runner's pid is BGPID (env execs bash)
  local job=$1; shift; local envs=(); while [ "$1" != -- ]; do envs+=("$1"); shift; done; shift
  rm -rf "$W/w-$job"
  /usr/bin/env ISOLAB_RESULTS_URI=s3://b/$job STUDY_DIR=$W/w-$job BIN=${BIN:-$L/pkm_tower_pilot} G1BIN=$L/hky_fp_tower_check \
    S3CP="$HERE/s3shim.sh cp" S3SYNC="$HERE/s3shim.sh sync" RAYON_NUM_THREADS=2 "${envs[@]}" \
    /bin/bash -c "$(cat "$RUNNER")" stageb "$@" > "$W/out-$job.txt" 2>&1 < /dev/null &
  BGPID=$!; }
livepilots() { ps -eo stat,cmd | grep pkm_tower_pilot | grep -v grep | grep -vc '^Z'; }
happy() { # happy <job> : the T2 job (3 units)
  runner $1 PILOT_EXTRA="$SEED" UNIT_FILTER='^(B-m3-tower-t[23]|DB-m3-tower-t2)$' "${@:2}"; }
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
T7) rm -rf "$W"; mkdir -p "$W"   # SIGTERM, runner started without setsid
    runner_bg A PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t[34]$' --
    waits3 A B-m3-tower-t4; sleep 3; kill -TERM $BGPID; wait $BGPID; echo "runner rc=$?"
    echo "live pilots after TERM: $(livepilots)"
    echo "S3 progress:"; cat $W/s3/b/A/progress.txt; echo "units_done.txt (S3):"; cat $W/s3/b/A/units_done.txt; ls $W/s3/b/A ;;
T8) rm -rf "$W"; mkdir -p "$W"
    happy P --; echo "job P rc=$?"
    for v in log jsonl nosummary; do
      rm -rf $W/s3/b/Q; cp -a $W/s3/b/P $W/s3/b/Q
      case $v in log) rm $W/s3/b/Q/runs/B-m3-tower-t3.log;; jsonl) rm $W/s3/b/Q/runs/B-m3-tower-t3.jsonl;; nosummary) sed -i '$d' $W/s3/b/Q/runs/B-m3-tower-t3.jsonl;; esac
      echo "=== variant: $v"; runner R-$v PILOT_EXTRA="$SEED" UNIT_FILTER='^(B-m3-tower-t[23]|DB-m3-tower-t2)$' -- s3://b/Q; echo "rc=$?"
      cat $W/w-R-$v/progress.txt; echo "unit starts: $(grep -c 'start B-\|start DB-' $W/w-R-$v/progress.txt)"
    done ;;
T9) rm -rf "$W"; mkdir -p "$W"; touch $W/fail1
    echo "=== progress.txt upload fails (marker present from the start)"
    runner u1 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t2$' SHIM_FAIL_PROGRESS=$W/fail1 -- ; echo "rc=$?"
    cat $W/w-u1/progress.txt; echo "files in runs/ (no pilot started): $(ls $W/w-u1/runs | wc -l)"
    touch $W/fail2
    echo "=== sync of runs/ fails after the unit"
    runner u2 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t2$' SHIM_FAIL_SYNC_RUNS=$W/fail2 -- ; echo "rc=$?"
    echo "local progress:"; cat $W/w-u2/progress.txt; echo "S3 progress:"; cat $W/s3/b/u2/progress.txt
    echo "end-exit-0 lines in S3: $(grep -c 'end B-m3-tower-t2 exit 0' $W/s3/b/u2/progress.txt)" ;;
T10) rm -rf "$W"; mkdir -p "$W/fakebin"
    printf '#!/bin/bash\ntouch %s/aws-called\n' "$W" > $W/fakebin/aws; chmod +x $W/fakebin/aws
    for v in PILOT_EXTRA UNIT_FILTER BIN G1BIN STUDY_DIR; do
      /usr/bin/env -u S3CP -u S3SYNC PATH="$W/fakebin:$PATH" ISOLAB_RESULTS_URI=s3://b/x "$v=something" /bin/bash -c "$(cat "$RUNNER")" stageb; echo "$v set: rc=$?"
    done
    ls $W/aws-called 2>&1 | head -1 ;;
T11) rm -rf "$W"; mkdir -p "$W"
    happy s1 SHIM_READS_STDIN=1 -- ; echo "rc=$?"; cat $W/w-s1/progress.txt; cat $W/w-s1/units_done.txt ;;
T12) rm -rf "$W"; mkdir -p "$W/bin"
    for t in bash awk gawk basename cat cp cut date dirname grep head ls mkdir mktemp mv nproc rm rmdir sed sha256sum sleep sort stat tail tee timeout touch tr uname wc find xargs; do ln -s "$(command -v $t)" $W/bin/$t; done
    echo "tools in PATH: $(ls $W/bin | wc -l); flock present: $(PATH=$W/bin command -v flock || echo no); python3 present: $(PATH=$W/bin command -v python3 || echo no)"
    happy t12 PATH=$W/bin -- ; echo "rc=$?"; cat $W/w-t12/progress.txt; ls $W/w-t12/.up.lock.d 2>&1 | head -1 ;;
T13) rm -rf "$W"; mkdir -p "$W"; export SHIM_LOG=$W/ops.log
    happy j1 ISOLAB_RESULTS_URI=s3://b/jobS/ UNIT_FILTER='^(B-m3-tower-t[23]|DB-m3-tower-t2)$' -- ; echo "job1 rc=$?"
    runner j2 PILOT_EXTRA="$SEED" ISOLAB_RESULTS_URI=s3://b/jobS2/ UNIT_FILTER='^(B-m3-tower-t[2-4]|DB-m3-tower-t2)$' -- 's3://b/jobS/'; echo "job2 (PREV=s3://b/jobS/) rc=$?"
    cat $W/w-j2/progress.txt
    echo "--- raw URIs of job1 (first 12) and the PREV reads of job2:"; grep jobS/ $W/ops.log | head -12; grep 'jobS//\(progress\|runs\)' $W/ops.log | grep -c . ; grep 'cp s3://b/jobS//' $W/ops.log | head -8
    echo "--- shim directory layout:"; (cd $W/s3 && find . -type f | sort | head -40) ;;
T14) rm -rf "$W"; mkdir -p "$W"
    happy j1 --; echo "T2 job (list exhausted) rc=$?"
    echo "=== forced exit 14: a prev says ladder B-m3-tower closed at t=4 (done t4 with 'timed out; stopping') but t2, t3 were never done"
    mkdir -p $W/s3/b/X/runs
    printf '2026-11-10T00:00:00Z start B-m3-tower-t4\n2026-11-10T00:00:01Z end B-m3-tower-t4 exit 0\n' > $W/s3/b/X/progress.txt
    echo '-- kummer m=3 tower: every run at t = 4 timed out; stopping' > $W/s3/b/X/runs/B-m3-tower-t4.log
    echo '{"summary":{}}' > $W/s3/b/X/runs/B-m3-tower-t4.jsonl
    runner j3 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t[234]$' -- s3://b/X; echo "rc=$?"; cat $W/w-j3/progress.txt ;;
T15) rm -rf "$W"; mkdir -p "$W"; F='^(B-m3-tower-t[2-4]|DB-m3-tower-t2)$'
    for j in A B; do
      if [ $j = A ]; then PV=(); else PV=(s3://b/A); fi
      bgrunner $j PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- "${PV[@]}"; waits3 $j B-m3-tower-t4 && killjob $j; sleep 1
    done
    cat $W/s3/b/B/progress.txt
    echo "=== job C without the rule: lost"; runner C0 PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A s3://b/B; echo "rc=$?"
    echo "=== job C with close:B-m3-tower:4"; runner C PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A s3://b/B close:B-m3-tower:4; echo "rc=$?"
    cat $W/w-C/progress.txt; echo "ladders_closed.txt:"; cat $W/w-C/ladders_closed.txt; echo "units_done.txt:"; cat $W/w-C/units_done.txt; grep -E "prev|closed_by_rule" $W/w-C/host.txt
    for a in close:NOPE:3 close:B-m3-tower:99 close:B-m3-tower:1 close:B-m3-tower:x; do
      runner bad PILOT_EXTRA="$SEED" -- $a; echo "$a: rc=$?"; done ;;
*) echo "unknown test"; exit 2 ;;
esac
