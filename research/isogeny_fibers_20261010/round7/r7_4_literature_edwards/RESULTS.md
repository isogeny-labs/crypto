# R7.4: King §5 and Eagle–Galbraith–Ong (literature), and a binary-Edwards check of Corollary 9.2

**Date.** 2026-10-31. **Items.** Two of the three small open items of §19.17 of
`RESEARCH_ISOGENY_FIBERS_ECDLP.md`: (1) the two sources R6.10 left unread, (2) the binary-Edwards check of
Corollary 9.2 (R5.4). **Code.** `examples/r7_4_binary_edwards.rs` (standalone, native Rust, no library code).
**Raw data.** `binary_edwards.json` (item 2), `raw/` (item 1 fetches). **Figure.** `fig_edwards_yield.svg`
(script `plot_edwards.py`, reads `binary_edwards.json`). **Report.** `report.pdf` (source `report.typ`).
Toy instances only (n = 9 … 14). No sub-rho claim and no claim about any deployed curve. The note
`RESEARCH_ISOGENY_FIBERS_ECDLP.md` is not edited; a ready-to-paste "Note text" is at the end.

## Hypotheses, falsifiers, verdicts

### Item 1 — literature (no hypothesis, no code)

*Question.* What do King (PKC 2004, §5) and Eagle–Galbraith–Ong (EGO; Adv. Math. Commun. 5(1) (2011); ePrint
2009/086) actually say about the note's statements: the kernel-saturation / μ^{m−2} law, symmetrisation, and a
cofactor-4 test?

*Verdict.* **The two primary texts could not be read.** King's chapter is behind a Springer login redirect and
the IACR PDF returns a Cloudflare 403; the EGO PDF returns 403 on ePrint and an empty body on the publisher's
export endpoint. What I did read (and nothing else is cited below): the ePrint 2009/086 abstract page, the
publisher page of EGO (abstract and its 13-entry reference list), Eagle's 2008 PhD thesis (the precursor of
EGO's Chapter on Koblitz compression, but a different document), and Semantic Scholar citation metadata. None of
these contradicts or pre-empts a statement of the note. **King §5 and the EGO cofactor-4 test remain unread.**
Details in "Item 1 record" below.

### Item 2 — binary Edwards (R5.4 Corollary 9.2)

