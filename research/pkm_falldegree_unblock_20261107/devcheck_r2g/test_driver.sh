#!/bin/bash
# Local dev-check of stageb_units_runner.sh (revision 3) against the REAL frozen pilot binary.
# Usage: test_driver.sh T1 | T2 | ... | T22 | T24 | T25 | T26 | T5k   (T23 is run_suite.sh with BOOT_REQUEST)       (run_suite.sh runs them all and writes the outputs)
# How the runner is started: by default as a FILE, /bin/bash <runner> <args>. With BOOT_REQUEST=<request.json> (a request
# written by make_request_bu.sh) exactly as a job starts it: the request's command words up to and including its last runner
# piece (/bin/bash -c <BOOT> stageb <piece>...), then the test's own arguments. Each test ends with "RESULT Tn PASS|FAIL",
# computed from its "check PASS|FAIL" lines; a few tests also print what a person must read (README says which).
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
RUNNER=${RUNNER:-$HERE/../stageb_units_runner.sh}
SP=${SP:-/tmp/claude-0/-home-user/354dbe26-6a93-506a-a404-2fc6ef137fdc/scratchpad}
L=${LAYER:-$SP/r2g_layer/usr/local/bin}
W=${WROOT:-$SP/r2g_test}/${1:-x}; export FAKE_S3=$W/s3; S3=$W/s3/b
SEED="--seed 777"          # NOT the study seed
if [ -n "${BOOT_REQUEST:-}" ]; then
  np=$(jq -r '.job.command[2]' "$BOOT_REQUEST" | sed -n 's/.*shift \([0-9][0-9]*\);.*/\1/p')
  mapfile -t RUNCMD < <(jq -r --argjson k "$((4 + np))" '.job.command[0:$k][]' "$BOOT_REQUEST")
else RUNCMD=(${IMG_BASH:-/bin/bash} "$RUNNER"); fi
# IMG_BASH=<bash binary> runs the runner with the job image's own bash; IMG_AWK=<wrapper script> puts an awk and a gawk that
# resolve to the image's gawk first on PATH (img_gawk_wrapper.sh sets LD_LIBRARY_PATH for gawk only).
if [ -n "${IMG_AWK:-}" ]; then IMGBIN=${WROOT:-$SP/r2g_test}/imgbin; mkdir -p "$IMGBIN"; ln -sf "$IMG_AWK" "$IMGBIN/awk"; ln -sf "$IMG_AWK" "$IMGBIN/gawk"; export PATH="$IMGBIN:$PATH"; fi
RSHA_EXPECT=$(sha256sum "$RUNNER" | cut -d' ' -f1)

# prep <jobname> <extra-env...> -- <args...>: sets ENVARR (env assignments) and ARGS; job results go to s3://b/<jobname>
# Extra env words: SHIM=<shim script> chooses the S3 stand-in; PRELOCK=1 pre-creates the lock directory; the rest goes to the runner.
prep() {
  local job=$1 shim=$HERE/s3shim.sh e prelock=0; shift; local envs=()
  while [ "$1" != -- ]; do envs+=("$1"); case "$1" in SHIM=*) shim=${1#SHIM=};; PRELOCK=1) prelock=1;; esac; shift; done; shift
  rm -rf "$W/w-$job"; [ $prelock = 1 ] && mkdir -p "$W/w-$job/.up.lock.d"
  ENVARR=(ISOLAB_RESULTS_URI=s3://b/$job STUDY_DIR=$W/w-$job BIN=${BIN:-$L/pkm_tower_pilot} G1BIN=$L/hky_fp_tower_check
          S3CP="$shim cp" S3SYNC="$shim sync" RAYON_NUM_THREADS=2 PILOT_REAL=$L/pkm_tower_pilot "${envs[@]}")
  ARGS=("$@")
}
runner() { prep "$@"; /usr/bin/env "${ENVARR[@]}" "${RUNCMD[@]}" "${ARGS[@]}"; }
runner_bg() { # same, in the background without setsid; the runner's pid is BGPID (env execs bash)
  prep "$@"; /usr/bin/env "${ENVARR[@]}" "${RUNCMD[@]}" "${ARGS[@]}" > "$W/out-$1.txt" 2>&1 < /dev/null &
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
  { echo "echo \$\$ > $W/pid-$job"; declare -f prep runner; declare -p RUNCMD; echo "W=$W HERE=$HERE RUNNER=$RUNNER L=$L; export FAKE_S3=$FAKE_S3"
    echo "runner $(printf '%q ' "$@")"; } > "$W/job-$job.sh"
  setsid bash "$W/job-$job.sh" > "$W/out-$job.txt" 2>&1 < /dev/null &
  sleep 0.3; }
killed_in() { # killed_in <job> <unit> <env...> -- <args...>: start the job, kill it (SIGKILL, whole group) when <unit> is in flight
  local job=$1 u=$2; shift 2; bgrunner "$job" "$@"; waits3 "$job" "$u" && killjob "$job"; sleep 1; }
