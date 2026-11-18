#!/bin/bash
# Run the 24 registered sessions one at a time (L0, --cpus none), then verify each with --replay-all. cwd = repo root.
# usage: flock /tmp/crypto-bench.lock bash research/ic_ksum_split_20261118/run_all.sh <ecbench binary>
set -u; R=research/ic_ksum_split_20261118; BIN=${1:?ecbench binary}; export RAYON_NUM_THREADS=1
want=$(cut -d' ' -f1 $R/BINARY.sha256); have=$(sha256sum "$BIN" | cut -d' ' -f1)
[ "$want" = "$have" ] || { echo "binary sha256 $have is not the registered $want" >&2; exit 2; }
mkdir -p $R/sessions $R/data; : > $R/data/runlog.tsv
for rep in 1 2 3; do for b in P20 P22 P24 B15 B18 B21 B24 B27; do
  n=$b-r$rep; [ -f $R/sessions/$n/session.json ] && continue; rm -rf $R/sessions/$n
  s=$(date +%s); "$BIN" run --spec $R/specs/$n.json --out $R/sessions/$n --cpus none > $R/data/$n.run.log 2>&1; rc=$?
  printf '%s\t%s\t%d s\n' "$n" "$rc" $(( $(date +%s)-s )) >> $R/data/runlog.tsv
done; done
: > $R/data/verify.txt
for d in $R/sessions/*/; do n=$(basename $d); "$BIN" verify --dir $d --replay-all --exit-code --out $R/data/$n.verify.json > /dev/null 2> $R/data/$n.verify.err; echo "$n verify rc=$?" >> $R/data/verify.txt; gzip -f $R/data/$n.verify.json; done
