#!/usr/bin/env bash
# A2 order change: stop the A1 queue at the end of A0_262231, then U4_4111, then the rest of A1.
cd "$(dirname "$0")/.."
while [ ! -f results/A1/A0_262231/finished_at.txt ]; do sleep 5; done
kill -STOP 3415; pkill -P 3415; kill -9 3415; sleep 2
date -u +"%FT%TZ A1 queue (pid 3415) stopped after A0_262231"
for d in results/A1/*; do b=$(basename "$d"); case "$b" in U3_4111|A0_262231|_aborted_*) ;; *) mv "$d" "results/A1/_aborted_$b"; echo "moved $b unrun";; esac; done
bash run_a2.sh U4_4111
bash run_a1.sh U1F_262231 A0_33554371 U1F_33554371 U2_4111 U1P_4111
date -u +"%FT%TZ queue done"