# checks: chk "<what>" <command...> prints "check PASS|FAIL: <what>"; the test's RESULT is PASS iff no check failed
NF=0
chk() { local d=$1; shift; if "$@" > /dev/null 2>&1; then echo "check PASS: $d"; else echo "check FAIL: $d"; NF=$((NF + 1)); fi; }
has() { grep -q -- "$1" "$2"; }
hasnt() { ! grep -q -- "$1" "$2"; }
eq() { [ "$1" = "$2" ]; }
ge() { [ "$1" -ge "$2" ]; }
lt() { [ "$1" -lt "$2" ]; }
nstart() { grep -c ' start \(B-\|DB-\)' "$1"; }    # number of unit starts in a progress.txt
ends0() { awk '$2=="end"&&$4=="exit"&&$5=="0"&&$3!="g1"{print $3}' "$1"; }
inv() { local u; for u in $(ends0 "$S3/$1/progress.txt"); do grep -qx "$u" "$S3/$1/units_done.txt" || return 1; done; }  # S3: every end-0 line has its unit in units_done
snapinv() { local f u; for f in "$1"/*.progress; do for u in $(ends0 "$f"); do grep -qx "$u" "${f%.progress}.units_done" || return 1; done; done; }
argv_of() { echo "$(bash "$RUNNER" --argv "$1" "$2") $SEED --out runs/$1-t$2.jsonl"; }

case ${1:-} in
T1) rm -rf "$W"; mkdir -p "$W"
    "${RUNCMD[@]}" --list > $W/list.txt; rc=$?; echo rc=$rc
    wc -l < $W/list.txt; awk '{print $2}' $W/list.txt | uniq | wc -l
    awk '{print $2}' $W/list.txt | uniq | tr '\n' ' '; echo
    grep -c '^B-' $W/list.txt; grep -c '^DB-' $W/list.txt
    awk '$2 ~ /^B-/ {print $2}' $W/list.txt | uniq | wc -l
    awk '{print $2, $3}' $W/list.txt | awk '{m[$1]=$2} END{for(k in m) print k, "max-t="m[k]}' | sort
    sha256sum $W/list.txt
    chk "rc 0" eq $rc 0
    chk "91 lines" eq "$(wc -l < $W/list.txt)" 91
    chk "25 ladders" eq "$(awk '{print $2}' $W/list.txt | uniq | wc -l)" 25
    chk "49 B- and 42 DB- units" eq "$(grep -c '^B-' $W/list.txt)/$(grep -c '^DB-' $W/list.txt)" 49/42
    chk "list sha256 unchanged from revision 2" eq "$(sha256sum < $W/list.txt | cut -d' ' -f1)" 0f80105da0a47729f6a5242a522356e0f3877f29e2ca4c0c9a8e532ca583d2b5 ;;
T2) rm -rf "$W"; mkdir -p "$W"
    runner j1 PILOT_EXTRA="$SEED" UNIT_FILTER='^(B-m3-tower-t[23]|DB-m3-tower-t2)$' NODE_NAME=node-x POD_UID=pod-y ISOLAB_JOB_ID=job-z -- ; rc=$?; echo rc=$rc
    cat $W/w-j1/progress.txt; wc -l $W/w-j1/runs/*.jsonl; ls $W/s3/b/j1 $W/s3/b/j1/runs; cat $W/w-j1/units_done.txt $W/w-j1/ladders_closed.txt
    echo "--- runs/*.argv (S3 copy)"; for f in $W/s3/b/j1/runs/*.argv; do echo "$(basename $f): $(cat $f)"; done
    echo "--- host.txt"; cat $W/w-j1/host.txt
    chk "rc 0" eq $rc 0
    chk "3 units end exit 0" eq "$(ends0 $W/w-j1/progress.txt | wc -l)" 3
    chk "each jsonl has 2 rows + summary" eq "$(wc -l < $W/w-j1/runs/B-m3-tower-t2.jsonl)" 3
    chk "S3 holds the 9 unit files and 3 argv files" eq "$(ls $S3/j1/runs | wc -l)" 12
    chk "argv record of B-m3-tower-t3 = pilot words" eq "$(cat $S3/j1/runs/B-m3-tower-t3.argv)" "$(argv_of B-m3-tower 3)"
    chk "argv record of DB-m3-tower-t2 = pilot words" eq "$(cat $S3/j1/runs/DB-m3-tower-t2.argv)" "$(argv_of DB-m3-tower 2)"
    chk "S3 units_done.txt lists the 3 units" eq "$(cat $S3/j1/units_done.txt | tr '\n' ' ')" "B-m3-tower-t2 B-m3-tower-t3 DB-m3-tower-t2 "
    chk "host.txt runner_sha256 = sha256 of the runner" has "^runner_sha256: $RSHA_EXPECT\$" $S3/j1/host.txt
    chk "host.txt records NODE_NAME, POD_UID, ISOLAB_JOB_ID" eq "$(grep '^\(node\|pod_uid\|job\):' $S3/j1/host.txt | tr '\n' '|')" "node: node-x|pod_uid: pod-y|job: job-z|"
    chk "host.txt has bash, node, pod_uid, job, seed, lock_wait, closed_by_rule, allowed, closed_tool_failure lines" eq "$(grep -c '^\(bash\|node\|pod_uid\|job\|seed\|lock_wait\|closed_by_rule\|allowed\|closed_tool_failure\):' $S3/j1/host.txt)" 9 ;;
T3) rm -rf "$W"; mkdir -p "$W"
    for B in 0 2; do
      echo "=== T3 budget $B: runner, ladder B-m3-tower"
      runner b$B PILOT_EXTRA="$SEED --budget $B" UNIT_FILTER='^B-m3-tower-t' -- ; rc=$?; echo rc=$rc
      cat $W/w-b$B/progress.txt; cat $W/w-b$B/ladders_closed.txt
      grep -H 'timed out; stopping' $W/w-b$B/runs/*.log
      echo "--- registered-style single invocation, same extra flags"
      ( cd $W && RAYON_NUM_THREADS=2 $L/pkm_tower_pilot $HKY --kinds kummer --m 3 --controls tower --t-min 1 --max-t-m3 8 --max-t-m4 6 --max-t 6 $SEED --budget $B --out reg-b$B.jsonl ) 2>&1 | grep -E 'timed out|TIMEOUT|^kummer.*hky' | cut -c1-150
      chk "budget $B: rc 0" eq $rc 0
    done
    chk "budget 0 closes at t=2" has 'closed B-m3-tower at t=2$' $W/w-b0/progress.txt
    chk "budget 0: t3..t8 ladder-closed" eq "$(grep -c 'ladder-closed' $W/w-b0/progress.txt)" 6
    chk "budget 2 closes at t=4" has 'closed B-m3-tower at t=4$' $W/w-b2/progress.txt
    chk "budget 2: t5..t8 ladder-closed" eq "$(grep -c 'ladder-closed' $W/w-b2/progress.txt)" 4 ;;
T4) rm -rf "$W"; mkdir -p "$W"; cd $W
    echo "=== T4a hky B-m3-tower levels 2..4"
    RAYON_NUM_THREADS=2 $L/pkm_tower_pilot $HKY --kinds kummer --m 3 --controls tower --t-min 1 --max-t-m3 4 --max-t-m4 4 --max-t 4 $SEED --out reg-B.jsonl 2> reg-B.log
    runner e1 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t[234]$' -- ; rc1=$?; echo rc=$rc1
    rows reg-B.jsonl | strip > reg-B.stripped; cat w-e1/runs/B-m3-tower-t2.jsonl w-e1/runs/B-m3-tower-t3.jsonl w-e1/runs/B-m3-tower-t4.jsonl | rows /dev/stdin | strip > units-B.stripped
    wc -l reg-B.stripped units-B.stripped; cmp reg-B.stripped units-B.stripped && echo "IDENTICAL (all fields except ms, wall_s, closures[].ms)"
    chk "B-m3-tower rc 0 and rows identical" cmp reg-B.stripped units-B.stripped
    echo "=== T4b F4 DB-m3-tower levels 2..3"
    RAYON_NUM_THREADS=2 $L/pkm_tower_pilot $F4 --kinds kummer --m 3 --controls tower --t-min 1 --max-t-m3 3 --max-t-m4 3 --max-t 3 $SEED --out reg-DB.jsonl 2> reg-DB.log
    runner e2 PILOT_EXTRA="$SEED" UNIT_FILTER='^DB-m3-tower-t[23]$' -- ; echo rc=$?
    rows reg-DB.jsonl | strip > reg-DB.stripped; cat w-e2/runs/DB-m3-tower-t2.jsonl w-e2/runs/DB-m3-tower-t3.jsonl | rows /dev/stdin | strip > units-DB.stripped
    wc -l reg-DB.stripped units-DB.stripped; cmp reg-DB.stripped units-DB.stripped && echo "IDENTICAL (all fields except ms, wall_s, closures[].ms)"
    chk "DB-m3-tower rows identical" cmp reg-DB.stripped units-DB.stripped
    echo "=== T4c P0 isogeny B-m3-tower-p0-isogeny levels 2..3 (extra)"
    RAYON_NUM_THREADS=2 $L/pkm_tower_pilot --p 786433 --ladder-t none --planted 0 --random 2 --engine hky --hky-cmax 7 --budget 1800 --hky-max-cols 2000000 --hky-max-entries 300000000 --kinds isogeny --m 3 --controls tower --t-min 1 --max-t-m3 3 --max-t-m4 3 --max-t 3 $SEED --out reg-P0.jsonl 2> reg-P0.log
    runner e3 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-p0-isogeny-t[23]$' -- ; echo rc=$?
    rows reg-P0.jsonl | strip > reg-P0.stripped; cat w-e3/runs/B-m3-tower-p0-isogeny-t2.jsonl w-e3/runs/B-m3-tower-p0-isogeny-t3.jsonl | rows /dev/stdin | strip > units-P0.stripped
    wc -l reg-P0.stripped units-P0.stripped; cmp reg-P0.stripped units-P0.stripped && echo "IDENTICAL"
    chk "P0 isogeny rows identical" cmp reg-P0.stripped units-P0.stripped
    echo "=== T4d null chain B-m3-null levels 2..3 (extra)"
    RAYON_NUM_THREADS=2 $L/pkm_tower_pilot $HKY --kinds kummer --m 3 --controls null --null-chain --t-min 1 --max-t-m3 3 --max-t-m4 3 --max-t 3 $SEED --out reg-N.jsonl 2> reg-N.log
    runner e4 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-null-t[23]$' -- ; echo rc=$?
    rows reg-N.jsonl | strip > reg-N.stripped; cat w-e4/runs/B-m3-null-t2.jsonl w-e4/runs/B-m3-null-t3.jsonl | rows /dev/stdin | strip > units-N.stripped
    wc -l reg-N.stripped units-N.stripped; cmp reg-N.stripped units-N.stripped && echo "IDENTICAL"
    chk "null chain rows identical" cmp reg-N.stripped units-N.stripped
    chk "the runner exited 0 for B-m3-tower" eq $rc1 0 ;;
T5) rm -rf "$W"; mkdir -p "$W"; F='^(B-m3-tower-t[2-4]|DB-m3-tower-t2)$'
    echo "=== job A (PREV none), killed with SIGKILL while B-m3-tower-t4 is in flight"
    bgrunner A PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- ; waits3 A B-m3-tower-t4 && killjob A; sleep 1
    # a killed child can linger as a defunct zombie (no reaper in this sandbox): count live pilots only
    ps -eo stat,cmd | grep pkm_tower_pilot | grep -v grep | grep -qv '^Z' && echo "pilot still running (BAD)" || echo "no live pilot left after the kill (defunct zombie entries ignored)"
    echo "S3 progress of A:"; cat $W/s3/b/A/progress.txt; ls $W/s3/b/A/runs
    echo "=== job D (PREV = A): resume to the end"
    runner D PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A; rcD=$?; echo rc=$rcD; cat $W/w-D/progress.txt; cat $W/w-D/units_done.txt; grep prev $W/w-D/host.txt
    echo "=== job B (PREV = A), killed again while B-m3-tower-t4 is in flight"
    bgrunner B PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A; waits3 B B-m3-tower-t4 && killjob B; sleep 1
    cat $W/s3/b/B/progress.txt
    echo "=== job C (PREV = A, B): t4 in flight in two prevs"
    runner C PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A s3://b/B; rcC=$?; echo rc=$rcC; cat $W/w-C/progress.txt; echo "lost_units.txt:"; cat $W/w-C/lost_units.txt; ls $W/w-C/runs; ls $W/s3/b/C
    chk "A: S3 has start t4 and no end t4" eq "$(grep -c 'start B-m3-tower-t4' $W/s3/b/A/progress.txt)/$(grep -c 'end B-m3-tower-t4' $W/s3/b/A/progress.txt)" 1/0
    chk "D: rc 0" eq $rcD 0
    chk "D: t2 and t3 skipped done-in 0" eq "$(grep -c 'done-in 0' $W/w-D/progress.txt)" 2
    chk "D: t4 re-run and DB-m3-tower-t2 run" eq "$(ends0 $W/w-D/progress.txt | tr '\n' ' ')" "B-m3-tower-t4 DB-m3-tower-t2 "
    chk "C: rc 75, lost B-m3-tower-t4" eq "$rcC/$(cat $W/w-C/lost_units.txt)" 75/B-m3-tower-t4
    chk "C: no unit started" eq "$(nstart $W/w-C/progress.txt)" 0 ;;
T6) rm -rf "$W"; mkdir -p "$W"
    BIN=$HERE/fail_unit_wrapper.sh runner f1 FAIL_UNIT=B-m3-tower-t3 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t[234]$' -- ; rc=$?; echo rc=$rc
    cat $W/w-f1/progress.txt; grep -c "end B-m3-tower-t3 exit 0" $W/w-f1/progress.txt; cat $W/s3/b/f1/progress.txt | tail -3; ls $W/s3/b/f1/runs; cat $W/w-f1/units_done.txt
    chk "rc 20" eq $rc 20
    chk "end t3 exit 3 in S3, no end t3 exit 0" eq "$(grep -c 'end B-m3-tower-t3 exit 3' $S3/f1/progress.txt)/$(grep -c 'end B-m3-tower-t3 exit 0' $S3/f1/progress.txt)" 1/0
    chk "t4 never started" hasnt 'start B-m3-tower-t4' $S3/f1/progress.txt
    chk "t2 files and the failed unit's log and out are in S3" eq "$(ls $S3/f1/runs | grep -c 'B-m3-tower-t2.jsonl\|B-m3-tower-t3.log\|B-m3-tower-t3.out')" 3
    chk "units_done.txt = t2 only" eq "$(cat $S3/f1/units_done.txt)" B-m3-tower-t2 ;;
T5k) rm -rf "$W"; mkdir -p "$W"   # kill check only: is the pilot gone after the SIGKILL of the runner's process group?
    bgrunner A PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t4$' -- ; waits3 A B-m3-tower-t4 && { ps -eo pid,pgid,cmd | grep -E "pkm_tower|job-A" | grep -v grep | cut -c1-90; echo "pgid file: $(cat $W/pid-A)"; killjob A; }
    sleep 1; echo "after kill:"; ps -eo pid,pgid,cmd | grep -E "pkm_tower|job-A|stageb" | grep -v grep | cut -c1-90 ;;
T7) rm -rf "$W"; mkdir -p "$W"   # SIGTERM, runner started without setsid
    runner_bg A PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t[34]$' --
    waits3 A B-m3-tower-t4; sleep 3; kill -TERM $BGPID; wait $BGPID; rc=$?; echo "runner rc=$rc"
    echo "live pilots after TERM: $(livepilots)"
    echo "S3 progress:"; cat $W/s3/b/A/progress.txt; echo "units_done.txt (S3):"; cat $W/s3/b/A/units_done.txt; ls $W/s3/b/A
    chk "runner rc 143" eq $rc 143
    chk "no live pilot" eq "$(livepilots)" 0
    chk "S3: start t4 without end t4" eq "$(grep -c 'start B-m3-tower-t4' $S3/A/progress.txt)/$(grep -c 'end B-m3-tower-t4' $S3/A/progress.txt)" 1/0
    chk "S3 units_done.txt = t3 only" eq "$(cat $S3/A/units_done.txt)" B-m3-tower-t3
    chk "stop file present, lock directory gone" eq "$(test -e $W/w-A/.stop && echo stop)/$(test -e $W/w-A/.up.lock.d && echo lock || echo nolock)" stop/nolock ;;
T8) rm -rf "$W"; mkdir -p "$W"
    happy P --; echo "job P rc=$?"
    for v in log jsonl nosummary argv hostfile progressfile binfile; do
      rm -rf $W/s3/b/Q; cp -a $W/s3/b/P $W/s3/b/Q
      case $v in log) rm $W/s3/b/Q/runs/B-m3-tower-t3.log;; jsonl) rm $W/s3/b/Q/runs/B-m3-tower-t3.jsonl;; nosummary) sed -i '$d' $W/s3/b/Q/runs/B-m3-tower-t3.jsonl;;
        argv) rm $W/s3/b/Q/runs/B-m3-tower-t3.argv;; hostfile) rm $W/s3/b/Q/host.txt;; progressfile) rm $W/s3/b/Q/progress.txt;; binfile) rm $W/s3/b/Q/binary.sha256;; esac
      echo "=== variant: $v"; runner R-$v PILOT_EXTRA="$SEED" UNIT_FILTER='^(B-m3-tower-t[23]|DB-m3-tower-t2)$' -- s3://b/Q > $W/o-$v.txt 2>&1; rc=$?; echo "rc=$rc"; cat $W/o-$v.txt
      cat $W/w-R-$v/progress.txt; echo "unit starts: $(nstart $W/w-R-$v/progress.txt)"
      chk "$v: rc 12 and no unit started" eq "$rc/$(nstart $W/w-R-$v/progress.txt)" 12/0
    done
    echo "=== control: the unedited copy is accepted"
    rm -rf $W/s3/b/Q; cp -a $W/s3/b/P $W/s3/b/Q
    runner R-ok PILOT_EXTRA="$SEED" UNIT_FILTER='^(B-m3-tower-t[23]|DB-m3-tower-t2)$' -- s3://b/Q; rc=$?; echo "rc=$rc"
    chk "control: rc 0, 3 units skipped done-in 0" eq "$rc/$(grep -c 'done-in 0' $W/w-R-ok/progress.txt)" 0/3 ;;
T9) rm -rf "$W"; mkdir -p "$W"; touch $W/fail1
    echo "=== progress.txt upload fails (marker present from the start)"
    runner u1 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t2$' SHIM_FAIL_PROGRESS=$W/fail1 -- ; rc1=$?; echo "rc=$rc1"
    cat $W/w-u1/progress.txt; echo "files in runs/ (no pilot started; only the argv record): $(ls $W/w-u1/runs | tr '\n' ' ')"
    touch $W/fail2
    echo "=== sync of runs/ fails after the unit"
    runner u2 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t2$' SHIM_FAIL_SYNC_RUNS=$W/fail2 -- ; rc2=$?; echo "rc=$rc2"
    echo "local progress:"; cat $W/w-u2/progress.txt; echo "S3 progress:"; cat $W/s3/b/u2/progress.txt
    echo "end-exit-0 lines in S3: $(grep -c 'end B-m3-tower-t2 exit 0' $W/s3/b/u2/progress.txt)"
    chk "u1: rc 13, local abort start-upload-failed, no pilot output" eq "$rc1/$(grep -c 'abort B-m3-tower-t2 start-upload-failed' $W/w-u1/progress.txt)/$(ls $W/w-u1/runs | grep -c '\.out$')" 13/1/0
    chk "u2: rc 13, local abort files-upload-failed" eq "$rc2/$(grep -c 'abort B-m3-tower-t2 files-upload-failed' $W/w-u2/progress.txt)" 13/1
    chk "u2: no end-exit-0 line in S3" eq "$(grep -c 'end B-m3-tower-t2 exit 0' $W/s3/b/u2/progress.txt)" 0 ;;
T10) rm -rf "$W"; mkdir -p "$W/fakebin"
    printf '#!/bin/bash\ntouch %s/aws-called\n' "$W" > $W/fakebin/aws; chmod +x $W/fakebin/aws
    for v in PILOT_EXTRA UNIT_FILTER BIN G1BIN STUDY_DIR LOCK_WAIT; do
      /usr/bin/env -u S3CP -u S3SYNC PATH="$W/fakebin:$PATH" ISOLAB_RESULTS_URI=s3://b/x "$v=something" "${RUNCMD[@]}"; rc=$?; echo "$v set: rc=$rc"
      chk "$v refused with exit 15" eq $rc 15
    done
    ls $W/aws-called 2>&1 | head -1
    chk "the fake aws was never called" test ! -e $W/aws-called
    LOCK_WAIT=abc /usr/bin/env PATH="$W/fakebin:$PATH" ISOLAB_RESULTS_URI=s3://b/x S3CP=x "${RUNCMD[@]}"; rc=$?; echo "LOCK_WAIT=abc in test mode: rc=$rc"
    chk "LOCK_WAIT=abc is refused (exit 15)" eq $rc 15 ;;
T11) rm -rf "$W"; mkdir -p "$W"
    happy s1 SHIM_READS_STDIN=1 -- ; rc=$?; echo "rc=$rc"; cat $W/w-s1/progress.txt; cat $W/w-s1/units_done.txt
    chk "rc 0 and all 3 units ran although the S3 command reads stdin" eq "$rc/$(ends0 $W/w-s1/progress.txt | wc -l)" 0/3 ;;
T12) rm -rf "$W"; mkdir -p "$W/bin"
    for t in bash awk gawk basename cat cp cut date dirname grep head ls mkdir mktemp mv nproc rm rmdir sed sha256sum sleep sort stat tail tee timeout touch tr uname wc find xargs; do command -v $t > /dev/null && ln -s "$(command -v $t)" $W/bin/$t; done
    [ -n "${IMG_AWK:-}" ] && { ln -sf "$IMG_AWK" $W/bin/awk; ln -sf "$IMG_AWK" $W/bin/gawk; }
    echo "tools in PATH: $(ls $W/bin | wc -l); flock present: $(PATH=$W/bin command -v flock || echo no); python3 present: $(PATH=$W/bin command -v python3 || echo no); jq present: $(PATH=$W/bin command -v jq || echo no)"
    F='^(B-m3-tower-t[23]|DB-m3-tower-t2)$'
    happy t12a PATH=$W/bin -- ; rc=$?; echo "t12a rc=$rc"; cat $W/w-t12a/progress.txt; ls $W/w-t12a/.up.lock.d 2>&1 | head -1
    chk "t12a: rc 0, 3 units, no lock directory left" eq "$rc/$(ends0 $W/w-t12a/progress.txt | wc -l)/$(ls -d $W/w-t12a/.up.lock.d 2>/dev/null | wc -l)" 0/3/0
    echo "--- t12b: resume with PREV and an allow: argument, restricted PATH"
    runner t12b PILOT_EXTRA="$SEED" UNIT_FILTER="$F" PATH=$W/bin -- s3://b/t12a allow:B-m3-tower-t2:3; rc=$?; echo "t12b rc=$rc"; cat $W/w-t12b/progress.txt
    chk "t12b: rc 0, 3 skips" eq "$rc/$(grep -c 'done-in 0' $W/w-t12b/progress.txt)" 0/3
    mkprog() { local f=$1; shift; printf '2026-11-10T00:00:00Z %s\n' "$@" > "$f"; }   # a hand-made progress.txt
    echo "--- t12c: hand-made earlier jobs (copies of t12a with their progress.txt replaced): X1 has t2 done and t3 started, X2 has t3 started: t3 lost, exit 75; then close: runs"
    for x in X1 X2; do rm -rf $S3/$x; cp -a $S3/t12a $S3/$x; done
    mkprog $S3/X1/progress.txt "start g1" "end g1 exit 0" "start B-m3-tower-t2" "end B-m3-tower-t2 exit 0" "start B-m3-tower-t3"
    mkprog $S3/X2/progress.txt "start g1" "end g1 exit 0" "start B-m3-tower-t3"
    runner t12c PILOT_EXTRA="$SEED" UNIT_FILTER="$F" PATH=$W/bin -- s3://b/X1 s3://b/X2; rc=$?; echo "t12c rc=$rc"; cat $W/w-t12c/lost_units.txt
    chk "t12c: rc 75" eq $rc 75
    runner t12d PILOT_EXTRA="$SEED" UNIT_FILTER="$F" PATH=$W/bin -- s3://b/X1 s3://b/X2 close:B-m3-tower:3; rc=$?; echo "t12d rc=$rc"; cat $W/w-t12d/progress.txt
    chk "t12d: rc 0, t3 closed-by-rule" eq "$rc/$(grep -c 'skip B-m3-tower-t3 closed-by-rule' $W/w-t12d/progress.txt)" 0/1
    echo "--- t12e: two hand-made failures (end t3 exit 3 in Y1 and Y2): tool-failure closure"
    for x in Y1 Y2; do rm -rf $S3/$x; cp -a $S3/t12a $S3/$x; done
    mkprog $S3/Y1/progress.txt "start g1" "end g1 exit 0" "start B-m3-tower-t2" "end B-m3-tower-t2 exit 0" "start B-m3-tower-t3" "end B-m3-tower-t3 exit 3"
    mkprog $S3/Y2/progress.txt "start g1" "end g1 exit 0" "start B-m3-tower-t3" "end B-m3-tower-t3 exit 3"
    runner t12e PILOT_EXTRA="$SEED" UNIT_FILTER="$F" PATH=$W/bin -- s3://b/Y1 s3://b/Y2; rc=$?; echo "t12e rc=$rc"; cat $W/w-t12e/progress.txt; cat $W/w-t12e/ladders_closed.txt
    chk "t12e: rc 0, closed tool-failure at t=3" eq "$rc/$(grep -c 'closed B-m3-tower at t=3 tool-failure' $W/w-t12e/progress.txt)" 0/1 ;;
T13) rm -rf "$W"; mkdir -p "$W"; export SHIM_LOG=$W/ops.log
    happy j1 ISOLAB_RESULTS_URI=s3://b/jobS/ UNIT_FILTER='^(B-m3-tower-t[23]|DB-m3-tower-t2)$' -- ; rc1=$?; echo "job1 rc=$rc1"
    runner j2 PILOT_EXTRA="$SEED" ISOLAB_RESULTS_URI=s3://b/jobS2/ UNIT_FILTER='^(B-m3-tower-t[2-4]|DB-m3-tower-t2)$' -- 's3://b/jobS/'; rc2=$?; echo "job2 (PREV=s3://b/jobS/) rc=$rc2"
    cat $W/w-j2/progress.txt
    echo "--- raw URIs of job1 (first 12) and the PREV reads of job2:"; grep jobS/ $W/ops.log | head -12; grep 'jobS//\(progress\|runs\)' $W/ops.log | grep -c . ; grep 'cp s3://b/jobS//' $W/ops.log | head -12
    echo "--- shim directory layout:"; (cd $W/s3 && find . -type f | sort | head -50)
    chk "job 1 rc 0" eq $rc1 0
    chk "job 2 rc 0, 3 skips done-in 0, t4 run" eq "$rc2/$(grep -c 'done-in 0' $W/w-j2/progress.txt)/$(ends0 $W/w-j2/progress.txt | tr '\n' ' ')" "0/3/B-m3-tower-t4 "
    chk "PREV files were read with the double slash" has 'cp s3://b/jobS//host.txt' $W/ops.log
    chk "PREV argv records were read with the double slash" has 'cp s3://b/jobS//runs/B-m3-tower-t3.argv' $W/ops.log ;;
T14) rm -rf "$W"; mkdir -p "$W"
    happy j1 --; rc1=$?; echo "T2 job (list exhausted) rc=$rc1"
    echo "=== forced exit 14: a prev says ladder B-m3-tower closed at t=4 (done t4 with 'timed out; stopping') but t2, t3 were never done"
    mkdir -p $S3/X/runs; cp $S3/j1/host.txt $S3/j1/binary.sha256 $S3/X/
    printf '2026-11-10T00:00:00Z start B-m3-tower-t4\n2026-11-10T00:00:01Z end B-m3-tower-t4 exit 0\n' > $S3/X/progress.txt
    echo '-- kummer m=3 tower: every run at t = 4 timed out; stopping' > $S3/X/runs/B-m3-tower-t4.log
    echo '{"summary":{}}' > $S3/X/runs/B-m3-tower-t4.jsonl; argv_of B-m3-tower 4 > $S3/X/runs/B-m3-tower-t4.argv
    runner j3 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t[234]$' -- s3://b/X; rc=$?; echo "rc=$rc"; cat $W/w-j3/progress.txt
    chk "T2 job rc 0 with the filtered list exhausted" eq $rc1 0
    chk "forced exit 14, t2 and t3 skipped ladder-closed" eq "$rc/$(grep -c 'ladder-closed' $W/w-j3/progress.txt)" 14/2 ;;
T15) rm -rf "$W"; mkdir -p "$W"; F='^(B-m3-tower-t[2-4]|DB-m3-tower-t2)$'
    for j in A B; do
      if [ $j = A ]; then PV=(); else PV=(s3://b/A); fi
      bgrunner $j PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- "${PV[@]}"; waits3 $j B-m3-tower-t4 && killjob $j; sleep 1
    done
    cat $W/s3/b/B/progress.txt
    echo "=== job C without the rule: lost"; runner C0 PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A s3://b/B; rc0=$?; echo "rc=$rc0"
    echo "=== job C with close:B-m3-tower:4 (t4 is lost, so the rule is accepted)"; runner C PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A s3://b/B close:B-m3-tower:4; rc=$?; echo "rc=$rc"
    cat $W/w-C/progress.txt; echo "ladders_closed.txt:"; cat $W/w-C/ladders_closed.txt; echo "units_done.txt:"; cat $W/w-C/units_done.txt; grep -E "prev|closed_by_rule" $W/w-C/host.txt
    chk "C0 (no rule): rc 75" eq $rc0 75
    chk "C: rc 0, t4 closed-by-rule, DB-m3-tower-t2 run" eq "$rc/$(grep -c 'skip B-m3-tower-t4 closed-by-rule' $W/w-C/progress.txt)/$(ends0 $W/w-C/progress.txt)" 0/1/DB-m3-tower-t2
    chk "C: ladders_closed.txt has B-m3-tower t=4 by-rule" has '^B-m3-tower t=4 by-rule$' $W/w-C/ladders_closed.txt
    chk "C: host.txt closed_by_rule lists the word" has '^closed_by_rule: close:B-m3-tower:4$' $W/w-C/host.txt
    for a in close:NOPE:3 close:B-m3-tower:99 close:B-m3-tower:1 close:B-m3-tower:x; do
      runner bad PILOT_EXTRA="$SEED" -- $a; rcb=$?; echo "$a: rc=$rcb"
      chk "$a: rc 16" eq $rcb 16
    done ;;
T16) rm -rf "$W"; mkdir -p "$W"; export WRAP_LOG=$W/wrap.log; F='^B-m3-tower-t[2-4]$'
    echo "=== T16a: B-m3-tower-t3 is a tool failure in jobs A and B (exit 3 of the pilot)"
    BIN=$HERE/fail_unit_wrapper.sh runner A FAIL_UNIT=B-m3-tower-t3 PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- ; rcA=$?; echo "A rc=$rcA"
    BIN=$HERE/fail_unit_wrapper.sh runner B FAIL_UNIT=B-m3-tower-t3 PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A; rcB=$?; echo "B rc=$rcB"
    echo "=== job C (PREV A B), the failure switched OFF: the rule, not a rerun, must skip t3"
    BIN=$HERE/fail_unit_wrapper.sh runner C PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A s3://b/B; rcC=$?; echo "C rc=$rcC"; cat $W/w-C/progress.txt; cat $W/w-C/ladders_closed.txt; grep -E '^closed_tool_failure|^closed_by_rule' $W/w-C/host.txt
    echo "=== job D (PREV A B C), also without an argument"
    BIN=$HERE/fail_unit_wrapper.sh runner D PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A s3://b/B s3://b/C; rcD=$?; echo "D rc=$rcD"; cat $W/w-D/progress.txt; cat $W/w-D/ladders_closed.txt
    echo "--- wrapper calls (unit, fail|run), in order:"; cat $W/wrap.log
    chk "A and B: rc 20 each" eq "$rcA/$rcB" 20/20
    chk "C: rc 0, closed tool-failure at t=3 (log line)" eq "$rcC/$(grep -c 'closed B-m3-tower at t=3 tool-failure' $W/w-C/progress.txt)" 0/1
    chk "C: t3 and t4 skipped closed-tool-failure, t2 done-in 0" eq "$(grep -c 'closed-tool-failure' $W/w-C/progress.txt)/$(grep -c 'skip B-m3-tower-t2 done-in 0' $W/w-C/progress.txt)" 2/1
    chk "C: ladders_closed.txt = B-m3-tower t=3 tool-failure" eq "$(cat $W/w-C/ladders_closed.txt)" "B-m3-tower t=3 tool-failure"
    chk "C: host.txt closed_tool_failure: B-m3-tower:3" has '^closed_tool_failure: B-m3-tower:3$' $W/w-C/host.txt
    chk "D: rc 0, same closure, no unit started" eq "$rcD/$(cat $W/w-D/ladders_closed.txt)/$(nstart $W/w-D/progress.txt)" "0/B-m3-tower t=3 tool-failure/0"
    chk "the pilot was called for t3 exactly twice (A, B), never in C or D" eq "$(grep -c '^B-m3-tower-t3 ' $W/wrap.log)" 2
    echo "=== T16b: a unit that failed once and passes on the rerun (B-m3-tower-t2 fails in E, passes in F)"
    rm -f $W/wrap.log
    BIN=$HERE/fail_unit_wrapper.sh runner E FAIL_UNIT=B-m3-tower-t2 PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t[23]$' -- ; rcE=$?; echo "E rc=$rcE"
    BIN=$HERE/fail_unit_wrapper.sh runner F PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t[23]$' -- s3://b/E; rcF=$?; echo "F rc=$rcF"; cat $W/w-F/progress.txt; cat $W/w-F/ladders_closed.txt
    BIN=$HERE/fail_unit_wrapper.sh runner G PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t[23]$' -- s3://b/E s3://b/F; rcG=$?; echo "G rc=$rcG"; cat $W/w-G/progress.txt
    chk "E rc 20, F rc 0" eq "$rcE/$rcF" 20/0
    chk "F: t2 rerun done and t3 run (ladder continues), no closure" eq "$(ends0 $W/w-F/progress.txt | tr '\n' ' ')/$(wc -c < $W/w-F/ladders_closed.txt)" "B-m3-tower-t2 B-m3-tower-t3 /0"
    chk "G: rc 0, both skipped done-in 1" eq "$rcG/$(grep -c 'done-in 1' $W/w-G/progress.txt)" 0/2
    echo "=== T16c: exit 75 still lists every closure (stop line of a done unit, a carried rule, a tool failure)"
    mkprog() { local f=$1; shift; printf '2026-11-10T00:00:00Z %s\n' "$@" > "$f"; }   # a hand-made progress.txt
    X="PILOT_EXTRA=$SEED --budget 0"; F3='^(B-m3-tower-t2|B-m3-null-t2|DB-m3-tower-t2)$'
    runner J "$X" UNIT_FILTER="$F3" -- ; echo "J (budget 0, 3 units done, each closes its ladder) rc=$?"
    for x in X1 X2 X3; do rm -rf $S3/$x; cp -a $S3/J $S3/$x; done
    mkprog $S3/X1/progress.txt "start g1" "end g1 exit 0" "start B-m3-tower-t2" "end B-m3-tower-t2 exit 0" "start B-m3-null-t2"
    mkprog $S3/X2/progress.txt "start g1" "end g1 exit 0" "start B-m3-null-t2" "start DB-m3-tower-t2" "end DB-m3-tower-t2 exit 3"
    mkprog $S3/X3/progress.txt "start g1" "end g1 exit 0" "start DB-m3-tower-t2" "end DB-m3-tower-t2 exit 3"
    sed -i 's/^closed_by_rule:.*/closed_by_rule: close:DB-m3-tower-p0-kummer:3/' $S3/X1/host.txt
    runner K "$X" UNIT_FILTER="$F3" -- s3://b/X1 s3://b/X2 s3://b/X3; rcK=$?; echo "K (PREV X1 X2 X3) rc=$rcK"; cat $W/w-K/progress.txt; echo "lost_units.txt:"; cat $W/w-K/lost_units.txt; echo "ladders_closed.txt (S3):"; cat $S3/K/ladders_closed.txt
    chk "K: rc 75, lost B-m3-null-t2" eq "$rcK/$(cat $W/w-K/lost_units.txt)" 75/B-m3-null-t2
    chk "K: S3 ladders_closed.txt lists stop line, tool failure and carried rule" eq "$(cat $S3/K/ladders_closed.txt | tr '\n' '|')" "B-m3-tower t=2 prev 0|DB-m3-tower t=2 tool-failure|DB-m3-tower-p0-kummer t=3 by-rule|"
    chk "K: no unit started" eq "$(nstart $W/w-K/progress.txt)" 0 ;;
T17) rm -rf "$W"; mkdir -p "$W"; F='^B-m3-tower-t[23]$'
    echo "=== T17(i): the upload of the end line of t2 fails (progress upload #2), the exit handler's retry succeeds"
    runner i1 SHIM=$HERE/shim_faults.sh CNT=$W/cnt-i1 FAILN=2 SNAP_DIR=$W/snap-i1 PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- ; rc=$?; echo "rc=$rc"
    echo "local progress:"; cat $W/w-i1/progress.txt; echo "S3 progress:"; cat $S3/i1/progress.txt; echo "S3 units_done.txt:"; cat $S3/i1/units_done.txt
    echo "snapshots of what S3 held before each progress upload (n: units_done | progress last line | last line being uploaded):"
    for n in $(ls $W/snap-i1 | sed 's/\..*//' | sort -un); do echo "$n: [$(tr '\n' ' ' < $W/snap-i1/$n.units_done 2>/dev/null)] | $(tail -n1 $W/snap-i1/$n.progress 2>/dev/null) | $(cat $W/snap-i1/$n.lastline)"; done
    runner i2 PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/i1; rc2=$?; echo "next job rc=$rc2"; cat $W/w-i2/progress.txt; echo "units_done.txt of the next job:"; cat $W/w-i2/units_done.txt
    chk "rc 13, abort end-upload-failed after the end line" eq "$rc/$(grep -c 'abort B-m3-tower-t2 end-upload-failed' $W/w-i1/progress.txt)" 13/1
    chk "S3 progress has end t2 exit 0 and S3 units_done has t2" eq "$(grep -c 'end B-m3-tower-t2 exit 0' $S3/i1/progress.txt)/$(grep -cx 'B-m3-tower-t2' $S3/i1/units_done.txt)" 1/1
    chk "invariant at the end: every end-0 line in S3 has its unit in units_done" inv i1
    chk "invariant in every snapshot taken during the run" snapinv $W/snap-i1
    chk "at the failing upload (#2) S3's units_done already listed t2" has 'B-m3-tower-t2' $W/snap-i1/2.units_done
    chk "next job: t2 skipped done-in 0, t3 run, rc 0" eq "$rc2/$(grep -c 'skip B-m3-tower-t2 done-in 0' $W/w-i2/progress.txt)/$(ends0 $W/w-i2/progress.txt)" 0/1/B-m3-tower-t3
    chk "the union of the two jobs' units_done.txt has both units" eq "$(cat $S3/i1/units_done.txt $S3/i2/units_done.txt | sort | tr '\n' ' ')" "B-m3-tower-t2 B-m3-tower-t3 "
    echo "=== T17(ii): SIGTERM while the end line of t2 is being uploaded (the upload is held 4 s by the shim)"
    runner_bg i3 SHIM=$HERE/shim_faults.sh CNT=$W/cnt-i3 SNAP_DIR=$W/snap-i3 SLOW_RE='end B-m3-tower-t2 exit 0' SLOW_MARK=$W/slow-i3 SLOW_S=4 PILOT_EXTRA="$SEED" UNIT_FILTER="$F" --
    for _ in $(seq 1 600); do [ -e $W/slow-i3 ] && break; sleep 0.05; done; kill -TERM $BGPID; wait $BGPID; rc3=$?; echo "runner rc=$rc3"
    echo "S3 progress:"; cat $S3/i3/progress.txt; echo "S3 units_done.txt:"; cat $S3/i3/units_done.txt
    chk "rc 143" eq $rc3 143
    chk "S3 progress has end t2 exit 0, S3 units_done has t2, t3 never started" eq "$(grep -c 'end B-m3-tower-t2 exit 0' $S3/i3/progress.txt)/$(grep -cx 'B-m3-tower-t2' $S3/i3/units_done.txt)/$(grep -c 'start B-m3-tower-t3' $S3/i3/progress.txt)" 1/1/0
    chk "invariant at the end and in every snapshot" eq "$(inv i3 && snapinv $W/snap-i3 && echo ok)" ok ;;
