#!/bin/bash
# make_tables.sh : per-chunk band JSON (frozen ecbench) -> analysis/perchunk.tsv, analysis/summary.tsv. jq + awk only.
# usage: scripts/make_tables.sh   (reads /home/user/scratch_S2/k*-c*-a<last complete>)
cd "$(dirname "$0")/.."
B=/home/user/frozen-bin/ecbench_scale
mkdir -p analysis/band_perchunk
: > analysis/perchunk.tsv
for d in $(ls -d /home/user/scratch_S2/k*-c*-a* | sort); do
  grep -q '"complete"' $d/session.json || continue
  n=$(basename $d); k=${n%%-*}; c=${n#*-}; c=${c%%-*}
  $B band --dir $d --json analysis/band_perchunk/$n.json >/dev/null 2>&1
  jq -r --arg k "${k#k}" --arg c "${c#c}" '.log2_r as $x | .rows[] | [$k,$c,.arm,.verified,.runs,.log2_r,.mean_s[0],.mean_s[1],.mean_s[2]]|@tsv' analysis/band_perchunk/$n.json 2>/dev/null >> analysis/perchunk.tsv ||
  jq -r --arg k "${k#k}" --arg c "${c#c}" '.rows[] | [$k,$c,.arm,.verified,.runs,.log2_r,.mean_s[0],.mean_s[1],.mean_s[2]]|@tsv' analysis/band_perchunk/$n.json >> analysis/perchunk.tsv
done
