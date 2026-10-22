#!/bin/bash
# The follow-up runs of PROTOCOL.md section 12.3. From anywhere:
#   BIN=/path/to/fixed/pkm_tower_pilot BASE=/path/to/baseline/pkm_tower_pilot \
#     research/pkm_p224_kummer_shape_20261022/followup_20261022/run_followup.sh E1|E2|E3|R20c
# Each run writes runs/<id>.jsonl (rows), runs/<id>.log (stderr) and, for the fixed
# binary, runs/<id>.steps.jsonl; runs/<id>.rss is the process's resident set sampled
# every 5 s from /proc (the same instrument for every arm). progress.txt records start
# and end with the exit status of the run itself.
set -u
here=$(cd "$(dirname "$0")" && pwd)
bin=${BIN:?set BIN to the fixed pkm_tower_pilot}
cd "$here/runs"
SL20=1032193; SL24=16760833; SEED=1347112276
common=(--engine generic --ladder-t none --planted 0 --random 2 --seed $SEED --kinds kummer --controls tower)

# run ID BINARY THREADS BUDGET [env VAR=val ...] -- flags...
run() {
  id=$1; exe=$2; threads=$3; budget=$4; shift 4
  envs=()
  while [ "$1" != "--" ]; do envs+=("$1"); shift; done
  shift
  echo "$(date -u +%FT%TZ) start $id: threads=$threads ${envs[*]} $*" >> ../progress.txt
  env RAYON_NUM_THREADS=$threads "${envs[@]}" nice -n 10 "$exe" "${common[@]}" --budget "$budget" "$@" \
    --out "$id.jsonl" 2> "$id.log" &
  pid=$!
  ( while kill -0 $pid 2>/dev/null; do
      echo "$(date +%s) $(awk '/VmRSS/{print $2}' /proc/$pid/status 2>/dev/null)" >> "$id.rss"
      sleep 5
    done ) &
  sampler=$!
  wait $pid
  rc=$?
  kill $sampler 2>/dev/null
  echo "$(date -u +%FT%TZ) end $id exit $rc" >> ../progress.txt
}
free_mb() { awk '/MemAvailable/{print int($2/1024)}' /proc/meminfo; }

case ${1:?E1, E2, E3 or R20c} in
  E1)
    ulimit -v 4000000
    run R20a-SL20-m2-N16-20 $bin 1 1800 -- --p $SL20 --m 2 --t-min 8 --max-t 10
    run R20b-SL24-m2-N20 $bin 1 7200 -- --p $SL24 --m 2 --t-min 10 --max-t 10 --targets 0
    ;;
  E2)
    base=${BASE:?set BASE to the baseline pkm_tower_pilot}
    for t in 10 11; do
      n=$((2 * t))
      run R20e-N$n-base $base 2 7200 -- --p $SL24 --m 2 --t-min $t --max-t $t --targets 0 --trace
      run R20e-N$n-fixed $bin 2 7200 -- --p $SL24 --m 2 --t-min $t --max-t $t --targets 0 --steps-out R20e-N$n-fixed.steps.jsonl
      if [ $n = 20 ]; then
        run R20e-N$n-fixed-arena2 $bin 2 7200 MALLOC_ARENA_MAX=2 -- --p $SL24 --m 2 --t-min $t --max-t $t --targets 0 --steps-out R20e-N$n-fixed-arena2.steps.jsonl
      fi
    done
    ;;
  E3)
    run R20c0-SL24-m2-N24-steps1-25 $bin 2 7200 -- --p $SL24 --m 2 --t-min 12 --max-t 12 --targets 0 \
      --max-nnz 600000000 --steps-out R20c0-SL24-m2-N24.steps.jsonl
    ;;
  R20c)
    avail=$(free_mb)
    echo "$(date -u +%FT%TZ) R20c host MemAvailable ${avail} MB" >> ../progress.txt
    if [ "$avail" -lt 10240 ]; then
      echo "$(date -u +%FT%TZ) not run (MemAvailable ${avail} MB < 10240) R20c" >> ../progress.txt
      exit 0
    fi
    run R20c-SL24-m2-N24 $bin 4 18000 MALLOC_ARENA_MAX=2 -- --p $SL24 --m 2 --t-min 12 --max-t 12 --targets 0 \
      --max-mem-mb 7800 --steps-out R20c-SL24-m2-N24.steps.jsonl
    ;;
esac
