def med: sort | length as $n | if $n % 2 == 1 then .[($n-1)/2] else (.[$n/2-1] + .[$n/2]) / 2 end;
(group_by(.arm) | map({
  arm: .[0].arm,
  records: length,
  complete: (map(select(.work.Ok.complete == true and (.work.Ok.unpriced|length)==0 and (.work.Ok.known_uncounted|length)==0)) | length),
  median_bitops: (map(.work.Ok.bitops|tonumber) | med),
  min_bitops: (map(.work.Ok.bitops|tonumber) | min),
  max_bitops: (map(.work.Ok.bitops|tonumber) | max),
  median_gae: (map(.total_gae) | med)
})) as $arms
| {
  schema: "ic-acct7.work-medians/v1",
  source: "audit/n17-work-counted.jsonl",
  computed_by: "jq -s -f audit/medians.jq audit/n17-work-counted.jsonl (median of an even count = mean of the two middle values)",
  arms: $arms,
  median_bitops_ic_over_rho: (($arms[] | select(.arm=="ic-symmetrised-d9") | .median_bitops) / ($arms[] | select(.arm=="rho-frob") | .median_bitops))
}
