#!/bin/sh
# Turns the recorded-value summaries (summary.json, ratios.json, fraction_below.jsonl,
# counts_diff.jsonl, audits/*.txt, audits/band-*.json) into table rows. jq/awk formatting only.
set -eu
R=research/ecbench_allocator_confound_20261023; A=$R/analysis; T=$A/tables
f() { echo "$1"; }
# T1 smoke: arm, n base, n fix, base med ms, fix med ms, base/fix, share of fix runs below base median, ns/GAE base, fix
jq -r --slurpfile fb "$A/fraction_below.jsonl" '. as $s | ["rho-neg","rho-plain","rho-frozen","rho-neg-aa","bsgs","bsgs-il","bsgs-neg","kangaroo","ic-mitm"][] | . as $a
  | ($s["smoke-base"].arms[$a]) as $b | ($s["smoke-fix"].arms[$a]) as $x
  | [$a, $b.solve_wall_ns.n, $x.solve_wall_ns.n, ($b.solve_wall_ns.median/1e6*1000|round/1000), ($x.solve_wall_ns.median/1e6*1000|round/1000),
     ($b.solve_wall_ns.median/$x.solve_wall_ns.median*100|round/100),
     (($fb[] | select(.arm==$a and .metric=="solve" and .n_a==12) | .fraction*100|round)|tostring+"%"),
     ($b.ns_per_gae.median|round), ($x.ns_per_gae.median|round)] | @tsv' $A/summary.json > $T/t1_smoke.tsv
# T2 claim: arm, metric, n base, n fix, base med us, fix med us, pooled ratio, ratio order 1, ratio order 2, share fix below base median
jq -r --slurpfile fb "$A/fraction_below.jsonl" --slurpfile r "$A/ratios.json" '. as $s | $r[0] as $r | ["ic","rho","rho-strong"][] | . as $a | ("solve_wall_ns","online_wall_ns") as $m
  | ($s["claim-base"].arms[$a][$m]) as $b | ($s["claim-fix"].arms[$a][$m]) as $x
  | [$a, ($m|sub("_wall_ns";"")), $b.n, $x.n, ($b.median/1000*10|round/10), ($x.median/1000*10|round/10),
     ($b.median/$x.median*1000|round/1000),
     ($r["base-claim__fix-claim"][$a][$m].ratio_a_over_b*1000|round/1000), ($r["base-claim-rev__fix-claim-rev"][$a][$m].ratio_a_over_b*1000|round/1000),
     (($fb[] | select(.arm==$a and .metric==($m|sub("_wall_ns";"")) and .n_a==40) | .fraction*100|round)|tostring+"%")] | @tsv' $A/summary.json > $T/t2_claim.tsv
# T3 ablations (smoke, non-IC arms): arm, base, fix, notrim, noprobe, notrim2 medians in ms
jq -r '. as $s | ["rho-neg","rho-plain","rho-frozen","rho-neg-aa","bsgs","bsgs-il","bsgs-neg","kangaroo","ic-mitm"][] | . as $a
  | [$a] + (["smoke-base","smoke-fix","smoke-notrim","smoke-noprobe","smoke-notrim2"] | map(. as $g | ($s[$g].arms[$a].solve_wall_ns.median/1e6*1000|round/1000)))
  | @tsv' $A/summary.json > $T/t3_ablation.tsv
# T3b claim ablation: arm, metric, medians us
jq -r '. as $s | ["ic","rho","rho-strong"][] | . as $a | ("solve_wall_ns","online_wall_ns") as $m
  | [$a, ($m|sub("_wall_ns";""))] + (["claim-base","claim-fix","claim-notrim","claim-noprobe","claim-notrim2"] | map(. as $g | ($s[$g].arms[$a][$m].median/1000*10|round/10))) | @tsv' $A/summary.json > $T/t3b_claim_ablation.tsv
# T4 counts
jq -r '[.a,.b,.matched,.differing,(.only_in_a|length)+(.only_in_b|length),(if .identical then "identical" else "DIFFERENT" end)]|@tsv' $A/counts_diff.jsonl > $T/t4_counts.tsv
# T5 audits
for s in $(ls $R/sessions); do echo "$s	$(tail -1 $R/audits/$s.audit.txt | sed 's/^audit //')	$(jq -r '.session_id // empty' $R/sessions/$s/session.json)"; done > $T/t5_audits.tsv
# T6 isolation levels per pooled group
jq -r 'to_entries[] | select(.key|test("^(smoke|claim)-")) | . as $e | [$e.key, ([$e.value.arms[].isolation_levels] | reduce .[] as $x ({}; reduce ($x|to_entries[]) as $q (.; .[$q.key] += $q.value)) | to_entries | map("\(.key)x\(.value)") | join(" "))] | @tsv' $A/summary.json > $T/t6_levels.tsv
# T7 band (fix-claim): arm, S at 0, 0.1, 1
jq -r '.rows[] | [.arm, (.mean_s[0]*1000|round/1000), (.mean_s[1]*1000|round/1000), (.mean_s[2]*1000|round/1000)] | @tsv' $R/audits/band-fix-claim.json > $T/t7_band.tsv
# T8 A/A: group, rho-neg median ms, rho-neg-aa median ms, ratio
jq -r '. as $s | ["smoke-base","smoke-fix","smoke-notrim","smoke-noprobe","smoke-notrim2"][] | . as $g | [$g, ($s[$g].arms["rho-neg"].solve_wall_ns.median/1e6*1000|round/1000), ($s[$g].arms["rho-neg-aa"].solve_wall_ns.median/1e6*1000|round/1000), ($s[$g].arms["rho-neg-aa"].solve_wall_ns.median/$s[$g].arms["rho-neg"].solve_wall_ns.median*1000|round/1000)] | @tsv' $A/summary.json > $T/t8_aa.tsv
# T9 allocator block: group, records, pre_solve_trim true, absent, trim_result values
for g in base fix notrim noprobe notrim2; do for k in smoke claim; do echo "$k-$g"; done; done > /dev/null
jq -r 'to_entries[] | select(.key|test("^(smoke|claim)-")) | [.key, ([.value.arms[].records]|add), ([.value.arms[].pre_solve_trim_true]|add), ([.value.arms[].pre_solve_trim_absent]|add)] | @tsv' $A/summary.json > $T/t9_alloc.tsv
# T10 session inventory
for s in $(ls $R/sessions); do jq -r --arg n $s '[$n, .session_id, (.started_unix_ms/1000|strftime("%H:%M:%S")), ((.finished_unix_ms-.started_unix_ms)/1000|round|tostring+" s"), .status, .executions_planned, .records_written, .binary_sha256[0:12], .grading_version, .spec_id] | @tsv' $R/sessions/$s/session.json; done | sort -k3 > $T/t10_sessions.tsv
# markdown and typst renderings of every tsv
for t in $T/*.tsv; do
  b=${t%.tsv}
  awk -F'\t' '{printf "|"; for(i=1;i<=NF;i++) printf " %s |", $i; printf "\n"}' $t > $b.rows.md
  awk -F'\t' '{for(i=1;i<=NF;i++) printf "[%s], ", $i; printf "\n"}' $t | sed 's/\[\([^]]*\)\]/[`\1`]/' > $b.rows.typ
done
