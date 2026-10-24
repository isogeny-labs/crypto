# LEAD-16 — Nagao's quadratic system against the D₄-symmetrised S₅ at k = 4

**Verdict.** closed-by-theorem. Nagao's system on E(F_{p⁴}) is the same
4096-point variety as Gaudry's S₄ form, only 2 of the 8 translations in the
D₄ construction act on it, so its solution count cannot go below 2048
against 512 for the D₄ system, and the card's second kill criterion fires.
The cost prediction (slower, not faster) rests on published and repository
anchors, not on a k = 4 run.
**Confidence.** high for the symmetry and degree statements (derived, and
the orbit count 4096 → 512 is the repository's own); medium for the cost
numbers, because they are transferred from two published timings and a
repository scaling law, and no msolve run exists at k = 4.
**Expected gain if it works.** none. Predicted per-decomposition cost
3·10¹⁰–10¹² F_p-multiplication equivalents (unverified desk conversion,
±10×), against 2.5·10¹⁰ for the committed D₄ solve (parity at best) and
6.9·10⁷ for the current D₄ baseline of `gaudry_quartic_d4_cost_20261014`
(450–14,000× worse). Row it would have moved: the k = 4 C₄ / `ic-d4-chains`
row.
**Cost of the discriminating test.** 1–1.5 person-days of new Rust (a Nagao
exporter, an msolve writer, a witness test); runs under 3 CPU-hours
(predicted 2–7 min per Nagao solve at one thread), memory cap 12 GB.

## 1. Mechanism (precise)

**Two inputs of the card, corrected before use.**
(i) "About 85 solutions after the quotient" is not the repository's number.
The D₄ quotient has dimension **512 = 4096/8** (`gaudry_quartic_d4_20261013`,
`d4_symmetrised_s5_20261015` §1: weighted Bézout 8⁴/8, orbit count |D₄|/|S₄| = 8,
measured on every solved target). 4096/48 = 85.3 is the cost ratio 48×
turned into a count. The card's kill threshold "4096/8" is correct.
(ii) C₄ = 2.54·10¹⁰ is the committed solve. The 2026-10-14 cost study moved it
to **6.9·10⁷** (weighted grading, chains, sparsest pivots, replay; 370×, of
which about 26× is pivot choice and about 11× the formulation), so under the
ratchet rule the baseline Nagao must beat is 6.9·10⁷ (first solve 1.4·10⁸).
Also: the D₄ base is Y ∈ F_p, i.e. x − x₀ ∈ F_p on a curve with
x₀ ∈ F_{p⁴} \ F_p, c ∈ F_p, T = (x₀, 0) (`gaudry_quartic_d4.rs`); below I
shift x so that T = (0,0) and the base is x ∈ F_p.

**The system, m = n = 4, g = 1.** E: y² = x³ + ax + b over K = F_{p⁴}, odd p,
target R, Q = −R. L(5∞) = ⟨1, x, x², y, xy⟩ has dimension 5. Take
f = c₀ + c₁x + c₂x² + c₃y + xy (chart c₄ = 1, so f is monic) with f(Q) = 0, which
gives c₀ = −(c₁x_Q + c₂x_Q² + y_Q(c₃ + x_Q)). Unknowns: (c₁, c₂, c₃) ∈ K³, that
is **12 unknowns over F_p**. Then
N(f) = f f^ι = (c₀ + c₁x + c₂x²)² − (x³ + ax + b)(c₃ + x)² has degree 5 with
leading coefficient −1, and F = N(f)/(x − x_Q) has degree 4 with roots x(P₁),…,x(P₄),
where div f = Q + ΣP_i − 5∞ (up to the signs ±P_i). F is quadratic in c with
K-linear coefficient maps. The condition "all x(P_i) ∈ F_p" needs F ∈ F_p[x]:
4 coefficients, each K-valued, each with 3 conditions (K/F_p has dimension 4),
so **12 equations over F_p, all of degree 2**, inhomogeneous (for example
F₃ = c₂² − 2c₃ − x_Q ∈ F_p). Bézout number 2¹² = 4096.

*Unverified desk check* (scratch script, not committed,
`scratchpad/desk/nagao_k4_check.py`): at p = 13, K = F_p[t]/(t⁴ − 2), random
(a, b) ∈ K², a planted decomposition R = P₁+…+P₄ with abscissae in F_p: the
symbolic system has 12 unknowns and 12 equations, all of degree 2, at most 85
terms each (a dense quadric in 12 variables has 91); the division by
(x − x_Q) is exact mod p; the planted f satisfies all 12 equations. This checks
the counts and the chart, not the ideal degree.

**Ideal degree over F̄_p.** Over F̄_p, Res_{K/F_p} splits into four copies
c^{(i)}, i = 0…3, of the same chart, and the system says
φ_i(c^{(i)}) = e(F) is one common point of A⁴ (e = elementary symmetric
functions of the roots). Each φ_i is birational onto the hypersurface
{S₅^{(i)}(x_Q^{(i)}; e) = 0}, because f is determined by its zeros. So the
Nagao variety W is birational to the intersection of the four conjugate S₅
hypersurfaces in e-space, which is **Gaudry's S₄-form system** (degree 8 in 4
unknowns, 4096 points, Macaulay 29). Both are Ω/S₄, where Ω is the variety
of labelled 4-tuples (98,304 = 4096·24 points). Expected degree: 4096 for
both. At k = 3 the repository measured staircase 64 for both presentations
(`RESEARCH_GLV_INDEX_CALCULUS.md` §6), and the JV cover (6 quadrics) read 64
over 3,360 tests; at k = 4 the count 4096 for Nagao is not measured (solutions
at infinity could lower it slightly; JV (2,3) min 61).

