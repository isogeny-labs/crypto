#!/usr/bin/env bash
# P7/P8 measurement: one process at a time, single thread.  usage: run_measure.sh <v3 binary> <raw dir>
set -u
B=$1; O=$2; mkdir -p "$O"
echo "$(date -u +%FT%TZ) start load=$(cut -d' ' -f1 /proc/loadavg)" >> "$O/run.log"
nice -n 5 "$B" instances 98299:3 196579:3 393187:3 > "$O/instances.jsonl" 2> "$O/instances.err"
for q in 98299 196579 393187; do
  for s in 1 2 3; do
    nice -n 5 "$B" ic $q $s D 3.0 1.7 > "$O/ic_D_${q}_${s}.jsonl" 2> "$O/ic_D_${q}_${s}.err"
    nice -n 5 "$B" ic $q $s B0 > "$O/ic_B0_${q}_${s}.jsonl" 2> "$O/ic_B0_${q}_${s}.err"
    nice -n 5 "$B" rho $q $s 0 4 plain,neg > "$O/rho_${q}_${s}.jsonl" 2> "$O/rho_${q}_${s}.err"
    echo "$(date -u +%FT%TZ) done q=$q seed=$s" >> "$O/run.log"
  done
done
echo "$(date -u +%FT%TZ) end" >> "$O/run.log"
(cd "$O" && sha256sum *.jsonl > SHA256SUMS)
