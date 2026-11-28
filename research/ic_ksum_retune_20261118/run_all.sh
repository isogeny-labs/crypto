#!/bin/bash
# Confirmatory: 9 registered sessions (P24, B18, B24 x 3 replicates), one at a time, L0 --cpus none, then verify --replay-all.
# usage (cwd = repo root): bash research/ic_ksum_retune_20261118/run_all.sh <ecbench> <sessions dir (scratchpad)>
set -u; R=research/ic_ksum_retune_20261118; BIN=${1:?ecbench}; OUT=${2:?sessions dir}; export RAYON_NUM_THREADS=1
[ "$(sha256sum "$BIN" | cut -d' ' -f1)" = "$(head -1 $R/BINARY.sha256 | cut -d' ' -f1)" ] || { echo "binary sha256 mismatch" >&2; exit 2; }
mkdir -p $OUT $R/data; : > $R/data/runlog.tsv
for rep in 1 2 3; do for b in P24 B18 B24; do
  n=$b-r$rep; [ -f $OUT/$n/session.json ] && grep -q '"complete"' $OUT/$n/session.json && continue; rm -rf $OUT/$n; s=$(date +%s)
  "$BIN" run --spec $R/specs/$n.json --out $OUT/$n --cpus none --wait > $OUT/$n.run.log 2>&1; rc=$?
  printf '%s\t%s\t%d s\n' $n $rc $(( $(date +%s)-s )) >> $R/data/runlog.tsv
done; done
: > $R/data/verify.txt
for rep in 1 2 3; do for b in P24 B18 B24; do n=$b-r$rep
  "$BIN" verify --dir $OUT/$n --replay-all --exit-code --out $OUT/$n.verify.json > /dev/null 2> $OUT/$n.verify.err; echo "$n verify rc=$?" >> $R/data/verify.txt
done; done
