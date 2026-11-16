# LEAD-10 — Chord-parametrised (slope-variable) small-root formulation for prime-field PDP

**Verdict.** closed-by-theorem for the small-unknown polynomial family; the slope-as-large-unknown variant is closed by theorem only for top-corner leading monomials and by desk MILP at `ℓ ≤ 3` otherwise. The chord system at `m = 2` is Semaev's `S₃` in the symmetric `(e₁, e₂)` encoding after an exact, free, linear elimination of the slope. At `m = 3, 4` the slopes are eliminated the same way (leaving `S₄`, `S₅` in some encoding) or kept as large unknowns, which adds nothing in the tests run. Every encoding has the same asymptotic reach `2/(m(m+1)d)` and the same ceiling `1/(md)`. A constant-factor lattice-size gain is real at `m = 2, 3` and sits on oracles that cannot beat rho.
**Confidence.** medium. The `m = 2` identity is exact (symbolic and numeric desk check). The reach rests on a hand integral and a Python LP that reproduces W10 and `05`; the ceiling on W10's charging argument (re-derived here, generic coefficients). The large-unknown case beyond `ℓ = 3` with non-top leading vertices is untested.
**Expected gain if it works.** none in the exponent. Constant only: at equal multiplicity the symmetric encoding reaches 5–11 % higher `δ` (unverified desk LP: `m = 2, ℓ = 3`: 0.1361 at dimension 45 against 0.1289 at dimension 100; `m = 3, ℓ = 1, 2`: +7 %, +5 %). No leaderboard row moves: `m = 2` can never beat `√p`, `m = 3` ties only at `δ = 1/4` against a ceiling of `1/12`.
**Cost of the discriminating test.** the zero-compute test of the card is done in this dossier (about 3 person-hours and 2 CPU-hours, all unverified desk checks). A frozen confirmation (§6): 0.5 person-day to patch a 2-line weight option into the `05` MILP harness, then under 1 CPU-hour, under 2 GB.

## 1. Mechanism (precise)

**Setting.** `E: y² = x³ + ax + b` over `F_p`, target `Q = (x_Q, y_Q)`, base `F = {P : 0 < x(P) ≤ B}`, `B = p^δ`. Relation: `Q ± P₁ ± … ± P_m = O`.

**`m = 2`, exact.** The line through `Q` with slope `λ` is `y = λx + ν`, `ν = y_Q − λx_Q`. Substituting gives `x³ − λ²x² + (a − 2λν)x + (b − ν²) = 0` with roots `x_Q, x₁, x₂`. Vieta, with `s₁ = x₁ + x₂`, `s₂ = x₁x₂`:

- (C-a) `s₁ = λ² − x_Q`
- (C-b) `s₂ = x_Qλ² − 2y_Qλ + a + x_Q²`

(the third Vieta relation `x_Q s₂ = ν² − b` is the same equation modulo the curve equation of `Q`). Necessary, not sufficient, for small `x₁, x₂`: `|s₁| < 2B`, `|s₂| < B²` (`x₁, x₂` are the roots of `T² − s₁T + s₂`).

**Elimination is free.** (C-b) − `x_Q`·(C-a) gives `2y_Qλ = x_Qs₁ + 2x_Q² + a − s₂`: the slope is affine in `(s₁, s₂)`. Substituting into (C-a):

`F(s₁, s₂) = (x_Qs₁ − s₂ + 2x_Q² + a)² − 4y_Q²(s₁ + x_Q) ≡ 0 (mod p)`.

With `y_Q² = x_Q³ + ax_Q + b` this is **exactly** `S₃(x₁, x₂, x_Q)` written in `(e₁, e₂) = (s₁, s₂)` (unverified desk check: sympy difference `0`; toy `p = 1000003`: planted pair `x = 1, 2` gives `F ≡ 0`). Support: `{1, s₁, s₂, s₁², s₁s₂, s₂²}`, the triangle `i + j ≤ 2`, bounds `(2B, B²)`, weights `(δ, 2δ)`. The unknown count is `(λ, s₁, s₂)` against two equations; mod `p` the ideal `(g₁, g₂)` contains the linear relation and `F`, so the slope carries no information beyond `F`. This `F` is W10's `e`-encoding and probe C's `Sym` lattice.

