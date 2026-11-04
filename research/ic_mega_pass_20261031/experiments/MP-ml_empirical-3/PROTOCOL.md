# MP-ml_empirical-3 PROTOCOL (pre-registered 2026-10-31, written before any run)

Hypothesis: annealing over F_2-subspaces V (dim l) of F_{2^n} finds V whose descended binary S_3 (m=2, E: y^2+xy=x^3+1)
has refutation/solving degree below the random-subspace distribution on FRESH targets.

Metric (certified, not F4 heuristic): D* = least D with 1 in the degree-D Boolean Macaulay span (refutation degree of UNSAT
instances), computed by `../MP-ml_empirical-3_skeptic/dstar_vs_V.py` (reused, unmodified). Basis changes are ignored: moves act
on V (replace one basis vector by a random vector), so search is over Gr(l,n).
Note: hky_last_fall_degree cannot take a custom V (fixed nprime=9) so it is not usable as an annealing oracle; at n=19 it is flat (d_last=4).

Training objective f(V) = mean D* over the UNSAT members of a fixed set of 6 training x-coordinates (seed 11).
Fresh evaluation: 20 UNSAT x-coordinates drawn with a disjoint seed (seed 999) per V.

Arms, equal budget B objective evaluations each:
 A  random best-of-B (null / winner's-curse control)
 B  anneal: B/2 random, then B/2 moves from best (Metropolis, T=0.3 -> 0.02)
 C  anneal on a shuffled objective (score replaced by a seeded random number) -- optimisation-free control
 D  structured baseline: polynomial-basis family P=<1,t,..,t^{l-1}> (explicit all-n family), 5 draws
 R  null band: 40 random V evaluated on the fresh targets (5-95% band of fresh mean D*)
Main cell n=19, l=5, B=200; scaling cells n=17 and n=23, l=5, B=100. Seeds: arm A 101, B 202, C 303, D 404, R 505.

prediction_if_true: arm-B top-5 fresh mean D* below the R 5th percentile AND below arm A and arm C top-5
 (one-sided permutation test p<0.05 B vs A), and the gap does not shrink from n=17 -> 23.
prediction_if_false: arm-B top-5 fresh mean D* within the R 5-95% band, or not significantly below arm A (winner's curse).
Any win only by family D (already known: linearization-excess / polynomial-basis lead) does not count for annealing.
Classification: even a win is engineering unless the winner is an explicit all-n family with flat D* while random grows.
