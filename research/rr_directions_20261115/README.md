# Riemann–Roch decomposition: three cheap yes/no tests (2026-11-15)

**Type.** Search round plus one stage diagnostic. **Standing.** Every statement here is
`[pcc:unverified]`: local L0 runs on a shared 4-vCPU cloud container, no replay
certificate, no validator report, no rho reference. No row for the leaderboard or
`benchmarks/`. Nothing here is a statement about n = 83 or 131.

The owner listed directions for the Riemann–Roch (Nagao) step of index calculus and named
three to run first. This directory answers them:

| pick | question | answer | how |
|:--|:--|:--|:--|
| 1 | torsion-quotient versions of the RR system | **no new system**: on y²+xy=x³+1 the quotient by ⟨T⟩ is the curve itself and the quotient coordinate is x(P+τP); the ladder already measured it | divisor argument + exhaustive check, §1 |
| 2 | SAT against F4 on the quadratic RR system over F₂ⁿ | see §2 (panel, this PR's measurement) | `examples/rr_quadratic_panel.rs`, §2 |
| 3 | parameter count for the lattice / Coppersmith small-x idea over F_p | **fails the count**: the coefficient-box relaxation is loose by ≈ p^{(m−1)/2}∏C(m,k); after eliminating a, one modular equation in m small unknowns with ∏X_k = p^{(m+1)/2} > p | count + brute force at m = 3, §3 |

Sources checked before writing (the `research-kb` server and CLI are not available in this
container; a grep sweep of both checkouts stood in): `research/nagao_relations/README.md`
(incidence and norm encodings, hybrid, support test), `research/ic_rr_norm_ladder_20260930/`
(norm form refutation degree ≈ ℓ+2, constant lever; its reopening list names the
incidence form's first-fall degree as the open item), `research/ic_symmetry_lever_slope_20260929/`
(torsion-symmetrised S₄: constant lever, s̄ = 1.033), `research/notes/ecc2k130/RESEARCH_ECC2K130_WDSAT.md`
and `research/wdsat_rr_20260920/` (CryptoMiniSat decides RR-norm 8/8 and RR-incidence 7/8 at
n = 9 in 10 s; WDSat cannot load the incidence form), `research/semaev_alternatives_audit_20261104/`
(Idea 1 shift lattices at m = 2; univariate Coppersmith control), `research/prime_norm_rank_20261112/`
(prescribed-norm reduction, linear relaxation refuted), `research/split_prime_ecdlp_20261107/followups/p1_rr_dickson/`
(RR with a Dickson tower: D = 4 at m3 t4), `research/non_semaev_decomposition_20261113/`
(isogeny relations closed at the exponent; RR on a membership tower is the surviving form),
`ecc2k130/codegen/{nagaodecomp,nagaoannihilator,wdsat_rr_panel}.py`, `examples/rr_degree_ladder.rs`,
`src/cryptanalysis/pq_f4_f2.rs`, `src/cryptanalysis/sat.rs`.

## 1. Torsion quotient of the RR system: the quotient is the curve

**Claim (derived, checked exhaustively at three sizes).** Let E: y² + xy = x³ + 1 over
GF(2^m), m odd, T = (0, 1) its rational 2-torsion point, τ the 2-power Frobenius
(x, y) ↦ (x², y²). Then

1. x(P + T) = 1/x(P) for every affine P ≠ T. (Divisor argument: x has divisor 2(T) − 2(O);
   x∘τ_T has divisor 2(O) − 2(T) = div(1/x), so x∘τ_T = c/x, and c = 1 by evaluating.)
2. t := x + 1/x is therefore constant on {P, P+T}: it is the quotient coordinate of
   E → E/⟨T⟩, with a double pole at O only.
3. τ̄ := −1 − τ satisfies τ τ̄ = 2 and ker τ̄ = ⟨T⟩ (τ̄P = O ⟺ τP = −P ⟺ x ∈ F₂,
   y² = x + y, whose only solution for m odd is T). So E/⟨T⟩ ≅ E through τ̄, and
   **x(P + τP) = t(P) exactly** — the quotient coordinate is the x-coordinate of E itself
   after the endomorphism.

`pick1_torsion_quotient/isogeny_check.py` checks 1–3 on every affine point at m = 5, 9, 11
(42, 506, 2,114 points; output in `isogeny_check.txt`; the Rust reimplementation
`examples/rr_directions_checks.rs isogeny` adds m = 7, 13). The constant in 3 is 0 at every
size.

**Consequence for the RR system.** "Decompose on E/⟨T⟩ with factor base {t ∈ V} and pull
back" is, point for point, "decompose on E with factor base {x ∈ V} and target τ̄(−R)":
the same curve equation, a uniformly random ℓ-dimensional subspace, a uniformly random
target in the prime-order subgroup. The torsion-quotient RR system is the x-form RR system
with the target relabelled, so its solving-degree distribution is the one
`ic_rr_norm_ladder_20260930` already measured (≈ ℓ + 2 at m = 3, constant lever), and the
quotient does not reduce the variable count: t_i ∈ V still costs ℓ bits per summand. The
pull-back ambiguity ΣP_i ∈ {R, R+T} is bookkeeping (log T is known). The FGHR gain for
summation polynomials comes from expressing the *degree-2^{m−1}* polynomial S_{m+1} in the
invariant t; the RR norm form is already degree 2 in the function coefficients and degree
≤ 3 in the summand bits, so there is no degree to halve.

For a general binary curve y² + xy = x³ + ax² + b the same divisor argument gives
x(P+T) = b/x and E/⟨T⟩ a 2-isogenous curve; the RR system there is the RR system on that
curve, and `research/isogeny_class_ic_yield_20261102/` recorded the solver cost as an
isogeny-class invariant (ratios 1 ± 3.85·10⁻⁵). Over prime fields with prime group order
there is no rational 2- or 4-torsion, which `research/prime_s5_small_root_leads_20261028/`
already recorded against the FGHR symmetrisation. The "isogenies: decompose on E/⟨T⟩"
item collapses to the same statement.

## 3. Lattices for the small-x factor base through the RR parametrisation: the count

Setting: E/F_p, m summands, f ∈ L((m+1)O) monic with f(−R) = 0, so a ∈ F_p^{m−1} free
parameters; F_a(X) = N(f)/(X − x_R) monic of degree m with coefficients c_k(a) quadratic in
a. Relation ⟺ F_a splits with roots x_i ∈ [0, B] that are abscissas, B ≈ p^{1/m}.

**Necessary box condition.** c_k = ±e_k(x_1, …, x_m) with 0 ≤ e_k ≤ C(m,k)B^k. Treating the
c_k(a) as independent uniform values, the expected number of a satisfying all m boxes is

    p^{m−1} · ∏_k C(m,k)B^k / p  =  B^{m(m+1)/2} ∏C(m,k) / p  =  p^{(m−1)/2} ∏C(m,k)   (B = p^{1/m}),

against ≈ B^m/(m! p) ≈ 1/m! genuine decompositions. The relaxation from "roots in [0, B]"
(an m-dimensional set of size B^m) to "coefficients in the boxes" (volume B^{m(m+1)/2}) loses
a factor B^{m(m−1)/2} = p^{(m−1)/2} in selectivity. At m = 2 that is p^{1/2} candidates per
target, each needing a split test, i.e. rho cost per relation; at m = 3, p per target.

**The same count from the Coppersmith side.** Eliminating a from the quadratic map
F_p^{m−1} → F_p^m leaves one hypersurface Φ(c_1, …, c_m) ≡ 0 (mod p) — the summation
polynomial S_{m+1} written in the elementary symmetric functions of the x_i — and the task
becomes one modular equation in m small unknowns with bounds X_k = C(m,k)B^k. Even for a
*linear* modular equation Coppersmith/LLL needs ∏X_k < p; here ∏X_k = p^{(m+1)/2}∏C(m,k) > p
for every m ≥ 2, by exactly the factor above, and the real equation has degree > 1. To be
inside reach B must shrink to B ≲ p^{2/(m(m+1)d)}, which makes the decomposition
probability B^m/(m!p) so small that relation collection alone exceeds p^{1/2}.

**Brute-force check at m = 3** (`pick3_lattice_count/lattice_count.py`, data
`lattice_count.json`; Rust reimplementation `examples/rr_directions_checks.rs lattice`):
y² = x³ + 2x + 3, all (a, b) ∈ F_p², 8 random targets per prime, B = round(p^{1/3}).

| p | B | base pts | boxes (e₁, e₂, e₃) | box-passing (a,b) per target, measured | heuristic | splitting over [0,B] | genuine relations |
|--:|--:|--:|:--|--:|--:|--:|--:|
| 503 | 8 | 7 | 25, 193, 513 | 4,820.1 | 4,825.0 | 0.50 | 0.50 |
| 1,019 | 10 | 5 | 31, 301, 1,001 | 9,161.1 | 9,166.2 | 0.00 | 0.00 |
| 2,027 | 13 | 9 | 40, 508, 2,198 | 20,305.8 | 20,320.0 | 0.63 | 0.63 |
| 4,091 | 16 | 9 | 49, 769, 4,097 | 37,698.9 | 37,681.0 | 0.25 | 0.25 |
| 8,191 | 20 | 12 | 61, 1,201, 8,001 | 71,594.9 | 71,561.6 | 0.25 | 0.25 |

The box count tracks the independence heuristic to 0.1 % at every p (the quadratic forms
behave like random maps for this purpose), and the gap to the genuine count is 10⁴–3·10⁵
and growing with p. Figure: `figures/lattice_count.svg`. At these B the e₃ box already
covers all of F_p, so the measured loss is the e₁, e₂ part alone; the asymptotic count
above is the full one.

**Verdict.** The lattice idea does not survive the parameter count: the quantity a lattice
can make small is not selective enough by a polynomial factor in p, and shrinking B to
restore selectivity destroys the yield. This agrees with the m = 2 shift-lattice harness in
`semaev_alternatives_audit_20261104` (degree-2 Coppersmith reaching e = 0.48, failing at
0.52) and sharpens it: the loss is structural (box volume against variety size), not a
matter of the lattice construction.

## 2. SAT against F4 on the quadratic RR system over GF(2^m)

Preregistration: [`PREREGISTRATION.md`](PREREGISTRATION.md) (frozen before the panel).
Six follow-ups were added at the owner's request and preregistered separately in [`PREREG_FOLLOWUPS.md`](PREREG_FOLLOWUPS.md).


### 2.1 Panel of record: incidence (degree 2) against norm (degree 3), SAT and F4

`examples/rr_quadratic_panel.rs`, curve y² + xy = x³ + 1, factor base every affine point
with x ∈ V (T included), ℓ = ⌊(m+2)/3⌋, 4 planted + 4 unplanted draws per cell, 600 s
budget, F4 cap per [`PREREGISTRATION.md`](PREREGISTRATION.md) amendments 1–2. Every
decided F4 count and every SAT verdict equals the group-arithmetic ground truth (0
mismatches in every cell); every SAT model verifies. Table:
[`tables/panel_snapshot_0910Z.md`](tables/panel_snapshot_0910Z.md) (regenerated at the
end). `[pcc:unverified]`

| m | ℓ | incidence (3ℓ+8m vars): F4 | incidence: SAT conflicts (median) | norm (3ℓ+2m vars): F4 solving degree | norm: SAT conflicts (median) |
|--:|--:|:--|--:|:--|--:|
| 7 | 3 | 2/8 decided (rootless draws, refuted at degree 3); the rest refuse a degree-4 matrix ≈ 5–6.6·10⁵ square | 5,875 | 4 (8/8) | 144 |
| 9 | 3 | 4/8 (rootless only); refused ≈ 1.3–1.5·10⁶ square at degree 4 | 55,171 | 3–4 (8/8) | 217 |
| 11 | 4 | 0/8 (draws 0–3: 600 s budget under the 8 GiB cap; draws 4–7: a degree-3 matrix ≈ 10⁵ × 1.5·10⁵ refused under the 1 GiB cap) | 537,100 (5/8 decided) | 4 (8/8) | 688.5 |
| 13 | 5 | 0/8 (degree-3 matrix ≈ 10⁵ × 1.5·10⁵ refused, 1 GiB cap) | 0/8 decided in 600 s | 0/7 so far: degree-5 matrix ≈ 7·10⁵ square (≈ 60 GB) refused under the 8 GiB cap | 3,385.5 (8/8) |

Reading. The quadratic incidence form is the hard one for **both** engines: F4 needs a
degree-4 step it cannot build from m = 7, and SAT needs 10³× the conflicts of the norm form
at every m. The norm form, with a quarter of the unknowns and one degree more, is decided
by SAT on every draw through m = 13 (median 144 → 3,386 conflicts from m = 7 to 13), and by F4
at solving degree 4 through m = 11; at m = 13 F4 needs a degree-5 step it cannot build.
Owner's hypothesis "quadratic is where SAT wins": on these systems the number of unknowns
decides, not the degree.

### 2.2 F2 — guess-and-solve floors

`rr_sat_amortise guess`, norm arm, the panel's draws, k = 0 … 2ℓ guessed summand bits,
warm (one solver, branches as assumptions) and cold (fresh solver per branch); 320 rows,
0 verdict mismatches, 0 undecided. `[pcc:unverified]`

| m | k = 0 | k = ℓ (guess one summand) | k = 2ℓ | argmin k per draw (cold) |
|--:|--:|--:|--:|:--|
| 11 | 688 | 3,325 (4.8×) | 3,910 | 0 on 7 of 8 draws |
| 13 | 3,385 | 29,378 (8.7×) | 32,311 | 0 on 7 of 8 draws |

(median total conflicts over all 2^k branches, warm mode. Cold lies between −4 % and +16 %
of warm at every k ≥ 1, except m = 13, k = 1, where it is +72 %.) P2 holds:
no guessing split beats the solver on the median draw.

### 2.3 F4 — amortising across targets

`rr_sat_amortise targets`, m = 11, 4 draws × 64 targets (amendment 3), three modes on the
same V: warm (one solver, symbolic target bits as assumptions), cold_sym (fresh solver on
the same symbolic encoding, target bits as units), cold (fresh solver on the constant-target
system). 0 undecided, 0 truth mismatches. `[pcc:unverified]`

| draw | cold_sym / warm (incrementality) | cold / cold_sym (encoding) | cold / warm (net) |
|--:|--:|--:|--:|
| 0 | 1.023 | 0.0187 | 0.0191 |
| 1 | 1.394 | 0.0206 | 0.0288 |
| 2 | 1.237 | 0.0165 | 0.0204 |
| 3 | 1.252 | 0.0173 | 0.0217 |

Keeping learnt clauses across targets saves a median 1.24× in conflicts; making the target
symbolic, which is what lets one solver serve every target, costs ≈ 55×. Net, a fresh solver
per target is 35–52× cheaper. P4 (warm ≤ ½ cold) is falsified.

### 2.4 F6 — Gauss–Jordan in CryptoMiniSat

[`cms/`](cms/): the unmodified CryptoMiniSat 5.11.21 library from its PyPI sdist behind a
thin DIMACS-XOR driver (the upstream CLI could not be built: no GitHub access), single
thread; Gauss–Jordan off sets `max_num_matrices = 0`. 46 panel CNFs × 2 configurations;
every SAT model checked; every decided verdict equals the panel's. `[pcc:unverified]`

| m | arm | CMS Gauss on (median conflicts) | Gauss off | Gauss factor | repo CDCL (panel) |
|--:|:--|--:|--:|--:|--:|
| 7 | norm | 126 | 835.5 | 6.6× | 144 |
| 9 | norm | 274 | 1,513.5 | 5.5× | 217 |
| 11 | norm | 2,197.5 | 22,745.5 | 10.4× | 437 |
| 7 | incidence | 3,586 | 17,021.5 | 4.7× | 5,875 |
| 9 | incidence | 30,336.5 | 222,941 | 7.3× | 55,171 |
| 11 | incidence | 934,076 (5/7) | 5,127,626 (4/7) | 5.5× | 537,100 (5/7) |

Gauss–Jordan is worth 5–10× to CryptoMiniSat on these systems (P6: ≥ 10× on the norm arm at
m ≥ 11 holds at m = 11, by 0.4). The repository's own CDCL, which handles XORs natively
with its own elimination, needs fewer conflicts than CryptoMiniSat-with-Gauss on the norm
arm at m = 11 (437 against 2,198); conflicts are not the same unit across solvers, so this is
reported, not ranked.

<!-- RESULTS-PENDING: F1 (ℓ-ladder), F3 (norm4), F5 (XL), F6 m = 13 -->

