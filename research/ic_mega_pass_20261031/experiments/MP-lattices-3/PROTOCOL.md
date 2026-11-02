# MP-lattices-3 protocol (pre-registered 2026-10-31, before any run)

Hypothesis: joint lattice on {S3(x1,x2,u), S3(x3,x4,v), S3(u,v,z)} mod p^l, with x_i <= p^delta and full-size u,v <= p,
using shifts with u,v multiplier degree >= 2, beats S5's closure-optimal delta_l.

Method: LM-diagonal accounting (same criterion as lit2026_w10_coppersmith_bound): each monomial m in a box gets the shift
(m/LM^k) g1^k1 g2^k2 g3^k3 with largest k=k1+k2+k3<=l whose monomials stay in the box; weight w(m)=k-log_p(X^m);
lattice = closed monomial set (triangular, w.r.t. a fixed term order); delta_l = sup delta with max closure weight > 0.
Max-weight closure by min-cut, scipy.sparse.csgraph.maximum_flow (integer capacities, scale 1e6). Binary search on delta, 1e-5.
Term orders: several weight vectors (lex tiebreak); best reported. Deterministic (no randomness; seed n/a, z=A=B fixed generic ints).

Arms: A0 controls (univariate d=2,3,5; BD f=1+x(A+y), Y=p^1/2; single S3(x1,x2,z)) vs W10 tool table.
A1 multiplier u,v-degree s=0; A2 s=1; A3 s<=2l (multiplicity >=2). x-deg box Dx=2l, u,v box Du=2l. l=1,2 (3 if time).
B: S5 baseline = W10 tool S5 rows (not recomputed: dense S5 too large in budget).
Not run (budget): Howell true-det arm C, qflll arm E. Isolation arm D by count only.

prediction_if_true: delta_l(A3) > delta_l(S5 W10) by >10% at l=2 or 3 (at matched dim).
prediction_if_false: A3 selects no u,v-multiplier shifts of degree>=2, or delta_l(A3) <= S5 at matched l/dim.
delta_needed 0.169-0.2 printed with every result.