T18) rm -rf "$W"; mkdir -p "$W"; F='^B-m3-tower-t[2-4]$'; X="PILOT_EXTRA=$SEED --budget 0"
    echo "=== T18a: an earlier job left out. A closes B-m3-tower at t=2 (budget 0); B (PREV A) skips; C names only B"
    runner A "$X" UNIT_FILTER="$F" -- ; echo "A rc=$?"; cat $W/w-A/ladders_closed.txt
    runner B "$X" UNIT_FILTER="$F" -- s3://b/A; echo "B rc=$?"; cat $W/w-B/progress.txt; grep '^prev' $W/w-B/host.txt
    runner C "$X" UNIT_FILTER="$F" -- s3://b/B > $W/o-C.txt 2>&1; rcC=$?; echo "C (PREV B only) rc=$rcC"; cat $W/o-C.txt; cat $W/w-C/progress.txt
    runner C2 "$X" UNIT_FILTER="$F" -- s3://b/A s3://b/B; rcC2=$?; echo "control C2 (PREV A B) rc=$rcC2"
    chk "C: exit 12, message names the left-out job, no unit started" eq "$rcC/$(grep -c 'is not among the PREVs' $W/o-C.txt)/$(nstart $W/w-C/progress.txt)" 12/1/0
    chk "control C2: rc 0" eq $rcC2 0
    echo "=== T18b: a duplicate PREV is refused with exit 16"
    runner D PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A s3://b/A > $W/o-D.txt 2>&1; rcD=$?; echo "rc=$rcD"; cat $W/o-D.txt
    chk "duplicate PREV: exit 16" eq $rcD 16
    echo "=== T18c: a unit done in two PREVs is refused with exit 12"
    happy X --; happy Y --; happy Z -- s3://b/X s3://b/Y > $W/o-Z.txt 2>&1; rcZ=$?; echo "rc=$rcZ"; cat $W/o-Z.txt; echo "unit starts: $(nstart $W/w-Z/progress.txt)"
    chk "done in two PREVs: exit 12, no unit started" eq "$rcZ/$(grep -c 'done in more than one PREV' $W/o-Z.txt)/$(nstart $W/w-Z/progress.txt)" 12/1/0 ;;