**Regularity.** 12 quadrics forming a regular sequence have Hilbert series
(1+t)¹² and d_reg ≤ Σ(d_i − 1) + 1 = 13 (Macaulay; the semi-regular value is
also 13, desk). Affine monomials of degree ≤ 13 in 12 variables: C(25,12) =
5,200,300, against 40,920 (S₄ form, degree 29), 11,628 (D₄ plain, degree 14)
and 4,200 (D₄ weighted, degree 26). F4's selective matrices are far smaller
than the Macaulay bound: msolve on random quadric systems n×n over F_65521 is
0.44, 2.4, 17.8 s (n = 9, 10, 11, one thread), growth 5.6× then 7.3× per variable.

**Symmetries.** Let N = (Z/2)³ be the even translations (P_i ↦ P_i + T on an
even subset of the summands) and D₄ = N ⋊ S₄ acting on Ω.

- S₄ is quotiented in Nagao's coordinates, as the card says.
- An element n ∈ N descends to W = Ω/S₄ only if it normalises S₄, that is
  only if it is fixed by S₄-conjugation: C_N(S₄) = {identity, all-four}. So
  **only the diagonal translation ι: P_i ↦ P_i + T acts**, order 2. The six
  pair-translations act on Ω and induce only a 6-valued correspondence on W.
  This is the answer to "does translation by a rational 2-torsion point act":
  yes, but as (Z/2)¹, not (Z/2)³.
