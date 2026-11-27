# one line per (call): bits seed arm  F4/solver ops total, median per-solve s_ops_over_rho (1 op = 1 GAE), median s_over_rho (host GAE)
def med: sort | (if length%2==1 then .[length/2|floor] else (.[length/2-1]+.[length/2])/2 end);
select(.id|startswith("timed")) | select((.id|test(":2$"))|not) |
(.arm // (.id|split(":")[1])) as $arm |
[ (.result.rows[0].log2_r|ceil), .seed,
  ({"baseline":"int-f4","candidate":"cand-f4"}[$arm] // $arm),
  ([.result.rows[].decomposition.solver.ops // 0]|add),
  ([.result.rows[] | ((.decomposition.solver.ops // 0) + (.decomposition.cost.native.pkm_grid_fp_mults_uncharged // 0)) / ((.r|sqrt) * .rho_s)] | med),
  ([.result.rows[].s_over_rho]|med) ] | @tsv
