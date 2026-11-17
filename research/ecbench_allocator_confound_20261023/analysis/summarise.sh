#!/bin/sh
# Descriptive summaries of RECORDED values only (no result is computed by Python).
# Usage: summarise.sh OUT.json GROUP=SESSION_DIR[,SESSION_DIR...] ...
# Per group and arm over measured (non-warm-up) records that verified:
#   n, min / q1 / median / q3 / max (linear-interpolation quantiles on the sorted
#   values, the median of an even n is the mean of the two middle values) of
#   time.solve_wall_ns, online.wall_ns and solve_wall_ns / cost.total_gae (ns per
#   GAE, per record), plus the isolation levels and allocator.pre_solve_trim counts.
# Every statistic below is a plain order statistic of the recorded numbers.
set -eu
out=$1; shift
tmp=$(mktemp)
for spec in "$@"; do
  g=${spec%%=*}; dirs=${spec#*=}
  for d in $(echo "$dirs" | tr ',' ' '); do
    jq -c --arg g "$g" --arg s "$(basename "$d")" 'select(.warmup == false) | {
      g: $g, sess: $s, arm, round, ti: .workload.target_index, status: .outcome.status,
      solve: .time.solve_wall_ns, online: .online.wall_ns, gae: .cost.total_gae,
      nspg: (if .cost.total_gae > 0 then (.time.solve_wall_ns / .cost.total_gae) else null end),
      level: .isolation.level, trim: (.allocator.pre_solve_trim // null),
      trim_ret: (.allocator.trim_result // null), adm: (.time.wall_admitted // null)}' "$d/records.jsonl" >> "$tmp"
  done
done
jq -s '
  def q($a; $p): ($a | length) as $n | (($n - 1) * $p) as $i | ($i | floor) as $lo | ($i | ceil) as $hi
    | $a[$lo] + ($a[$hi] - $a[$lo]) * ($i - $lo);
  def stats($xs): ($xs | map(select(. != null)) | sort) as $a
    | if ($a | length) == 0 then null else
      {n: ($a | length), min: $a[0], q1: q($a; 0.25), median: q($a; 0.5), q3: q($a; 0.75), max: $a[-1]} end;
  group_by(.g) | map({key: .[0].g, value: {
    sessions: (map(.sess) | unique),
    arms: (group_by(.arm) | map({key: .[0].arm, value: {
      records: length,
      verified: (map(select(.status == "verified")) | length),
      solve_wall_ns: stats(map(select(.status == "verified") | .solve)),
      online_wall_ns: stats(map(select(.status == "verified") | .online)),
      ns_per_gae: stats(map(select(.status == "verified") | .nspg)),
      isolation_levels: (group_by(.level) | map({key: (.[0].level // "none"), value: length}) | from_entries),
      pre_solve_trim_true: (map(select(.trim == true)) | length),
      pre_solve_trim_absent: (map(select(.trim == null)) | length)}}) | from_entries)}}) | from_entries
' "$tmp" > "$out"
rm -f "$tmp"
