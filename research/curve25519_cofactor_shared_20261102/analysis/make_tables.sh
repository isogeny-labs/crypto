#!/bin/bash
# jq-only tables from f4cost_shared.jsonl. Writes base_check.txt, table.md, ratios.md.
D="$(cd "$(dirname "$0")" && pwd)"
J=$D/f4cost_shared.jsonl
# Equal-base assertion: every arm of a cell has the same base signature.
jq -s -r '[.[]|select(.failed|not)] | group_by([.form,.t,.guess]) | .[] |
  (map([.columns,.covered_points,.covered_points_all_cosets,.covered_projected_points])|unique|length) as $u |
  "\(.[0].form) t=\(.[0].t) g=\(.[0].guess) columns=\(.[0].columns) covered=\(.[0].covered_points) projected=\(.[0].covered_projected_points) all_cosets=\(.[0].covered_points_all_cosets) distinct-base-signatures=\($u)"' $J > $D/base_check.txt
if grep -v "distinct-base-signatures=1$" $D/base_check.txt | grep -q .; then echo "BASE MISMATCH" >&2; exit 1; fi
sci() { printf '%s' "$1"; }
jq -s -r '
 def arm: if .lifts_mode==0 then "0" elif .lifts_mode==1 then "1" elif .trace then "2+trace" else "2" end;
 def r: (.covered_points/.coverage_exact);
 def cov: (if .lifts_mode==0 then .covered_points else .covered_projected_points end);
 def e: (if .==null then "n/a" else (. * 1000 | round / 1000 | tostring) end);
 def sc: if . == null then "undef" else (if . == 0 then "0" else (log10 | floor) as $k | ((. / pow(10;$k)) * 100 | round / 100 | tostring) + "e" + ($k|tostring) end) end;
 "| form | t | g | lifts | run | targets | hits | verified | exact | F4 calls/tgt | mul-adds/tgt | max solv. deg | max rows | max cols | max nnz | undecided | mul-adds/hit | mul-adds/tgt / (cov/r) | load (1m) |",
 "|:--|:-:|:-:|:-:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|",
 (.[] | select(.failed|not) |
  "| \(.form) | \(.t) | \(.guess) | \(arm) | \(if .run|test("prefix") then "prefix" else "full" end) | \(.targets_done)\(if .capped then " (capped)" else "" end) | \(.hits) | \(.hits_verified) | \(.hits_exact) | \(.calls_per_target|e) | \(.muladds_per_target|sc) | \(.solving_degree_max) | \(.solver_extra.max_rows_max) | \(.solver_extra.max_cols_max) | \(.solver_extra.max_nnz_max) | \(.undecided) | \(if .hits>0 then (.muladds_per_hit|sc) else "undef" end) | \((.muladds_per_target/(cov/r))|sc) | \(.uptime|capture("load average: (?<l>[0-9.]+)").l) |"),
 ([.[]|select(.failed)]|if length>0 then "FAILED RUNS: \(map(.run)|join(", "))" else "No failed runs." end)
' $J > $D/table.md
# ratios: lifts=2 / lifts=1 and lifts=2 / lifts=0, on matched targets (prefix rows where the lifts=2 arm was capped)
jq -s -r '
 def key: "\(.form) t=\(.t) g=\(.guess)";
 def pick(l; n): [.[]|select(.failed|not)|select(.lifts_mode==l and (.trace|not) and .targets_done==n)] | .[0];
 [.[]|select(.failed|not)|select(.lifts_mode==2 and (.trace|not))] as $two |
 "| cell | lifts=2 targets | calls/tgt 2 vs 1 vs 0 | mul-adds/tgt 2/1 | mul-adds/tgt 2/0 | rows 2/1 | cols 2/1 | nnz 2/1 | max solving degree 0,1,2 | hits 0,1,2 (same targets) |",
 "|:--|--:|:--|--:|--:|--:|--:|--:|:--|:--|",
 ($two[] as $b | . as $all |
  ([$all[]|select(.failed|not)|select(.form==$b.form and .t==$b.t and .guess==$b.guess and .lifts_mode==1 and .targets_done==$b.targets_done and (.trace|not))]|.[0]) as $o |
  ([$all[]|select(.failed|not)|select(.form==$b.form and .t==$b.t and .guess==$b.guess and .lifts_mode==0 and .targets_done==$b.targets_done and (.trace|not))]|.[0]) as $z |
  "| \($b|key) | \($b.targets_done) | \($b.calls_per_target*100|round/100) vs \($o.calls_per_target*100|round/100) vs \($z.calls_per_target*100|round/100) | \(($b.muladds_per_target/$o.muladds_per_target*100|round/100)) | \(($b.muladds_per_target/$z.muladds_per_target*100|round/100)) | \(($b.solver_extra.max_rows_max/$o.solver_extra.max_rows_max*100|round/100)) | \(($b.solver_extra.max_cols_max/$o.solver_extra.max_cols_max*100|round/100)) | \(($b.solver_extra.max_nnz_max/$o.solver_extra.max_nnz_max*100|round/100)) | \($z.solving_degree_max), \($o.solving_degree_max), \($b.solving_degree_max) | \($z.hits), \($o.hits), \($b.hits) |")
' $J > $D/ratios.md
# trace arm
jq -s -r '
 [.[]|select(.failed|not)] as $a |
 "| cell | lifts=2 mul-adds/tgt (targets) | lifts=2,trace mul-adds/tgt (targets) | ratio | learning mul-adds (once) | replays | refuted by replay | fallbacks | fallback plain mul-adds | amortised/tgt incl. learning |",
 "|:--|:--|:--|--:|--:|--:|--:|--:|--:|--:|",
 ($a[]|select(.trace==true)) as $t |
 ($a[]|select(.form==$t.form and .t==$t.t and .guess==$t.guess and .lifts_mode==2 and (.trace|not))) as $p |
 "| \($t.form) t=\($t.t) | \($p.muladds_per_target|floor) (\($p.targets_done)) | \($t.muladds_per_target|floor) (\($t.targets_done)) | \(($t.muladds_per_target/$p.muladds_per_target*1000|round/1000)) | \($t.trace_learn_muladds) | \($t.native.pkm_trace_replays) | \($t.native.pkm_trace_refuted // 0) | \($t.native.pkm_trace_fallbacks // 0) | \($t.native.pkm_trace_fallback_plain_muladds // 0) | \($t.amortised_muladds_per_target|floor) |"
' $J > $D/trace.md
echo done
