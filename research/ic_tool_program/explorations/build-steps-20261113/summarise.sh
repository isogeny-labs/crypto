#!/usr/bin/env bash
# PLAN.md's read-out: per process, each step's ns per stored pair (ticks /
# tsc_hz / (builds x stored pairs per build)); per row, median and min-max
# over processes; the steps' sum against `whole`; `whole` against
# setup_phases_ns.build. Writes summary.json.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
med='def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;'
for row in k0n53 k0n61; do
  jq -s --arg row "$row" "$med"'
    map(.scan_probes as $s | ($s.build.stored_pairs) as $pairs |
        {per_pair_ns: ($s.build.steps | map_values(. / $s.tsc_hz * 1e9 / $pairs)),
         steps_over_whole: (([$s.build.steps | to_entries[] | select(.key != "whole") | .value] | add) / $s.build.steps.whole),
         whole_over_phase: (($s.build.steps.whole / $s.tsc_hz * 1e9 / $s.build.builds) / ([.repetitions[].setup_phases_ns.build] | add / length)),
         builds: $s.build.builds, stored_pairs_per_build: ($pairs / $s.build.builds)}) as $p |
    {($row): {processes: ($p | length), builds: ($p | map(.builds)), stored_pairs_per_build: $p[0].stored_pairs_per_build,
       steps_over_whole: ($p | map(.steps_over_whole)), whole_over_phase: ($p | map(.whole_over_phase)),
       per_pair_ns: ($p[0].per_pair_ns | keys | map(. as $k | {($k): {median: ($p | map(.per_pair_ns[$k]) | med),
          min: ($p | map(.per_pair_ns[$k]) | min), max: ($p | map(.per_pair_ns[$k]) | max)}}) | add)}}' "$here"/runs/$row-r*.json
done | jq -s 'add' > "$here/summary.json"
jq -c 'to_entries[] | [.key, .value.steps_over_whole, .value.whole_over_phase, (.value.per_pair_ns | map_values(.median*100|round/100))]' "$here/summary.json"
