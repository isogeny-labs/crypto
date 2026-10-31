#!/bin/sh
# Per-target table of S for every arm at w = 0, 0.1, 1 GAE per unpriced native op.
# usage: tabulate.sh results/lever-mNN   (reads records.jsonl only; layout, no new computation)
jq -r -s '
 def rho: map(select(.arm=="rho-neg"))|map({key:(.workload.target_index|tostring),value:.})|from_entries;
 (rho) as $rho
 | map(select(.method.id=="ic.pipeline"))
 | group_by(.workload.target_index)[]
 | . as $g | ($g[0].workload.target_index|tostring) as $t | $rho[$t] as $r
 | ($r.boundaries.floor_s) as $floor
 | ($r.workload.curve.r) as $rr
 | ($r.cost.total_gae) as $rg
 | ($r.counters.canonicalisations_uncharged // $r.cost.canonicalisations_uncharged // 0) as $cn
 | ($rr|sqrt) as $sr
 | [ $g[] | {arm, s:.cost.s, ver:.outcome.status, trials:.phases[]?|select(.name=="relations")|.native.trials} ] as $a
 | $a[] | [$t, .arm, .s, .ver, .trials,
     ($rg/$sr), (($rg+0.1*$cn)/$sr), (($rg+$cn)/$sr),
     (.s/($rg/$sr)), (.s/(($rg+0.1*$cn)/$sr)), (.s/(($rg+$cn)/$sr))] | @tsv' "$1/records.jsonl"
