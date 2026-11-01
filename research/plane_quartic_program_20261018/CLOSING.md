# Plane-quartic index calculus and the Smith walk: closing note

2026-10-26. This closes the program scoped in [PROGRAM.md](PROGRAM.md). It adds no
new measurements. Every number below is copied from the README of the study it
names, as of that study's PR head on 2026-10-26. All five studies are still in open
pull requests, so a number can change before they merge. Where a study's frozen verdict and its
post hoc reading differ, the frozen verdict comes first.

Scope, unchanged from PROGRAM.md: `E/F_{q^3}` with a genus-3 GHS descent to `F_q`
(odd prime `q`), synthetic or known-answer instances only. Nothing here touches
prime-field curves, binary standard curves or any deployed curve. It makes no
key-recovery claim and no asymptotic sub-rho claim. Magma, Sage, PARI and msolve
are not installed, so nothing was compared with an external implementation.

## The question and the answer

**Question (PROGRAM.md).** For this family, what does solving the ECDLP through a
smooth plane quartic with Diem's index calculus cost, end to end, compared with a
matched Pollard rho on `E(F_{q^3})`? At what `q`, if any, do the two cost curves
cross?

**Answer, as a measured fit over q = 2^8 .. 2^22 on the GHS-weak family only.**
The arm compared is the best pre-registered pipeline: double large primes,
structured Gaussian elimination and block Lanczos ("DL" in N4 v2). Against rho with
the negation map, the curves cross at:

| measure | crossover `q*` [95% bootstrap] | source |
|:--|:--|:--|
| `F_q`-multiplication equivalents (M-eq), w = 0 | 555 [389, 752]; below rho at every size from q = 991, across the pricing band | N4 v2 |
| CPU time (addendum measure) | 1,889 [1,733, 4,373] | N4 v2 |
| wall time (frozen measure; shared host) | 5,704 [1,495, 7,613] | N4 v2 |

At q = 2,097,091, DL costs 0.005 [0.004, 0.008] of negation-map rho in M-eq.
The exponent gap (rho with negation map minus DL), fitted over the full range
k = 12..22, is 0.560 [0.510, 0.600] at w = 0.
These results agree with the theory PROGRAM.md cites: `Õ(q)` for plane-quartic
index calculus (Diem 2006; Diem–Thomé 2008) against `Õ(q^{3/2})` for rho. They are
our measurements of a known result with this implementation's constants, not a new
attack.

## What each workstream established

### Route Q: the direct m = 2 quartic

