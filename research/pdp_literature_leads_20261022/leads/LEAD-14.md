# LEAD-14 — Prime-field digit and Kummer-tower systems re-modelled as multi-homogeneous blocks

**Verdict.** closed-by-theorem. The block bounds the card names (Faugère–Safey El Din–Spaenlehauer bilinear, Faugère–Spaenlehauer–Svartz unmixed multilinear) do not apply to these systems: two structural hypotheses fail (§5), and their formal values exceed the measured solving degree at every m ≥ 3 size tabulated, with a slope about 3 to 7 times too steep. Not closed here, and not a block-bound question: a Fröberg-type or last-fall-degree law for tower-quotient algebras (LEAD-05's ground).

**Confidence.** medium, because the hypotheses and values are verified at source and by exact arithmetic (an unverified desk check), while the sparse-to-total degree conversion of Theorem 6.2 and the bodies of Amadori–Pintore–Sala and Kudo et al. are unsettled, though unable to reverse the verdict.

**Expected gain if it works.** Exponent: none, and the sign is wrong. A valid bound of this type is linear in N with slope (m−1)/m (0.5 to 0.75 for m = 2–4), against a measured 0.11 to 0.17 per unit N. By `RESEARCH_PKM_TOWER_ORACLE.md` §3.6, slope ≥ 1/2 means width ≥ 2^{N−1}, at least √r per target. Constant: unknown. The nearest measured analogues (signature F5: D_sig > D and 1.7–28× the memory, rejected; early full-rank exit: 25–39 % of multiply-adds at m = 4, ≤ 0.1 % at m = 3) suggest at most about 1.5×, on a stage diagnostic only. The PKM tower has no scoreboard row, so no baseline row moves.

**Cost of the discriminating test.** T1 (§6): 0.5 person-day, under 1 CPU-minute, kilobytes. The desk check in §4 already performs it, unverified.

## 1. Mechanism (precise)

**1.1 What PKM write, corrected against the source.** The card says PKM write x_i in digits with Boolean constraints. They do not (PKC 2016, read at source). §3.1, Algorithm 1: F = {(x,y) ∈ E(F_p) : L(x) = 0}, L = ∘ L_j a composition of low-degree maps, and system (4) is S_{m+1}(x_{11},…,x_{m1},X) = 0, x_{i,j+1} = L_j(x_{ij}), 0 = L_{n0}(x_{i,n0}). §3.3 (smooth p−1): V = the subgroup of order ∏p_j of F_p^*, L_j = x^{p_j}, L_{n0} = 1 − x^{p_{n0}}; with every p_j = 2 this is the Kummer tower x_{j+1} = x_j², x_t² = 1 (the repository's, g = 1). §3.4: V = x-coordinates of a coset of a smooth subgroup of an auxiliary curve, L_j = Vélu x-maps (the repository's isogeny tower). Neither has digits or Boolean equations. Boolean digit presentations come from Semaev's interval V = {x < p^{1/m}} (PKM §2.1, no algorithm given), the autoresearcher's base-2 "digit twin" (KN-OPEN-d6ad3f, -02200b, -2e7514), and binary Weil descent. PKM's experiments (§4) are all m = 2 (S_3), |V| = 2^{n1}, n1 = 1…11, p ≈ 2^{2n1}.

**1.2 PKM's cost statement and its Gröbner dependence.** §3.2: cost = P(p,m) + (m!·p/(deg L)^{m−1})·T(E,m,L) + (deg L)^ω, which beats generic methods iff T < p^{1/2−1/m} when (deg L)^m ≈ m!·p. T, the Gröbner cost, is left open ("The complexity of our algorithms remains an open problem"). No first-fall or regularity assumption is invoked for their system (§2.3 calls the binary one "also known to be false in general"). Their one yardstick is Bardet's semi-generic degree (3+n)/2 for n quadrics plus one quartic; they observe "a much smaller dependency in n". There is no PKM heuristic complexity to inherit.

**1.3 The systems, m = 3, t levels per summand, N = 3t.** S_4 was computed on three random integer (a, b, x_R) draws (unverified desk check): 125 monomials, degree 4 in each x_i, total degree 12, top monomial x_1⁴x_2⁴x_3⁴ with coefficient 1, which matches KN-OPEN-02200b.

| presentation | unknowns / equations | block degree of the summation equation | membership equations |
|:--|:--|:--|:--|
| Kummer tower, raw `full` | 3t / 3t+1 | S_4(g·y_{10}, g·y_{20}, g·y_{30}, x_R): (4,4,4), total 12, level-0 unknowns only | y_{ij}² − y_{i,j+1}: degree 2 in its own block, 0 elsewhere |
| Kummer tower, reduced (what `f4_fp_tower` measures) | 3t / 3t+1 | x^e ↦ ∏_{j∈bits(e)} y_j: tower weight ≤ 2 per block (x³ = y_0y_1), top weight 6, single top monomial c·∏y_{i0}y_{i1}, c = coefficient of x_1³x_2³x_3³ ≠ 0. S̄ lives on levels 0–2: a symmetric 5×5×5 tensor of flattening rank 5 | square-free normal forms |
| chain (the measured m = 3 line) | 3t+1 / 3t+2 | f_1 = S_3(x_1,x_2,u), f_2 = S_3(u,x_3,x_R): multi-affine in the summand blocks after reduction (weight ≤ 1), degree 2 in u. S_3 at fixed x_R is a 3×3 matrix of rank 3; S_3 is a 13-monomial 3×3×3 tensor of flattening rank 3 | as above |
| integer digits x = Σ_{j<s} 2^j a_j, V = [0,2^s) | 3s / 3s+1 | unreduced (4,4,4); modulo a² = a, min(4,s) per block (top ∏L_i⁴ is nonzero mod squares only for s ≥ 4, p > 3) | a² − a: degree 2 in its block |
| product digits x = g∏(1 + c_j b_j) | N / N+1 | block degree t − v₂(e) for x^e, so t for odd e (the factor ζ^{e2^j} − 1 vanishes exactly when 2^t | e·2^j) | b² − b |

So "degree 4t-ish per digit block" is what no presentation gives: 4 for integer digits (linear in the digits), at most 2 in tower weight for Kummer, t for product digits. The product-digit ring is the Fourier dual of the tower and strictly worse (x has Boolean degree t, tower weight 1). In the Veronese picture S_{m+1} is a multilinear (1,…,1)-form on the vectors (1,x_i,…,x_i^{2^{m−1}}), with block size 2^{m−1} independent of t. That is the real block structure, and it matches the "level window" trace (`ic_next10_20261014/03-pkm-degree` README §2.3, "item 3": F4's trace is identical in t up to the first top-degree step). The measured chain systems carry it; the tower quadrics spoil multilinearity.

**1.4 Bounds the formulation would be expected to obey.** (a) Multi-homogeneous Bézout in (P^t)^m: the tower alone gives 2^N (the grid, exact); replacing one tower quadric by S gives 2^{N+1}; generic multilinear gives N!/(t!)^m. It counts solutions, not degrees, and is the closed BKK/MV = Bézout item. (b) FSS (1001.4004) Thm 6.1: n_x + n_y generic affine bilinear equations have d_reg ≤ min(n_x, n_y)+1. (c) Faugère–Spaenlehauer–Svartz (1402.7205) Thm 6.2: a regular sequence of n polynomials of multidegree (d_1..d_ℓ) over blocks (n_1..n_ℓ), Σn_i = n, has witness degree ≤ n + 2 − max_i ⌈(n_i+1)/d_i⌉; for d_i = 1 this is Σn_i − max n_i + 1, as the card says. (d) Conj 6.3, a sparse Fröberg: for m > n generic forms, HS = [HS_P(t)·∏(1−t^{d_i})]_+. For the tower the governing bounds are the repository's: BFS regularity N+1 from the top part, and the exact XL degree N+4 (Prop 2, δ = N).

## 2. Prior art: literature

- Petit, Kosters, Messeng, *Algebraic approaches for the ECDLP over prime fields*, PKC 2016, christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf. **read-at-source**. Claims taken: the tower construction, the cost formula with T open, and Tables 4–5 (m = 2, Magma; D_av^corr = 4,4,4,4,4,5,5,5,5,6,6 for n1 = 1…11, subgroup base; isogeny base 4, then 5 for n1 = 2…9, then 6,6). **New against the repository note:** `RESEARCH_PKM_TOWER_ORACLE.md` §10.10 says whether PKM saw the same degrees was unchecked. They did, and the repository's raw m = 2 lines (Kummer: 4 through N = 10, 5 for N = 12–18, 6 at N = 20, 22; isogeny: 5 for N = 4–18, 6 at N = 20) match them, from an independent instrument (Magma V2.18-5) and different primes.
- Amadori, Pintore, Sala, *On the discrete logarithm problem for prime-field elliptic curves*, FFA 51 (2018) 168–182, ePrint 2017/609. **abstract-only**. Claim taken: the variant reduces the Gröbner computations to one basis. What that basis is for (a parametric target? a single relation?) is unread; a search snippet's "needs only one relation among factor-base points" is unconfirmed.
- Kudo, Yokota, Takahashi, Yasuda, *Acceleration of Index Calculus for Solving ECDLP over Prime Fields and Its Limitation*, CANS 2018, LNCS 11124, 377–393, DOI 10.1007/978-3-030-00434-7_19. **abstract-only** (Kyushu portal). This is the "Momonari–Kudo 2018" source (Momonari is Kudo's given name; Hu's abstract writes "Momonari and Kudo"). Claim taken: it accelerates APS's system by a hybrid of exhaustive search and Gröbner bases plus summation-polynomial symmetries, with "a limitation of our acceleration". Body unread.
- Hu, *Implementation analysis of index calculus method on elliptic curves over prime finite fields*, ePrint 2024/1923, EasyChair preprint 15613. **abstract-only**. Claim taken: a critical review of PKM and Kudo et al.; no subexponential method known. It reports no degree data; low weight.
- Faugère, Safey El Din, Spaenlehauer, *Gröbner bases of bihomogeneous ideals generated by polynomials of bidegree (1,1)*, arXiv 1001.4004 (JSC 2011, recalled; the arXiv page lists no journal). **read-at-source**. Claim taken: Thm 6.1 (generic, n_x + n_y equations, d_reg ≤ min(n_x,n_y)+1, "sharp"), Remark 6.1 (against Macaulay's m+1).
- Faugère, Spaenlehauer, Svartz, *Sparse Gröbner bases: the unmixed case*, ISSAC 2014, arXiv 1402.7205 (the card attributes it to Safey El Din; the third author is Svartz). **read-at-source**. Claim taken: Thm 1.1/6.2 with the hypotheses of §1.4, and "we do not know how to extend the algorithms … to mixed structures".
- Kosters, Yeo, arXiv 1503.08001, via `KN-LIT-7604`. **abstract-only (secondhand)**. Claim taken: binary S_3 has first fall degree 2 because a group morphism to F_2 gives a linear form. The tower has no such morphism (V's multiplicative group is unrelated to E's group law), consistent with null = tower. Semaev's ePrint 2015/310 (**abstract-only**) likewise rests its regularity-4 claim on a first-fall assumption.
- Huang, Kosters, Yeo, CRYPTO 2015 (last fall degree); Yokoyama, Yasuda, Takahashi, Kogure, JMC 2020; Bardet–Faugère–Salvy. **recalled**, via PKM's reference list and repository notes. Claim taken: the last fall degree is the invariant that can bound F4 with falls.

## 3. Prior art: these repositories

- `/home/user/crypto/research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md` §0, §2–3.4, §10–17: the exact tower system, N = m·t, Lemma 1, Prop 2, the top-part degeneracy (BFS regularity N+1), and every measured D. §10.4: the shape-matched null equals the tower in D, width and steps at every N; the naive presentation gives 2^t+2. **Supports closing.**
- `/home/user/crypto/src/cryptanalysis/f4_fp_tower.rs` (header): measures the reduced presentation; `solving_degree_max` is a tower-weight degree, not a multi-homogeneous one. **Supports** (unit caveat, §4).
- `/home/user/crypto/research/ic_next10_20261014/03-pkm-degree/README.md` ("item 3"): m = 4, D = 6, 7, 7, ≥ 8 at N = 8, 12, 16, 20 (the last from msolve, productive degree-8 rounds). D_XL = N+4 exactly (δ = N). The top series is (1+t)^N at m = 2 and positive-dimensional at infinity for the m = 3 chain; no Hilbert-series quantity reproduces D at N ≤ 18. **Closes the Hilbert-series route.**
- `/home/user/crypto/research/pkm_m3_n21_msolve_20261018/README.md`, `RESULTS.md`: m = 3, N = 21, 24: single-thread runs hit wall stops inside degree 7; the 16-thread N = 21 run exhausted degree 7 without the unit ideal, selected 398,409 degree-8 pairs and was memory-stopped at 99.6 GiB, with no productive degree-8 round. The repository reads it as inconclusive. **Bounds the data**: the m = 3 line is 6, 6, 7, 7 at N = 9…18 only. The card's "6, 6, 7, 7, ≥ 8 at N = 9–20" splices it with the m = 4 point.
- `/home/user/crypto/research/notes/index-calculus/RESEARCH_HAMMING_IDEAL_PDP.md`: binary Lucas-digit lifted presentations; Gröbner calls grow as |F_w|^{1.9–2.5}, the lifts trade degree for auxiliary unknowns, and the unlifted monomial control resolves 6–8 levels higher. Re-encoding digits without new structure is accounting. **Supports closing.**
- `KN-OPEN-d6ad3f`, `-02200b`, `-2e7514` (`/home/user/crypto-autoresearcher/knowledge/open-problems/`): the digit twin was measured by Macaulay deficit, never by solving degree; H-TOP holds at m = 3 (agrees with my numeric check); the digit first-fall form m·2^{m−1} + ⌊(s−2^{m−1})/2⌋ + 1 gives 13 at m = 3, s = 4–5. **Context**: integer digits sit at 13+, the tower at 6–7.
- `/home/user/crypto/research/xl_bilinear_core_20261006/README.md`, `xl_trilinear_t_unknown_20261006/README.md`: binary multihomogeneous XL on the S_3 chain is closed below 2^61 at n = 131, and both name "structured Gröbner on multihomogeneous systems" as the candidate left. LEAD-14 is its prime-field cousin. **Supports caution.**

## 4. Why it could lower the Gröbner cost or the degree of regularity

It would have to give a bound, or a Fröberg-type estimate, below the measured D. Unverified desk check, predictors for the reduced Kummer system (w = top tower weight: 2, 6, 12 at m = 2, 3, 4), against committed D:

| m, N | measured D | semi-regular: first k with C(N,k) − C(N,k−w) ≤ 0 | multigraded: min total degree with ∏C(t,a_i) ≤ ∏C(t,a_i−w_i) | FSS (m−1)t+1 | FSS Thm 6.2, d_i = w_i | XL N+4 |
|:--|--:|--:|--:|--:|--:|--:|
| 3, 9 | 6 | 8 | 8 | 7 | 9 | 13 |
| 3, 12 | 6 | 9 | 9 | 9 | 11 | 16 |
| 3, 15 | 7 | 11 | 11 | 11 | 14 | 19 |
| 3, 18 | 7 | 12 | 12 | 13 | 16 | 22 |
| 4, 12 | 7 | 12 | 12 | 10 | 12 | 16 |
| 4, 16 | 7 | 14 | 14 | 13 | 16 | 20 |
| 4, 20 | ≥ 8 | 16 | 16 | 16 | 20 | 24 |
| 2, 12 / 20 / 22 | 5 / 6 / 6 | 7 / 11 / 12 | same | same | same | 16 / 24 / 26 |

The chain presentation (top form c·y_{10}y_{20}u², positive-dimensional at infinity) with HS = (1+z)^N(1+z+z²)(1−z⁴) gives 6, 8, 9, 11, 12, 14 at N = 6…21, against measured 5, 6, 6, 7, 7 (N = 21 undecided). Every predictor exceeds the measurement at every m ≥ 3 size tabulated. At m = 2 the formal values (2 and 3 at N = 2, 4) fall below the measured 3 and 4, which shows they are not bounds. Slopes at m = 3 over N = 9…18: predictors 0.44 to 0.78 per N, measured 0.11 (leave-one-N-out 0.12 to 0.17, PKM note §16.10). The multigraded refinement adds nothing: the minimum over multidegrees equals the single-graded value here.

*Unit caveat.* Thm 6.2 bounds the sparse (diagonal) grading, in which each multilinear generator has degree 1. The repository's D is the total degree of F4 steps, with tower quadrics at degree 2. The two are not interchangeable. If total degree ≤ m·d_wit, the block bound is m times weaker, which only strengthens the closure.

*Where a block bound would pay.* min(n_x,n_y)+1 is "guess the small block, solve the rest linearly": it is small only when most unknowns enter linearly. The tower unknowns enter quadratically, and guessing is the hybrid of Kudo et al., enumeration in disguise.

## 5. Obstructions

1. **Hypotheses of Thm 6.2 that fail.** (a) *Multilinear and unmixed.* The tower quadrics have block degree 2 in their own block, and the summation equations have different supports, so the sequence is mixed. Duplicating unknowns (y_j·z_j = y_{j+1}, z_j = y_j) makes everything affine bilinear, but then n_i = N and the bound is N+1. (b) *Regular, n equations in n unknowns.* The full system has N+1 equations in N unknowns, and the chain has N+2 in N+1 and is positive-dimensional at infinity. What does *not* fail is genericity given the support: random coefficients on the same support reproduce D at every N (§3). So the missing theory is a Fröberg-type statement for a generic element of the tower-quotient algebra with a given support. FSS explicitly lack the mixed case.
2. **Top-part degeneracy** (theorem, `RESEARCH_PKM_TOWER_ORACLE.md` §3.4). The top ideal contains all y², so its series is (1+z)^N and d_reg ≤ N+1 whatever the blocks. A monomial top form changes this to N.
3. **XL is exactly N+4** (item 3: δ = N exactly at m = 2; Macaulay rank confirms 13 and 12 at m = 3, N = 9 and m = 4, N = 8). The block structure leaves this equality untouched; F4 escapes it only through degree falls, which top-part Hilbert series cannot see.
4. **The low degree is not curve-specific.** Null = tower (PKM note §10.4), torus = split (§17), Kummer = Dickson (the same F4 run on different numbers, §10.2). A block theory can therefore only describe the algebra R plus a support, and cannot use E.
5. **Generic bounds.** The table and enumeration side of the tower oracle (`tower-exhaustive`, `mitm`) is built from group operations and membership tests, so Corrigan-Gibbs–Kogan S·T² ≳ n covers it. Block structure adds no structure on E (obstruction 4), so the lead stays on the generic line unless the solve itself beats enumeration; the structured-model classification belongs to LEAD-12. §3.7's floor r^{2/(m+1)} stands: m = 3 at best ties rho's exponent, and m = 4 gives r^{2/5} only if D is bounded, which the data does not support. The distance to rho is whatever the PKM-tower rows already show; this lead moves none.
6. **Closed neighbours.** BKK/MV = Bézout (§1.4a); the Kummer-exponent Fourier spectrum, flat (the dual picture of the tower); the integer-digit presentation is Semaev's interval base, whose small-root form is the closed single-polynomial Coppersmith.

## 6. Minimal discriminating test (frozen protocol)

**T1, zero compute, mechanical.** Add one Rust example, `examples/lead14_block_bounds.rs` (about 60 lines; no Python in `/home/user/crypto`), with exact integer arithmetic.
- *Instances.* (m, t) = (2, 1…11), (3, 2…8), (4, 2…6), N = m·t.
- *Committed D, copied from the notes, not recomputed.* m = 2, N = 2…22: 3,4,4,4,4,5,5,5,5,6,6 (PKM note §11.7–11.8). m = 3, N = 9, 12, 15, 18: 6, 6, 7, 7 (§16.10); N = 6: 5 (§10.2, raw). m = 4, N = 8, 12, 16, 20: 6, 7, 7, ≥ 8 (item 3 README).
- *Recorded per (m, t).* P1 = min{k : C(N,k) − C(N,k−w) ≤ 0}, w = 2, 6, 12. P2 = min Σa_i over a ∈ [0,t]^m with ∏C(t,a_i) ≤ ∏C(t,a_i − w_i), w⃗ = (1,1), (2,2,2), (3,3,3,3); record "none" if no a qualifies. P3 = (m−1)t+1. P4 = N + 2 − ⌈(t+1)/min_i w_i⌉. P5 = N + 4. The chain series of §4 as P6.
- *Prediction.* P1–P4 ≥ D at every m = 3 size N = 9…18 and every m = 4 size N = 8…20. At m = 2 they match or exceed D from N = 6 (strictly from N = 8) and fall below it only at N = 2, 4. The output is expected to equal §4.
- *Kill criterion (the card's).* Kill if P1–P4 ≥ D at every N ≥ 9 on the m = 3 line. The prediction is that the kill fires. Any entry that differs from §4 is a bug to report before anything else is read.
- *Cost.* 0.5 person-day, under 1 CPU-minute.

**T2, optional, director tier, not part of the runbook.** The one quantity the desk check cannot supply is D for the m = 3 `full` presentation (monomials of weight 2 exist only there) at N = 9, 12, against the chain's 6, 6. The pilot ran `full` only at m = 2, and I did not check that the CLI supports it at m = 3. Seconds to minutes of CPU, under 2 GB. It answers a presentation question, not the block-bound question.

## 7. Ranking inputs

expected gain: none on the exponent, ≤ ~1.5× constant, unmeasured | confidence: medium | cost: 0.5 person-day (T1) | tier: neither; closed (the sparse-engineering variant would be constant) | depends on: LEAD-05 (growth law of D for the residual question), LEAD-12 (classification), PKM-tower rows at m = 4, N ≥ 21 (memory-bound, not this lead's)

## 8. Open questions

1. The conversion between Thm 6.2's sparse witness degree and total degree. I did not read the proofs; the table holds in either reading (§4).
2. What "one Gröbner basis" is in APS (bears on LEAD-02's parametric eliminant). The ePrint PDF returned 403, and the body is unread.
3. What Kudo et al.'s limitation theorem says, and whether their hybrid changes degree or only enumeration. Body unread.
4. PKM's D_av^corr (largest step degree with a reduced pair) against the repository's `solving_degree_max` (highest step where the basis gained an element). Agreement at all 11 m = 2 sizes (raw through N = 18, reduced at 20, 22) suggests equivalence there, not proved.
5. `RESEARCH_PKM_TOWER_ORACLE.md` §10.10 reads APS as quoting PKM "at m = 3", but PKM's tables are m = 2 (S_3, |V|² ≈ p). Probably an index shift; unverified, APS body unread.
6. A Fröberg-type law for R/(generic f of tower weight w) that predicts the observed one rise per 6–8 unknowns. This is the real residual and belongs to LEAD-05.

## 9. Sources

URLs, with outcome:
- https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf: 200; WebFetch returned unreadable compressed streams, so I extracted text with `pdftotext -layout` from the saved copy and read all of it.
- https://arxiv.org/abs/1001.4004, https://arxiv.org/pdf/1001.4004, https://arxiv.org/abs/1402.7205, https://arxiv.org/pdf/1402.7205: 200; PDFs read through pdftotext (Thm 6.1, Cor 6.1, Remark 6.1; Thm 1.1, 6.2, Conj 6.3, §7). Schema-less `arxiv.org/abs/…` calls were rejected by the tool before any request.
- https://eprint.iacr.org/2017/609, https://eprint.iacr.org/2024/1923, https://eprint.iacr.org/2015/310, https://iris.unitn.it/handle/11572/202653, https://easychair.org/publications/preprint/6SDg: 200 (abstract or metadata pages). https://eprint.iacr.org/2017/609.pdf, https://eprint.iacr.org/2024/1923.pdf: 403.
- https://kyushu-u.elsevierpure.com/en/publications/acceleration-of-index-calculus-for-solving-ecdlp-over-prime-field/: 200 (abstract, DOI). https://myasuda.imi.kyushu-u.ac.jp/publications: 200 (entry only). https://link.springer.com/chapter/10.1007/978-3-030-00434-7_19: 303 to idp.springer.com, not followed. https://dblp.org/pid/174/0971 and the dblp search API: Anubis "Access Denied". https://www.researchgate.net/publication/327360141_…, https://doaj.org/article/bb64b7a98fd2462c84a20ffe0d32dd6b: 403.
- Four WebSearch queries (Momonari Kudo; APS FFA 2018; the CANS title, twice), used to locate records only.

Repository paths read: every path named in §3, plus `/home/user/crypto/research/pdp_literature_leads_20261022/BRIEF_SONNET.md`, `LEADS.md` (standing constraints, LEAD-12, LEAD-14), `/home/user/crypto/examples/pkm_tower_pilot.rs` (header), `/home/user/crypto-autoresearcher/knowledge/literature/KN-LIT-7604.md`, and `/home/user/crypto/research/pdp_root_finder_requirements_20261012/README.md` (D-line grep).

Unverified desk-check scripts (scratchpad, not committed; every number from them is labelled unverified): `/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/lead14/s4_support.py`, `hilbert_bounds.py`, `digit_pred.py`, `chain_pred.py` (same directory).
