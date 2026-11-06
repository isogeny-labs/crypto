# Base-field theta setup data dictionary

`results/theta-models.jsonl` has one row for each of 21 checked Scholten
quintics, keyed by prime `p`. All field values are canonical integers in
`[0,p)`. `ordered_pairs` is the number of ordered pairs of distinct finite
Weierstrass roots (20); `square_pairs` counts those whose transformed leading
coefficient has a square root in Fp (10). `eligible_orders` is the six
permutations of the remaining three roots for every square pair (60).
`theta_tuples` counts distinct `(root order, a,b,c,d,e,f)` solutions found
under the fixed gauges `b=f=1`; it does not quotient model isomorphisms.
`fp_theta_tuple_found` is true exactly when that count is positive. `selected`
is the first passing tuple under the deterministic source-root order and
permutation sequence, with `r0,r1,delta,sqrt_scale,lambda_mu_nu`, and
`constants`. The constants include `a,b,c,d,e,f,A,B,C,D,alpha` and the
Rosenhain triple; `all_equations_verified` records exact modular checks.

`results/summary.json` aggregates only these rows and records explicit nulls
for unmeasured candidate/rho costs. The initial Kummer reference gate's
price-grid nulls are in `../kummer_walk_20261022/results/price-grid.csv`.
This addendum does not replace them. `PROTOCOL.md` fixes the search family,
and `VALIDATION.md` gives independent replay and scope limits.
