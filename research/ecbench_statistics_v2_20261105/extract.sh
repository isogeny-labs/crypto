#!/bin/sh
# Thin orchestration (AGENTS.md "no Python": the numbers come from the two native
# ecbench binaries and jq only). Usage: extract.sh OLD_ECBENCH NEW_ECBENCH
#   OLD_ECBENCH: ecbench built at 7edc35a9fb^1 (= b58840ef4f, pre-#2442)
#   NEW_ECBENCH: ecbench built at the study's base (current main)
# Run from the repository root with the sessions' records.jsonl present.
set -eu
OLD=$1; NEW=$2
R=research/ecbench_statistics_v2_20261105; D=$R/data
mkdir -p $D/raw/old $D/raw/new
while read tag dir a b; do
  $OLD compare --dir $dir --a $a --b $b --json > $D/raw/old/$tag.$a.$b.json
  $NEW compare --dir $dir --a $a --b $b --json > $D/raw/new/$tag.$a.$b.json
done < $D/pairs.txt
printf 'tag\ta\tb\told_ratio\told_lo\told_hi\told_ci_method\told_pairs\told_unpaired\told_wall_median_ratio\told_wall_lo\told_wall_hi\tnew_ratio\tnew_lo\tnew_hi\tnew_ci_method\tnew_dof\tnew_indicative\tboot_lo\tboot_hi\tcensored_pairs\tbound\tverified_only_ratio\tband_order\ta_heavy\ta_top_share\tb_heavy\tb_top_share\tb_verified\tb_measured\tnew_wall_status\tnew_wall_ratio\tnew_wall_lo\tnew_wall_hi\tcorrected_ratio\toutside_noise\tlegacy_outside_noise\tns_per_gae_a\tns_per_gae_b\ttotal_wall_ratio\ttotal_wall_lo\ttotal_wall_hi\tadmissible\n' > $D/comparisons.tsv
while read tag dir a b; do
  o=$D/raw/old/$tag.$a.$b.json; n=$D/raw/new/$tag.$a.$b.json
  jq -r --slurpfile o $o --arg tag $tag --arg a $a --arg b $b '
    $o[0] as $O | [ $tag,$a,$b,
      $O.ops.ratio_b_over_a, $O.ops.ci95[0], $O.ops.ci95[1], $O.ops.ci_method, $O.ops.pairs, $O.ops.unpaired_runs,
      $O.wall.median_ratio, $O.wall.ci95[0], $O.wall.ci95[1],
      .ops.ratio_b_over_a, .ops.ci95[0], .ops.ci95[1], .ops.ci_method, .ops.dof, .ops.indicative,
      .ops.ci95_bootstrap[0], .ops.ci95_bootstrap[1], .ops.censored_pairs, .ops.bound, .ops.verified_only_ratio, .ops.band.order,
      .a.tail.heavy_tail, .a.tail.top_run_share, .b.tail.heavy_tail, .b.tail.top_run_share, .b.verified, .b.measured,
      .wall.status, .wall.ratio_b_over_a, .wall.ci95[0]?, .wall.ci95[1]?, .wall.corrected_ratio, .wall.outside_noise, .wall.legacy_outside_noise,
      .wall.ns_per_gae_a, .wall.ns_per_gae_b, .wall.total_wall_ratio, .wall.total_wall_ci95[0]?, .wall.total_wall_ci95[1]?, .admissible ] | map(if . == null then "NA" else . end) | @tsv' $n >> $D/comparisons.tsv
done < $D/pairs.txt
# Per-(arm, curve) tables of every session used (old and new binaries).
mkdir -p $D/tables
for dir in $(awk '{print $1" "$2}' $D/pairs.txt | sort -u | awk '{print $2}'); do
  n=$(echo $dir | sed 's|research/||; s|/sessions||; s|/|_|g')
  $OLD table --dir $dir > $D/tables/old.$n.md
  $NEW table --dir $dir > $D/tables/new.$n.md
done
