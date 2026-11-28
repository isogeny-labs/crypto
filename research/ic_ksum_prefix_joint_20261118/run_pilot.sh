#!/bin/bash
# Pilot (not data): 8 sessions, one per row, L0 --cpus none, then verify --replay-all.
# usage (cwd = repo root): bash research/ic_ksum_prefix_joint_20261118/run_pilot.sh <ecbench> <sessions dir (scratchpad)>
set -u; R=research/ic_ksum_prefix_joint_20261118; BIN=${1:?ecbench}; OUT=${2:?sessions dir}; export RAYON_NUM_THREADS=1
[ "$(sha256sum "$BIN" | cut -d' ' -f1)" = "$(head -1 $R/BINARY.sha256 | cut -d' ' -f1)" ] || { echo "binary sha256 mismatch" >&2; exit 2; }
mkdir -p $OUT $R/pilot; : > $R/pilot/runlog.tsv; : > $R/pilot/verify.txt
for b in P20 P22 P24 B15 B18 B21 B24 B27; do
  rm -rf $OUT/$b; s=$(date +%s)
  "$BIN" run --spec $R/pilot/specs/$b.json --out $OUT/$b --cpus none --wait > $OUT/$b.run.log 2>&1; rc=$?
  printf '%s\t%s\t%d s\n' $b $rc $(( $(date +%s)-s )) >> $R/pilot/runlog.tsv
  "$BIN" verify --dir $OUT/$b --replay-all --exit-code --out $OUT/$b.verify.json > /dev/null 2> $OUT/$b.verify.err; echo "$b verify rc=$?" >> $R/pilot/verify.txt
done