*Hypothesis.* Symmetrisation by the small rational torsion of a binary Edwards curve is also null: (a) as an
algebraic device (the same-size equivalence of R5.4's Corollary 9.2); (b) for the pair-table (mitm) oracle, the
yield ratio of a symmetrised base against an unsymmetrised base of equal size has a 95% bootstrap interval that
contains 1.

*Falsifier (as worded in the task, fixed before the final run).* A yield-ratio interval that excludes 1 on some
curve family. Not a falsifier, by the law itself: the naive full sweep of a saturated base, whose ratio is
predicted to be 1/μ.

*Verdict.* **Split.**

* **(a) Algebraic: proved and structurally tested, not falsified.** Corollary 9.2 extends to every rational
  subgroup M of 2-power order on every ordinary binary curve, in any model, so to binary Edwards curves
  (Proposition 9.3 below, proved). On 46 toy binary Edwards curves, 151 034 points, every identity checked has 0
  exceptions (11 identity families, table below).
* **(b) Pair-table yield: the falsifier as worded fires at n ≤ 12.** On the 40 curves with n ≤ 12 the pooled
  95% interval of the quotient-sweep ratio excludes 1 in 39 of 40 (family × operating point) cells under the
  primary metric and in 40 of 40 under each of the other three. The direction depends on the metric: ratio
  0.80–0.97 when the relation matrix is charged c + 1 rows (a loss), 1.05–1.49 for s2 and s4 when charged c rows
  (the primary metric), and 1.15–2.83 for the hit-fraction metric. The scale check at n = 13, 14 (6 curves)
  moves toward the law: the primary metric's intervals contain 1 in 10 of 16 cells, and the naive full-sweep
  ratio contains the law value 1/μ in 12 of 16 (the tables give all 16). I therefore do **not** confirm
  hypothesis (b) as worded at toy size, and I do not claim a gain from symmetrisation: the c versus c + 1 pair
  brackets 1 with the size of the derived finite-size term, the exact identity of Lemma 7 holds in
  268 000 of 268 000 checks, and only the hit-fraction metric keeps an excess (1.14 at n = 14, falling with n)
  that I cannot attribute. The finite-size reading is derived, not proved; the n = 13, 14 sample is small.

### What binary Edwards has that Weierstrass lacks (tested, with derivation)

Nothing at the group level. The complete binary Edwards curve with Tr(d2) = 1 is birational over F_q to
y² + xy = x³ + (d1² + d2)x² + d1⁴(d1² + d1 + d2)² and the map is a group isomorphism here (tested on all 46
curves, 4 000 random pairs each, 0 failures). Its neutral element (0,0) goes to O and the order-2 point (1,1)
goes to T = (0, √b). By R5.4 Theorem 9(d), 4 | N ⇔ Tr(a₂) = 0, and a₂ = d1² + d2, so with Tr(d2) = 1:
**4 | N ⇔ Tr(d1) = 1.** Tested on 1 600 random complete curves (n = 9 … 12): 0 exceptions, and 0 curves with
4 | N and Tr(d1) = 0. So the complete Edwards family contains a half of its curves with Z/2 as the rational
2-torsion and a half with a cyclic Z/4 or larger 2-part, exactly as for Weierstrass curves with Tr(a) = 1 or 0
(the Weierstrass tally agrees: 4 | N ⇔ Tr(a) = 0, 1 600 draws, 0 exceptions). The differences are not
torsion but coordinates and arithmetic:

* The Edwards sign-invariant coordinate w = x + y has fibres equal to the orbits of ⟨T, −1⟩ on every curve
  (37 792 fibres, 0 exceptions). Therefore **the natural Edwards frame is T-symmetrised by construction**:
  w is a degree-4 function, F_q(w) = F_q(u) with u = x_W + √b/x_W (Theorem 9), and w is a Möbius function of
  the quotient coordinate. A w-frame has no unsymmetrised counterpart in the same coordinate; the unsymmetrised
  frame is the Weierstrass abscissa x_W of the same curve.
* When 4 | N the order-4 points are (x, x + 1): w(Q4) = 1 on all 21 curves with Z/4 (tested), consistent with
  2·Q4 = T.
* Completeness (no exceptional additions) is an arithmetic property: the group law has no special cases when
  Tr(d2) = 1 (no kept curve produced a vanishing denominator; a curve that did would have been dropped, and selection drops
  are not logged by reason). It changes no algebra of the summation systems.

### Proposition 9.3 (proved) — Corollary 9.2 for all rational 2-power torsion, any binary model

Let E/F_q, q = 2^n, be ordinary, in any model (Weierstrass, binary Edwards, …), and let M ≤ E(F_q) have order 2^j.
Then (i) M is the unique subgroup of order 2^j of the étale 2-part E(F̄)[2^∞] ≅ Q₂/Z₂, so M = ker V^j for the
j-fold Verschiebung V^j: E → E^{(2^{−j})}; (ii) Fr^j ∘ V^j = [2^j], so the M-invariant coordinate is
u_M = (x ∘ [2^j])^{1/2^j}, and [2^j] maps F_{u_M}(X) exactly 2^j-to-1 onto F_{2^jE}(X^{2^j}); (iii) summation
polynomials have integer coefficients in the curve parameters, so S_k^{E^{(2^{−j})}}(u₁, …, u_k)^{2^j} =
S_k^E(u₁^{2^j}, …, u_k^{2^j}); (iv) after Weil descent the substitution X = u^{2^j} is an F₂-linear bijection
of coordinate vectors and g^2 = g in the Boolean ring, so the M-symmetrised algebraic system is the plain
x-frame system over V^{2^j} with target x(2^j R): same variables, same degrees, same Macaulay matrices.
*Proof.* (i)–(ii): Theorem 9 of R5.4 applied j times (V is separable of degree 2 with kernel the étale part of
E[2], AEC II.2.12, III.6.2, cited); the unique cyclic subgroup of each order in Q₂/Z₂ gives uniqueness. (iii):
Frobenius on coefficients and variables. (iv): as in Corollary 9.2. ∎
*Where it applies.* Every binary Edwards curve is ordinary: b = d1⁴(d1² + d1 + d2)² ≠ 0 because d1 ≠ 0 and
d2 ≠ d1² + d1, i.e. d1² + d1 + d2 ≠ 0. *Scope.* This is the statement that an algebraic symmetrised oracle is
no cheaper per call than the plain one; it says nothing about an implementation constant, and nothing about
formulations of the summation polynomials that I have not seen. For M = Z/4 the quotient coordinate is
u_M = u + b^{1/4}/u (tested below).

*Tested on the toy curves (identity families, all points of each curve, 0 exceptions in every row).*

| check | curves | points tested | exceptions |
|:--|--:|--:|--:|
| (1,1) = T, −(x,y) = (y,x) equals index negation | 46 | 151 034 | 0 |
| Edwards → Weierstrass map is on the curve (b = d1⁴(d1²+d1+d2)²) | 46 | 151 034 | 0 |
| map is a homomorphism (4 000 random pairs per curve) | 46 | 184 000 pairs | 0 |
| fibres of w are the ⟨T, −1⟩ orbits | 46 | 37 792 fibres | 0 |
| x_W(P + T) = √b / x_W(P) | 46 | 150 942 | 0 |
| x_W(2P) = u(P)², u = x_W + √b/x_W | 46 | 150 942 | 0 |
| u(P + T) = u(P) | 46 | 150 942 | 0 |
| u4 = u + b^{1/4}/u is invariant under Q4 (21 curves with Z/4) | 21 | all points | 0 |
| x_W(4P) = u4(P)⁴ (21 curves) | 21 | all points | 0 |
| fibres of u4 are the ⟨Q4, −1⟩ orbits (21 curves, 12 202 fibres) | 21 | 12 202 fibres | 0 |
| 4 | N ⇔ Tr(d1) = 1 (complete Edwards, 400 random per n) | 1 600 draws | 0 |
| 4 | N ⇔ Tr(a) = 0 (Weierstrass, 400 random per n) | 1 600 draws | 0 |

## Item 2 method (exact commands and instance set)

```
rustc --edition 2021 -O examples/r7_4_binary_edwards.rs -o r74ed        # rustc 1.97.0
./r74ed 1000 10 > research/isogeny_fibers_20261010/round7/r7_4_literature_edwards/binary_edwards.json
python plot_edwards.py binary_edwards.json fig_edwards_yield.svg
```

(`cargo run --release --example r7_4_binary_edwards -- 1000 10` is the same program; I built with `rustc`
because the sparse worktree has no workspace crates and the shared disk had 6 GB free. Clippy was run as
`clippy-driver` 1.98 on the single file with `-D warnings`, and `rustfmt --check` passes.)

* **Curves.** Binary Edwards d1(x+y) + d2(x²+y²) = xy + xy(x+y) + x²y², Tr(d2) = 1 (complete), d1 ≠ 0, over
  F_{2^n}, n = 9, 10, 11, 12 (10 curves each requested; 40 found) and n = 13, 14 (3 each, scale check beyond
  the stated n ≤ 12). Selection: cyclic E(F_q) (so points are indices in Z/N), N even, r = largest prime factor
  ≥ 100 with r² ∤ N. Family labels by v2(N): 1 (24 curves with n ≤ 12, only Z/2 as rational 2-part), 2 (12), ≥ 3 (4); equivalently Tr(d1) = 0 (24) or 1 (16). 21 of the 46 curves have 4 | N (16 of the 40 with n ≤ 12). Curves are named by (n, d1, d2) in the JSON; none is a standard curve.
* **Oracle.** Pair table T₂ = {P + Q : P, Q ∈ F}; for every target R ∈ G \ {O} count the P in the swept set with
  R − P ∈ T₂. Exact over all r − 1 targets; randomness only in the choice of bases (1 000 per arm per cell).
* **Arms (equal |F|, every base a union of ⟨M, −1⟩-orbits).** plain (M = 1); s2 (M = ⟨T⟩, the fibres of w);
  s4 (M = ⟨Q4⟩, 4 | N only). Each saturated arm swept over all of F (`full`) or over one point per M-coset
  (`quot`). |F| = F0 ∈ {8, …, 40}, a multiple of 8, calibrated per curve so that the plain arm's decomposable
  fraction H is near 0.05, 0.1, 0.25, 0.5 (realised values in the JSON).
* **Yields.** λ = mean number of relations modulo M per target (distinct relations); H = fraction of targets
  with ≥ 1 relation. Cost = (lookups per target) × U, U = number of distinct signed projected columns ±[h]P ≠ 0
  (`c`), or c + 1 (the R5.1 convention). Yield ratio = [Σ λ or H / Σ U·W](arm) ÷ the same for the plain arm.
  **Primary: λ with U = c** (linear in the hit rate; I chose it after a smoke run showed the H metric saturates
  and U = c + 1 carries an O(1/c) edge; all four are reported). 95% bootstrap over bases, 2 000 resamples,
  stratified by curve.
* **Prediction (small hit rate, many columns, m = 3, k = 2).** Full sweep 1/μ (μ = 2, 4), quotient sweep 1.
* **Lemma 7 check.** With T₂ M-closed, hits(full sweep) = μ · hits(quotient sweep) for every target and base:
  268 000 (base, arm) checks, 0 violations.

## Item 2 results (pair table)

Pooled over the 40 curves with n ≤ 12 (stratified bootstrap); realised plain H in parentheses (target 0.05, 0.1,
0.25, 0.5 → 0.08, 0.12, 0.18, 0.58 for s2; s4 curves 0.08, 0.11, 0.19, 0.60). Ratio [95% CI].

| arm, metric | H target 0.05 | 0.1 | 0.25 | 0.5 |
|:--|:--|:--|:--|:--|
| s2 quot, λ/c (primary) | 1.119 [1.110, 1.128] | 1.084 [1.077, 1.091] | 1.061 [1.056, 1.067] | 1.048 [1.044, 1.050] |
| s2 quot, λ/(c+1) | 0.934 [0.926, 0.941] | 0.942 [0.936, 0.948] | 0.940 [0.934, 0.945] | 0.963 [0.960, 0.966] |
| s2 quot, H/c | 1.445 [1.435, 1.456] | 1.349 [1.341, 1.358] | 1.301 [1.294, 1.307] | 1.489 [1.485, 1.493] |
| s4 quot, λ/c (primary) | 1.481 [1.464, 1.500] | 1.342 [1.329, 1.354] | 1.212 [1.198, 1.228] | 1.109 [1.103, 1.116] |
| s4 quot, λ/(c+1) | 0.928 [0.918, 0.940] | 0.924 [0.915, 0.933] | 0.880 [0.870, 0.891] | 0.884 [0.878, 0.889] |
| s4 quot, H/c | 2.829 [2.797, 2.862] | 2.375 [2.353, 2.397] | 1.987 [1.966, 2.009] | 1.992 [1.981, 2.003] |
| s2 full, λ/c (law 1/2) | 0.559 [0.555, 0.564] | 0.542 [0.538, 0.545] | 0.530 [0.528, 0.533] | 0.524 [0.522, 0.525] |
| s4 full, λ/c (law 1/4) | 0.370 [0.366, 0.374] | 0.336 [0.333, 0.338] | 0.303 [0.299, 0.307] | 0.277 [0.276, 0.279] |

Scale check, 3 curves each at n = 13 and 14 (2 and 3 of them with 4 | N), primary metric λ/c:

| arm | n | 0.05 | 0.1 | 0.25 | 0.5 |
|:--|--:|:--|:--|:--|:--|
| s2 quot | 13 | 1.038 [1.002, 1.075] | 1.013 [0.978, 1.050] | 0.999 [0.980, 1.018] | 0.997 [0.984, 1.010] |
| s2 quot | 14 | 1.027 [0.991, 1.065] | 0.998 [0.977, 1.017] | 1.000 [0.986, 1.014] | 0.992 [0.982, 1.003] |
| s4 quot | 13 | 1.109 [1.001, 1.223] | 1.095 [1.012, 1.177] | 1.051 [1.001, 1.105] | 0.994 [0.961, 1.026] |
| s4 quot | 14 | 1.146 [1.080, 1.213] | 1.074 [1.040, 1.111] | 1.020 [0.997, 1.043] | 1.016 [0.998, 1.033] |
| s2 full (law 0.5) | 13 | 0.519 [0.501, 0.537] | 0.507 [0.490, 0.525] | 0.499 [0.489, 0.509] | 0.498 [0.492, 0.505] |
| s2 full (law 0.5) | 14 | 0.514 [0.495, 0.532] | 0.499 [0.489, 0.510] | 0.500 [0.493, 0.507] | 0.496 [0.491, 0.501] |
| s4 full (law 0.25) | 13 | 0.277 [0.249, 0.306] | 0.274 [0.254, 0.295] | 0.263 [0.250, 0.276] | 0.248 [0.240, 0.257] |
| s4 full (law 0.25) | 14 | 0.286 [0.270, 0.304] | 0.269 [0.260, 0.277] | 0.255 [0.249, 0.261] | 0.254 [0.249, 0.258] |

Reading (derived from the rows above).

* **The full sweep pays the law.** Ratio 0.50 ± 0.02 (s2) and 0.25–0.29 (s4) at n = 13, 14, with the n ≤ 12
  values above the law (0.52–0.56 and 0.28–0.37) because the bases hold only 2–20 columns. This is the
  kernel-saturation penalty μ^{m−2} of Proposition 15.2′ on a binary Edwards curve.
* **The quotient sweep removes it.** Exactly the same relations with 1/μ lookups (Lemma 7), and the yield ratio
  is near 1 but never converges to a consistent sign: it is below 1 when charged c + 1 and above 1 when
  charged c. The two conventions differ by the factor (c_a + μ)/(c_a + 1) (derived: c_s = c_a/μ columns on the
  saturated base), 1.05–1.6 for the plain-arm column counts c_a = 4–20 used here, which accounts for the
  size of the gap between the observed 0.88–0.96 and 1.05–1.5. Under the primary metric the s2 interval at n = 13, 14
  contains 1 in 7 of 8 cells.
* **The hit-fraction metric H shows an apparent gain** (1.1–2.8) that decreases with n (s2, H target 0.05:
  1.45 at n ≤ 12, 1.21 at n = 13, 1.14 at n = 14) but whose interval still excludes 1 at n = 14 (1.140
  [1.105, 1.176]). H undercounts the plain arm's repeated hits (a decomposition R = a + b + c is found three
  times, once per summand), so I do not read it as a gain in independent relations; this reading is not
  proved, and larger instances than n = 14 would be needed to see whether H/c reaches 1.