T19) rm -rf "$W"; mkdir -p "$W"; F='^(B-m3-tower-t[23]|DB-m3-tower-t2)$'
    happy P --; echo "job P (seed 777) rc=$?"
    echo "=== control: an unedited copy of P is accepted"
    happy R0 -- s3://b/P; rc=$?; echo "rc=$rc"; chk "control: rc 0" eq $rc 0
    echo "=== (a) pilot_extra differs: a genuine job with --seed 778, then a seed 777 job naming it"
    runner P778 PILOT_EXTRA="--seed 778" UNIT_FILTER="$F" -- ; echo "P778 rc=$?"
    happy R1 -- s3://b/P778 > $W/o-R1.txt 2>&1; rc=$?; echo "rc=$rc"; cat $W/o-R1.txt; chk "pilot_extra: exit 12, no unit started" eq "$rc/$(nstart $W/w-R1/progress.txt)" 12/0
    echo "=== (b) runner text differs: a genuine job run from a copy of the runner with one more comment line"
    { cat "$RUNNER"; echo "# a different file"; } > $W/alt_runner.sh
    OLDCMD=("${RUNCMD[@]}"); RUNCMD=(/bin/bash "$W/alt_runner.sh"); happy PA --; echo "PA rc=$?"; RUNCMD=("${OLDCMD[@]}")
    happy R2 -- s3://b/PA > $W/o-R2.txt 2>&1; rc=$?; echo "rc=$rc"; cat $W/o-R2.txt; chk "runner_sha256: exit 12, no unit started" eq "$rc/$(nstart $W/w-R2/progress.txt)" 12/0
    echo "=== (c) edited copies of P: testmode 0; first binary hash; the argv record of t3; no runner_sha256 line (as a revision 2 host.txt)"
    for v in testmode binhash argv nosha; do
      rm -rf $S3/Q; cp -a $S3/P $S3/Q
      case $v in testmode) sed -i 's/^testmode: 1/testmode: 0/' $S3/Q/host.txt;; binhash) sed -i '1s/^\(.\)/X\1/' $S3/Q/binary.sha256;;
        argv) sed -i 's/--seed 777/--seed 778/' $S3/Q/runs/B-m3-tower-t3.argv;; nosha) sed -i '/^runner_sha256:/d' $S3/Q/host.txt;; esac
      happy R-$v -- s3://b/Q > $W/o-$v.txt 2>&1; rc=$?; echo "--- $v: rc=$rc"; cat $W/o-$v.txt
      chk "$v: exit 12, no unit started" eq "$rc/$(nstart $W/w-R-$v/progress.txt)" 12/0
    done ;;
