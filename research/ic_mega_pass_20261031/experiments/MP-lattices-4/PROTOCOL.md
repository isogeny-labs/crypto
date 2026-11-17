# MP-lattices-4 protocol (pre-registered 2026-10-31)
Reframed per skeptics: one run of the missing D4 Coppersmith-lattice reach cell; engineering/accounting expected.
Arms (same code, JM componentwise-lead strategy, l=1..6, reach delta = sum_mu k(mu) / sum_mu wdeg(mu), units log_p):
 A x-box [0,8l]^4 (calibration -> 1/80); B S4 e(x) a+b+c+d<=8l weights(1,2,3,4); C Y-box (= A by degree preservation);
 D D4 invariant (e1,e2,e3 of Y^2, pi) polytope 2(a+b+c)+d<=8l weights (2,4,6,4), every vertex tried as lead.
 Claim-B arms: Mobius Y, x->x+k, x->1/x (all degree-8-per-variable -> same polytope).
Identity check (gp, p~2^32, seed 1): 4 random points + R=-(sum) on y^2=x(x^2+Ax+c^2); even sign flips of Y_1..Y_4 keep sum=O, odd flips give sum=T; negative controls wrong c, odd-2-torsion-free curve.
Monomial count: |{2(a+b+c)+d<=8}| = 105 expected.
prediction_if_true: D within 5% of A/B at matched l; dim ratio Y-box/D ~ 8 (free-orbit limit); all reach <= 1/32.
prediction_if_false: D exceeds B by >10% at matched l.
Dropped: finite-p fplll dim-105 validation and dim 881/1200 runs (budget); reported as not_run.
