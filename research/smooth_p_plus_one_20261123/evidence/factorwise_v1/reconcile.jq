group_by([.prime,.depth]) | map(
  (map(select(.type=="factorwise_fiber_audit"))|.[0]) as $n |
  (map(select(.type=="cutoff_partition_audit"))|.[0]) as $o |
  {prime:$n.prime,depth:$n.depth,
   counts_match:([$n.unsigned_tuples,$n.signed_bindings,$n.identity_exclusions,$n.target_x_exclusions,$n.rr5_bindings_checked]
    == [$o.unsigned_tuples,$o.signed_bindings,$o.identity_exclusions,$o.target_x_exclusions,$o.rr5_bindings_checked]),
   charts_match:([$n.charts[]|[.images,.unsigned_tuples,.old_effective_ranks]]
    == [$o.charts[]|[.images,.unsigned_tuples,.effective_jacobian_ranks]]),
   new_full_rank:all($n.charts[]; .factorwise_ranks[7]==.rr5_bindings_checked)}
)
