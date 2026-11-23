# L2-lattice: small roots of Semaev polynomials beyond Coppersmith — report (2026-11-13)

Every number here is `[pcc:unverified]`. Labels: **measured** (run here), **computed** (exact finite
computation here, not a proof), **derived** (argument written here), **cited** (repo note, not re-checked).
Protocol frozen before computation: `PROTOCOL.md` (sha256 in `PROTOCOL.sha256`); `AMENDMENTS.md` has two
post-data interpretation notes, no rule change.

## 0. Verdict

**KILL at desk/toy scale for every idea examined (9 ideas).** None changes the exponent. No idea passed the
step-3 gate, so no fpylll/LLL run was made (the protocol forbids running one without a SURVIVE).
The best measured reach is still the S3 lattice of Probe B/C. In total IC cost it gives exponent **0.84–0.90**
(heuristic δ 0.12–0.16 to rigorous δ 0.10). That is **+0.34 to +0.40** above rho's 0.5 (§4).

## 1. Sources checked
- research-kb and the lead atlas were **not available** in this environment. I grepped both repos instead.
- **Prior closures I build on and do not repeat** (cited):
  - `crypto:research/prime_s5_small_root_leads_20261028/README.md`: F1–F6, K1–K16, L4-A/B/C, §7c;
  - `crypto:research/smallroot_optimal_reach_20261026/` (strategy-independent bound);
  - `crypto:research/smallroot_followups_20261026/README.md` (any-monomial-set h_N theorem; joint (4,2) ν_s);
  - `crypto:RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §2–§4 (Probe B/C);
  - `crypto:research/notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md`.
- **Gap this lab targets.** Those notes leave one lattice item open: "lattices that use more than the one
  ideal (p,S)^t, e.g. several targets' polynomials at once" (lead-4 note §7c). This lab computes that item.

## 2. Instrument
- **Tool.** `nu.py` computes ν_s(T), the least weight of a nonzero element of Ī^s found in a Macaulay matrix
  of degree ≤ T, over F_q with q = 1000003, on a random curve and target.
- **Weights.** Each variable gets weight log_p(bound):
  - x_i gets δ; e_i gets iδ;
  - anything ranging over all of F_p gets 1: y_i, the slope λ, the coefficients of the function through R,
    and an intermediate x_T.
- **Criterion.** The method is the one in smallroot_followups §3. A lattice built from (p, I)^t can satisfy
  Howgrave-Graham only if ν_s < s for some s.
- **The comparison.** Each system's ν_s/s is compared with min(1, h_S) of the single eliminated S.
- **What a result can show.** ν_s(T) is an *upper* bound on ν_s, so the instrument can certify a
  low-weight element but cannot prove one is absent. Results are "computed, not proved" (s ≤ 3, T up to 14).
- **Controls.** 405 cells; no command was censored.

| control | expected | result |
|---|---|---|
| POS: {t − a·e1, t − b·e1 − e2}, t of weight 1 | ν_1 = 2δ (elimination through a large unknown) | 2δ at all 6 δ, all (s, T) — **fires** (δ = 1/2 gives exactly 1; see AMENDMENTS 1) |
| NEG S3, x-form and e-form | ν_s/s = 4δ | 4δ at δ ≤ 1/4, all s ≤ 3, T and T+2 — **matches** |
| RAND_A2 / RAND_A4: random coefficients, same support as A2/A4 | — | identical to A2/A4 in every cell. The curve structure is not load-bearing |

## 3. Ideas, reach, verdict
"Ref" means min(1, h_S) for the eliminated single polynomial. Computed values are in `grid_summary.md`,
with raw data in `grid.jsonl`.

| # | idea (structure used) | reach computation | result | verdict |
|---|---|---|---|---|
| A1 | (a) Extra shared-root equations: curve equations for y1, y2 plus the collinearity of P1, P2, −R | **derived, rigorous for s=1:** an element of weight < 1 contains no y, so it lies in the elimination ideal (S3) (F1), and its weight is ≥ 4δ. **computed** s ≤ 2: ν_s/s ≥ ref everywhere (1.17–1.5 at δ ≥ 1/6, truncation-limited) | y has bound p, so each y-degree costs weight 1, the same as a factor p | **KILL** |
| A2/A3 | (a) The rational parametrisation (new presentation). Decompositions of R ⇔ a line through R with slope λ ∈ F_p, giving e1 = λ² − x_R and e2 = x_Rλ² − 2y_Rλ + a + x_R², in e- and x-form | **computed**, s ≤ 3, T and T+2 | ν_s/s = ref = min(1, 4δ) **exactly** in all 72 cells | **KILL** (adds nothing to S3) |
| A4 | (a) The same for m = 3: the function x² + a1x + a0 + cy through R, with 2 large parameters and e1..e3 quadratic in them | **derived s=1** (weight < 1 ⇒ no parameter ⇒ in (S4)); **computed** s ≤ 3 | ν_s/s = 1.0 at δ = 1/12. It is **1.5 at δ = 1/4 (milestone)**, stable over s = 1..3 and T 8–12 | **KILL**: necessary condition fails at δ = 1/4 |
| A5 | (a) The same for m = 4: the function xy + a2x² + a1x + a0 + cy, with 3 large parameters | **computed**, s ≤ 2, T ≤ 8 | ν_s/s = **1.9–2.0 at δ = 1/5 (target)** and 2.0 at δ = 1/4 | **KILL**: necessary condition fails at δ = 1/5 |
| B | (b) Several targets in one joint lattice (disjoint unknowns); free choice of R = aP+bQ | **computed**: ν_s/s = 4δ = ref in all cells. **derived**: the initial forms lie in disjoint variables, so in_w(I^s) = (in S1, in S2)^s; the lattice is block-like. Random R only changes the per-target success probability B^m/p, which §2.4 already counts | no gain | **KILL** |
| C | (c) Other models and coordinates: Möbius / Montgomery u / Edwards y; small y; small x and y together; Farey u/v | **cited** F3/F4, K1, K2, K6, K7, K9, K14. **derived**: Montgomery u and Edwards y are degree-2 functions, so each is M∘x∘τ (F3). Farey x = u/v is bihomogeneous of degree (2,2), and its JM count with D = 2ℓ reproduces the affine condition with X ↔ B² = \|F\|, so the reach is identical per base size. A small-x-and-y base has size ≈ B²/p | same reach or worse | **KILL** |
| D | (d) 2-adic, Hensel or ACD reformulation; small-height targets x_R = n/d | **derived**: (p, g)^ℓ depends only on g mod p, so small integer coefficients give an identical lattice L(M), and exactness needs \|g\| < p (F6/K10). ACD needs an unknown common modulus, but p is known. **cited** K11, K12. Choosing targets of height ≤ p^η costs p^{1−2η} scans per target, exchanged at best 1:1 for box volume | no exponent change | **KILL** |
| E | (e) Amortised target-independent basis. S3 = x_R²·A(x) + x_R·B(x) + C(x), so all lattices lie in one fixed 3-dimensional coefficient family | **derived**: this changes only T_dec. At reach δ_r the IC exponent is 1 − (m−1)δ_r for any T_dec ≤ p^{o(1)}, so reach binds, not per-call cost (lead-4 §3.1) | polynomial factor only | **KILL** |
| F | (f) S3 chains with an intermediate x_T, and any choice of its bound p^γ | **computed** γ = 1: ν_s/s = 1.42–3.0 ≥ 1 for all δ ≥ 1/12. γ = 1/2: 0.917 < 1 at δ = 1/12 (one literal H1 cell), and ≥ 1.33 at δ ≥ 1/6, 1.5 at δ = 1/4. **derived (allocation)**: restricting x_T ≤ p^γ multiplies the per-target success by p^{γ−1}. With LA (δ < 1/4) the IC exponent stays < 1/2 only if Σ_i(1−γ_i) < (m−1)δ − 1/2 < (m−3)/4. That leaves no slack at m = 3, < 0.1 at m = 4 with δ = 1/5, and < 0.5 at m = 5. **cited**: joint (4,2) ν_s/s ≥ 2.08 in the demand region | the restricted chain passes the necessary condition only where it cannot pay | **KILL** (milestone cell fails; γ < 1 is excluded by accounting). Under the literal H0 rule the γ = 1/2 row is INCONCLUSIVE; see AMENDMENTS 2 |
| G | (new) Large-prime variation / asymmetric bounds (one summand from a larger set L = p^λ) | **derived**: JM volume limit β + λ ≤ 1/3. Partial relations needed: √(B·L); per-target success B·L/p. Relation cost is p/√(BL) = p^{5/6}, the same as plain δ = 1/6 (volume invariance) | none | **KILL** |
| H | (new) Vallée-style near-square-root generation on the λ-parametrisation (non-lattice) | **derived**: λ = λ0 + u with a 2-D lattice gives λ² − x_R ∈ [0, H] only for H ≳ p^{2/3}, whereas e1 ≤ 2B is needed with B ≤ p^{1/3} | reach far short | **KILL** |

No row reaches the gate: a real S3 margin over 1/6, or ν_s < s at δ = 1/4 (S4) or 1/5 (S5).
Step 3 (fpylll at 40 and 56 bits) was therefore **not run**.

## 4. Best measured reach → total IC cost exponent (§2.4, LA included; `jm_and_cost.json`)
- **Model.** cost ≈ p·T_dec/B^{m−1} + m·B², with B = p^{min(δ_r, 1/(m+1))} and a polynomial-time oracle.
- **The best route is m = 2**, because at every arity reach falls faster than the arity helps.

| row | m | reach δ | IC exponent | distance to rho (0.5) |
|---|---|---:|---:|---:|
| S3, rigorous HG, measured (Probe B 40/56 bit) — cited | 2 | 0.10 | 0.90 | +0.40 |
| S3, heuristic ≥2 vanishing, 56 bit level 3 — cited measured | 2 | 0.12 | 0.88 | +0.38 |
| S3, heuristic, best 40-bit cell (0.16 at 5/8, level 3) — cited measured | 2 | 0.16 | 0.84 | +0.34 |
| S3 JM asymptote (derived; control reproduced 1/18, 1/10, 0.119 → 1/6) | 2 | 1/6 | 0.833 | +0.333 |
| S4 at the best feasible-dimension bound (cited, dim 969) | 3 | 1/35.5 | 0.944 | +0.444 |
| S4 JM / any-lattice bound | 3 | 1/24 / 1/21.8 | 0.917 / 0.908 | +0.42 / +0.41 |
| S5 JM / any-lattice bound | 4 | 1/80 / 1/70.8 | 0.963 / 0.958 | +0.46 |
| milestone S4 δ = 1/4 / target S5 δ = 1/5 | 3 / 4 | — | 0.5 / 0.4 | 0 / −0.1 |

**Comparison with other measured routes.** Every lattice row is worse than the measured MITM exponents in
the closure note A6 (0.69 / 0.80; cited). It is also worse than rho, which costs exponent 0.5.

## 5. What is NOT shown
- **Computed, not proved, for s ≥ 2.** The ν_s results are truncated upper bounds, for s ≤ 3 (A5: s ≤ 2)
  and one random curve with q ≈ 10⁶. Rigorous only at s = 1, by the large-unknown argument.
- **Necessary condition only.** The criterion is necessary, not sufficient. Passing it (the F, γ = 1/2,
  δ = 1/12 cell) does not mean a lattice exists.
- **Lattices only.** Nothing here bounds non-lattice algorithms: Gröbner/F4 on these presentations,
  probabilistic success beyond HG (L4-D), or factor bases that are not coordinate boxes.
- **Still open from earlier notes.** No proof was given for the e-form with unequal bounds.
- **Not done.** No LLL run, no pcc claim, no bench record.

## 6. Single next action with more compute
Run the lead-4 L4-D measurement: the success-probability decay q(δ, p) past the HG bound for S4, in the
Sym lattice at 40/48/56 bits. It is the only lattice lane not closed by a determinant or ideal argument,
and the §2.4 allowance (T_dec ≤ p^{1/4} at m = 4) could in principle be spent there.

## 7. Files (this directory)
- `PROTOCOL.md`, `PROTOCOL.sha256`, `AMENDMENTS.md`
- `nu.py`: instrument
- `run_grid.sh`, `grid.jsonl` (405 cells), `grid_summary.md`, `summarize.py`
- `jm_and_cost.py`, `jm_and_cost.json`
- `MANIFEST.sha256`