* **Per-curve rows** (`per_curve_ratios`): under λ/c, 133 of 184 s2-quot and 72 of 84 s4-quot curve cells have
  an interval above 1 and 1 below; under λ/(c+1), 158 of 184 and 77 of 84 are below 1 and 8 and 0 above.
  Family splits (v2(N) = 1, 2, ≥ 3; Tr(d1) = 0, 1) in `pooled_ratios` show the same pattern; the v2(N) = 1
  family, which has only T, gives the s2 numbers of the other families.

## Limits

* Toy n ≤ 14; bases hold 1–20 projected columns, the regime where R5.1 already found the law not reached
  (R5.1 reports its n ≤ 23 rows but does not use them for its verdict). The n = 13, 14 sample is 6 curves (2 at n = 13 and 3 at
  n = 14 have 4 | N, so s4 rests on 5 curves).
* The pair-table yield depends only on the abstract group Z/N and the base; the Edwards model enters through the
  coordinate w (whose fibres are the s2 orbits) and the torsion identification, which were checked
  exhaustively. A curve-specific effect of the coordinate would show in the algebraic oracle, which I did not run.
* **No Gröbner / F4 oracle was run on Edwards summation polynomials.** The algebraic claim rests on the
  proved Proposition 9.3 and the structural identities; whether a specific implementation (for example the
  Galbraith–Gebregiyorgis or Huang–Petit–Shinohara–Takagi binary-Edwards setups mentioned in R5.4) reaches
  the same constants on the two formulations is untested, and I have not read those papers.
