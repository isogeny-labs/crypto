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
Results: §2 below is filled from `runs/`, see RESULTS section.

<!-- RESULTS -->