**The literal reading** ("two small-value conditions on polynomials of one unknown `λ`") is not a small-root problem: `λ` is uniform in `F_p` (weight 1), and `{λ : |λ² − x_Q| < 2B}` is a set of square roots, not an interval. Localising `λ` to a window of width `Y` costs `p/Y` guesses, and even a perfect oracle then needs `Y ≲ p²/(2B³)` (three unknowns, two equations): `p^{1/2}` windows at `B = p^{1/2}`, which is enumeration.

**Reach for `m = 2`, by hand.** Take `a = 0` (constant monomial) as leading vertex and `M = {β : β₁ + β₂ ≤ T}`. The shift `x^β f^k` fits iff `β + kP ⊂ M`, i.e. `k = (T − |β|)/2`. Continuum: `Σk = T³/12`, cost `Σ(β₁ + 2β₂) = T³/2`, so `δ = 1/6`. For general `m`, degree `d` per variable, simplex support `Σi ≤ d` and weights `1..m`: `Σk = T^{m+1}/(d(m+1)!)`, cost `= (m(m+1)/2)·T^{m+1}/(m+1)!`, so

`δ_e = 2/(m(m+1)d) = 1/6, 1/24, 1/80` for `S₃, S₄, S₅`,

the same limit as W10's `x`-encoding box. It equals the symmetric-encoding information bound `1/Σj = 2/(m(m+1))` divided by `d`. (Hand integral, lower bound on the encoding's reach; the upper bound is the ceiling in §5.) Desk LP, §4, agrees numerically.

**`m = 3`, two slopes.** `T = −(Q + P₁)`, so `P₂ + P₃ = T`. Chord 1 (through `Q, P₁, T`): `λ₁² = x_Q + x₁ + x_T`. Chord 2 (through `P₂, P₃, −T`): `λ₂² = x₂ + x₃ + x_T`. With `x_T`, `y_T = λ₁(x_T − x_Q) + y_Q` eliminated:

- (C1) `x₁x_T = x_Qλ₁² − 2y_Qλ₁ + a + x_Q²`, `x_T = λ₁² − x_Q − x₁`
- (C2) `x₂ + x₃ = λ₂² − x_T`
- (C3) `x₂x₃ = x_Tλ₂² + 2y_Tλ₂ + a + x_T²`

Five unknowns `(λ₁, λ₂, x₁, x₂, x₃)`, three equations, so the information bound is `B³ < p`. The degrees in the slopes rise to 4 (`λ₁⁴`, `λ₁³λ₂` in C3), which is the card's "higher degree per unknown". Each slope again eliminates linearly: `S₃(x₁, x_T, x_Q) = 0` and `S₃(x₂, x₃, x_T) = 0`, and `Res_{x_T}` is `S₄` (degree 4 per variable, total 12). The small-unknown polytopes are: the `x` box `[0,4]³` (W10), the fully symmetric simplex `i + j + k ≤ 4`, weights `(1,2,3)` (35 monomials, W10), and the **partially symmetric prism** `(x₁; E₁ = x₂ + x₃, E₂ = x₂x₃)`, `i ≤ 4`, `j + k ≤ 4`, weights `(1,1,2)`, which chord 2 produces and W10 did not run. Continuum reach of the prism with region `[0,T] × {j + k ≤ T}`: `0.0416665 ≈ 1/24` at box ratio 1.0 (desk quadrature); a single simplex region gives only 0.0335.

**`m = 4`.** `S₅`, degree 8 per variable, reach `1/80`, needed `≈ 1/5` (program §2.4), card trigger `1/10`.

**Bound the formulation obeys.** (This is a lattice, not a Gröbner, lead: there is no degree of regularity.) The top vertex of `S_{m+1}` has weighted degree `md` in the small unknowns in every monomial encoding (`e_j` carries weight `j`), so `δ < 1/(md)` = `1/4, 1/12, 1/32` for any triangular single-polynomial lattice (W10 charging argument, §5).

## 2. Prior art: literature