T20) rm -rf "$W"; mkdir -p "$W"; F='^B-m3-tower-t2$'
    echo "=== T20: the upload of the start line fails in job A and in job B (progress upload #1), the exit handler's retry succeeds"
    for j in A B; do
      if [ $j = A ]; then PV=(); else PV=(s3://b/A); fi
      runner $j SHIM=$HERE/shim_faults.sh CNT=$W/cnt-$j FAILN=1 PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- "${PV[@]}"; echo "$j rc=$?"
      echo "S3 progress of $j:"; cat $S3/$j/progress.txt; echo "files in S3 runs/: $(ls $S3/$j/runs | tr '\n' ' ')"
    done
    echo "=== job C (PREV A B)"
    runner C PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- s3://b/A s3://b/B; rcC=$?; echo "C rc=$rcC"; cat $W/w-C/progress.txt; cat $W/w-C/lost_units.txt 2>&1
    chk "A and B: S3 has start t2 then abort start-upload-failed, no end" eq "$(grep -c 'start B-m3-tower-t2' $S3/A/progress.txt)/$(grep -c 'abort B-m3-tower-t2 start-upload-failed' $S3/A/progress.txt)/$(grep -c 'end B-m3-tower-t2' $S3/A/progress.txt)" 1/1/0
    chk "C: rc 0, no lost unit, t2 run and done" eq "$rcC/$(test -e $W/w-C/lost_units.txt && echo lost || echo none)/$(ends0 $W/w-C/progress.txt)" 0/none/B-m3-tower-t2 ;;
T21) rm -rf "$W"; mkdir -p "$W"; F4='^B-m3-tower-t[2-4]$'; F3='^B-m3-tower-t[23]$'
    echo "=== T21a: close: is carried forward. A and B are killed in t3 (t2 done in A); C is given close:; D is not"
    killed_in ca B-m3-tower-t3 PILOT_EXTRA="$SEED" UNIT_FILTER="$F4" --
    killed_in cb B-m3-tower-t3 PILOT_EXTRA="$SEED" UNIT_FILTER="$F4" -- s3://b/ca
    runner c0 PILOT_EXTRA="$SEED" UNIT_FILTER="$F4" -- s3://b/ca s3://b/cb; rc0=$?; echo "C0 (no close:) rc=$rc0"
    runner cc PILOT_EXTRA="$SEED" UNIT_FILTER="$F4" -- s3://b/ca s3://b/cb close:B-m3-tower:3; rcC=$?; echo "C (close:B-m3-tower:3) rc=$rcC"; cat $W/w-cc/progress.txt; grep '^closed_by_rule' $W/w-cc/host.txt
    runner cd PILOT_EXTRA="$SEED" UNIT_FILTER="$F4" -- s3://b/ca s3://b/cb s3://b/cc; rcD=$?; echo "D (no close: argument) rc=$rcD"; cat $W/w-cd/progress.txt; cat $W/w-cd/ladders_closed.txt; grep '^closed_by_rule' $W/w-cd/host.txt
    chk "C0: rc 75" eq $rc0 75
    chk "C: rc 0, t3 and t4 closed-by-rule" eq "$rcC/$(grep -c 'closed-by-rule' $W/w-cc/progress.txt)" 0/2
    chk "D: rc 0, t3 and t4 still closed-by-rule, nothing started" eq "$rcD/$(grep -c 'closed-by-rule' $W/w-cd/progress.txt)/$(nstart $W/w-cd/progress.txt)" 0/2/0
    chk "D: ladders_closed.txt and host.txt carry the closure" eq "$(cat $W/w-cd/ladders_closed.txt)/$(grep '^closed_by_rule' $W/w-cd/host.txt)" "B-m3-tower t=3 by-rule/closed_by_rule: close:B-m3-tower:3"
    echo "=== T21b: close: without a lost unit is refused (exit 16)"
    closebad() { # closebad <why> "<PREVs>" <close word>
      runner bad PILOT_EXTRA="$SEED" UNIT_FILTER="$F4" -- $2 $3 > $W/o-bad.txt 2>&1; local rcb=$?
      echo "[$1] $3: rc=$rcb"; cat $W/o-bad.txt; echo "unit starts: $(nstart $W/w-bad/progress.txt)"
      chk "[$1] $3: exit 16, nothing started" eq "$rcb/$(nstart $W/w-bad/progress.txt)" 16/0; }
    closebad "t3 in flight once only" "s3://b/ca" close:B-m3-tower:3
    closebad "t2 is done, not lost" "s3://b/ca s3://b/cb" close:B-m3-tower:2
    closebad "t4 never started" "s3://b/ca s3://b/cb" close:B-m3-tower:4
    closebad "no PREV at all" "" close:B-m3-tower:3
    echo "=== T21c: allow:. A and B killed in t3 (in flight twice)"
    killed_in aa B-m3-tower-t3 PILOT_EXTRA="$SEED" UNIT_FILTER="$F3" --
    killed_in ab B-m3-tower-t3 PILOT_EXTRA="$SEED" UNIT_FILTER="$F3" -- s3://b/aa
    runner a0 PILOT_EXTRA="$SEED" UNIT_FILTER="$F3" -- s3://b/aa s3://b/ab; rc=$?; echo "without allow: rc=$rc"
    chk "without allow:, t3 in flight twice: exit 75" eq $rc 75
    runner a3ok PILOT_EXTRA="$SEED" UNIT_FILTER="$F3" -- s3://b/aa s3://b/ab allow:B-m3-tower-t3:3; rc=$?; echo "allow:B-m3-tower-t3:3: rc=$rc"; cat $W/w-a3ok/progress.txt; grep '^allowed' $W/w-a3ok/host.txt
    chk "allow:...:3, in flight twice: the unit runs (rc 0, end t3 exit 0)" eq "$rc/$(ends0 $W/w-a3ok/progress.txt | tr '\n' ' ')" "0/B-m3-tower-t3 "
    echo "--- chain with allow 3: C3 is given allow:3 and killed in t3 (in flight a third time); D3 has no argument"
    killed_in a3 B-m3-tower-t3 PILOT_EXTRA="$SEED" UNIT_FILTER="$F3" -- s3://b/aa s3://b/ab allow:B-m3-tower-t3:3
    runner d3 PILOT_EXTRA="$SEED" UNIT_FILTER="$F3" -- s3://b/aa s3://b/ab s3://b/a3; rc=$?; echo "D3 (allowance carried, in flight 3 times): rc=$rc"; cat $W/w-d3/lost_units.txt
    chk "in flight three times with allow 3 (carried): exit 75 again" eq "$rc/$(cat $W/w-d3/lost_units.txt)" 75/B-m3-tower-t3
    echo "--- chain with allow 4: C4 is given allow:4 and killed in t3; D4 has no argument and runs t3"
    killed_in a4 B-m3-tower-t3 PILOT_EXTRA="$SEED" UNIT_FILTER="$F3" -- s3://b/aa s3://b/ab allow:B-m3-tower-t3:4
    runner d4 PILOT_EXTRA="$SEED" UNIT_FILTER="$F3" -- s3://b/aa s3://b/ab s3://b/a4; rc=$?; echo "D4 (allow 4 carried, in flight 3 times): rc=$rc"; cat $W/w-d4/progress.txt; grep '^allowed' $W/w-d4/host.txt
    chk "allowance carried to a later job without being repeated: t3 runs (rc 0)" eq "$rc/$(ends0 $W/w-d4/progress.txt | tr '\n' ' ')/$(grep '^allowed' $W/w-d4/host.txt)" "0/B-m3-tower-t3 /allowed: allow:B-m3-tower-t3:4"
    echo "=== T21d: bad allow:/close: arguments (exit 16)"
    for a in allow:B-m3-tower-t3:2 allow:NOPE-t3:3 allow:B-m3-tower-t3:x allow:B-m3-tower-t3:3: "allow:B-m3-tower-t3:3 allow:B-m3-tower-t3:4" "close:B-m3-tower:3 close:B-m3-tower:4"; do
      runner bad PILOT_EXTRA="$SEED" UNIT_FILTER="$F3" -- s3://b/aa s3://b/ab $a > $W/o-bad.txt 2>&1; rcb=$?; echo "$a: rc=$rcb"; cat $W/o-bad.txt
      chk "$a: exit 16" eq $rcb 16
    done ;;
