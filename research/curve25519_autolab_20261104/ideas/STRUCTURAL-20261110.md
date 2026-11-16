# Structural routes to 0.75x rho on a prime-field curve: what is left after the closed table (2026-11-10)

Owner question: devise new structural ideas for S / rho-neg <= 0.75 on the Curve25519-family miniatures, index calculus only (BSGS-class excluded, owner ruling 2026-11-08). Everything below is derivation, not measurement [pcc:unverified]. The research-kb server was unavailable in this session; the lead atlas is not on disk. A literature sweep runs separately (`LITERATURE-20261110.md`).

## 0. The one sentence

Every route in this family reduces to a count of pairs, and the count has two regimes: with few unknown logarithms the relation test is cheap but the Poisson floor binds; with many unknown logarithms the floor loosens but the relation test costs at least one group operation per base element. Over a prime field no subset of x-coordinates is known that is both large and cheap to test, and section 4 says why one is unlikely to exist. The two accounting escapes (many targets, shared generator) are closed in section 1 by the same count. What remains is a short list of checks with expected negative outcomes and a precise statement of what a winning idea must contain.

## 1. The accounting escapes, closed by the floor itself

### 1.1 Many targets at once

A walk with C unknown seeds and T unknown targets stops at rank K = C + T. The Poisson floor (OBSTRUCTION.md) gives expected points E_K = sqrt(r) Gamma(K + 1/2) / Gamma(K). Rho on the same T targets is the K = T case of the same formula (no seeds), which is the Kuhn-Struik multi-target cost. Per target:

    IC(C, T) / rho(T) = [Gamma(C+T+1/2) / Gamma(C+T)] / [Gamma(T+1/2) / Gamma(T)]  ~  sqrt(1 + C/T)

| T | C = 1 | C = 2 |
|--:|--:|--:|
| 1 | 1.50 | 1.875 |
| 2 | 1.26 | 1.46 |
| 4 | 1.12 | 1.24 |
| 8 | 1.06 | 1.12 |
| 32 | 1.015 | 1.03 |

The ratio to the matched multi-target rho tends to 1 from above and never crosses it. A single-target rho reference against a multi-target IC arm would show 0.4x at T = 8, which is a mismatched reference, not a result. So multi-target accounting does not open the 0.75 line; it closes the gap to 1.0 asymptotically, which is the parity the loop's long-term goal names, reached by amortisation rather than by a better relation source.

### 1.2 Shared generator (the realistic Curve25519 threat model)

Every Curve25519 user shares G, so a target-independent table is realistic. The C = 2 incumbents already have one: the walk to rank-2 seed rows is target-independent and the per-target marginal cost is the probe phase, 0.39 / 0.37 / 0.50 sqrt(r) at k = 24 / 28 / 32 (OBSTRUCTION.md headroom table) [pcc:unverified]. But the matched generic reference under this accounting is rho with precomputation (Bernstein-Lange 2012, distinguished-point tables): about r^(2/3) precomputation, r^(1/3) storage, about r^(1/3) group operations per target. At k = 32 (r about 2^29) that is about 1.1e3 operations per target against the IC probe phase's about 8.8e3 (0.38 sqrt(r)); the generic method wins by about r^(1/6). Shared-generator accounting therefore moves the comparison further against index calculus, not toward 0.75.

Both escapes are owner decisions about the metric, and under either one the matched reference changes with the arm. Neither is an algorithmic idea, and neither yields 0.75 against its matched reference.

## 2. New mechanisms considered and closed by counting

