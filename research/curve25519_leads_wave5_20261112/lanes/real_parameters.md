> Idea Generator lane report (2026-11-12). Proposals only: nothing was run, no file outside this one was edited, no status changed. Every statement in this file is `[pcc:unverified]`.

# Wave 5, lane "real parameters": Curve25519 leads from its actual p, A, λ, ℓ (2026-11-12)

## Sources checked

- research-kb: no `kb_status`/`search_knowledge` tool in this lane, so "KB unavailable". Lead atlas: no `knowledge/leads/ecdlp/` checkout under `/home/user` (glob empty), so novelty is checked against the brief's tried list and the repository files below only. ecdb: not reachable from this lane. No shell, so every number-theoretic fact below is hand arithmetic, shown in the table, and must be machine-checked (stage 0 of R3) before use [pcc:unverified].
- Repository (read): `research/curve25519_exponent_study_20261108/README.md` (cells, L = 1 fits, validator joint fit, §6 enumeration remark, §8 unpriced work); `research/curve25519_exponent_study_ext_20261110/README.md` lines 1-60 (d = 12/14/18/28 cells, 12 GB memory wall, 2.14 per prime factor); `research/pkm_dickson_torus_20261015/README.md` (torus D = split D for 2-power Dickson towers, B' holds 93 % of memory); `research/leads_20261112/lanes/lane_P_prime.md` (P3 coarse radix, P1 quotient law); `docs/session-logs/2026-11-12.md`, `docs/session-logs/2026-11-06.md` lines 831-859 (atlas summary of `LEAD-ICP-algebraic-membership-factor-base`).
- Literature: Petit-Kosters-Messeng, PKC 2016 ([pdf](https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf); read §2.4-§4.1: condition 2 "L can be written as a composition of low degree maps", §3.3 needs a B-smooth factor of p-1 near p^(1/m), §3.4 auxiliary-curve variant for arbitrary p, Table 1); Amadori-Pintore-Sala 2018 (FFA); "Acceleration of index calculus ... over prime fields and its limitation" (CANS 2018, abstract only); Hu, ePrint [2024/1923](https://eprint.iacr.org/2024/1923) (abstract); Faugère-Lachartre 2010 and GBLA (ISSAC 2016) for split F4 linear algebra; Cheng-Chen-Chou-Niederhagen-Yang, XL with block Wiedemann (CHES 2012, [slides](https://www.iacr.org/workshops/ches/ches2012/slides/S4_Talk1_Niederhagen.pdf)); Kosters-Yeo, [arXiv:1503.08001](https://arxiv.org/pdf/1503.08001). Negative searches: no published comparison of flat x^d = 1 against power-map chains, of subgroup against random univariate bases at equal degree, or of odd-prime torus bases for prime-field ECDLP was found. A negative search is not proof of novelty [pcc:unverified].

## Real-parameter facts used (hand-derived here; every row `[pcc:unverified]`)

| fact | derivation |
|:--|:--|
| p ≡ 5 (mod 8); p ≡ 1 (mod 3), p ≡ 7 (mod 9), so v3(p-1) = 1 | 2^255 ≡ 0 (8); 2^255 ≡ 2 (3); 2^255 ≡ 8 (9) |
| 65147 \| p-1 and 65147 is prime | 2^255 ≡ 20 (mod 65147) by square-and-multiply (2^16 ≡ 389, 2^32 ≡ 21027, 2^64 ≡ 47187, 2^128 ≡ 18803); trial division by every prime ≤ 251 |
| No prime 5 ≤ q ≤ 47 divides p-1 | p mod q computed for each q |
| **C ≈ 2^235.4, not 221 bits** | log2(12·65147) = 19.58; 255 - 19.58 = 235.4. The brief's "about 221 bits" does not fit p-1 = 4·3·65147·C with C prime |
| Divisors of 12·65147 also include 2, 6, 130294, 390882 (missing from the brief's list) | arithmetic |
| p+1 = 2·5^2·M, no prime factor of M below 50 | p ≡ 24 (25), p ≡ 74 (125); p mod q for q ≤ 47 |
| Φ3(p) = p²+p+1 divisible by 3 (exactly once) and 37 | p ≡ 26 (37): 676+26+1 = 703 = 19·37 |
| Φ6(p) = p²-p+1 divisible by 7 (exactly once) and 19 | p ≡ 3 (7): 7; p ≡ 8 (19): 57 = 3·19; p ≡ 38 (49): 1407 ≢ 0 |
| Φ4(p) = p²+1 not divisible by 5, 13, 17, 29, 37, 41 | p mod q |
| PKM §3.3 needs a B-smooth part of p-1 near p^(1/m); for Curve25519 that part is ≤ 2^19.58 for every B < C, so §3.3 applies only at m ≥ 14 (P-224: m ≥ 3 at B = 2, PKM's own statement) | 255/19.58 = 13.02 |
| The k = 20 miniature p = 1,048,573 = 2^20 - 3 already has p ≡ 5 (8) and v2(p-1) = 2; it differs from the real curve by smooth p-1 = 4·3^3·7·19·73; #E = 8·131,203 (t = -1050); p+1 = 2·(2^19 - 1), a Mersenne prime, so it has no torus base beyond order 2 | exponent-study census table; 2^18 - 1 = 511·513 |

**Consequence the proposals test.** Curve25519 has no divisor of p-1 in [781,764, ≈2^235.4]; the useful |V| window (PKM: p^(1/m), i.e. 2^85 at m = 3, 2^64 at m = 4) is empty. Whether that empty window *matters* depends on two unmeasured things: whether the chain presentation (the only reason PKM need smoothness) is the cheapest (R1, R5), and whether subgroup structure matters at all once the base is presented as one univariate (R2, R3). R4 removes the memory wall that left the lab's m = 3 verdict "undetermined" [pcc:unverified].

---

```yaml
id: W5-RP-1-flat-vs-chain-membership
class: measured-improvement candidate (decomposition-oracle component, miniatures)
title: One binomial y^d = 1 per summand instead of the power-map chain, for composite d
obstruction: >
  PKM condition 2 (L a composition of low-degree maps) forces one chain variable per prime factor of d
  per summand. Measured cost tracks that count: validator joint fit 2.14 [1.35, 2.93] in ln T per prime factor
  (x8.5), k = 20, m = 3 (ext README). The chain is also the only reason the mu_d route needs smooth p-1.
real_parameter_fact: Curve25519's smooth part of p-1 is 2^19.58; if the flat presentation is the cheapest, only d | p-1 matters, not smoothness.
claim: >
  At k = 20 (p = 1,048,573), m = 3, sym form, the exponent study's seeds and targets, F4 field_ops of the flat
  presentation (x_i = lo + g*y_i, y_i^d - 1, one variable per summand, factorisation of d unused) at
  d in {9, 12, 18, 21, 27, 28, 36} lies within [lo/3, hi*3] of the band below, and is below the measured
  chain T at every d with L >= 2.
mechanism: >
  F_p[y_0..y_{L-1}]/(y_j^{e_j} - y_{j+1}, y_{L-1}^{e_{L-1}} - 1) is isomorphic to F_p[y_0]/(y_0^d - 1)
  (eliminate y_1..y_{L-1}): same quotient of dimension d^m, same solution set. F4 cannot see how d factors
  when given y^d - 1, so the flat system should cost what a prime d of the same size costs (L = 1 curve,
  solving degree d + 1 in every sym m = 2, 3 prime-d cell).
prediction: |
  d (L): chain T measured -> flat band [lo, hi] = [min, max] of two derived estimates: (i) L = 1
  interpolation through d = 7 (3.30e6) and d = 19 (2.20e8) at slopes 4.2-4.95; (ii) chain T / 8.5^(L-1)
   9 (2):  1.48e8 -> [9.5e6, 1.8e7]      21 (2): 2.79e9  -> [3.3e8, 3.6e8]
  12 (3):  7.75e8 -> [1.1e7, 4.8e7]      27 (3): 2.63e11 -> [1.0e9, 3.6e9]
  18 (3): 2.43e10 -> [1.7e8, 3.4e8]      28 (3): 2.02e11 -> [1.1e9, 2.8e9]
  36 (4): out of memory at 12 GB -> [3.2e9, 5.9e9], completing under 3.5 GB
  Chain/flat ratio predicted 70x-260x at d = 27, 28; >= 7x at d = 9, 21.
discriminating_test: >
  Rust: let `subgroup_semaev.rs` accept the factor list [d] (L forced to 1) behind a `--flat` flag; run
  `curve25519_family f4sub` on the 7 cells x 6 targets (seed 20261108 + 1000 m + d) on a 4-core box. Sum of
  the band's highs x 6 is about 8e10 field ops, minutes at the recorded 2.9e8 ops/s. Secondary: m = 2 at
  d in {9, 21, 27, 63, 133, 171, 189}, where chain cells exist.
falsification: >
  Flat T above chain T at >= 2 of the 6 measured composite cells, or outside [lo/3, hi*3] at >= 3 cells:
  the factorisation of d then matters through the algebra, and the smooth-p-1 premise stands.
cost_band: >
  Reference arm: chain F4 (measured, column above). Candidate: flat F4 (band). Matched rho at k = 20:
  1.5*sqrt(131,203) = 543 group operations (README §8 convention); no field-op conversion is recorded, so the
  distance to rho stays the README's >= 3.98e5 field_ops per rho group operation. Unpriced native work, equal
  in both F4 arms: per-target symmetrisation about 3.1e4 multiplications at m = 3 (validator), priced at 0,
  0.1, 1 GAE per operation; the order of the arms is claimed only where it holds across the band (any cell
  with a >= 2x gap, since 3.1e4 is below every T).
controls: >
  Positive: flat at prime d = 19 must equal the chain cell 2.20e8 to the unit (identical system). A/A: rerun
  chain d = 21 (2.79e9). Consistency flags per target identical between arms (same solution set). Coset
  gidx 0/1/2 at d = 27 within 1.5x.
risks: >
  Max degree rises to d + 1 (37 at d = 36); the k = 22 L = 1 cell at d = 41 completed (7.3e9), so the band
  is reachable. m = 4 is not rescued: prime d = 19 at m = 4 is already out of memory at 12 GB. The gain is
  a fixed-|V| component ratio; the L = 1 exponent (4.2-4.95 at m = 3) stays far above a lever (s < 0) and
  far above enumeration (|V|^(m-1)), so the distance to rho does not change.
closest_baseline: >
  PKM 2016 §3.1 condition 2 and §3.3 (chain of x^{p_j}); lane P P3 (coarse radix, 2-power chains): this is
  the extreme radix r = d for odd and even composite d, with measured reference cells already on disk.
why_not_tried: The tried list names "mu_d bases with chain presentations"; the module implements only the chain (exponent README §1); no flat composite-d cell exists on disk.
prior: 0.65 that flat < chain at >= 5 of 6 cells and inside the band.
lean_obligation: >
  The chain-to-flat ring isomorphism (an AdjoinRoot tower equals AdjoinRoot of y^d - 1) and equality of the
  F_p-solution sets of the two m-summand systems. Not written; about 150 lines over MvPolynomial quotients.
```

```yaml
id: W5-RP-2-binomial-vs-dense-vs-random-base
class: security statement (where Curve25519 sits); improvement only if a dense base is cheaper
title: Does the subgroup matter at L = 1? Binomial mu_d, the same set as a dense polynomial in x, a random set
obstruction: The lab's prime programme assumes a structured V (d | p-1 or p+1); Curve25519's useful window has none. Unknown whether structure buys anything once V is one univariate.
real_parameter_fact: No d | p-1 in [781,764, 2^235.4]; a random set of any size exists on every prime.
claim: >
  At k = 20, m = 2 (d in {19, 27, 57, 73}) and m = 3 (d in {7, 19, 21, 27}), with each x_i constrained by
  (a) y^d - 1 and x = lo + g*y (flat, as R1), (b) the same set as the expanded degree-d polynomial in x,
  (c) a uniformly random d-subset of F_p as prod(x - v): T_b/T_a in [1, 8] and T_c/T_b in [0.5, 2] in every
  cell, with identical solving degree across arms.
mechanism: >
  For squarefree split f with roots V, F_p[x]/(f) is isomorphic to F_p^V, so the system is consistent iff
  S_{m+1} has a zero on V^m: V enters only through which points sum to the target. F4's work then depends on
  f only through its sparsity in the chosen variable. (b) vs (c) isolates set structure; (a) vs (b) isolates
  sparsity.
prediction: "T_b/T_a: point 2, interval [1, 8]. T_c/T_b: point 1, interval [0.5, 2]. Solving degree d + 1 in all arms at m = 2, 3."
discriminating_test: >
  Rust: a `Membership::Dense(f)` variant (f built from its roots, d^2 multiplications). 8 cells x 3 arms x 6
  targets at k = 20. Then arm (c) on a Curve25519-matched miniature at k = 20: p ≡ 5 (mod 8), v2(p-1) = 2,
  p - 1 = 12*q*C with q < 16 prime and C prime (no divisor of p - 1 in [16, 1024], the k = 20 analogue of the
  empty window), Montgomery A with A^2 - 4 a non-square, #E = 8r and twist order 4r' with r, r' prime;
  predicted T_c within [0.5, 2] of the k = 20 arm (c). Under 2 hours on 4 cores (largest cell about 2e11).
falsification: >
  T_c/T_a > 16 in >= 2 of the 4 m = 3 cells: binomial sparsity is a real lever and Curve25519's empty window
  is a measurable defence of that factor (reported as the security statement). T_c/T_b outside [0.25, 4]
  in >= 2 cells: set structure matters beyond sparsity. T_c/T_a < 0.5: a random dense base is cheaper
  (improvement candidate; mu_d would be obsolete).
cost_band: "As R1, plus building f (<= 5e3 multiplications per curve, priced in); rho 543 group ops; 0/0.1/1 GAE band on the symmetrisation, equal in all arms."
controls: >
  Random sets from 3 seeds per cell: spread <= 1.5x. Covered-set yield of (c) follows binomial(d+m-1, m)/N,
  as in LEAD-ICP-algebraic-membership-factor-base. Positive: (a) at prime d reproduces the frozen cells to the unit.
risks: Dense f may fill in and hit memory first; T mostly measures refutation (8 of 414 systems consistent in the parent study); a factor-8 band is wide.
closest_baseline: >
  LEAD-ICP-algebraic-membership-factor-base (subgroup yield equals random-base yield; msolve deg^4.4 on
  subgroup bases at m = 3, per the 2026-11-06 atlas summary): yield, not solver cost on a random set.
  PKM §2.4 (L = prod(x - v) as the general constraint). Exponent study gidx test (cosets of one subgroup only).
why_not_tried: No committed cell runs F4 on a non-subgroup univariate base; the bit-interval base is a Boolean chain, not one univariate.
prior: 0.55 inside both intervals; 0.20 that the binomial is a > 16x lever; 0.05 that dense is cheaper.
lean_obligation: >
  For squarefree split f with root set V: F_p[x]/(f) ≃ (V -> F_p), and (S) + (f(x_1), ..., f(x_m)) = ⊤ iff
  S has no zero on V^m. Mathlib has the Chinese remainder equivalence for pairwise coprime ideals; about 200
  lines; not written.
```

```yaml
id: W5-RP-3-torus-trace-bases-census
class: security statement
title: Torus bases on p+1 and the cyclotomic tori T_3, T_4, T_6: census for Curve25519 and one odd-prime Dickson-trace arm
obstruction: The brief asks whether torus bases (d | p+1, trace coordinates) or higher tori give Curve25519 a structured window that p-1 lacks.
real_parameter_fact: p+1 = 2*5^2*M (M without factors below 50); 3*37 | Phi_3(p); 7*19 | Phi_6(p) (table above).
claim: >
  (C1, desk) For k in {1, 2, 3, 4, 6}, the largest 2^32-smooth divisor of Phi_k(p) is below 2^64.
  (C2, measured) On a matched miniature with an odd prime d | p+1, the T_2 base V = {lo + g*(u + 1/u) : u^d = 1}
  of size (d+1)/2, presented by the dense polynomial (t - 2)*h(t) of degree (d+1)/2, has T within [0.5, 2]
  of R2's arm (c) (random dense set of the same size) at m = 2 and 3.
mechanism: >
  D_d(u + 1/u) = u^d + u^-d and D_d(t) - 2 = (t - 2)*h(t)^2 for odd d, so the membership polynomial is a dense
  univariate in t; F4 sees only that. The 2-power case already gave torus D = split D, "same F4 computation
  on different numbers" (pkm_dickson_torus §2-§3).
prediction: "C1: prior 0.6 jointly over the five k. C2: ratio point 1, interval [0.5, 2]."
discriminating_test: >
  C1: complete small-factor census of Phi_k(p) with GMP-ECM / PARI (native tools, no Python), B1 to the
  2^32 level; also confirm every row of the fact table. Hours of desk CPU. C2: a new k = 20 prime with
  p ≡ 5 (8), v2(p-1) = 2, 37*53 | p+1 (sizes (d+1)/2 = 19 and 27, matching R2's cells), cofactor-8
  Montgomery curve (the k = 20 miniature cannot serve: its p+1 = 2*(2^19 - 1)); dense variant from R2; minutes.
falsification: >
  C2 ratio outside [0.25, 4] in >= 2 cells: torus structure matters beyond density. C1: a 2^32-smooth divisor
  >= 2^64 of some Phi_k(p): Curve25519 has a structured window at that k; it is then priced as a chain base,
  which R1 predicts loses, so this flags rather than opens a lead.
cost_band: "C1 desk only. C2 as R2; rho reference 1.5*sqrt(r) of the C2 curve; same 0/0.1/1 band."
controls: "C2, on the new prime: torus d = 37 vs random dense sets of size 19 (3 seeds); cross-reference to R2's k = 20 arms at d = 19."
risks: C1 is a fact table and changes nothing if R2 finds structure irrelevant; T_3/T_6 bases need a rational map T_k -> F_p (trace) whose orbit collapse (size about d/k) is not derived here.
closest_baseline: pkm_dickson_torus_20261015 (2-power Dickson torus, H_same); PKM §3.3; XTR (Lenstra-Verheul 2000) and CEILIDH (Rubin-Silverberg 2003) trace compression of T_6, T_2.
why_not_tried: The lab's torus arm was 2-power Dickson chains at T24/T31; no odd-prime torus base and no census of Phi_k(p) for Curve25519 is on disk.
prior: 0.7 that C2 holds; 0.6 that C1 holds.
lean_obligation: >
  Mathlib's `Polynomial.dickson_one_one_eval_add_inv` (to be confirmed at the pinned revision) gives the
  first identity; D_d - 2 = (t - 2)*h^2 for odd d is a new lemma; the census divisibilities (25 | p+1,
  37 | Phi_3(p), 133 | Phi_6(p), 65147 | p-1, Nat.Prime 65147) by norm_num.
```

```yaml
id: W5-RP-4-sparse-split-f4-memory
class: measured-improvement candidate (solver component: memory, field_ops neutral or lower); instrument for the undetermined m = 3 verdict
title: Faugère-Lachartre sparse/dense split in f4_fp against the 12 GB wall, with a block-Wiedemann arm priced
obstruction: The ext study's binding limit is memory (k20 m3 d36, k20 m4 d12/d19, k22 m3 d55 died at 12 GB), so the preregistered m = 3 estimator never applied (span 3.1x < 4x).
real_parameter_fact: None of Curve25519's own; this unblocks the measurement that decides whether any subgroup-base lever exists at all.
claim: >
  On the four complete ext cells (k = 20, m = 3, chain, d = 12, 14, 18, 28) an F4 whose pivot block A and the
  matching C block stay sparse (only B|D densified) returns identical solving degree and consistency flags,
  with peak RSS <= 1/4 of dense f4_fp and field_ops in [0.5, 1.2]x; it then completes k = 20, m = 3, d = 36
  (L = 4) under 12 GB. The k = 20 m = 3 series then spans d = 9..36 (4x), the estimator applies, and the
  fitted s lies in [4, 8] (verdict "not met").
mechanism: >
  Chain rows are binomials and sym S rows have about 35 terms; dense storage of reducer rows dominated memory
  on the tower engine (B' = 93 % of peak, pkm_dickson_torus §5). Keeping pivots at natural sparsity attacks
  exactly that storage.
prediction: "Peak RSS ratio <= 0.25 (point 0.15) on >= 3 of 4 cells; field_ops ratio in [0.5, 1.2]; d = 36 completes; s in [4, 8]."
block_wiedemann_arm: >
  XL at the measured solving degree, consistency as a sparse linear system: cost about 3*N*nnz. At d = 36
  chain (15 variables, D about 10, N = C(25, 10) = 3.3e6, nnz about 3.3e7) that is about 3e14 field ops,
  about 250x the roughly 1.3e12 dense F4 spent before its abort (4,521 s at 2.9e8/s). With the flat plain
  form (R1) the quotient caps N at d^m = 46,656 but 3*N*nnz is still about 8e11, above the flat F4 band.
  Derived; memory instrument only.
discriminating_test: >
  Rust change in `f4_fp` (split storage, same pivot order, same field_ops counter). Replay the 4 ext cells
  at frozen seeds, peak RSS from `/usr/bin/time -v`, then d = 36 under 12 GB / 10,800 s per target. About a
  day of engineering, <= 8 hours of runs on 4 cores.
falsification: >
  RSS reduction < 2x on >= 3 of 4 cells, any basis or flag mismatch, or d = 36 still out of memory: the wall
  is the reduced basis, not reducer storage, and only a larger host decides m = 3.
cost_band: >
  Reference: dense f4_fp (measured field_ops). Candidate: split F4. BW arm: derived above. Rho as R1. Unpriced
  native work of the split (row hashing, sort, index bookkeeping) priced at 0, 0.1, 1 GAE per row operation;
  the memory claim holds at every weight; the field_ops claim is ranked only if it holds across the band.
controls: Byte-identical reduced bases on d = 12, 14 (cheap); A/A on d = 18; m = 2 d = 133 as an L = 2 control.
risks: f4_fp's memory composition is not measured (the 93 % figure is from the tower engine); fill-in of B|D may dominate at d = 36.
closest_baseline: Faugère-Lachartre (PASCO 2010) and GBLA (ISSAC 2016); XL + block Wiedemann (CHES 2012); disk-backed B' (pkm_dickson_torus §5, not built).
why_not_tried: No solver-memory item in the tried list; the ext study names "a sparse or degree-bounded engine" as its open engine change (session log 2026-11-12 03:20Z).
prior: 0.4 for the full claim; 0.8 that s is "not met" if d = 36 completes.
lean_obligation: Soundness only: a row-space certificate of 1 at degree D implies 1 in the ideal, so an "inconsistent" flag is certified. Completeness is not claimed.
```

```yaml
id: W5-RP-5-crt-tensor-presentation
class: measured-improvement candidate (miniatures), low prior
title: For coprime d = d1*d2, present mu_d as mu_d1 x mu_d2 with x = lo + g*a*b
obstruction: Between the chain (L variables, top variable of degree-d meaning) and the flat binomial (one variable of degree d) there is a third presentation that lowers univariate degrees without a chain.
real_parameter_fact: Every composite divisor of Curve25519's p-1 below C is built from 4, 3, 65147 (pairwise coprime); the matched family's p - 1 = 12qC likewise.
claim: >
  At k = 20, m = 3, d in {12 (4*3), 21 (3*7), 28 (4*7)} and m = 2, d in {21, 57, 133, 219}, the tensor system
  (a^d1 = 1, b^d2 = 1, x = lo + g*a*b) costs less than the chain at >= 2 of the 3 m = 3 cells and more than
  the flat binomial (R1) in every cell.
mechanism: Two univariates of degrees d1, d2 replace one of degree d; x becomes bilinear, doubling the Semaev degree per summand in (a, b). The trade is the experiment.
prediction: "T_tensor/T_chain in [0.1, 1] at >= 2 of 3 m = 3 cells; T_tensor/T_flat in [1, 20] in all cells."
discriminating_test: One more membership variant in `subgroup_semaev.rs`; same seeds and cells; under 1 hour on 4 cores.
falsification: Tensor below flat by >= 2x in any cell (a new presentation lead, beating R1); tensor above chain in all cells (close the axis).
cost_band: "As R1: chain (measured) and flat (R1) are the reference arms; rho 543 group ops; same 0/0.1/1 band."
controls: Same consistency flags as chain and flat per target; d prime (no split) is excluded by construction.
risks: Bilinear x may raise the solving degree enough to lose everywhere; three m = 3 cells are few.
closest_baseline: PKM §3.3 (chain); lane P P3 (radix axis); exponent study chain cells.
why_not_tried: Not in the tried list; only chain presentations exist in the module.
prior: 0.15 that tensor beats flat anywhere; 0.5 that it beats the chain at >= 2 cells.
lean_obligation: For coprime d1, d2 in a field with primitive d1*d2-th roots, mu_(d1 d2) ≃ mu_d1 x mu_d2 via multiplication; about 80 lines.
```

---

## Ranking

1. **R1 (flat vs chain).** Under an hour, frozen reference cells already on disk, and the predicted ratio (70x-260x at d = 27, 28) is large enough that either outcome is decisive. It is also the hinge of "where Curve25519 sits": if the flat binomial is cheapest, smooth p-1 stops mattering and only d | p-1 does.
2. **R2 (structure at L = 1).** Decides whether the empty window of Curve25519 is a defence at all. Cheap, three arms, one Rust variant.
3. **R4 (split F4).** Engineering, but the only route that turns the m = 3 verdict from "undetermined" into a reading inside 12 GB.
4. **R3 (tori).** Requested; expected to add a fact table and one null.
5. **R5 (tensor).** Same harness as R1; low prior.

None of the five is predicted to bring any row closer to matched rho. R1 and R5 can give a measured component gain over the chain-presentation decomposition oracle at fixed |V|; R4 a memory gain. The distance to rho stays at the exponent study's illustration of >= 3.98e5 field_ops per rho group operation at k = 20, m = 2 [pcc:unverified].

## Considered and not proposed (category iv, real-parameter levers), with reasons

- **λ = 121665/121666 and A = 486662 small height.** Over F_p coefficient size is invisible to every algebraic method; height enters only through lifts to Q, which are closed (tried list: dense lifts; Xedni-type lifting fails by height growth, Jacobson-Koblitz-Silverman-Stein-Teske 2000). Frames are tried.
- **ℓ = 2^252 + c, c of 125 bits.** ℓ is prime, so generic structure is that of any prime-order group. Its shape speeds reduction mod ℓ in the linear-algebra phase, which is 0.08-2.5 % of cost in `mt_fieldops_v3`. Factors of ℓ ± 1 matter only to Cheon-type attacks with auxiliary inputs (an oracle setting, chased in `research/cheon_aux_leads_20261112`), not to the ECDLP.
- **p = 2^255 - 19 carry structure.** Helps SNFS for logarithms in F_p^*, but the embedding degree is huge, so no transfer. Small-x / height-box decompositions are closed by the convex reach bound 0.354 < 1/2 (lane P P5).
- **Enumeration or NTT grid-evaluation oracle on mu_d.** By R2's lemma, consistency is "S has a zero on V^m", testable in O(d^m log d) by an m-dimensional NTT (needs d | p-1) or d^(m-1) enumerations plus a root test. Both beat F4's d^4.95 at m = 3, but the total is then p by the conservation identity. That is known (exponent README §6), so not a new lead.
- **μ_4 ⊂ F_p, the full 2-torsion of E', the Z/8 cofactor.** Torsion symmetrisation, cofactor cosets and torsion-shifted targets are tried. The FGHR-type (Faugère-Gaudry-Huot-Renault 2014) invariant reduction needs a base closed under the torsion action, which is the tried symmetrisation.
- **v2 = 2 at p-1 and v2 = 1 at p+1.** Every 2-power chain (Kummer or Dickson) collapses at μ_4 / T_2[2]. Over F_(p^(2^j)) the 2-adic valuation grows only as j + 2 (lifting the exponent), so extension towers give no Kummer chain either.

## Process notes

- I had no shell, so the "small Python checks" were done by hand and are flagged as such. R3's stage 0 must recompute every row with native tools before anything relies on them. The correction of C's size (about 2^235.4, not 221 bits) should be checked first [pcc:unverified].
- Every test names a Rust change in `src/cryptanalysis/ic_framework/subgroup_semaev.rs` or `f4_fp`, or a native factoring tool. No Python harness is proposed (AGENTS.md "Implementation language: no Python").
- Experiment routing, resume checkpoints and bench records are the director's and executor's to freeze. I authorise no run.
