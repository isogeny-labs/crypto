#!/usr/bin/env bash
# Amendment 3: the cubic arm alone, same flags as pod_run.sh.
cd /work
BIN=./pkm_tower_pilot
echo "$(date -u +%FT%TZ) start m4t4-cubic (alone, Amendment 3)" >> runs/progress.txt
$BIN --p 2013265921 --engine f4 --kinds kummer --budget 28800 --ladder-t none --planted 1 --random 0 \
  --controls tower --m 4 --t-min 4 --max-t-m4 4 --cover-map cubic --out runs/m4t4-cubic.jsonl > runs/m4t4-cubic.log 2>&1
rc=$?
echo "$(date -u +%FT%TZ) end m4t4-cubic (alone) exit=$rc" >> runs/progress.txt