T22) rm -rf "$W"; mkdir -p "$W"; F='^B-m3-tower-t2$'
    echo "=== T22a: a leftover lock directory, LOCK_WAIT=3: exit 13 after the bounded waits, not a hang"
    SECONDS=0; runner l1 PRELOCK=1 LOCK_WAIT=3 PILOT_EXTRA="$SEED" UNIT_FILTER="$F" -- ; rc=$?; el=$SECONDS; echo "rc=$rc elapsed=${el}s"
    echo "local progress:"; cat $W/w-l1/progress.txt; echo "S3 progress:"; cat $S3/l1/progress.txt; ls -a $W/w-l1 | tr '\n' ' '; echo
    chk "rc 13" eq $rc 13
    chk "elapsed is 20..60 s (3 s lock wait + the exit handler's 20 s wait, then the lock is removed regardless)" test $el -ge 20 -a $el -le 60
    chk "abort start-upload-failed logged; S3 progress has it (exit handler uploaded after removing the lock)" eq "$(grep -c 'abort B-m3-tower-t2 start-upload-failed' $W/w-l1/progress.txt)/$(grep -c 'abort B-m3-tower-t2 start-upload-failed' $S3/l1/progress.txt)" 1/1
    chk "stop file created, lock directory removed" eq "$(test -e $W/w-l1/.stop && echo stop)/$(test -e $W/w-l1/.up.lock.d && echo lock || echo nolock)" stop/nolock
    chk "no pilot started" eq "$(ls $W/w-l1/runs | grep -c '\.out$')" 0
    echo "=== T22b: the lock is released by someone else after 8 s: the exit handler stops waiting then (not after 20 s)"
    SECONDS=0; runner_bg l2 PRELOCK=1 LOCK_WAIT=3 PILOT_EXTRA="$SEED" UNIT_FILTER="$F" --; ( sleep 8; rmdir $W/w-l2/.up.lock.d ) & HP=$!
    wait $BGPID; rc=$?; el=$SECONDS; wait $HP; echo "rc=$rc elapsed=${el}s"; cat $W/out-l2.txt; cat $S3/l2/progress.txt
    chk "rc 13, elapsed under 16 s" test $rc -eq 13 -a $el -lt 16
    chk "abort line reached S3" has 'abort B-m3-tower-t2 start-upload-failed' $S3/l2/progress.txt
    echo "=== T22c: the syncer. A fake sleep makes its 300 s period 2 s; B-m3-tower-t4 runs (about 70 s); .stop is created by hand after 7 s"
    mkdir -p $W/fakebin; printf '#!/bin/bash\ncase "$1" in 300) exec /bin/sleep 2;; *) exec /bin/sleep "$@";; esac\n' > $W/fakebin/sleep; chmod +x $W/fakebin/sleep
    export SHIM_LOG=$W/ops.log; : > $W/ops.log
    runner_bg l3 PATH="$W/fakebin:$PATH" PILOT_EXTRA="$SEED" UNIT_FILTER='^B-m3-tower-t4$' --
    waits3 l3 B-m3-tower-t4; sleep 7; n1=$(grep -c 'sync .*l3/runs' $W/ops.log)
    touch $W/w-l3/.stop; sleep 3; n2=$(grep -c 'sync .*l3/runs' $W/ops.log); sleep 7; n3=$(grep -c 'sync .*l3/runs' $W/ops.log)
    kill -TERM $BGPID; wait $BGPID; rc=$?; n4=$(grep -c 'sync .*l3/runs' $W/ops.log)
    echo "syncer sync-runs calls: after 7 s: $n1; stop file created, 3 s later: $n2; 7 s later: $n3; after the TERM and the exit handler: $n4; runner rc=$rc"
    chk "the syncer uploaded periodically before the stop file (>= 2 calls in 7 s)" ge $n1 2
    chk "no syncer upload after the stop file (counts equal 3 s and 10 s after it)" eq $n2 $n3
    chk "the exit handler uploaded after the TERM" lt $n3 $n4
    chk "rc 143" eq $rc 143
    unset SHIM_LOG ;;