- ι acts as f ↦ h_Q·τ_T^*f, where div h_Q = (Q) − (Q+T) + 5(T) − 5(∞). It is a
  K-linear involution of the 4-dimensional space L(5∞ − Q), provided the base
  is τ_T-stable (in the shifted model, β = f'(x₀) ∈ F_p; the D₄ curves, with
  c ∈ F_p, satisfy this, and so does any T with f'(x₀) ∈ F_p).
- Its fixed points in P(L(5∞−Q)) are the f with divisor
  Q + P + (P+T) + P′ + (P′+T) − 5∞, P + P′ = S, 2S = −Q: two components (S and
  S+T give the same f), each rational, so the eigenspaces have K-dimensions
  (2, 2). The invariant ring of diag(1,1,−1,−1) is not polynomial (it needs
  v₁², v₁v₂, v₂²), so even the factor 2 does not come as a weighted polynomial
  system. In W, fixed points would need 4 equations in 2 unknowns, so ι is
  free for generic Q (argued by dimension count, not verified). Best case:
  **2048 orbits**.
- Function-side D₄ invariants do not help. The quotient coordinates
  σ_k = e_k(Y²) are the coefficients of N_{E/E′}(f), quartic in c, and
  π = ΠY_i is quadratic. A "Nagao-D₄" system would be 3 quadrics and 9
  quartics in 12 unknowns, Bézout 2³·4⁹ = 2²¹, far above 512.
- Structural reading: the D₄ system is the sub-system of the decomposition
  system on E′ = E/⟨T⟩ whose lifts are F_p-rational. The E′ system has 4096
  solutions = 8 sign patterns × 512; Nagao on E′ has 4096 again and does not see
  the lift, so it is not a route to 512 either.

## 2. Prior art: literature

- K. Nagao, *Decomposed attack for the Jacobian of a hyperelliptic curve over
  an extension field*, ePrint 2007/112, https://eprint.iacr.org/2007/112.
  **abstract-only** (HTTP 200). Claim taken: the test reduces to a system of
  (n²−n)g quadrics via Riemann–Roch; Magma examples (g,n) = (1,3), (2,2), (3,2).
  The g = 1, n = 4 count 12 follows by substitution, not from a stated example.
- K. Nagao, *Decomposition attack for the Jacobian of a hyperelliptic curve
  over an extension field*, ANTS-IX, LNCS 6197, pp. 285–300, 2010,
  https://link.springer.com/chapter/10.1007/978-3-642-14518-6_23.
  **abstract-only** (HTTP 200, preview paywalled). Same claim, adds large
  primes to the base. The full text was not read.
- K. Nagao, *Index calculus attack for Jacobian of hyperelliptic curves of small
  genus using two large primes*, Japan J. Indust. Appl. Math. 24(3), 289–305,
  2007. **recalled**, plus a WebSearch snippet (Project Euclid link not
  fetched). Two large primes in Thériault's setting; not used for the system.
- A. Joux, V. Vitse, *Cover and decomposition index calculus on elliptic curves
  made practical*, Eurocrypt 2012 (ePrint 2011/020),
  https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/CoverDecomp.pdf.
  **read-at-source** (§2–§6, Appendices A, B, D). Claims taken: §2, "Nagao's
  approach is less efficient than Semaev's in the elliptic case"; §3.1, (n−1)ng
  quadrics and variables, probability 1/(ng)!; §4.2, genus 2 over F_{p³} gives
  12 variables and 12 equations; Appendix D, **one decomposition test on that
  system takes 3,780 s (Magma V2-17-5, Core 2 at 2.6 GHz), 150,000 times slower
  than the genus-3/F_{p²} test** (126 s per 5,000 tests); §5, E/F_{p⁴} has a genus-2
  cover over F_{p²} when #E is odd (j ∉ F_{p²}) or divisible by 4, Nagao on it is 4
  quadrics in 4 variables with Õ(p^{3/2}), and direct Gaudry–Diem on E is 4
  equations in 4 variables of degree 8; Appendix B, imaginary model of that cover
  needs full 2-torsion.
- J.-C. Faugère, A. Wallet, *The point decomposition problem over hyperelliptic
  curves*, Des. Codes Cryptogr. 86(10), 2279–2314, 2018,
  https://researchportal.ip-paris.fr/en/publications/the-point-decomposition-problem-over-hyperelliptic-curves-toward-/.
  **abstract-only** (HTTP 200). Claim taken: reductions (4096 → 64) are for
  even characteristic only.
- A. Wallet, talk, GT-BAC 2016-12-08,
  https://gtbac.telecom-paristech.fr/slides/161208wallet.pdf.
  **read-at-source** (text extracted). Claims taken: slide 26, deg W = 2^{n(n−1)g},
  "no degree reduction known", g = 2, n = 3: degree 4096, 12 variables; the
  "Situation" slide, "g = 1: Summation more efficient", one PDP₆ instance about
  1500 s over F_{2^15}; slide 36, 4096 → 1500 s; slide 43, limits: "no reduction in
  characteristic p > 2", "can't exploit Jacobian automorphisms (2-torsion, …)".
- Gaudry (JSC 2009), Diem (Compositio 2011), Galbraith–Gaudry (DCC 2016):
  **recalled** (their statements as restated in Joux–Vitse §2 and the repository
  survey). Claim taken: the decomposition attack on E(F_{p^n}) with base x ∈ F_p.

## 3. Prior art: these repositories

- `research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md` §4.2, §5.
  Nagao is the un-eliminated S_{m+1}, same Bézout degree; JV: less efficient than
  Semaev at g = 1. Supports; silent on the composite k = 4 count and on symmetry.
- `research/pdp_literature_leads_20261022/leads/LEAD-13.md`. Nagao system and
  Faugère–Wallet read at slide level; odd characteristic has no degree reduction;
  no 2-torsion quotient at g ≥ 2. Supports.
- `research/gaudry_quartic_d4_20261013/README.md`. Quotient dimension 512, C₄ =
  2.535–2.552·10¹⁰, char. poly of the 4096-dimensional S₄ form 7.06·10¹⁰, decomposition
  rate 1/8, one target in 20 degenerate (dimension 517). Supports.
- `research/gaudry_quartic_d4_cost_20261014/README.md`. Baseline now 6.9·10⁷
  (replayed mean), first solve 1.4·10⁸; the formulation share is about 11×.
  Moves the baseline Nagao must beat.
- `research/d4_symmetrised_s5_20261015/README.md`. Weighted Bézout 512, orbit
  count |D₄|/|S₄| = 8, arm w 1.4·10⁹. Supports the orbit argument above.
- `research/ic_next10_20261014/01-fp4-ladder/README.md`. D₄ curves all have
  cofactor h = 4 (T ∈ 2E(K)); D₄ is the cheapest arm, handover 2^{126}–2^{137}.
  Unrelated to Nagao except as the baseline arm.
- `src/cryptanalysis/gaudry_quartic_d4.rs` (header). Instance x₀ ∈ F_{p⁴}\F_p,
  c ∈ F_p, base Y ∈ F_p, `--export-magma` writes the D₄ system. No Nagao code at k = 4.
- `src/cryptanalysis/jv_cover.rs` (header), `research/jv_cover_sparse_fglm_20261018/README.md`.
  Nagao on the genus-3 cover, 6 quadrics in 6 unknowns, ideal degree 64, odd p.
  The only Nagao system the repository solves at scale; supports the 6×6 anchor.
- `src/cryptanalysis/glv_gaudry.rs` lines 1643–1714 and
  `RESEARCH_GLV_INDEX_CALCULUS.md` §6. Function-first system at k = 3, 9 unknowns,
  degree ≤ 3, same 64-point variety, **3,110× the ordinary S₄ solve**, classed
  relabelling. Caveat: that chart normalises the y-coefficient (cubic equations,
  t₁,t₂,t₃ kept); the quadric chart analysed here was not costed at k = 3.
  Supports, by analogy.
- `research/f4_fp_msolve_parity_20261011/` (raw `external.round*.tsv`, `versions.txt`).
  msolve 0.6.5, `f4_fp` within 0.96–1.22× of it at one thread; msolve one-thread
  times on random quadrics n = 9, 10, 11: 0.44, 2.4, 17.8 s. Supports the cost
  extrapolation.
- `research/magma_f4_comparison_20261013/` (`ours_d4.jsonl` only). No external
  solver timing of the D₄ system exists. Open.
- `research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md` line 58
  (the cited 150,000×) and §16.3. §16.3 dismisses genus-2 covers of curves over F_{q²}
  as ordinary genus-2 index calculus (Õ(q) against rho's Õ(q)); it does not treat
  Nagao decomposition over F_p ⊂ F_{p²}. Relevant to section 8.
- `src/cryptanalysis/jv_quartic.rs` (header). The (k−1)-point Semaev test at
  k = 4; not a Nagao system. Unrelated.
- `/home/user/cryptanalysis/ecc2k130/runner/codegen/nagaodecomp.py` (header).
  Binary restricted chart for three summands, "roots in a subspace" rationality.
  Unrelated to odd-p k = 4 beyond the shape of f.

## 4. Why it could lower the Gröbner cost or the degree of regularity

It does not, and each step says why.

1. *Degree.* 4096 against 512. The most the central involution could give is
   2048, and only in a non-polynomial quotient. Assumption: ι is free on W.
2. *Post-Gröbner step.* Any solver that follows the repository's path on a
   D-dimensional quotient pays about D³ for the characteristic polynomial:
   1.0·D³ is measured (7.06·10¹⁰ at D = 4096, 1.4·10⁸ at D = 512). That one step
   is 2.8× the whole committed D₄ solve and about 10³× the current baseline.
   A sparse FGLM (D·nnz) is cheaper but still scales as D² or worse.
3. *Gröbner step.* Anchors for the exact system shape (12 quadrics, 12
   unknowns, 4096 points): Joux–Vitse 3,780 s (Magma 2012, 27-bit prime), Wallet
   1500 s (Magma 2.19, F_{2^15}), repository msolve scaling extrapolated to n = 12
   at 130–225 s (one thread; growth 7.3×, 9.6×, 12.7× per added variable; unverified
   desk, assumes the random-quadric proxy). Conversion to F_p multiplications
   (assumed 5·10⁷–10⁹ per second across Magma-2012 and msolve): 3·10¹⁰–10¹².
4. *Table (per decomposition, F_p-multiplication equivalents).*

   | system | D | cost | source |
   |:--|--:|--:|:--|
   | S₄ form, x-line | 4096 | 1.2·10¹² | measured |
   | **Nagao, 12 quadrics** | 4096 | 3·10¹⁰–10¹² | desk, anchors above |
   | D₄ plain, committed | 512 | 2.5·10¹⁰ | measured |
   | D₄ weighted (arm w) | 512 | 1.4·10⁹ | measured |
   | **D₄ current baseline** | 512 | 6.9·10⁷ | measured (replayed) |

   Nagao is in the S₄-form class, as Joux–Vitse say for g = 1. Against the
   committed D₄ it is at best level; against the baseline it is 450–14,000× worse.
5. *Per relation.* On the τ-stable base Nagao has the same hit rate (about 1/181
   per target) as the D₄ system, so the per-solve comparison is the per-relation
   comparison. On the plain x-line base ι does not exist and Nagao is the S₄ form
   in other coordinates (24 solves per relation at about 10¹¹–10¹² each).
6. *The one thing Nagao needs less of.* It needs no interpolation of the 105
   invariant coefficients and no c ∈ F_p; any E with f'(x₀) ∈ F_p admits ι. That
   buys no cost.

## 5. Obstructions

- **Theorem (orbit).** Only C_N(S₄) = {1, all-four} descends to Ω/S₄, so the best
  quotient has 2048 points; the card's kill "solution count not below 4096/8"
  fires. Wallet's slide 43 states the same limit in general ("can't exploit
  Jacobian automorphisms (2-torsion, …)").
- **Same variety.** Nagao's W and Gaudry's S₄ form are birational (both Ω/S₄). At
  the variety level this is a relabelling in the sense of AGENTS.md; only the
  presentation (12 degree-2 against 4 degree-8 unknowns) differs, and the
  repository measured what a presentation with more unknowns costs at k = 3
  (3,110×).
- **D³ lower bound** on the post-Gröbner step (section 4, item 2).
- **Function-side D₄ invariants are quartic** (Bézout 2²¹).
- **Published and measured anchors** (section 4, item 3), all consistent: the
  12-quadric system is 10⁴–10⁵× slower than the 6-quadric one, and the
  6-quadric system already costs 1.6–5·10⁶ in this repository.
- **Generic bounds.** The Nagao oracle uses the subfield F_p ⊂ F_{p⁴} (base
  ≈ p, group ≈ p⁴), so it is not a group-operation oracle and Shoup,
  Corrigan-Gibbs–Kogan do not bound it; the structured-GGM requirement is met
  by construction, as for every k = 4 route. The exponent is rho's (p² without large
  primes). This is a constant-factor lead on the structured line, and it moves
  no exponent.
- **Class.** A stage-level diagnostic at best; it would need an ecbench session
  before any end-to-end claim (AGENTS.md §5a prices are not triggered, since
  no two arms are ranked here).

## 6. Minimal discriminating test (frozen protocol)

Written for the mechanical tier. The test is a confirmation of a closed lead; it
is cheap and is worth running only if the owner wants the k = 4 number on file.

**T0 (director-level prerequisite, new Rust, 1–1.5 person-days).** A new example,
modelled on `function_first_system` (`glv_gaudry.rs` 1656) and the `Fp4`/`E4` types
of `gaudry_quartic.rs`, that writes (a) the Nagao system of section 1 (12
unknowns, 12 quadrics, chart c₄ = 1) and (b) the D₄ plain system (σ₁,σ₂,σ₃,σ₄,π;
four Weil components of degree 4 and π² − σ₄), both in msolve's `.ms` format
(variable line, characteristic line, comma-separated polynomials). Gate: for
each planted target the witness (f through Q, P₁…P₄) vanishes on all 12 equations,
and the D₄ system's rational solutions equal `gaudry_quartic_d4`'s. If a gate fails,
stop and report.

**Instances.** p = 1009 (prime, ≡ 1 mod 4, below 2¹⁵), D₄ instance seed 1 of
`generate_d4_instance` (the p = 1009 ladder row of the cost study). Targets:
3 planted (targets 0–2 of `--planted`) and 3 uniform, same seed. The same curve
and targets feed both arms.

**Arms and instrument.** (A) D₄ plain system; (B) Nagao 12 quadrics; (C,
informational) Nagao with the four e-unknowns kept (16 unknowns). msolve
(record `msolve -h` and the package version; the repository's harness used
0.6.5-1build2) with `msolve -v 2 -g 2 -t T -f IN -o OUT`, T ∈ {1, 4}, three
interleaved rounds, `prlimit --as=12000000000`, timeout 3600 s, host and
isolation level recorded (L0 unless isolab). The same two systems also go through
`f4_fp` (counted F_p multiplications, as `gb_cas_export` does).

**Record** per system: variables, equations, degrees, GB length, staircase size
(quotient dimension, counted from the leading monomials; the counter is part of
T0 if none exists), largest F4 matrix (rows × columns), wall time at T = 1 and 4,
peak RSS, `f4_fp` multiplications.

**Predictions (before the run).**
- P1. Quotient dimension 4096 for B on all six targets (4,000–4,096 allowed for
  solutions at infinity); 512 for A, except a degenerate target (509–517).
- P2. msolve, one thread: A about 1–10 s, B about 100–400 s; B/A ≥ 30.
- P3. `f4_fp` counted multiplications for B ≥ 10¹⁰·⁵ per target.

**Kill criterion (the card's).** The lead survives only if the median over the six
targets of msolve-time(B)/msolve-time(A) is ≤ 0.5 **and** B's quotient dimension
is below 512. If B's quotient is ≥ 512, or B is not ≥ 2× below A, the lead closes by
measurement. A timeout or out-of-memory on B counts as "not below" provided A
solved. **Improvement criterion** (separate, the owner's rule): even if the card's
criterion were met, B's counted cost must be below the current baseline 6.9·10⁷
per solve to count as an improvement; predicted probability below 2%.

**Cost.** 6 systems × 3 rounds × (about 200 s at one thread + about 80 s at four) for
B, seconds for A: under 3 CPU-hours, under 12 GB, plus T0.

## 7. Ranking inputs

expected gain: none (predicted 1–40× worse than the committed D₄, 450–14,000×
worse than the 6.9·10⁷ baseline) | confidence: high (symmetry and degree), medium
(cost) | cost: 1–1.5 person-days plus under 3 CPU-hours | tier: constant (closed;
no exponent) | depends on: nothing; the better Nagao system at k = 4 is on a cover
(section 8).

## 8. Open questions

- **No k = 4 solver run.** The cost numbers are transfers (JV Magma 2012,
  Wallet Magma 2.19, msolve on random quadrics). Conversion to F_p multiplications is
  good to about ±10×. msolve is not installed here and no experiment was run.
- **Solutions at infinity** for the Nagao system at k = 4 (count below 4096) and the
  freeness of ι on W are argued, not computed (the n = 3 analogues are measured).
- **A chart variant.** The (k−1) analogue (f ∈ L(4∞), 9 quadrics in 8 unknowns, hit
  rate about 1/(6p), semi-regular d_reg 5, desk) is the Nagao counterpart of
  `jv_quartic.rs`. JV and Wallet both say Semaev wins at g = 1, so the expectation is
  low; not costed.
- **Next improvement to try (keep-going rule): Nagao on the genus-2 cover, not on E.**
  Joux–Vitse §5 and Appendix B: E/F_{p⁴} in Scholten form has a genus-2 cover H over
  F_{p²}, and Nagao decomposition over F_p on Jac_H(F_{p²}) is **4 quadrics in 4
  unknowns, ideal degree 16**, hit rate 1/24, Õ(p^{3/2}) with large primes. The
  repository has no k = 4 cover route: ledger §16.3 dismisses a different thing (the
  subfield-curve construction and ordinary genus-2 index calculus), and the
  repository's Nagao code is the (2,3) cover at k = 6. Unverified desk estimate: 24
  solves of about 10⁴–10⁵ F_p multiplications (cf. 5·10⁶ at 6×6, D = 64) per relation
  against 181 × 6.9·10⁷, a per-relation gap of several thousand, which would move
  the model handover well below 2^{103}. Obstructions not settled from here: the
  imaginary model needs full rational 2-torsion over F_{p⁴} (a narrower class than the
  D₄ curves, whose cofactor-4 condition gives only T ∈ 2E(K)); the kernel of the
  conorm-norm map against the large-prime subgroup; Cantor arithmetic and the
  Scholten construction are not in `src/`. This deserves its own card (it is
  LEAD-13's territory moved to k = 4), and it is the Nagao system that matters at
  k = 4.
- **Unread.** The full text of Nagao's ANTS 2010 paper and of Faugère–Wallet 2018
  (abstract pages and slides only); the Springer related-content title "A New
  Perspective on the Decomposition in the Jacobian of Small Genus Hyperelliptic
  Curve" (seen, not fetched).

## 9. Sources

URLs fetched, with outcome:

- https://eprint.iacr.org/2007/112: HTTP 200, abstract page read.
- https://link.springer.com/chapter/10.1007/978-3-642-14518-6_23: HTTP 200, abstract
  and reference list read, full text paywalled.
- https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/CoverDecomp.pdf:
  HTTP 200, 384 KB PDF, read (§2–§6, Appendices A, B, D).
- https://gtbac.telecom-paristech.fr/slides/161208wallet.pdf: HTTP 200, 2.1 MB PDF,
  read (slides 25–45).
- https://researchportal.ip-paris.fr/en/publications/the-point-decomposition-problem-over-hyperelliptic-curves-toward-/:
  HTTP 200, abstract read.
- WebSearch for Nagao's JJIAM 2007 paper: returned the Project Euclid links
  (https://projecteuclid.org/euclid.jjiam/1197909114) and a summary; not fetched.
- Not fetched: HAL and Springer full text of Faugère–Wallet (blocked in LEAD-13);
  IACR PDFs (Cloudflare).

Repository paths read (all under `/home/user/crypto` unless stated):
`research/pdp_literature_leads_20261022/BRIEF_SONNET.md`; `…/LEADS.md` (standing
constraints, LEAD-16); `…/leads/LEAD-13.md`;
`research/notes/index-calculus/RESEARCH_PDP_SAT_SMT_NAGAO_SURVEY.md` (§4.2–§5);
`research/gaudry_quartic_d4_20261013/README.md`;
`research/gaudry_quartic_d4_cost_20261014/README.md`;
`research/d4_symmetrised_s5_20261015/README.md`;
`research/ic_next10_20261014/01-fp4-ladder/README.md`;
`research/notes/index-calculus/RESEARCH_GLV_INDEX_CALCULUS.md` (§6);
`research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md` (line 58, §16.3);
`research/jv_cover_sparse_fglm_20261018/README.md`;
`research/f4_fp_msolve_parity_20261011/` (README, raw tsv/jsonl, versions);
`research/magma_f4_comparison_20261013/` (listing, `ours_d4.jsonl`);
`src/cryptanalysis/gaudry_quartic_d4.rs` (header), `jv_cover.rs` (header),
`jv_quartic.rs` (header), `glv_gaudry.rs` (1643–1714);
`/home/user/cryptanalysis/ecc2k130/runner/codegen/nagaodecomp.py` (header).

Unverified desk checks (scratchpad, not committed, not measurements):
`/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/desk/nagao_k4_check.py`
(system counts, degrees and witness at p = 13; Macaulay and monomial numbers; the
msolve n = 12 extrapolation 130–225 s; 3780/0.0252 = 150,000; 4096/48 = 85.3), and
the multiplication-equivalent conversions in section 4.
