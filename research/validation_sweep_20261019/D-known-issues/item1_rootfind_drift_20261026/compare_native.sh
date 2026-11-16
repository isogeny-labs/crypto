#!/bin/sh
# usage: compare_native.sh <committed session dir> <re-run session dir>
# For every ic-rootfind record (paired by seq), compare each phase's native
# counters and arithmetic: keys present on one side only, values that differ.
# Prints one row per distinct pattern, with the number of (record, phase) pairs.
set -eu
jq -n --slurpfile a "$1/records.jsonl" --slurpfile b "$2/records.jsonl" '
  ($a | map({key: (.seq | tostring), value: .}) | from_entries) as $A |
  ($b | map({key: (.seq | tostring), value: .}) | from_entries) as $B |
  [ $A | to_entries[] | select(.value.arm | startswith("ic-rootfind"))
    | .key as $s | .value as $ra | $B[$s] as $rb
    | $ra.phases[] as $pa | ($rb.phases[] | select(.name == $pa.name)) as $pb
    | { arm: $ra.arm, phase: $pa.name,
        only_in_committed: (($pa.native | keys) - ($pb.native | keys)),
        only_in_rerun: (($pb.native | keys) - ($pa.native | keys)),
        value_changed: [ ($pa.native | keys[]) as $k
                         | select($pb.native | has($k))
                         | select($pa.native[$k] != $pb.native[$k]) | $k ],
        arithmetic_changed: (($pa.adds != $pb.adds) or ($pa.doubles != $pb.doubles)
                             or ($pa.scalar_mults != $pb.scalar_mults)) } ]
  | group_by([.arm, .phase, .only_in_committed, .only_in_rerun, .value_changed, .arithmetic_changed])
  | map({pairs: length} + .[0])[]' -c