| idea | mechanism | why it fails | number |
|:--|:--|:--|:--|
| Three-point relations (Semaev S_3 on walk points) | a zero-sum triple among T points appears at T ~ (6r)^(1/3), far below sqrt(r) | detecting it needs x(E_a + E_b) for every pair and a lookup: T^2 group operations (the 3SUM barrier); S_3 root-finding in z IS the addition x(E_a +/- E_b) | T^2 = (6r)^(2/3) >> sqrt(r); at k = 32, 2^20.7 against 2^14.5 |
| Small-root decomposition (Coppersmith / Jochemsz-May on summation polynomials with an interval base) | find x_1..x_m < N with S_(m+1)(x_1..x_m, x_R) = 0 mod p by lattice reduction instead of trial | the product of root bounds is N^m ~ r ~ p; lattice methods need it below about p^(1/deg) | bounds off by a factor p^(1 - 1/deg); no regime |
| Quotient by a scalar lambda of small multiplicative order mod r (fold by m > 2) | classes of size m instead of 2 | a canonical representative needs [lambda]P with lambda ~ r: about 1.5 log2 r additions per step against a gain of sqrt(m); an orbit invariant via division polynomials has degree lambda^2 ~ r^2 | at k = 32: 43 additions per step to gain sqrt(3) = 1.73 |
| Kummer line / twist / F_(p^2) walk | x-only steps are cheaper; E and its twist share the x-line | x-only pseudo-addition needs the difference, so no random walk; E(F_(p^2))[r] = E(F_p)[r] (SYNTHESIS section 2), so nothing is added to the subgroup | no gain |
| Scalar clamping (real Curve25519 keys) | private scalars lie in 2^254 + 8 [0, 2^251) | an interval of size about r / 2: kangaroo costs 2 sqrt(r/2) = 1.41 sqrt(r) against rho-neg's 0.886; ecbench targets are uniform anyway | loss |
| Walk in the full group E(F_p) (order 8r) with the 4-fold x / 1/x canonicalisation | free torsion translates P + T_0 | folding {+-P, +-P + T_0} over 8r points leaves 2r classes against r / 2 in the subgroup; a collision across cosets gives nothing (P - Q in <G> and in E[8] is O) | 4x worse |
| MOV / Frobenius transfer | finite-field smoothness is a relation source that is not a collision | needs small embedding degree; the family's curves are not chosen for it, and choosing one that has it is fenced (no curve selection to a target); for a 32-bit r the transferred field is hundreds of bits | a one-hour audit (section 5), expected negative |

## 3. Why the floor is not an artefact of random jumps (what NC-1, NC-2 added)

Designed point sequences were tested at K = 1, 2, 3 (`../nc1`, `../nc2`, `../snc`): held-out designs sit within 3 percent of the random-jump walk and 3 to 5 percent above the Poisson floor. The incidence bound (NC-2) allows 0.667 / 1.01 / 1.26 at K = 1 / 2 / 3, but no one-addition-per-point design realises it, and even the bound is above 0.75 of rho's measured S at k >= 24 for K >= 2.

## 4. The structural obstruction, stated as a dichotomy

Let B be the factor base (points whose logarithms are unknown, N of them) and let a relation be a decomposition R = b_1 + ... + b_m with b_i in B, found by some test on R.

1. If the test costs at least one group operation per base element (compute R - b_i and look it up), the total is at least N (operations per R) x (2 r / N^2) (R's per relation, m = 2) x N (relations) = 2 r, independent of N. With a precomputed pair table it is N^2 / 2 + 2 r / N >= 1.9 r^(2/3). Either way above sqrt(r) at every size in play (k = 32: 2^19.6 against 2^14.5).
2. If the test costs O(1) native operations independent of N, then membership of a sum in B is decidable without computing the sum. In the subgroup <G> = Z/r (prime), a set whose sumset is structured enough to test is, by Freiman-Kneser-type structure in Z/r, a union of a few arithmetic progressions in the exponent, which is a base with a few unknown logarithms (the progressions' bases and strides): the C-seed, G-jump designs already measured. The collision floor for K = C + 1 unknowns then applies.

A winning idea has to sit between these: a base B of about sqrt(r) (or r^(1/3)) points with many independent unknown logarithms, and a test for "R in B + B" (or B + B + B) costing O(1) native operations that does not go through the group law. Over F_(2^n) the subspace bases of Diem and Petit-Quisquater are exactly this (the summation polynomial becomes a linear system over F_2). Over F_p there is no subfield and no subspace, and the only additive structure, intervals, puts the problem in the small-root regime of section 2, where the bounds are off by a factor near p. I know of no candidate set, and the repository's algebraic track (interval and isogeny-structured bases with Groebner tests, about 18x rho at k = 20 [pcc:unverified]) is the measured form of regime 1.

This is a derivation in a model, not a theorem about all algorithms (Shoup's bound covers generic algorithms only). It says what a counterexample must contain.

## 5. What is worth doing (none of it reaches 0.75)

- Embedding-degree audit of the family curves at k = 20 to 32 (ecdb or a one-line order computation): closes MOV for the record. Expected: embedding degree of order r. One hour, no group operations.
- A multi-curve holdout for the k = 20 and k = 28 ratchets (the two sizes round 37 left null on one curve): ratchet work on the main track, not progress toward 0.75.
- The literature sweep (`LITERATURE-20261110.md`): if any prime-field result since 2015 gives a sub-birthday single-target count or an O(1) decomposition test, it goes to the top of this file. Expected: none that survived review.

## 6. Recommendation

Keep the 0.75x goal only if a relation source outside sections 1 to 4 is named. Otherwise the honest targets for this family are the ones the floor allows: the main-track ratchet toward the K = 3 floor (1.66 sqrt(r), about 1.6x to 1.7x measured rho at k = 28 to 32, where the incumbents now sit at 1.74 to 1.78), and multi-target parity (section 1.1) if the owner wants amortised accounting with its matched multi-target reference.
