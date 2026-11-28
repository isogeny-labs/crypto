# per timed call: bits, seed, arm, r, per-solve solver ops and trials, s_over_rho (GAE, host-dependent)
def med: sort | if length==0 then null else (if length%2==1 then .[length/2|floor] else (.[length/2-1]+.[length/2])/2 end) end;
select(.id|startswith("timed")) | select(.result!=null) |
{ bits: (.result.rows[0].log2_r|ceil), seed, arm: (.arm // (.id|split(":")[1])), call: .id,
  instance: .result.instance, r: .result.rows[0].r, rho_s: .result.rows[0].rho_s,
  all_verified: ([.result.rows[]|.verified]|all), n: (.result.rows|length),
  solver_ops_total: ([.result.rows[]|.decomposition.decomposition.solver.ops // .decomposition.solver.ops // 0]|add),
  solver_calls_total: ([.result.rows[]|.decomposition.decomposition.solver.calls // .decomposition.solver.calls // 0]|add),
  trials_total: ([.result.rows[]|.decomposition.decomposition.targets_tried // .decomposition.targets_tried // 0]|add),
  group_gae_total: ([.result.rows[]|.total_gae]|add),
  med_s_over_rho: ([.result.rows[]|.s_over_rho]|med) }