**W1, canonical `Pic^0` arithmetic** (`src/cryptanalysis/plane_quartic_pic.rs`, PR #2227).
- Every class has a unique key, checked against brute-force class counts on 21
  curves and full Cayley tables at q = 5.
- The old degree-3 keys merged distinct classes 1, 7 and 20 times at q = 5, 7 and
  11. W1 separates all of them.
- A generic addition costs 3,523 multiplications and 44 inversions, flat in `q` up
  to 2^31 − 1.

**N4, index calculus against matched plain rho** (`research/quartic_n4_20261019/`, in PR #2227).
- The earlier "pure IC" on the toy quartics was enumeration of the subgroup, not
  index calculus.
- The real pipeline (B0) verified all 51 of 51 units.
- B0 costs 0.122 [0.10, 0.16] of plain rho at q = 262,111 in M-eq. Its exponent
  is 1.083 [1.040, 1.129] on q ≥ 4,051, against rho's 1.485.
- The frozen stop rule was not triggered, so lead N4 stays **open**.

**N4 v2, wider range and better pipelines** (`research/quartic_n4v2_20261020/`, PR #2597; protocol commit `82a9e87de3`).
- Coverage: 501 IC units and 2,442 rho runs, all verified on `E0`. q runs up to
  4,194,271.
- Frozen bend rule on the exponent over k = 17..22 at w = 0:
  - DL is **below 1** (0.988 [0.986, 0.989]).
  - B0, S, L, D and DL1 fall **between 1 and 6/5**.
  - No arm is classed "toward 1" or "toward 6/5".
- Post hoc reading (labelled in the study): the double-large-prime arms settle at
  0.99–1.02, the `Õ(q)` model; the single-large-prime arms stay below 6/5.
- The intervals resample only 3 units per size. Using N4's conversion constants
  instead moves D's exponent to 1.048, which is more than any interval width.
- Each change against B0 at q = 2,097,091:

  | change | cost relative to B0 |
  |:--|:--|
  | sorted bucketing (S) | 1.001, cost-neutral; makes runs replayable |
  | SGE + block Lanczos (L) | 0.784 |
  | double large primes (D) | 0.081 |
  | D + SGE + block Lanczos (DL) | 0.042 |

  DL1 (D + SGE + scalar Lanczos, 0.022) is cheaper than DL but was not
  pre-registered.
- The negation map makes rho cost 0.715 [0.656, 0.777] of plain rho at w = 0
  (theory 0.707).
- Two of the six pre-registered predictions failed:
  - SGE shrank the matrix to only 0.74–0.78, not half.
  - Block Lanczos cost 3.6× scalar after SGE, more than the predicted 3×.
- Lead N4 stays **open** by the frozen rule.

### Route S: hyperelliptic H, then Smith's isogeny to a quartic

**W3a, the walk on the elliptic side** (`research/smith_elliptic_walk_20261018/`, PR #2589).
- Exhaustive at p = 7..31:
  - Every descended `H` has a rational tractable subgroup (cycle types 1+1+3+3 and
    2+6 occur).
  - 0.53–0.61 of the descended `H` pass Smith's full test. Random curves pass at
    0.1857, so the pre-registered expectation (about 0.186) was falsified, upward.
  - A failing weak `E` has a passing `E` in its 2-isogeny component in 74–98% of
    cases, at a mean distance of 1.5–3.6 steps.
- Sampled walks, p = 2^10 .. 2^20:
  - **Frozen verdict: `o(p)`.** Arm A's cost exponent is −0.14 [−0.80, 0.95].
  - Labelled post hoc analysis: the frozen estimator lets cap-sized censored walks
    dominate at small `p`. Restricted to successful walks at p ≡ 3 (mod 4), the
    cost scales as p^1.13 [1.00, 1.26], which reads as `Θ~(p)`.
  - At p ≡ 1 (mod 4) the walk branches and mostly ends in small trapped sets.
- If the post hoc reading holds, a walk is not negligible next to `Õ(q)` index
  calculus. This is unresolved by the frozen rule.

**W3b, Smith's isogeny ψ on divisor classes** (`research/smith_isogeny_divisor_map_20261018/`, PR #2598).
- ψ passed every check on two independent `Pic^0` engines (W1's keys and a
  degree-6 adjoint engine), with zero failures. Checks covered linearity,
  independence from the base divisor, kernel = S, ψ(G) ≠ 0, and ℓ·ψ(G) ~ 0.
- On the 18 curves with q ≤ 13 the image is exactly `J_H/S`.
- Route S end to end at q ≤ 23 solved 91,944 planted ECDLPs, each checked on `E0`.
  The discrete log in `J_X(F_q)` is brute force here, not index calculus.
- Props. 2 and 4 pass, per planted instance, on 67.9% (q = 7) to 82.5% (q = 11).
  This is not comparable with Smith's 18.6%. W3b's driver only produces degree-7
  `H` (split or linear `h`), and these have cycle type (3,3,1,1), so three rational
  tractable subgroups. W3a's 0.53–0.61 is a different measure: it is per descended
  `H` and includes cycle type 2+6 (irreducible `h`, one subgroup).
- Every attempt that reached a smooth plane-quartic `X` succeeded. The only reason
  an attempt was skipped was a hyperelliptic `X`.
- Computing ψ costs about 1e5 to 1e6 `F_q` multiplications per divisor, with
  polylog growth in `q`.

## Status of the leads

| lead | state | why |
|:--|:--|:--|
| N4 (plane-quartic IC on the GHS-weak family) | open by its frozen rule; **no further work planned** | the program question is answered over the stated range; extending it changes no conclusion about any curve outside the family |
| Smith walk (problem record `smith-kernel-isogeny-class-walk.md`) | open | frozen `o(p)` against post hoc `Θ~(p)`; clustering of weak curves in the isogeny graph is the only identified route to a cheaper walk |
| GHS → Smith pipeline (`ghs-to-smith-pipeline-characteristic-mismatch.md`) | odd characteristic executed at toy q; characteristic 2 still blocked | W3b closes the Prop. 2/4 and trigonal-construction items on toy instances |

## Not done, and what would reopen the program

- **Larger q.** N4 v2 stops at k = 22. B0's point packing needs q < 2^21, and both
  the mod-ℓ addition and N4's check tool overflow 64 bits near ℓ ≈ 2^64. Going
  further needs 128-bit ℓ arithmetic.
- **Index calculus on the Route S quartic.** W3b used brute force; pricing Route S
  against rho is the W2b-equivalent for that route.
- **External comparison.** None is installed, so nothing was compared.
- **Survey paper and weak-curve catalogue.** These results are not yet folded in.
- **Recheck triggers:**
  - a cited characteristic-2 or characteristic-3 trigonal analogue;
  - a walk rule that avoids the p ≡ 1 (mod 4) traps;
  - evidence that weak curves cluster in the 2-isogeny graph beyond density ~1/p.

## Relevance outside this family

- None for ECCp-131, ECC2-131 or any standardized curve. A 131-bit binary field
  (131 is prime) has no `F_{q^3}` subfield to descend to.
- The family is a vanishing fraction of curves over `F_{q^3}`, about `q^{-1}`
  (PROGRAM.md).

## Merge order

#2227 (W1 + N4) first. #2597 (N4 v2) is stacked on it, and #2598 (W3b) depends on
it. #2589 (W3a) is independent.
