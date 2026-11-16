#!/bin/bash
# Independent replay of run-2 sessions on a second host (this container), each session with the
# binary that produced it (fetched from the pod, sha256-checked against bin/SHA256SUMS).
S=/tmp/claude-0/-home-user/6c4d3551-cc8a-5111-aeb2-cdafa56866cd/scratchpad/rp
R=/home/user/crypto/research/wallfix_runpod_20261114
export RAYON_NUM_THREADS=1
one() { n=$1; sha=$(awk -F'\t' -v n="$n" '"pr"$1"-"$2"-r"$3"-"$4==n {print $5}' $S/ab/runlog.tsv | tail -1)
  $S/bin/ecbench-$sha verify --dir $R/sessions/run2/$n --replay-all --exit-code --out $R/verify-local/$n.json > /dev/null 2>$R/verify-local/$n.err
  echo "$n $sha rc=$?"; }
export -f one; export S R
ls $R/sessions/run2 | grep -- '-[AB]$' | xargs -P 4 -I{} bash -c 'one {}' | sort > $R/verify-local/summary.txt
echo done >> $R/verify-local/summary.txt
