group_by([.prime,.depth]) | map(
  (map(select(.type == "liftable_fiber_audit"))[0]) as $new |
  (map(select(.type == "factorwise_fiber_audit"))[0]) as $old |
  {prime:$new.prime,depth:$new.depth,
   records_present:(length == 2 and $new != null and $old != null),
   counts_match:([$new.base_size,$new.catalog_charts,$new.unsigned_tuples,$new.signed_bindings,$new.identity_exclusions,$new.target_x_exclusions,$new.rr5_bindings_checked]
      == [$old.base_size,$old.catalog_charts,$old.unsigned_tuples,$old.signed_bindings,$old.identity_exclusions,$old.target_x_exclusions,$old.rr5_bindings_checked]),
   charts_match:([$new.charts[]|{images,unsigned_tuples,rr5_bindings_checked,old_effective_ranks,factorwise_ranks}]
      == [$old.charts[]|{images,unsigned_tuples,rr5_bindings_checked,old_effective_ranks,factorwise_ranks}]),
   full_rank:all($new.charts[]; .factorwise_ranks[7] == .rr5_bindings_checked)}
)
