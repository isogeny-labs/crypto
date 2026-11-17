# MP-lattices-2 PROTOCOL (pre-registered 2026-10-31; written after a 2-min timing pilot at S3 l<=3, before the main runs)

Hypothesis under test: deleting "unhelpful" rows (log_p diag excess delta*|alpha| - k > tau) from the Jochemsz-May box
lattice of S_3(x1,x2,xR) (box [0..2l]^2, dims 25/49/81/...) and S_4(x1,x2,x3,xR) (box [0..4l]^3) yields a non-full-rank
sublattice (columns of all monomials kept => non-triangular, det(L_sub)^2 != prod diag^2) with larger HG reach than the full lattice.
Scope: W10 section 7 class 1. Prior art: Boneh-Durfee 1999, Herrmann-May 2008/2010, Ducas focus groups, Gao-Feng-Hu-Pan 2512.19076 (second-vector scoring).

Arms: A = full JM box lattice (triangular control). B = helpful-only (tau=0). B' = threshold scan tau in {-0.5,-0.25,0,0.25,0.5,1,1.5}.
C = B with second-nontrivial-vector rule (skip vectors whose polynomial is x1<->x2 antisymmetric; Layer 2 only).

Layer 1 (no LLL): exact Gram determinant (PARI matdet over Z) of retained rows with X=round(p^delta), p=nextprime(2^128).
HG surrogate margin = l - log_p(sqrt r) - log_p(vol)/r; delta*(arm,l) by 12-step bisection. S3 l=1..L (L as far as fits in ~5 CPU-min),
arm A also closed-form (det=prod diag) to l=16. S4 l=1..2 if feasible. Fit delta*(l)=delta_inf - c/l on A and best-B'.
Seeds: curve/xR seed 1 (and seed 2 replicate for S3).

Layer 2 (LLL): p=nextprime(2^256), PARI qflll, S3 l=2,3 arms A/B/C, planted x1,x2 uniform in [0,X), xR solved from S3 mod p
(retry until quadratic has a root). Per instance: bisected HG delta (two shortest retained vectors < p^l/sqrt r) and heuristic
success (two vectors vanishing at root over Z with nonzero resultant whose integer root set contains planted x1).
4 planted instances per cell, seeds 100..103.

Layer 3 controls: (a) generic-coefficient polynomial on same support (random monic coefficients, planted root) at l=2;
(b) unplanted random xR: false-positive rate of heuristic recovery (any integer root candidate satisfying f==0 mod p with |x|<X);
(c) delta=0.45 negative control; (d) arm A pipeline must give HG delta near the Layer-1 value (consistency check).

Pre-registered predictions.
prediction_if_true (class 1 closed for this construction): best B/B'/C delta* <= A delta* + 0.02 at every l, fitted delta_inf(B) <= 1/6 (S3)
  and <= 1/24 (S4); Layer 2 B/C HG delta within 0.02 of A.
prediction_if_false: some B/B'/C delta* exceeds A by > 0.02 at l>=3, or fitted delta_inf(B) > 1/6 beyond fit error, confirmed in Layer 2
  with no false-positive excess in control (b) => trigger S5 computation.