- Semaev, *Summation polynomials and the discrete logarithm problem on elliptic curves*, ePrint 2004/031, https://eprint.iacr.org/2004/031. **abstract-only.** Claim: the index calculus reduces to finding bounded solutions of explicit modular multivariate equations. The page says nothing on prime fields.
- Jochemsz, May, *A Strategy for Finding Roots of Multivariate Polynomials with New Applications in Attacking RSA Variants*, ASIACRYPT 2006, LNCS 4284, pp. 267–282, https://www.iacr.org/archive/asiacrypt2006/42840270/42840270.pdf. **recalled** (PDF HTTP 403; Springer redirected to an IdP, 303). Claim used: shifts are monomial multiples of powers of `f` and success is a determinant condition; my reach numbers are computed independently (§4).
- Ryan, *Solving Multivariate Coppersmith Problems with Known Moduli* (cuso), ePrint 2024/1577, EUROCRYPT 2025. **abstract-only** (ePrint page fetched). Claim: automated shift selection and polytope-based asymptotic bounds.
- Feng, Luo, Chen, Nitaj, Pan, *Computing Asymptotic Bounds for Small Roots in Coppersmith's Method via Sumset Theory*, ePrint 2024/1330. **abstract-only.** Claim: provable asymptotic-bound computation by sumset theory, covering Jochemsz–May extended strategies and systems.
- Corrigan-Gibbs, Kogan, *The Discrete-Logarithm Problem with Preprocessing*, EUROCRYPT 2018, ePrint 2017/1113. **abstract-only.** Claim: for generic algorithms `S·T² = Ω̃(εN)`; "generic" means working in every cyclic group, so an algorithm that takes square roots of coordinates is outside the model.
- Petit, Kosters, Messeng, *Algebraic approaches for the ECDLP over prime fields*, PKC 2016, https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf. **read-at-source** (6-page PDF, abstract and introduction). Claim: Semaev's original base is `L(x) = Π_{α<B}(x − α)`; the paper then uses Gröbner systems; `pdftotext` finds no "Coppersmith", "lattice" or "bounded".
- Dossou-Yovo, Nitaj, Togbé, *Solving modular cubic equations with Coppersmith's method*, Information and Computation, 2024. **listing-only** (search aggregator, not read). I found no "Hirai–Takagi" paper on finding curve points with Coppersmith; the card's attribution is unverified.
- Sibling dossier LEAD-12 (read for cross-check; an interpretation, not a theorem): for `m = 4` the SGGM-necessary `b` is `> 1/8`, above the card's `1/10` trigger.

I found no published lattice treatment of summation polynomials or of chord slopes (searched: Coppersmith plus summation polynomial/small-`x`; PKM text). Absence of results is not proof of absence.

## 3. Prior art: these repositories