T24) rm -rf "$W"; mkdir -p "$W/fakebin"
    printf '#!/bin/bash\ntouch %s/aws-called\n' "$W" > $W/fakebin/aws; chmod +x $W/fakebin/aws
    P1=s3://isolab-results-104186811093-us-east-2/runs/pfd-bu-1-20261110/abc/
    echo "=== outside test mode (no S3CP/S3SYNC): argument errors come before anything touches S3"
    for a in "notaprev" "s3://bucket/noslash" "s3://" "$P1 $P1" "close:B-m3-tower:3x" "allow:B-m3-tower-t3:3x" "$P1 close:B-m3-tower:1"; do
      /usr/bin/env -u S3CP -u S3SYNC PATH="$W/fakebin:$PATH" ISOLAB_RESULTS_URI=s3://b/x "${RUNCMD[@]}" $a > $W/o.txt 2>&1; rc=$?; echo "[$a]: rc=$rc"; head -2 $W/o.txt
      chk "[$a]: exit 16" eq $rc 16
    done
    chk "the fake aws was never called" test ! -e $W/aws-called
    echo "=== ISOLAB_RESULTS_URI unset or empty: exit 2 with a message"
    for v in unset empty; do
      if [ $v = unset ]; then /usr/bin/env -u ISOLAB_RESULTS_URI -u S3CP -u S3SYNC "${RUNCMD[@]}" > $W/o.txt 2>&1; rc=$?
      else /usr/bin/env -u S3CP -u S3SYNC ISOLAB_RESULTS_URI= "${RUNCMD[@]}" > $W/o.txt 2>&1; rc=$?; fi
      echo "$v: rc=$rc: $(cat $W/o.txt)"; chk "$v: exit 2" eq $rc 2
    done
    echo "=== in test mode a PREV need not match s3://, a valid-looking PREV is read (and is unreadable here: exit 12)"
    runner t1 PILOT_EXTRA="$SEED" -- "$P1" > $W/o.txt 2>&1; rc=$?; echo "rc=$rc: $(cat $W/o.txt)"; chk "unreadable PREV: exit 12" eq $rc 12
    echo "=== bash -c form (no readable \$0): refused with exit 2"
    /usr/bin/env ISOLAB_RESULTS_URI=s3://b/x S3CP="$HERE/s3shim.sh cp" S3SYNC="$HERE/s3shim.sh sync" STUDY_DIR=$W/w-bc /bin/bash -c "$(cat "$RUNNER")" stageb > $W/o.txt 2>&1; rc=$?; echo "rc=$rc: $(cat $W/o.txt)"
    chk "bash -c form: exit 2" eq $rc 2 ;;
T25) rm -rf "$W"; mkdir -p "$W"; cd "$HERE/.."
    echo "runner sha256 and size: $(sha256sum stageb_units_runner.sh | cut -d' ' -f1) $(wc -c < stageb_units_runner.sh) bytes, $(wc -l < stageb_units_runner.sh) lines"
    echo "capital Q count: $(grep -c Q stageb_units_runner.sh)"; chk "no capital Q in the runner" eq "$(grep -c Q stageb_units_runner.sh)" 0
    for f in stageb_units_runner.sh make_request_bu.sh validate_command_request.sh devcheck_r2g/test_driver.sh devcheck_r2g/run_suite.sh devcheck_r2g/s3shim.sh devcheck_r2g/shim_faults.sh devcheck_r2g/fail_unit_wrapper.sh; do
      bash -n $f; rc=$?; echo "bash -n $f: rc=$rc"; chk "bash -n $f" eq $rc 0
    done
    chk "unit list sha256 unchanged" eq "$(bash stageb_units_runner.sh --list | sha256sum | cut -d' ' -f1)" 0f80105da0a47729f6a5242a522356e0f3877f29e2ca4c0c9a8e532ca583d2b5
    echo "constructs newer than bash 4.2 or unsafe under set -u (each count must be 0):"
    for pat in '@Q' 'wait -n' 'local -n' 'declare -n' 'declare -g' '\^\^' ',,' 'mapfile' 'readarray' '\[\[ -v' '\[-[0-9]' 'coproc' '&>>' '|&' 'printf -v' 'read -N' 'EPOCH' '"\${[A-Za-z_]*\[@\]}"'; do
      c=$(grep -v '^ *#' stageb_units_runner.sh | grep -c -- "$pat"); echo "  $pat: $c"
    done
    echo "  (the last pattern lists expansions of whole arrays; each must be of a never-empty array:)"; grep -n '"\${[A-Za-z_]*\[@\]}"' stageb_units_runner.sh | grep -v '^ *[0-9]*: *#'
    chk "no forbidden construct" eq "$(grep -v '^ *#' stageb_units_runner.sh | grep -c -e '@Q' -e 'wait -n' -e 'local -n' -e 'declare -n' -e 'declare -g' -e '\^\^' -e ',,' -e 'mapfile' -e 'readarray' -e '\[\[ -v' -e '\[-[0-9]' -e 'coproc' -e '|&' -e 'printf -v')" 0
    echo "commands the runner calls (words in command position, from a trace-free scan of the text without comments):"
    grep -v '^ *#' stageb_units_runner.sh | grep -o '\b\(awk\|basename\|cat\|cp\|cut\|date\|dirname\|grep\|head\|ls\|mkdir\|mktemp\|mv\|nproc\|rm\|rmdir\|sed\|sha256sum\|sleep\|sort\|stat\|tail\|tee\|timeout\|touch\|tr\|uname\|wc\|find\|xargs\|aws\|flock\|jq\|python3\?\|seq\|comm\|diff\|cmp\|base64\|readlink\|realpath\|md5sum\|od\|xxd\|perl\|env\|printf\|kill\|wait\)\b' | sort | uniq -c | sort -rn | tr '\n' ';'; echo
    chk "no flock, jq, python, seq, comm, diff, cmp, base64 in the runner text (uncommented lines)" eq "$(grep -v '^ *#' stageb_units_runner.sh | grep -c '\b\(flock\|jq\|python3\?\|seq\|comm\|diff\|cmp\|base64\)\b')" 0 ;;
