# Derived summary: jq over ecbench compare --save and band --json outputs only.
# Run by scripts/summary.sh.
def r(c): {ratio: c.ops.ratio_b_over_a, ci95: c.ops.ci95, pairs: c.ops.pairs};
{ schema: "ic_planner_weight_rule.summary/v1",
  derivation: $deriv.rule,
  phase_a: [ $a[] | { tag: .tag, slug: .c0.curves[0].slug, log2_r: .c0.curves[0].log2_r,
       m141_over_m100: { w0: r(.c0), w1: r(.c1) },
       m100_over_rho: { w0: r(.rho0), w1: r(.rho1m100) }, m141_over_rho_w1: r(.rho1m141) } ],
  phase_b: [ $b[] | { tag: .tag, slug: .c0.curves[0].slug, log2_r: .c0.curves[0].log2_r,
       v2_over_v1: { w0: r(.c0), w1: r(.c1) },
       v1_over_rho: { w0: r(.rho0), w1: r(.rho1v1) }, v2_over_rho_w1: r(.rho1v2) } ] }
| .p3_w1_intervals_below_1 = ([.phase_b[] | select(.v2_over_v1.w1.ci95[1] < 1)] | length)
| .p3_held = (.p3_w1_intervals_below_1 >= 3)