* Metric choice (primary λ/c) was made after a development smoke run; all four are reported.
* No sub-rho claim, no deployed-curve claim.

## Item 1 record (what was fetched and read)

| source | what happened | status |
|:--|:--|:--|
| ePrint 2009/086 abstract page (`https://eprint.iacr.org/2009/086`) | HTML fetched (200), text saved in `raw/eprint_2009_086_abstract_page.txt` | read |
| ePrint 2009/086 PDF, IACR archive PDF of King (`…/pkc2004/29470331/29470331.pdf`) | HTTP 403, Cloudflare challenge page | **not read** |
| Springer page of King (`10.1007/978-3-540-24632-9_24`) | redirect to a Springer login (303); I did not follow it | **not read** |
| AIMS publisher page of EGO (`https://www.aimsciences.org/article/doi/10.3934/amc.2011.5.1`) | abstract and reference list shown; "PDF view" is a placeholder; the export endpoint gave an empty body to a plain request; a retry with browser-style request headers was denied by the session's permission check as bot-check circumvention and not pursued | abstract and references read; body **not read** |
| Eagle's PhD thesis, RHUL 2008, `www.math.auckland.ac.nz/~sgal018/Phil-Eagle-Final.pdf` (fetched, pdftotext) | read §4.1.2–4.1.3, Ch. 7 intro and Lemma 7.1.5, searched the whole text | read (a different document from EGO) |
| Semantic Scholar API, references of EGO and citations of King | JSON in `raw/` | metadata only |
| web-search summaries | quoted below as snippet-level only | not primary |