T26) rm -rf "$W"; mkdir -p "$W"; REPO=$(cd "$HERE/../../.." && pwd -P)
    jq '{image_uri: .job.image, source_revision: .provenance.source_revision}' "$HERE/../requests_r2g/pfd-b.json" > $W/image.json
    mk() { RUNNER_SHA_OVERRIDE=${SHA:-$RSHA_EXPECT} OUT_DIR_OVERRIDE=${OUT:-$W/out} "$HERE/../make_request_bu.sh" $W/image.json "$@"; }
    BK=s3://isolab-results-104186811093-us-east-2/runs/pfd-bu
    echo "=== T26a: N=1, no arguments"
    mk 1 20261110 > $W/o.txt 2>&1; rc=$?; echo "rc=$rc"; cat $W/o.txt; R=$W/out/pfd-bu-1.json
    echo "command word lengths: $(jq -c '.job.command | map(length)' $R)"; echo "BOOT: $(jq -r '.job.command[2]' $R)"
    chk "rc 0, VALID by validate_command_request.sh, prints boot_sha256" eq "$rc/$(grep -c '^VALID ' $W/o.txt)/$(grep -c '^boot_sha256: ' $W/o.txt)" 0/1/1
    chk "boot_sha256 printed = sha256 of the BOOT string in the request" eq "$(grep '^boot_sha256: ' $W/o.txt | cut -d' ' -f2)" "$(jq -j '.job.command[2]' $R | sha256sum | cut -d' ' -f1)"
    chk "every command word is 1..8192 characters without newline" eq "$(jq '[.job.command[] | (length >= 1 and length <= 8192 and (test("[\n\r]") | not))] | all' $R)" true
    chk "the runner needs 3 pieces; BOOT names 3" eq "$(jq '.job.command | length' $R)/$(jq -r '.job.command[2]' $R | grep -c 'shift 3;')" "7/1"
    chk "decoding the pieces gives the runner file byte for byte" eq "$(jq -j '.job.command[4:7] | add' $R | tr Q '\n' | cmp - "$HERE/../stageb_units_runner.sh" && echo same)" same
    echo "--- BOOT run as a job runs it: --list"
    mapfile -t CW < <(jq -r '.job.command[]' $R); /bin/bash -c "${CW[2]}" "${CW[@]:3}" --list | sha256sum
    chk "BOOT + --list gives the registered unit list" eq "$(/bin/bash -c "${CW[2]}" "${CW[@]:3}" --list | sha256sum | cut -d' ' -f1)" 0f80105da0a47729f6a5242a522356e0f3877f29e2ca4c0c9a8e532ca583d2b5
    echo "--- BOOT with one character of piece 2 changed: exit 17"
    BAD=("${CW[@]:3}"); BAD[1]="${BAD[1]:0:100}X${BAD[1]:101}"; /bin/bash -c "${CW[2]}" "${BAD[@]}" --list > $W/o17.txt 2>&1; rc17=$?; echo "rc=$rc17: $(cat $W/o17.txt)"
    chk "changed piece: exit 17 with the message, nothing run" eq "$rc17/$(grep -c 'differs from its frozen SHA-256' $W/o17.txt)/$(wc -l < $W/o17.txt)" 17/1/1
    echo "--- refuses to overwrite"; mk 1 20261110 > $W/o.txt 2>&1; rc=$?; echo "rc=$rc: $(tail -1 $W/o.txt)"; chk "second call: rc 4" eq $rc 4
    echo "=== T26b: arguments"
    P1=$BK-1-20261110/aaa/; P2=$BK-2-20261110/bbb/
    mk 3 20261110 $P1 $P2 > $W/o.txt 2>&1; rc=$?; echo "N=3 PREV1 PREV2: rc=$rc"; jq -c '.job.command[7:]' $W/out/pfd-bu-3.json
    chk "N=3 with two PREVs: rc 0 and the arguments follow the pieces" eq "$rc/$(jq -c '.job.command[7:]' $W/out/pfd-bu-3.json)" "0/[\"$P1\",\"$P2\"]"
    mk 4 20261110 $P1 skip:2:costguard $BK-3-20261110/ccc/ > $W/o.txt 2>&1; rc=$?; echo "N=4 PREV1 skip:2:costguard PREV3: rc=$rc"; jq -c '.job.command[7:]' $W/out/pfd-bu-4.json; jq -r '.provenance.note' $W/out/pfd-bu-4.json | grep -o 'jobs not given as PREV: [^.]*'
    chk "skip word is not passed to the runner and is recorded in the note" eq "$rc/$(jq '[.job.command[7:][] | select(startswith("skip"))] | length' $W/out/pfd-bu-4.json)/$(jq -r .provenance.note $W/out/pfd-bu-4.json | grep -c 'jobs not given as PREV: skip:2:costguard')" 0/0/1
    mk 2 20261110 $P1 close:B-m3-tower:3 allow:B-m3-tower-t3:3 > $W/o.txt 2>&1; rc=$?; echo "N=2 PREV1 close: allow:: rc=$rc"; jq -c '.job.command[7:]' $W/out/pfd-bu-2.json
    chk "close: and allow: words are passed to the runner in order" eq "$rc/$(jq -c '.job.command[7:]' $W/out/pfd-bu-2.json)" "0/[\"$P1\",\"close:B-m3-tower:3\",\"allow:B-m3-tower-t3:3\"]"
    for c in "3 $P1" "3 $P1 $P2 skip:1:neverran" "3 $P1 $BK-3-20261110/ccc/" "3 $P1 $P1" "3 $P1 skip:1:costguard" "3 $P1 skip:2:other" "3 skip:2:costguard skip:2:neverran" "3 $P1 skip:3:costguard" "2 s3://isolab-results-104186811093-us-east-2/runs/pfd-b-1-20261110/aaa/" "2 $BK-1-20261110/aaa" "2 $BK-1-20261110/aaa/x/" "13 $P1" "0 $P1" "2 notaprev"; do
      read -r -a ca <<< "$c"; nn=${ca[0]}
      OUT=$W/outbad mk $nn 20261110 "${ca[@]:1}" > $W/o.txt 2>&1; rc=$?; echo "[N=$nn ${ca[*]:1}]: rc=$rc: $(tail -1 $W/o.txt | cut -c1-110)"; chk "[N=$nn ${ca[*]:1}]: rejected with rc 5" eq $rc 5
    done
    chk "no request was written for the rejected calls" test -z "$(ls $W/outbad 2>/dev/null)"
    echo "=== T26c: overrides and the frozen hash"
    SHA=0000000000000000000000000000000000000000000000000000000000000000 OUT=$W/out9 mk 1 20261110 > $W/o.txt 2>&1; rc=$?; echo "wrong hash: rc=$rc: $(tail -1 $W/o.txt)"; chk "hash different from the runner's: rc 3" eq $rc 3
    RUNNER_SHA_OVERRIDE=$RSHA_EXPECT "$HERE/../make_request_bu.sh" $W/image.json 1 20261110 > $W/o.txt 2>&1; rc=$?; echo "RUNNER_SHA_OVERRIDE without OUT_DIR_OVERRIDE: rc=$rc: $(tail -1 $W/o.txt)"; chk "rc 8" eq $rc 8
    RUNNER_SHA_OVERRIDE=$RSHA_EXPECT OUT_DIR_OVERRIDE=$HERE/../requests_r2i "$HERE/../make_request_bu.sh" $W/image.json 1 20261110 > $W/o.txt 2>&1; rc=$?; echo "OUT_DIR_OVERRIDE inside the repository: rc=$rc: $(tail -1 $W/o.txt | cut -c1-120)"
    chk "inside the repository: rc 8 and no requests_r2i directory created" eq "$rc/$(test -e $HERE/../requests_r2i && echo created || echo none)" 8/none
    OUT_DIR_OVERRIDE=$W/out8 "$HERE/../make_request_bu.sh" $W/image.json 1 20261110 > $W/o.txt 2>&1; rc=$?; echo "placeholder hash, no override of it: rc=$rc: $(tail -1 $W/o.txt)"; chk "placeholder RUNNER_SHA: rc 3" eq $rc 3
    echo "=== T26d: a runner containing the letter Q is refused (private copy of the tree)"
    FT=$W/fake; mkdir -p $FT/research/pkm_falldegree_unblock_20261107 $FT/taskq; ln -s "$REPO/taskq/deploy" $FT/taskq/deploy; ln -s "$REPO/research/prime_fall_degree_20261027" $FT/research/prime_fall_degree_20261027
    cp "$HERE/../make_request_bu.sh" "$HERE/../validate_command_request.sh" $FT/research/pkm_falldegree_unblock_20261107/; { cat "$HERE/../stageb_units_runner.sh"; echo "# Q"; } > $FT/research/pkm_falldegree_unblock_20261107/stageb_units_runner.sh
    QS=$(sha256sum $FT/research/pkm_falldegree_unblock_20261107/stageb_units_runner.sh | cut -d' ' -f1)
    RUNNER_SHA_OVERRIDE=$QS OUT_DIR_OVERRIDE=$W/outq $FT/research/pkm_falldegree_unblock_20261107/make_request_bu.sh $W/image.json 1 20261110 > $W/o.txt 2>&1; rc=$?; echo "rc=$rc: $(tail -1 $W/o.txt)"
    chk "runner with a Q: rc 7, no request" eq "$rc/$(test -e $W/outq/pfd-bu-1.json && echo written || echo none)" 7/none ;;
*) echo "unknown test"; exit 2 ;;
esac
case ${1:-} in T5k) ;; *) echo "RESULT ${1:-x}: $([ $NF = 0 ] && echo PASS || echo FAIL) ($NF failed checks)";; esac
