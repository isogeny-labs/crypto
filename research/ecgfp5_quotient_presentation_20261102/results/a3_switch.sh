#!/usr/bin/env bash
# A3 order change: stop the A1 queue at the end of U2_4111, run U5 at the three primes, then U1P_4111.
cd "$(dirname "$0")/.."
Q=$(pgrep -f "bash run_a1.sh U1F_262231" | head -1)
while [ ! -f results/A1/U2_4111/finished_at.txt ]; do sleep 5; done
kill -STOP "$Q"; pkill -P "$Q"; kill -9 "$Q"; sleep 2
date -u +"%FT%TZ A1 queue (pid $Q) stopped after U2_4111"
for d in results/A1/*; do b=$(basename "$d"); case "$b" in U1P_4111) mv "$d" "results/A1/_aborted_$b"; echo "moved $b unrun";; esac; done
bash run_a3.sh U5_4111 U5_262231 U5_33554371
bash run_a1.sh U1P_4111
date -u +"%FT%TZ queue done"
