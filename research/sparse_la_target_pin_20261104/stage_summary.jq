# Aggregate sparse_la_stage/v1 JSON lines (jq -s -f stage_summary.jq results/stage-*.jsonl).
#
# Per (weight, columns, arm): the number of runs, how many recovered the planted value, how
# many were CENSORED (the arm used every row it was given without determining what it needs:
# a full-rank arm on a system with a base column no row touched cannot reach full rank), the
# median over solved seeds of rows used and counted row_ops and wall time, and the PAIRED
# ratios to structured-gauss: per seed, arm over structured-gauss, over seeds where both were
# solved and correct, then the median, minimum and maximum of those ratios.
# Layout only: every figure is a median, minimum or maximum of recorded values or a quotient
# of two recorded values of the same seed.
def median: sort | if length == 0 then null elif length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;
def ok: select(.solved and .correct);
group_by([.weight, .columns]) | map(
  . as $cell
  | ($cell | map(select(.arm == "structured-gauss" and .solved and .correct)) | map({key: (.seed | tostring), value: .}) | from_entries) as $sg
  | $cell | group_by(.arm) | map(
      . as $runs
      | ($runs | map(ok)) as $good
      | ($good | map(select($sg[(.seed | tostring)] != null)
          | { r: (.rows_used / $sg[(.seed | tostring)].rows_used), o: (.row_ops / $sg[(.seed | tostring)].row_ops) })) as $pair
      | {
          weight: $runs[0].weight,
          columns: $runs[0].columns,
          arm: $runs[0].arm,
          runs: ($runs | length),
          correct: ($good | length),
          censored: ($runs | map(select((.solved | not))) | length),
          wrong: ($runs | map(select(.solved and (.correct | not))) | length),
          rows_median: ($good | map(.rows_used) | median),
          row_ops_median: ($good | map(.row_ops) | median),
          wall_ms_median: (($good | map(.wall_ns) | median) / 1e6),
          paired_seeds: ($pair | length),
          rows_over_sg_median: ($pair | map(.r) | median),
          ops_over_sg_median: ($pair | map(.o) | median),
          ops_over_sg_min: ($pair | map(.o) | min),
          ops_over_sg_max: ($pair | map(.o) | max)
        }
    )
) | flatten
