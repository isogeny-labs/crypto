#!/usr/bin/env bash
# Runs on the RunPod pod (cwd = pod work dir), binary at ./jv_cover (sha256 checked by rp.sh put).
# Part 1 (replay): the frozen command of research/jv_nagao_hilbert_20261112/run.sh, unchanged flags,
#   aa_line_p1301_s1 + 9 hilbert instances, sequential, default rayon threads (all pod vCPUs).
# Part 2 (matched rho): the frozen command of research/jv_cover_negrho_rescore_20261101/run.sh,
#   run indices 0..13 on the 9 instances (126 runs), single-threaded jobs, 30 in parallel.
set -u
B=./jv_cover; R=replay; Q=rho
mkdir -p $R $Q
{ echo "host $(lscpu | sed -n 's/^Model name: *//p') nproc $(nproc) mem $(free -g | awk '/Mem:/{print $2}') GB"; uname -a; sha256sum $B; echo "T0 $(date -u +%FT%TZ)"; } > host.txt
run() { label=$1; shift; s0=$(date +%s)
  env JV_FB_LINE=65536 timeout 3600 $B "$@" --json $R/$label.json.tmp > $R/$label.log 2>&1; rc=$?
  [ $rc = 0 ] && mv $R/$label.json.tmp $R/$label.json
  echo "$label rc=$rc wall=$(( $(date +%s)-s0 ))s" >> $R/walls.txt; }
COMMON=(--exp dlp --rho-runs 0 --rho-ref 1.3 --check-every 64)
run aa_line_p1301_s1 "${COMMON[@]}" --sizes 1301 --first-seed 1 --seeds 1
for p in 1301 1451 1601; do for s in 1 2 3; do
  run hilbert_p${p}_s$s "${COMMON[@]}" --stop hilbert:16 --sizes $p --first-seed $s --seeds 1
done; done
echo "REPLAY-DONE $(date -u +%FT%TZ)" >> $R/walls.txt
export B Q; export T0=$(date +%s)
job() { local p=$1 s=$2 r=$3 a=$(date +%s); local L=rhostrong_p${p}_s${s}_run${r}
  RAYON_NUM_THREADS=1 timeout 5400 $B --exp rhostrong --sizes $p --first-seed $s --seeds 1 --neg 1 --batch 16 --r 256 --dp-bits 16 --run $r --json $Q/$L.json > $Q/$L.log 2>&1
  echo "$L exit $? wall_s $(( $(date +%s) - a )) at_s $(( $(date +%s) - T0 ))" >> $Q/walls.txt; }
export -f job
for r in $(seq 0 13); do for p in 1601 1451 1301; do for s in 1 2 3; do echo "$p $s $r"; done; done; done \
  | xargs -P30 -L1 bash -c 'job $0 $1 $2'
echo "ALL DONE total_s $(( $(date +%s) - T0 ))" >> $Q/walls.txt
tar czf out.tgz host.txt $R $Q
echo PACKED > packed.txt