What the read sources say, with section numbers.

* **EGO, abstract (ePrint 2009/086, AIMS page).** "A Koblitz curve is an elliptic curve E over F_2 …
  Wiener and Zuccherato and Gallant, Lambert and Vanstone showed that one can accelerate the Pollard rho
  algorithm … We present a method to reduce this bandwidth when a normal basis representation for F_{2^n} is
  used … with a low probability of failure, our method gives the expected bandwidth for a given security level."
  The ePrint page adds: "Note: Revised version of the paper with additional author." The abstract mentions no
  validation, halving or cofactor.
* **EGO, reference list (AIMS page), 13 entries:** Blake–Seroussi–Smart; Gallant–Lambert–Vanstone (2000);
  Gaudry–Hess–Smart (2002); **[4] King, PKC 2004, pp. 333–345**; Koblitz (1991); Lidl–Niederreiter; Miller (1985);
  Pollard (1978); Schilling (1990); Seroussi (HPL-98-94R1); Solinas (2000); van Oorschot–Wiener (1999);
  Wiener–Zuccherato (1998). No summation-polynomial, index-calculus or torsion-symmetrisation reference. This
  confirms that EGO cites King (as the note's §17.2 says) and shows that EGO cites no work on the note's
  saturation law.
* **Semantic Scholar, references of EGO:** the record attaches no citation context to the King reference.
* **Eagle thesis, Lemma 4.1.15** (§4.1.3, "Seroussi's Point Compression for Curves E/F_{2^n}"): "Let E/F_{2^n} be
  defined by … y² + xy = x³ + ax² + b and ⟨P⟩ ⊆ E(F_{2^n}) be of odd prime order ℓ. Then for all points Q ∈ ⟨P⟩
  one has Tr(x(P)) = Tr(a), a binary constant." The proof uses Lemma 4.1.13 (every point of an odd prime-order
  subgroup is a double). This is the odd-prime-order case that R6.10 L3 cited (confirmed); the thesis contains
  **no** "halv" and **no** "King" (searched), and Lemma 7.1.5's proof says only "Curves E/F_{2^n} always have a
  cofactor c ≥ 2. Here we assume a general curve has the same cofactor as the Koblitz system; c ∈ {2, 4}" — a
  bandwidth count, not a validation test.
