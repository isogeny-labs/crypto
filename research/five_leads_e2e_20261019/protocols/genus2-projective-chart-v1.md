# Genus-2 projective numerator chart: frozen before the 1,260-target run

The candidate is `ic.three_leads_genus2_projective_chart_v1`. It uses the
same two-choice relation policy and exact-order source instances as
`ic.three_leads_genus2_choice2_v1`. The only changed solver primitive is
Jacobian addition during jump creation, initialization, restarts, and the
relation walk. The Scholten transfer remains the original checked map.

For ordinary degree-two divisors, form the CRT interpolation numerator
`W = norm*v_a + (s1*x+s0)*u_a` before division by `norm`. Form
`H = (norm^2*f-W^2)/(u_a*u_b)` and normalize the output `u` and `v` using one
inversion of `norm*s1`. This keeps all intermediate CRT coordinates in a
projective numerator chart. Identity addition costs no field work, and
degree-one, doubled, or other exceptional inputs use the original complete
Cantor law. Record ordinary chart additions and exceptional fallbacks.
This is a fraction-free projective CRT chart, **not** the complete projective
group law of the published paper; a win or loss here does not settle that
paper's formula. Source elliptic rho uses the existing optimized source
addition and is paired on the same workload IDs.

Read the 21 original exact-order curves and 60 original targets per curve
from `research/three_leads_e2e_20261018/genus-order-fixtures-v1`. Preserve
all seven caps, target and measurement seeds. Run the projective candidate,
the affine two-choice candidate and source negation rho on every workload;
retain all failures and cap exhaustions. Check every returned scalar on the
source curve and independently replay all 3,780 arm records.

Charge construction, transfer, every chart operation and normalization,
exceptional fallback, both candidate additions, roots, graph work, restarts
and source verification. Price the complete one-target M/S/I vector on the
12-point arithmetic grid S/M={0,0.5,1}, I/M={1,10,100,1000} and the same
native logical-operation weights {0,0.1,1} for every arm. Report sums of
paired costs for the full cohort, every size and every curve, including
ratios above 1. This is L0 operation evidence; wall time is not qualified.
