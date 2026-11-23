#!/bin/bash
# run.sh: session-level ABAB with counterbalancing (odd rounds A first, even rounds B first),
# 3 rounds, every (PR, spec) pair interleaved inside each round. Same pod, same spec, same seeds.
set -u; cd /work; mkdir -p sess; export RAYON_NUM_THREADS=1
pairs="3850:5382b439bc:43e68a1d56:prime 3850:5382b439bc:43e68a1d56:koblitz 3850:5382b439bc:43e68a1d56:tensor 3991:6745ba0a5f:94c682e021:prime 4009:2c1dc30e16:21f21a45a3:koblitz 4009:2c1dc30e16:21f21a45a3:tensor"
R=${R:-3}
echo "start $(date -u +%FT%TZ) load $(cat /proc/loadavg)" >> runlog.tsv
for r in $(seq 1 $R); do
  for p in $pairs; do IFS=: read pr a b spec <<< "$p"
    order="A B"; [ $((r % 2)) -eq 0 ] && order="B A"
    for arm in $order; do
      sha=$([ $arm = A ] && echo $a || echo $b); out=sess/pr$pr-$spec-r$r-$arm
      [ -f $out/session.json ] && continue; rm -rf $out
      s=$(date +%s%N); bin/ecbench-$sha run --spec spec-$spec.json --out $out --cpus inherit --allow-busy > $out.log 2>&1; rc=$?; e=$(date +%s%N)
      printf '%s\t%s\t%s\t%s\t%s\t%s\t%d\t%s\n' "$pr" "$spec" "$r" "$arm" "$sha" "$rc" $(( (e-s)/1000000 )) "$(cut -d' ' -f1 /proc/loadavg)" >> runlog.tsv
    done
  done
done
echo "done $(date -u +%FT%TZ)" >> runlog.tsv
for d in sess/*/; do n=$(basename $d); sha=$(echo $n | awk -F- '{print $NF}'); bin=$(awk -F'\t' -v n="$n" '{ if ("sess/pr"$1"-"$2"-r"$3"-"$4 == "sess/"n) print $5 }' runlog.tsv | tail -1)
  bin/ecbench-$bin verify --dir $d --replay-all --exit-code > $d/../$n.verify.json 2>&1; echo "$n verify rc=$?" >> verify.txt; done
echo verify-done >> verify.txt
