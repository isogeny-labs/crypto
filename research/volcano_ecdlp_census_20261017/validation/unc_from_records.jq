# Validator re-derivation of A2.1's uncharged count per measured record.
# For each name in cost.unpriced: the top-level counters value if present,
# else the sum over phases of native[name minus "_uncharged"].
# Output TSV: workload_id target_index arm round total_gae s unc_total
select(.warmup == false)
| . as $r
| ([ $r.cost.unpriced[]? as $n
     | if ($r.counters | has($n)) then $r.counters[$n]
       else ([ $r.phases[]?.native[($n | sub("_uncharged$"; ""))] // 0 ] | add // 0) end ]
   | add // 0) as $u
| [ $r.workload.workload_id, $r.workload.target_index, $r.arm, $r.round, $r.cost.total_gae, $r.cost.s, $u ]
| @tsv