* **King's abstract (search-engine summary of the Springer page, not the page itself):** "a new elliptic curve
  point compression method, which provides the most efficient use of bandwidth whenever the elliptic curve is
  defined by y² + xy = x³ + a₂x² + a₆ and the trace of a₂ is zero." Snippet-level.
* **Snippet-level only (search results, not fetched pages):** one search result attributes to EGO the text "we
  refer to Section 5 of King [4] for the …" (cut off) and describes a cofactor-4 procedure that computes y_P,
  halves P to Q and checks a trace condition on x_Q against Tr(a₂), referring to "King's Section 5 (Table 2)".
  I could not open the passage, so I record this as the lead R6.10 had (its L2), not as a verified statement.

Bearing on the note's statements.

* *Kernel-saturation / μ^{m−2} law, symmetrisation, summation-polynomial speedups.* **Nothing in what I read
  bears on them.** King and EGO are compression papers; EGO's "equivalence classes" are Frobenius/negation
  classes for Pollard rho, and its reference list has no index-calculus entry. The "cofactor-4 test" is a
  point-validation/compression test, not a summation-polynomial speedup; the three topics the task listed
  together are not connected by these sources. The FHJRV (EUROCRYPT 2014, §3.2) attribution of the law,
  R6.10's finding, is neither confirmed nor contradicted here (not re-read).
* *§17.2 sentence.* "King … and Eagle–Galbraith–Ong … are point-compression papers that use halving when
  Tr(a) = 0; whether they state a cofactor-4 validation test was not verified (their texts were not read)" is
  not contradicted. Two refinements, both from what I read: (i) "use halving when Tr(a) = 0" is not supported by
  the EGO abstract (compression of Koblitz abscissae in a normal basis, using Seroussi's Tr(x) = Tr(a) trick for
  odd-order subgroups, as the thesis shows) and rests on snippets for King; (ii) "were not read" is still true of
  their bodies. No statement of the note is contradicted or pre-empted.

