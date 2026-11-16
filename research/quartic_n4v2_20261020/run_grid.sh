#!/opt/homebrew/bin/bash
# N4 v2 grid driver. usage: run_grid.sh <v2 binary> <n4 binary> <outdir> <PARAMS file (sourced)>
# 4 parallel single-thread processes under nice -n 10, largest jobs first; B0 at k = 21 runs alone
# in a memory lane; every job checks free disk first and the grid stops below 5 GB.
set -u
BIN=$1; N4=$2; OUT=$3; PARAMS=$4
. "$PARAMS"   # defines C_S C_D GAMMA BLOCK RHO_RUNS (assoc q -> runs per instance) B0_MAXQ SEEDS_*
mkdir -p "$OUT/units" "$OUT/locks"
jobs=()
# est: rough CPU-second weight for ordering only
for q in $QLIST; do
  ns=${NSEEDS[$q]}
  for sd in $(seq 1 $ns); do
    jobs+=("${EST_IC[$q]} ic_${q}_${sd}_SD ic $q $sd S,D $C_S $C_D $GAMMA $BLOCK")
    if [ "$q" -le "$B0_MAXQ" ]; then
      w=${EST_B0[$q]}; if [ "$q" = "$B0_HEAVY_Q" ]; then w=0; fi   # memory lane runs last
      jobs+=("$w ic_${q}_${sd}_B0 ic $q $sd B0"); fi
    nr=${RHO_RUNS[$q]}
    if [ "$sd" -gt "${RHO_SEEDS[$q]}" ]; then nr=0; fi
    if [ "$nr" -gt 0 ]; then
      if [ "${RHO_SPLIT[$q]}" = 1 ]; then
        for i in $(seq 0 $((nr - 1))); do
          jobs+=("${EST_RHO[$q]} rho_${q}_${sd}_${i}_plain rho $q $sd $i $((i + 1)) plain")
          jobs+=("${EST_RHO[$q]} rho_${q}_${sd}_${i}_neg rho $q $sd $i $((i + 1)) neg")
        done
      else
        jobs+=("$(( ${EST_RHO[$q]} * nr )) rho_${q}_${sd}_all rho $q $sd 0 $nr plain,neg")
      fi
    fi
  done
  for c in 1 2 3; do jobs+=("1 calib_${q}_$c calib $q $c"); done
  jobs+=("2 check_${q}_1 N4CHECK $q 1")
  # replay controls (arm c): seed 1 again for S,D at every q in REPLAY_Q; B0 replay where B0 runs
  for rq in $REPLAY_Q; do
    if [ "$rq" = "$q" ]; then
      jobs+=("${EST_IC[$q]} replay_${q}_1_SD ic $q 1 S,D $C_S $C_D $GAMMA $BLOCK")
      if [ "$q" -le "$REPLAY_B0_MAXQ" ]; then jobs+=("${EST_B0[$q]} replay_${q}_1_B0 ic $q 1 B0"); fi
    fi
  done
done
printf '%s\n' "${jobs[@]}" | sort -k1 -n -r > "$OUT/joblist.txt"
cut -d' ' -f2- "$OUT/joblist.txt" | xargs -P 4 -L 1 bash -c '
  tag=$0; shift 0
  OUT='"$OUT"'; BIN='"$BIN"'; B0HEAVY='"$B0_HEAVY_Q"'
  if [ -e "$OUT/STOP" ]; then echo "skip $tag (STOP)" >> "$OUT/grid.log"; exit 0; fi
  free=$(df -g /System/Volumes/Data | awk "NR==2{print \$4}")
  if [ "$free" -lt 5 ]; then echo "STOP: free disk ${free} GB < 5 at $tag" >> "$OUT/grid.log"; touch "$OUT/STOP"; exit 0; fi
  if [ -s "$OUT/units/$tag.jsonl" ] && grep -q "^ok" "$OUT/units/$tag.done" 2>/dev/null; then exit 0; fi
  heavy=0
  case "$tag" in ic_${B0HEAVY}_*_B0|replay_${B0HEAVY}_*_B0) heavy=1;; esac
  if [ $heavy = 1 ]; then while ! mkdir "$OUT/locks/mem" 2>/dev/null; do sleep 20; done; fi
  echo "$(date -u +%FT%TZ) start $tag" >> "$OUT/grid.log"
  if [ "$1" = N4CHECK ]; then shift; set -- check "$@"; RUN='"$N4"'; else RUN=$BIN; fi
  /usr/bin/time -l nice -n 10 $RUN "$@" > "$OUT/units/$tag.jsonl" 2> "$OUT/units/$tag.err"
  rc=$?
  if [ $heavy = 1 ]; then rmdir "$OUT/locks/mem"; fi
  echo "ok rc=$rc" > "$OUT/units/$tag.done"
  echo "$(date -u +%FT%TZ) end $tag rc=$rc" >> "$OUT/grid.log"
'
echo "grid done $(date -u +%FT%TZ)" >> "$OUT/grid.log"