- `research/lit2026_w10_s5_coppersmith_20261011/README.md` §2, §3, §5–§7 (+ `run.md` §4b, `examples/lit2026_w10_coppersmith_bound.rs`): exact closure LP, `1/6, 1/24, 1/80`, ceiling `1/(md)`, `e`-encoding rows. **Closes**: its `e`-encoding is this lead's `m = 2` polytope. Its `e` rows use a fixed box `L = m_x(ℓ+1)` (`GenSpec`, lines 1509–1530), which binds at `m = 3`: 0.018176 and 0.024903 are lower bounds (desk: 0.0201 and 0.0269 with a non-binding box).
- `research/ic_next10_20261014/05-multipoly-lattice/README.md` and `milp_multi.py`: multi-polynomial ceiling (§3), V1 curve equations (no gain), V2 chain with a large intermediate `t` (`δ = 0`, 16 leading-vertex tuples, `ℓ = 1`), shared targets 1.12×. **Closes** the large-unknown reading for top-corner leads (theorem) and `ℓ = 1` (MILP); input format `run_system(name, nv, large, supps, ℓ, box)`, weights `δ` small, `1` large.
- `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §2.1–§2.4, §3.2–§3.3, §4.6: needed `δ` (1/2, 1/4, 1/5), probes B/C (all variants fade at 0.12–0.16, `Sym` same threshold in 3–10× fewer dimensions). **Supports.** §4.6 gives the `(e₁, e₂)` asymptotic reach as `1/9 = 0.111`; W10's own `e` rows (0.125 at `ℓ = 2`, 0.1361 at `ℓ = 3`) already exceed it, so `1/9` looks like a basic-strategy bound, while the family value is `1/6` (§1, §4). A correction candidate for the program note, not an improvement.
- `src/cryptanalysis/semaev_higher.rs` (header): `S₄ = Res(S₃, S₃)`, the chord elimination's resultant. **Supports.** `src/cryptanalysis/prime_small_x.rs` (header): small-`x` base, yield `2D/(N−1)`, the `B²/p` density used in §5. `research/prime_field_smt_20261005/FOLLOWUP_S3_PROTOCOL.md` (grep): slope used only to lift SAT pairs. Both unrelated to reach.

## 4. Why it could lower the Gröbner cost or the degree of regularity

It does not lower either: this is a Coppersmith lead and the Gröbner solver is not involved. What the chord reading does contribute:

1. **Exact equivalence** with the `e`-encoding at `m = 2`, and with the symmetric or pair-symmetric encodings at `m = 3`.
2. **Closed-form asymptotics** `2/(m(m+1)d)` for the simplex region: the `e` limit equals the `x` limit (generic coefficients, full-rank triangular family).
3. **Finite-`ℓ` advantage** (unverified desk LP, a Python re-implementation of the W10 closure validated against W10 and `05`):

| `m` | `ℓ` | `x` box | `e` simplex | prism `(x₁; E)` |
|:--|--:|--:|--:|--:|
| 2 | 1 | 0.0900 (= W10) | 0.1000 (= W10) | n/a |
| 2 | 3 | 0.1289 (= W10) | 0.1361 (= W10) | n/a |
| 2 | 12 / 24 | 0.1548 (W10, ℓ=12) | 0.1575 / 0.1619 | n/a |
| 2 | fit ℓ→∞ | 1/6 | 0.1665 (fit A − B/ℓ + C/ℓ²) | n/a |
| 3 | 1 | 0.018780 (= `05` 1331/70875, reproduced) | 0.020147 (11/546) | 0.019206 (121/6300) |
| 3 | 2 | 0.025591 (`05`) | 0.026901 (23/855) | not finished (LP over 45 min) |

Non-binding box checked by growing `L` until `δ` was unchanged. The advantage is a boundary effect of the symmetric quotient (5–10 %), shrinking with `ℓ`; it buys 2–3× smaller lattices at equal reach, the probe-C observation, not a new one.

## 5. Obstructions

- **Algebraic equivalence** (§1). The only relations among the small unknowns generated by the chord ideal lie in the principal elimination ideal `(S_{m+1})`. Lattices over small unknowns are lattices over multiples of `S_{m+1}`; W10's monomial shifts span them.
- **Weighted-degree ceiling** `δ < 1/(md) = 1/4, 1/12, 1/32`: W10's charging argument forces `λ·c < 1` for every vertex `c` of the support. I re-derived it: the shift at `β` puts `u_j = β + j(c − a)` in `M`, `u_j ≥ jc`, so `Σk ≤ Σ λ·u/(λ·c)`. It is invariant under every monomial encoding because the top vertex has weighted degree `md`. Against the need: `m = 2`: 1/4 vs 1/2 (2×); `m = 3`: 1/12 vs 1/4 (3×); `m = 4`: 1/32 vs 1/5 (6.4×), vs card trigger 1/10 (3.2×), vs SGGM-necessary 1/8 (4×). The card's kill criterion at `m = 4` fires (`1/80 < 1/32`).
- **Large unknowns.** Slopes and `x_T` have weight 1; a monomial `λ²` weighs 2. `05` §3 proves the ceiling for multi-polynomial systems with top-corner leading monomials, and its MILP finds `δ = 0` for V1/V2 at `ℓ = 1`. The multi-polynomial charging argument does not extend to non-top leads (it only gives an `r`-fold weaker bound), so I ran a weight-patched copy of `milp_multi.py` (unverified desk check; all leading-vertex tuples; the harness family is monomial multiples of generator products, so raw-generator values are lower bounds on the full ideal):
  - raw chord generators `g₁, g₂`, `ℓ = 1`: `(λ, x₁, x₂)` 0 over 9 tuples; `(λ, s₁, s₂)` 0 over 7; plus the linear relation `h` as third generator: 0 over 10 at `ℓ = 1`, **1/15 at `ℓ = 2`** (10 tuples, below the control 1/8);
  - two-slope `m = 3` system C1–C3, boxes `[4]^5` and `[6]^5`, 36 tuples each, `ℓ = 1`: 0;
  - the informative test, bare `F` plus the slope and its relation `h`: `ℓ = 1, 2, 3`: 1/10, 1/8, 49/360 (best over 6 tuples each; at `ℓ = 3` three tuples tie exactly, the others are lower with a binding box), equal to the bare-`F` control (1/10, 1/8, 49/360 = W10's `e` values). Some optima put `λ¹` in `M` and still only tie. The slope adds nothing at `ℓ ≤ 3`.

  Open: non-top leads at `ℓ ≥ 4`, and the same test for `m = 3`.
- **Output size.** In the `(s₁, s₂)` encoding the number of small roots is `≈ 8B³/p` (desk count, toy `p = 1000003`, 40 targets: `B = p^{0.4}`: 126.7 against 126.5; slopes with small `s₁` alone: 3994 against `4B = 4000` at `B = p^{1/2}`, where the `s₂` bound is vacuous). Genuine decompositions: 0.43 per target there, so about `4p^{1/2}` slopes per genuine root (`≈ 10⁴` at `p ≈ 10⁶`). A polynomial-output oracle exists only for `δ ≤ 2/(m(m+1)) = 1/3, 1/6, 1/10`, below the needed `1/2, 1/4, 1/5`. The `x` encoding has output bound `1/m` and is not the obstruction there; the ceiling is.
- **Enumeration line.** Listing the `4B` slopes costs `≥ 4B` square roots per trial, with `p/B²` trials per relation: cost `~p/B` per relation, the IC-2 row of program §1 with a worse constant. CGK and Shoup do not formally bind a square-root enumeration (it reads coordinates), but its cost is at least its output, which is the same line. A lattice oracle is outside every generic model (LEAD-12 gate), so the measured negatives bind. Economics (program §2.4): `m = 2` never beats `√p`, `m = 3` ties at best, `m = 4` needs `T_dec ≪ p^{1/4}` at `δ ≈ 1/5`.
- **Not closed here** (W10 §7): non-full-rank sublattices beyond the triangular argument, Gröbner-optimal shifts, Herrmann–May linearisation, finite-`p` heuristic gains (probe B: roots to 0.12–0.16 at `m = 2`).

## 6. Minimal discriminating test (frozen protocol)

The card's zero-compute test is done (§1, §4). One frozen confirmation would reopen the lead if it fails: does adding the slope as a large unknown to the bare `e`-encoding generator ever beat the bare generator?

- **Prerequisite (core tier, once).** In a copy of `research/ic_next10_20261014/05-multipoly-lattice/milp_multi.py` add a global per-variable weight vector for small unknowns (`small_sum = Σ WTS[v]·m[v]`) and cases `ctrl`, `v4d`, `v4a`, `v5`. A Rust port of the same supports is equivalent.
- **Instances** (exponent order `(λ, s₁, s₂)`, `λ` large, weights `(·,1,2)`): `ctrl` = `F` alone, support `{i + j ≤ 2}`; `v4d` = `F` plus `h = {λ, s₁, s₂, 1}`; `v4a` = `(λ, x₁, x₂)`, weights `(·,1,1)`, `g₁ = {λ², x₁, x₂, 1}`, `g₂ = {λ², λ, x₁x₂, 1}`; `v5` = C1–C3 of §1 in `(λ₁, λ₂, x₁, x₂, x₃)`, weights `(·,·,1,1,1)`. Multiplicity `ℓ = 1, 2, 3` for `ctrl`, `v4d`; `ℓ = 1` for `v4a`, `v5`. Boxes `[5,12,12]`, `[4,6,6]`, `[6]^5`. All leading-vertex tuples with a common weight; per-MILP limit 90 s, a limit hit marks the row a lower bound.
- **Record**: exact `δ_ℓ` per tuple, `binds` flag, best over tuples.
- **Prediction**: `ctrl` = 1/10, 1/8, 49/360 for `ℓ = 1, 2, 3`; `v4d` equal to `ctrl` at each `ℓ`; `v4a`, `v5` = 0.
- **Closed stays closed**: `ctrl` reproduced and `v4d ≤ ctrl`, `v4a = v5 = 0`. **Reopen**: any `v4d` row (`binds = no`, no limit hit) exceeds `ctrl` by `≥ 2^{-30}`, or `v5 > 0`. **Invalid** if `ctrl` is not reproduced.
- **Cost**: 0.5 person-day core, then under 1 CPU-hour and under 2 GB for the mechanical tier. My desk run of the same cases (`ℓ ≤ 3`) took about 1 CPU-hour. No LLL run is justified: the card's gate (`δ* ≥ 1/10` at `m = 4`) fails by the ceiling alone.

## 7. Ranking inputs

expected gain: none in exponent; ≤ 1.1× reach / 2–3× smaller lattices at `m = 2, 3`, not a leaderboard row | confidence: medium | cost: 0.5 person-day, under 1 CPU-hour | tier: constant (closed for exponent) | depends on: W10 `lit2026_w10`, `05-multipoly-lattice`.

## 8. Open questions

- Jochemsz–May 2006, Ryan 2024/1577 and the 2026/1027 LP were not read at source (403). The reach is my own LP and integrals, validated against W10 (`ℓ ≤ 3`) and `05` (1331/70875); a check against 2026/1027's tables is owed.
- W10's Rust engine should rerun its `e` rows at `m = 3, 4` with a non-binding box to confirm 0.0201 and 0.0269 (mine is Python, unverified).
- The `S_5` pair-symmetric polytope `(e₁, e₂; e₁', e₂')` was not run; its information bound is `1/6`, below the needed `1/5`.
- Is the `e`-encoding limit exactly `2/(m(m+1)d)` (upper bound)? The closure optimum was a box in 70/70 cells for `x`; for `e` I have 12 values (`ℓ ≤ 24`) at `m = 2` and 2 at `m = 3`, no proof.
- Does the slope help at `ℓ ≥ 4` with non-top leads, or at `m = 3` (the `F + h` test was run only for `m = 2`)? Not settled; §6 is the test.
- Chord enumeration constants (`4B` square roots against `B` point additions) were not measured.

## 9. Sources

Fetched (outcome): `eprint.iacr.org/2004/031`, `/2024/1577`, `/2024/1330`, `/2026/423`, `/2017/1113`, `arxiv.org/abs/1504.01175` (all 200, abstract pages); `eprint.iacr.org/2024/1330.pdf` (403); `www.iacr.org/archive/asiacrypt2006/42840270/42840270.pdf` (403, WebFetch and curl); `link.springer.com/chapter/10.1007/11935230_18` (303 to IdP, not followed); `arxiv.org/pdf/2310.12572` (200, JM context, no formal statement found); `christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf` (200, read); `win.tue.nl/seminar/abstracts/jochemsz151106.html` (503). WebSearch only: the Information and Computation 2024 listing.

Repository paths read: `research/pdp_literature_leads_20261022/{BRIEF_SONNET.md,LEADS.md,leads/LEAD-12.md}`; `research/lit2026_w10_s5_coppersmith_20261011/{README.md,reports/run-20261011T2118Z/run.md}`; `examples/lit2026_w10_coppersmith_bound.rs` (lines 860–964, 1500–1570); `research/ic_next10_20261014/05-multipoly-lattice/{README.md,milp_multi.py,milp_v2.md}`; `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` (lines 1–400); `src/cryptanalysis/{prime_small_x.rs,semaev_higher.rs}` (headers); `research/prime_field_smt_20261005/FOLLOWUP_S3_PROTOCOL.md` (grep).

Scratch, not committed, all unverified desk checks: the `*.py` files in `/tmp/claude-0/-home-user/cf89363d-ad6e-5bef-bde6-195cbd206c93/scratchpad/lead10/` (`chk_identity.py`, `closure_lp.py`, `count_lambda.py`, `milp_multi_w.py`, `run_*.py`); venv with scipy 1.18.1, sympy 1.14.0.