**Still unread (list).** King PKC 2004 (all sections, in particular §5 and its Table 2); EGO body (including the
passage that cites King §5 for the cofactor-4 case); Knudsen 1999; Fong et al. were read by R6.10, not here;
Pornin ePrint 2022/1164 and 2022/1325 (blocked by the same Cloudflare check); Galbraith–Gebregiyorgis 2014 and
Huang–Petit–Shinohara–Takagi 2015 (binary Edwards setups; not accessed). Anyone with a browser session can fetch
the first two in a minute.

## Note text (ready to paste)

**§19.17 (Settled) addition — Corollary 9.2 for binary Edwards curves and for Z/4.** Corollary 9.2 is not
limited to the Weierstrass model or to the point T. For an ordinary binary curve in any model, every rational
subgroup M of 2-power order is the kernel of the j-fold Verschiebung, its quotient coordinate is the 2^j-th root
of x∘[2^j], and the M-symmetrised summation-polynomial system is the plain x-frame system after the F₂-linear
substitution X = u^{2^j} (proved; R7.4, Proposition 9.3). Binary Edwards curves (Tr(d2) = 1, complete addition)
are ordinary, and the map to y² + xy = x³ + (d1²+d2)x² + d1⁴(d1²+d1+d2)² is a group isomorphism (tested on 46
toy curves, n = 9 … 14). They have no torsion that Weierstrass binary curves lack: 4 | N holds exactly when
Tr(d1) = 1, as 4 | N holds for Weierstrass exactly when Tr(a) = 0 (1 600 random curves each, 0 exceptions). The
Edwards coordinate w = x + y has the orbits of ⟨T, −1⟩ as fibres (37 792 fibres, 0 exceptions), so the natural
Edwards frame is the T-symmetrised frame, a Möbius image of the quotient coordinate of Theorem 9; and the order-4
quotient coordinate u + b^{1/4}/u satisfies x(4P) = u4⁴ with fibres the orbits of ⟨Q4, −1⟩ (21 curves, 12 202
fibres, 0 exceptions). No algebraic (Gröbner, F4) oracle was run on Edwards summation polynomials, and the
binary-Edwards setups of Galbraith–Gebregiyorgis and Huang–Petit–Shinohara–Takagi were not read.

**Pair-table yield on binary Edwards (R7.4, toy n ≤ 14).** The kernel-saturation penalty reproduces: with the
naive full sweep the yield ratio of a ⟨T⟩- or ⟨Q4⟩-saturated base to a plain base of equal size is 0.50 ± 0.02 and
0.25–0.29 at n = 13, 14 (law 1/μ), and the quotient sweep returns the same relations with 1/μ lookups
(268 000 of 268 000 checks). The quotient-sweep yield ratio is not equal to 1 at toy size: 0.88–0.96 when the
relation matrix is charged c + 1 rows, 1.05–1.49 when charged c rows, and up to 2.8 for the hit fraction, with
n ≤ 12 intervals excluding 1 in 39 of 40 cells under the primary metric (so the falsifier as worded fires). At n =
13, 14 the primary metric's ratio is 0.99–1.15 and its interval contains 1 in 10 of 16 cells. These
departures have the sign and size of the c versus c + 1 finite-size term (derived), but this is not proved and no
instance above n = 14 was run, so symmetrisation of a binary Edwards base is reported as showing no gain beyond
finite-size terms at this scale, not as exactly null.

**Literature (R6.10's unread items).** King §5 and the Eagle–Galbraith–Ong cofactor-4 test were still not read:
King's chapter is behind a Springer login and the IACR PDFs return 403. What was read: the EGO abstract and its
13-entry reference list (King is reference [4]; no index-calculus or symmetrisation reference), and Eagle's 2008
thesis (Lemma 4.1.15: Tr(x) = Tr(a) on an odd-prime-order subgroup; no halving, no King, no cofactor-4 test).
Neither source bears on the kernel-saturation law, symmetrisation or summation polynomials, and nothing in the
note is contradicted. Search-engine snippets (not primary) say EGO refers to King §5 for the cofactor-4 case.
