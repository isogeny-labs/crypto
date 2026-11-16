#!/bin/bash
# Run the 9 registered sessions (specs/P<bits>-r<rep>.json), then verify each with --replay-all.
set -u; cd /work; mkdir -p sess; export RAYON_NUM_THREADS=1
echo "start $(date -u +%FT%TZ) load $(cat /proc/loadavg)" >> runlog.tsv
for rep in 1 2 3; do for b in 20 22 24; do
  n=P$b-r$rep; [ -f sess/$n/session.json ] && continue; rm -rf sess/$n
  s=$(date +%s%N); bin/ecbench run --spec specs/$n.json --out sess/$n --cpus inherit --allow-busy > sess/$n.log 2>&1; rc=$?; e=$(date +%s%N)
  printf '%s\t%s\t%d\t%s\n' "$n" "$rc" $(( (e-s)/1000000 )) "$(cut -d' ' -f1 /proc/loadavg)" >> runlog.tsv
done; done
echo "done $(date -u +%FT%TZ)" >> runlog.tsv
for d in sess/P*/; do n=$(basename $d); bin/ecbench verify --dir $d --replay-all --exit-code > sess/$n.verify.json 2>&1; echo "$n verify rc=$?" >> verify.txt; done
echo verify-done >> verify.txt
