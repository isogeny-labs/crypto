# The ranked entries of the six Cheon oracle domains of docs/bounds/frontier.json,
# copied as recorded (no computation): one object per domain, sorted by oracle
# and unit, entries in the frontier's own order.
[ .domains[]
  | select(.domain.tier == "medium" and (.domain.envelope.oracle // "" | startswith("static_dh:")))
  | { oracle: .domain.envelope.oracle, unit: .domain.unit, domain_id,
      entries: [ .entries[] | { method, bound_id, is_frontier,
                 ops: .axes.ops.value, lo: .axes.ops.ci95[0], hi: .axes.ops.ci95[1],
                 oracle_queries: .axes.oracle_queries.value, dominated_by } ] } ]
| sort_by(.oracle, .unit)
