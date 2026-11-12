#!/bin/bash
# P1 and P2 of PROTOCOL.md from the committed rows, with jq only.
# P1: per paired row (same size, target and repeat), the hashes of
#     identity_inputs and counts and the recovered logarithms of both arms
#     agree between v10 and v10c.
# P2: per row, the median over its repetitions of select + select_projection
#     (setup_phases_ns); the ratio v10 / v10c per row; per size its median,
#     quartiles and range.  Set-up and its select share are reported beside.
set -eu
D=$(cd "$(dirname "$0")/.." && pwd)
A=${P1P2_A:-$D/v10/runs}; B=${P1P2_B:-$D/v10c/runs}
med='def med: sort | if length == 0 then null elif length % 2 == 1 then .[length/2|floor] else (.[length/2-1] + .[length/2]) / 2 end;'
row() { # file -> one JSON object
  jq -c "$med"' {
    identity: (.identity_inputs | tojson),
    counts: (.counts | tojson),
    logs: ([.repetitions[] | [.ic_online.recovered, (.rho_online.recovered | tostring)]] | unique),
    select: ([.repetitions[].setup_phases_ns | .select + .select_projection] | med),
    setup: ([.repetitions[].setup_ns] | med)
  }' "$1"
}
{
for size in k1n47 k0n57 k0n41 k0n53 k1n59 k0n61; do
  for f in "$A/$size"/T[0-9][0-9]-R[0-9].price.json; do
    [ -e "$f" ] || continue
    name=$(basename "$f")
    # The harness keeps every attempt and stops at the first clean one, so a
    # row's figure is its last attempt (…-retryN.price.json when it has one).
    f=$(ls "$A/$size/${name%.price.json}"*.price.json | sort -V | tail -1)
    g=$(ls "$B/$size/${name%.price.json}"*.price.json 2>/dev/null | sort -V | tail -1)
    [ -n "$g" ] || { echo "{\"size\":\"$size\",\"row\":\"$name\",\"unpaired\":true}"; continue; }
    x=$(row "$f"); y=$(row "$g")
    jq -cn --arg size "$size" --arg row "$name" --argjson a "$x" --argjson b "$y" '{
      size: $size, row: $row,
      same_identity: ($a.identity == $b.identity),
      same_counts: ($a.counts == $b.counts),
      same_logs: ($a.logs == $b.logs),
      select_ms: [$a.select / 1e6, $b.select / 1e6],
      setup_ms: [$a.setup / 1e6, $b.setup / 1e6],
      select_ratio: ($a.select / $b.select),
      setup_ratio: ($a.setup / $b.setup)
    }'
  done
done
} | jq -s "$med"' def q($p): sort | if length == 0 then null else .[((length - 1) * $p) | floor] end;
  {
    what_this_is: "Rule v10, P1 and P2 from the rows (p1p2.sh): v10 against v10c paired by size, target and repeat",
    sizes: (group_by(.size) | map({
      size: .[0].size,
      rows_paired: (map(select(.unpaired | not)) | length),
      unpaired: (map(select(.unpaired)) | map(.row)),
      p1: {
        identity_inputs_equal: (map(select(.same_identity)) | length),
        counts_equal: (map(select(.same_counts)) | length),
        logarithms_equal: (map(select(.same_logs)) | length),
        differing_rows: (map(select((.unpaired | not) and ((.same_identity and .same_counts and .same_logs) | not))) | map(.row))
      },
      p2: {
        select_ratio_median: (map(select(.unpaired | not) | .select_ratio) | med),
        select_ratio_q25: (map(select(.unpaired | not) | .select_ratio) | q(0.25)),
        select_ratio_q75: (map(select(.unpaired | not) | .select_ratio) | q(0.75)),
        select_ratio_min: (map(select(.unpaired | not) | .select_ratio) | min),
        select_ratio_max: (map(select(.unpaired | not) | .select_ratio) | max),
        select_ms_median: [(map(select(.unpaired | not) | .select_ms[0]) | med), (map(select(.unpaired | not) | .select_ms[1]) | med)],
        setup_ms_median: [(map(select(.unpaired | not) | .setup_ms[0]) | med), (map(select(.unpaired | not) | .setup_ms[1]) | med)],
        setup_ratio_median: (map(select(.unpaired | not) | .setup_ratio) | med),
        met: ((map(select(.unpaired | not) | .select_ratio) | med) <= 0.25)
      }
    }) | sort_by(["k1n47","k0n57","k0n41","k0n53","k1n59","k0n61"] as $o | .size as $s | $o | index($s)))
  }'
