[.[] | {
  prime, depth, base_size, capacities, variables_per_chart,
  regular_charts, full_charts, unsigned_tuples, signed_bindings,
  identity_exclusions, target_x_exclusions, rr5_bindings_checked,
  ranks: ([range(0;4) as $r | [.charts[].effective_jacobian_ranks[$r]] | add]),
  patterns: (.charts | group_by(.images | unique | length) | map({
    distinct_images: (.[0].images | unique | length),
    charts: length,
    tuples: ([.[].unsigned_tuples] | add),
    ranks: ([range(0;4) as $r | [.[].effective_jacobian_ranks[$r]] | add])
  }))
}]
