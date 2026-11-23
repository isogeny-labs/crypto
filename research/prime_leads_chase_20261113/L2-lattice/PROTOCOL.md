# L2-lattice protocol (frozen before any computation) — 2026-11-13

## Question
Does any concrete, structurally new presentation of the m-decomposition problem
(small-x factor base, B = p^δ) admit a Coppersmith-type lattice whose reach exceeds
the known single-ideal (p, S_{m+1})^t reach — by a real margin for S3, or up to the
milestone δ ≥ 1/4 for S4 (m=3), or δ ≈ 1/5 for S5 (m=4)?

Prior closures that this lab does NOT redo (cited, [pcc:unverified]):
F1–F6 and K1–K16 in crypto:research/prime_s5_small_root_leads_20261028/README.md;
the strategy-independent bound (crypto:research/smallroot_optimal_reach_20261026);
the any-monomial-set h_N theorem and the joint (4,2) ν_s computation
(crypto:research/smallroot_followups_20261026). The still-open lattice item listed
there is "lattices that use more than the one ideal (p,S)^t, e.g. several targets'
polynomials at once"; this lab targets that item.

## Instrument (desk computation, no LLL unless the gate below opens)
For an ideal I = (f_1..f_k) ⊂ F_q[vars] (q = 1000003, random curve and target),
weights w(v) = log_p(bound of v) (small unknowns: x_i -> δ, e_i -> iδ; unknowns
ranging over F_p (y, λ, line/function coefficients, intermediate x_T) -> 1),
compute ν_s(T) = least w-weight of a nonzero element of Ī^s among Macaulay rows
{x^a·f^α : |α| = s, deg ≤ T}, by row echelon with columns ordered by decreasing weight.
Criterion (necessary condition for any lattice from (p, I)^t to satisfy
Howgrave-Graham, following smallroot_followups §3): ν_s < s for some s.
Also report ν_s/s against the single-polynomial value h_S(δ) of the eliminated S.

## Systems
- NEG: S3 x-form alone; S3 e-form alone; S4 e-form alone (expected ν_s/s = h_S).
- POS (positive control): {t − a·e1, t − b·e1 − e2}, t of weight 1 (expected ν_1 = 2δ < 1:
  the detector must see an elimination through a large unknown).
- A1 (idea a, y/collinearity): vars x1,x2 (δ), y1,y2 (1): curve equations + collinearity of P1,P2,−R.
- A2 (idea a, rational parametrisation, m=2, e-form): λ (1), e1, e2.
- A3 (idea a, m=2, x-form): λ (1), x1, x2 with e1=x1+x2, e2=x1x2.
- A4 (idea a, m=3, e-form): function x²+a1x+a0+cy through R; a1, c (1); e1,e2,e3.
- A5 (idea a, m=4, e-form): function xy + a2x² + a1x + a0 + cy... through R; 3 params (1); e1..e4 (s=1 only if too large).
- B (idea b, two targets jointly, disjoint unknowns): S3(x;z1), S3(x';z2).
- F (idea f, chain for m=3): S3(x1,x2,u), S3(u,x3,z), u of weight γ ∈ {1, 1/2}.
- RAND: random polynomials with the supports of A2 and A4 (is curve structure load-bearing?).
δ grid: {1/12, 1/6, 1/5, 1/4, 1/3, 1/2}; s ∈ {1,2,3} where size allows; truncation T and T+2.

## Hypotheses and predictions
H0 (predicted): for every idea system, ν_s(T)/s ≥ min(1, h_S(δ)) − 0.01 at every grid point, i.e.
the extra equations / large unknowns / extra targets add no low-weight ideal element.
H1: some idea system has ν_s/s < min(1, h_S(δ)) − 0.02 at some δ, stable under T -> T+2.

## Decision rule (per idea)
- SURVIVE (opens step 3, fpylll measurement at 40 and 56 bits with the basic lattice as control):
  H1 holds at a δ above the known reach (S3: δ > 1/6 + 0.02; S4: ν_s < s at δ = 1/4;
  S5: ν_s < s at δ = 1/5), stable in T, and POS fires (ν_1 < 1) and NEG matches h_S within 0.01.
- KILL (desk/toy scale): H0 holds at all grid points for that system with POS and NEG passing,
  or a written reduction to a cited closure (F1–F6, K1–K16, theorems above) applies.
- INCONCLUSIVE: controls fail, or ν_s(T) not stable under T -> T+2, or system too large to compute.
Ideas analysed purely on paper are KILL only with an explicit derivation; otherwise INCONCLUSIVE.

## Caveats fixed in advance
ν_s(T) is an upper bound on ν_s (truncation drops elements); the criterion is necessary,
not sufficient; s ≤ 3 only. Results are "computed, not proved".

## Resources
python3 single-thread, prlimit 3 GB, timeout 20 min per command, total ≤ 3 h wall.
