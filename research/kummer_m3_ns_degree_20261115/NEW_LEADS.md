# New leads: lower bounds on `d_F` for Kummer PKM chains at `m = 3` (2026-11-15)

**Class.** Proposals only, from the Idea Generator. Nothing here was run, proved or measured.
Every number quoted from disk keeps the standing its source gives it, which is `[pcc:unverified]`
throughout. Every claim below is a **proposal**. No hypothesis status, evidence or result file was
changed. The Coordinator decides what runs, and the experiment-director freezes the route
(AGENTS.md §9a, "Model tiers").

**Problem.** Prime-field index calculus on PKM Kummer towers `V = g·μ_{2^t}`, with Semaev `S₃`
chains of `m` summands. The question is whether there is a lower bound on the last fall degree
`d_F` that grows in `t` or `m`, or a convincing reason why this presentation has none.

## Sources checked

On disk (read in this session):

- `research/prime_fall_degree_20261027/DERIVED.md`: D1–D8, "What is not derived", Addendum. D9 is
  **not on disk yet**; it is cited from the task statement (`d_F ≤ 3t+5` at `m = 3` by a
  grid-indicator certificate `1 = Σ_x δ_x(H1^x f1 + H2^x f2)` with `deg_u H ≤ 1`).
- `research/prime_ic_leads_20261110/growth_law/README.md`: Theorems 1–4, Conjecture G, §5
  falsifying cell `t = 12`.
- `research/prime_ic_leads_20261110/growth_law/lower_bound/README.md`: Prop. 1 (twist =
  rescaling), Prop. 2 (`λ_f`), Theorem 3 (two-layer support criterion), Lemmas 4–5, Conjectures Λ
  and C. Its §6 computations were Python and are not admitted.
- `research/prime_ic_leads_20261110/growth_law/lean/GrowthLaw.lean`: `closure_image` and
  `closure_one`, with `hmul` = "variable ↦ variable or identity".
- `research/prime_ic_leads_20261110/devcheck/{README.md,table.md}`: `m = 3` rows at `t = 2, 3`
  only.
- `research/pkm_falldegree_unblock_20261107/RESULTS.md` §2 and `RESULTS_STAGE_C.md`, the `m`
  ladders.
- `research/lfd_lower_bound_20261111/NOTE.md`: §1 and §6, i.e. Prop. S, Prop. S*, Lemma F, and the
  counterexample to Conjecture S.
- `research/notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md`, §3.5 and §4.
- `research/non_semaev_decomposition_20261113/README.md`, §6.
- Code: `examples/pkm_tower_pilot.rs` (chain at lines 1237–1266: `f1 = S₃(x1,x2,u)`,
  `f2 = S₃(u,x3,x_R)`; `random_target` at line 1195), `src/cryptanalysis/hky_fp_tower.rs`
  (`closure`, line 478, returns dims only and keeps the echelon rows private) and
  `src/cryptanalysis/f4_fp_tower.rs` (`SquareRule {next,a,b,c,d}`, line 470).
- `problems/README.md`: there is no record for "growing lower bound on `d_F`, PKM Kummer". The
  Coordinator may want one (AGENTS.md "Name the obstruction"). This file does not create it.

Measured values these leads build on (all `[pcc:unverified]`):

- `m = 2`: `d_F` is 4 at `t = 2..5`, 5 at `t = 6..9` and 6 at `t = 10`. Source: `RESULTS.md`
  §2, medians.
- `m = 3`: `d_F` is 5/5 at `t = 2` and 6/6 at `t = 3`. Sources: `devcheck/table.md`,
  `RESULTS_STAGE_C.md` P1 Kummer tables. **No `m = 3` row exists at `t ≥ 4`**, even though G2's
  registered set includes `m = 3, t ≤ 4`.
- `m = 4`: `d_F` is 6 at `t = 2` and 7 at `t = 3`.
- The `m = 2, t = 12` closure at `c = 5` had 55,455 columns and 6.8·10⁷ entries and took about
  500 s. The `c = 6` closure was killed at the 15 GB limit.

Not available here:

- **research-kb.** The MCP server is not connected to this subagent, so there are no
  `source_id` receipts.
- **Lead atlas.** Not consulted. Earlier notes (`non_semaev_decomposition_20261113/README.md`
  §1) record it as vault-encrypted.

Literature (web, this session; closest results named in each record):

- Razborov's pseudo-reduction (R-operator) and the Alekhnovich–Razborov local construction, as
  described in [arXiv:2511.17272](https://arxiv.org/pdf/2511.17272) and
  [arXiv:2503.17022](https://arxiv.org/pdf/2503.17022).
- Dual Nullstellensatz certificates, e.g. [arXiv:2212.05365](https://arxiv.org/pdf/2212.05365).
- The Bettale–Faugère–Perret hybrid approach,
  [HAL hal-01148127](https://hal.archives-ouvertes.fr/hal-01148127).
- PKM 2016 ([PDF](https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf)).
- Sparse and lacunary polynomials on curves (Zannier school), e.g.
  [Corvaja–Levin–Zannier, arXiv:1307.5788](https://ar5iv.arxiv.org/html/1307.5788).
- Gorenstein socle and Macaulay inverse systems,
  [arXiv:1005.1677](https://arxiv.org/pdf/1005.1677).

Cited from memory and not re-read (†): Grigoriev 1998 (Tseitin, Nullstellensatz, characters);
Kummer's carry theorem.

## Notation (`m = 3`)

- `A = F_p[z1,z2,z3]/(z_i^K − 1)`, with `K = 2^t | p − 1`. Then `A ≅ F_p^{μ_K³}` (D1).
- `R = A[u]`. The degree is popcount plus `u`-degree, as in the `hky_fp_tower` key.
- `f1 = S₃(g z1, g z2, u)`, of degree 4 because of the `x1 x2 u²` term (`g² y10 y20 u²`).
- `f2 = S₃(u, g z3, x_R)`, of degree 3. Its `u²`-coefficient is `(g z3 − x_R)²`, which is a unit
  of `A` whenever `x_R ∉ V`. These degrees are hand-computed and must be checked against
  `HkyRing::degree`.
- `T = (K−1, K−1, K−1)`.
- `d_NS` is the least `D` with `1 = g1 f1 + g2 f2`, `deg g_i + deg f_i ≤ D` in `R`. This is the
  "no-fall" degree of D4.

## Ranking (information per CPU-hour)

| rank | id | title | CPU-h (4 vCPU, 15 GB) | what one run decides |
|--:|:--|:--|:--|:--|
| 1 | `KM3-LQR-NS` | Link-quotient residue functional: proved `d_NS(3,t) ≥ 2t+5`, sharpened by a `z3`-weight | < 0.3 | whether the NS-in-R degree at `m = 3` provably grows in `t`; an exact cross-check of the parallel lab |
| 2 | `KM3-SPEC-LADDER` | Partial specialisation `u ↦ ρ`: monotone in `m`, and the first `m = 3` cells at `t = 4, 5, 6` | 2–4 | whether `m = 3` growth in `t` is inherited from `m = 2` (`Δ` constant) or is its own |
| 3 | `KM3-LQR-CLOSURE` | Quotient by the last link loses nothing at the refutation threshold (`d_F^{(f2)} = d_F`); `λ₃` as the closure pseudo-solution | 2–4 (+ engine API) | whether the `m = 2` lower-bound toolkit (Λ, Theorem 3) ports to chains |
| 4 | `KM-ART-DEGEN` | Artinian degeneration of the top relation `y_{t−1}² = 0`: does `d_F` change? | 2–3 | whether growth lives in carry transport (shared) or in the wrap; if shared, a one-parameter proof target |
| 5 | `KM-POPCODE-STEPS` | Primitive low-popcount multiples on the curve predict the step positions (period 4) | 4–10 | the step positions beyond closure reach (`W = 2`, `t = 14`) from a `t`-independent computation |

The leads share no claim. `KM3-LQR-NS` and `KM3-LQR-CLOSURE` use the same functional but make
independent claims, about NS and about the closure, so each one can die without the other.

---

```yaml
id: KM3-LQR-NS
title: >
  Link-quotient residue (LQR) functional gives a growing lower bound on the Nullstellensatz-in-R
  degree of the Kummer m = 3 chain
obstruction_attacked: >
  DERIVED.md "What is not derived": no growing lower bound, for any proof system weaker than or equal
  to the closure. The binary analogue failed: lfd NOTE.md §6 "Where the attempt broke" (no structural
  dual p for Prop. S*; Lemma F kills flat-supported duals). At m = 2 D4 settles NS by uniqueness of
  f^{-1}; at m = 3 multipliers are not unique, so D4's argument does not apply.
claim: |
  (a) [to be proved] For an inconsistent Kummer m = 3 draw with x_R ∉ V, let B_x = F_p[u]/(f2(x3,u))
      (2-dimensional, basis {1,u}) and ⟨u⟩(c0 + c1 u) = c1. For ω: μ_K → F_p define
        λ_ω(g) = Σ_{x∈μ_K³} x1 x2 ω(x3) · ⟨u⟩( g(x,u) · f1(x,u)^{-1} in B_x ).
      Then λ_ω(g2 f2) = 0 for every g2 ∈ R, and for every normal-form monomial z^k u^j
        λ_ω(z^k u^j f1) = K² · [k1 = k2 = K−1] · Ω_j(k3),  Ω_j(k3) = Σ_{x3} x3^{k3} ω(x3) b_j(x3),
      where u^j ≡ a_j(x3) + b_j(x3) u mod f2(x3,u) (b_0 = 0, b_1 = 1).
  (b) [to be proved, short] If λ_ω(1) ≠ 0 and Ω_j(k3) = 0 for all j ≥ 1 with pop(k3) + j ≤ e, then
      d_NS ≥ 2t + e + 5. With ω(x3) = x3 (call it λ₃) the hypothesis is vacuous at e = 0, so
      λ₃(1) ≠ 0 alone gives d_NS(3,t) ≥ 2t + 5. Together with D9 (task statement):
      2t + 5 + e* ≤ d_NS(3,t) ≤ 3t + 5, where e* is the largest e for which such an ω exists.
  (c) [to be measured] λ₃(1) ≠ 0 on every draw, and e*(t) as predicted below.
mechanism: |
  Monomials z^k are characters of μ_K³, and Σ_{x∈μ_K} x^{k+1} = K·[k = K−1] for 0 ≤ k < K. The weight
  x1 x2 makes λ_ω blind to every multiple of f1 except those with all-ones exponents (popcount 2t)
  in z1 and z2. Quotienting by the last link f2 kills its whole ideal. Reparametrising
  (R/(f2))* by the unit f1^{-1} turns the f1-condition into a pure character orthogonality,
  the Tate/Euler trace form of a rank-2 extension. At m = 2 the same construction is λ_f of
  lower_bound/README.md Prop. 2, and it gives d_NS ≥ 2t + 2, which equals D4's exact value. That
  is the control. The z3-weight ω is a one-variable Prop. S*-type coding problem: ω must be
  orthogonal to the "twisted popcount code" {x3^{k3} b_j(x3) : pop(k3) + j ≤ e} while it pairs
  non-trivially with Φ(x3) = Σ_{x1,x2} x1 x2 ⟨u⟩(f1^{-1}).
prediction: |
  - λ₃(1) ≠ 0 on 100% of draws (P1 and one second prime with 2^t | p − 1; t = 2..8; 4 random
    targets per t).
  - e*(t) follows the generic counting rule. There are N(e) = Σ_{i<e} C(t, ≤ i) conditions
    against K unknowns. This gives e*(t) = ⌈t/2⌉ for t = 2..8, i.e. 1, 2, 2, 3, 3, 4, 4.
  - Declared interval: e*(t) ∈ [⌈t/2⌉ − 1, t]. The upper end is forced by D9.
  - Predicted bound: d_NS(3,t) ≥ 2t + 5 + ⌈t/2⌉, against the upper bound 3t + 5.
discriminating_test: |
  Native Rust, a new example (suggested examples/kummer_lqr_ns.rs) reusing the pilot's tower, curve
  and target construction (examples/pkm_tower_pilot.rs: `summation`, `random_target`; seed 0x504B4D54,
  P1 = 2013265921) and fp_backend.
  1. For each draw compute f1(x,·) and f2(x3,·) as u-polynomials at all K³ grid points.
     Invert f1 in B_x via the norm: f1^{-1} = conj(f1)/N(f1).
     Compute Φ(x3), and λ₃(1) = Σ_{x3} x3 Φ(x3).
  2. Compute b_j(x3) for j ≤ t + 1 by repeated reduction mod f2(x3,·).
  3. For e = 0..t, build the N(e) × K condition matrix with rows (j, k3) and columns x3, and decide
     whether Φ lies outside its row space by a rank test mod p. Candidate routine:
     src/cryptanalysis/diem_quartic.rs:2352 `dense_rank`; check that it fits before use.
     e* is the largest e with Φ outside the row space.
  4. m = 2 control: the same code with no third variable must return d_NS ≥ 2t + 2 (D4).
  The cost is K³ ≤ 1.7·10⁷ small-algebra evaluations at t = 8, plus ranks of matrices of at most
  a few hundred rows by K ≤ 256 columns.
falsification: |
  - λ₃(1) = 0 on any generic draw. Then claim (c) dies for this support and the bound needs
    another weight. Claims (a) and (b) are unaffected.
  - The parallel lab's exact minimum NS-in-R degree is below 2t + 5 + e* on a draw where this run
    certifies λ_ω(1) ≠ 0 with the vanishing conditions. That is a contradiction. Stop, and
    reconcile the degree conventions and the engines before anything else.
  - e*(t) ≤ ⌈t/2⌉ − 2 at some t ≤ 8. That falsifies the counting prediction only; the theorem
    stands.
cost_band:
  candidate_arm: "< 0.3 CPU-h for t = 2..8 × 2 primes × 4 draws; implementation about 1 engineer-day of Rust"
  reference_arm_m2_D4_control: "< 0.05 CPU-h (same code, m = 2; must reproduce 2t + 2)"
  reference_arm_D9_upper: "0 CPU-h (closed form 3t + 5; D9 text, not on disk yet)"
  cross_check_arm: "0 here; the parallel lab's NS-in-R measurement is consumed, not re-run"
controls:
  - "m = 2 control reproduces D4 (d_NS = 2t + 2) through the λ_f special case"
  - "the planted targets (consistent) must make λ undefined: f1 is not a unit in some B_x; the code must detect this, not produce a number"
  - "x_R ∈ V draws excluded and logged (lc of f2 vanishes)"
  - "independent re-check of λ₃(1) by the coefficient form K³·[z^T]⟨u⟩(f1^{-1} mod f2), computed with a second code path (inverse DFT of the grid values)"
risks:
  - "This bounds NS-in-R only. Measured d_F (5, 6 at t = 2, 3) is far below 2t + 5. It says nothing about d_F; KM3-LQR-CLOSURE is the d_F lead."
  - "The counting prediction for e* assumes the b_j-twisted conditions are independent. They are rational functions of x3, so dependences can occur (which helps) or Φ can fall in the span (which hurts)."
  - "Generic nonvanishing of λ₃(1) needs its own proof, like Prop. 2(c) at m = 2 (a specialisation point where f1^{-1} mod f2 is computable)."
lean_obligation: |
  - Abstract, core Lean, can live beside GrowthLaw.lean: `ns_no_cert_of_functional`. A linear
    functional that vanishes on span{m·f_i : deg m + deg f_i ≤ D} and not on 1 excludes
    degree-D NS certificates.
  - Concrete, needs Mathlib:
    - `lqr_kills_f2`;
    - `lqr_monomial_f1`: the character sum, via `IsPrimitiveRoot.geom_sum_eq_zero`;
    - `lqr_bound`: claim (b).
  - The generic nonvanishing of λ₃(1) is a separate obligation.
closest_baseline: |
  - On disk: D4 (m = 2, NS degree by uniqueness), Prop. 2 λ_f (m = 2, coefficient and residue form),
    and binary Prop. S* (selector dual).
  - Literature: dual NS certificates (designs; e.g. arXiv:2212.05365), and Grigoriev's
    character-based NS lower bounds for Tseitin† over group algebras.
  - What is new here: an explicit dual for a two-generator chain whose multipliers are not unique,
    obtained by quotienting the last link and pairing through the trace form of R/(f2). In the
    binary note the dual p had to be found by search and was never found structurally.
```

---

```yaml
id: KM3-SPEC-LADDER
title: >
  Partial specialisation (z3 ↦ x3, u ↦ ρ) makes d_F monotone in m along chains; first m = 3 cells
  at t = 4, 5, 6 and the gap Δ(t) to the specialised m = 2 systems
obstruction_attacked: >
  DERIVED.md D6: full specialisation to grid points levels off at 5 and "gives no growing lower
  bound". Growth in m is only "measured" (RESULTS_STAGE_C H-m "grows"). The m = 3 ladder stops at
  t = 3.
claim: |
  Lemma P (to be proved; proof sketch below). Kummer chain of arity m ≥ 3. Take x_m ∈ μ_K and a
  root ρ ∈ F_p of S₃(u, g x_m, x_R) in u. Let F_{m−1}(ρ) be the (m−1)-chain with target ρ. Then
    d_F(F_m(x_R)) ≥ d_F(F_{m−1}(ρ)).
  Hence d_F(m, t; x_R) ≥ max over admissible (x_m, ρ) of d_F(m−1, t; ρ). Iterating, any lower bound
  for m = 2 that holds for every inconsistent target lifts to every m.
  Sketch: φ: R_m → R_{m−1} sends the last tower's variables to the constants x_m^{2^j} and
  u_{m−2} ↦ ρ, and is the identity elsewhere. It is a ring map that does not raise degree. It sends
  f_{m−1} ↦ 0 and the other links to F_{m−1}(ρ). Closure maps into closure (closure_image, with
  variables allowed to go to scalars).
  Measurement claim (H_inherit): m = 3 steps where m = 2 steps.
mechanism: >
  The quotient keeps two full Kummer towers and only collapses the third summand. D6 collapsed all
  three and lost the curve, as D8 explains. So the specialised system is a genuine m = 2 Semaev
  system, whose d_F already grows (Conjecture G, with the t = 12 step observed). Whether m = 3 has
  growth of its own is measured by Δ(t) = d_F(3,t) − max_ρ d_F(2,t;ρ). The m = 4 P1 ladder
  (6, 7, 7, ≥8 at t = 2..5; PRIME_FIELD_IC_CLOSURE §4.3) is flat from t = 3 to t = 4, which
  suggests inheritance.
prediction: |
  - Lemma P holds on 100% of (m = 3 draw, ρ) pairs.
  - H_inherit: d_F(3,4) = 6 and d_F(3,5) = 6 on both draws, so Δ(3) = Δ(4) = Δ(5) = 2 (m = 2
    is 4 at t ≤ 5).
  - d_F(3,6) = 7, if the cell completes, coinciding with the m = 2 step 4 → 5 at t = 6.
  - Declared interval: d_F(3,4) ∈ [6,7], d_F(3,5) ∈ [6,8].
discriminating_test: |
  - m = 3 tower arm, Kummer P1, seed 0x504B4D54, --random 2, --engine hky, at:
    - t = 4 and t = 5 with --hky-cmax 8;
    - t = 6 with --hky-cmax 7 --hky-max-entries 2.5e8, so it is censored rather than killed.
  - Column counts from the hky monomial count: about 12.4k (t = 4, c = 8), 56.8k (t = 5, c = 8)
    and 112k (t = 6, c = 7).
  - For every m = 3 draw, enumerate x3 ∈ μ_K and the F_p-roots ρ of S₃(u, g x3, x_R); there are at
    most 2K. Run the m = 2 system F_2(ρ) to its d_F (cmax 7) for each.
  - Record muladds for both arms.
  - Engine: examples/pkm_tower_pilot.rs. Gap: there is no option to set x_R. Add `--target-xr <v>`
    (Rust), plus a small Rust driver or pilot mode that lists the admissible ρ for a draw.
    `--max-t-m3` exists.
  - Secondary readout, at component scope only: the guess-one-summand hybrid
    Σ_ρ muladds(F_2(ρ)) against muladds(F_3). It is reported as a stage ratio with its interval
    named. It is not an S value and makes no leaderboard claim. Any cross-method use needs the
    §5a band and the validation gate.
falsification: |
  - H_inherit dies if d_F(3,4) ≠ 6 or d_F(3,5) ≠ 6 on either draw. A value ≥ 7 means the second
    link generates growth of its own, and the lower-bound problem at m = 3 is then not reducible
    to m = 2.
  - Lemma P dies, or an engine defect is shown, if any pair has d_F(3,t) < d_F(2,t;ρ). Stop and
    debug before reading any degree.
cost_band:
  candidate_arm_m3: "t = 4: < 0.1 CPU-h; t = 5: about 0.5–1 CPU-h for 2 draws; t = 6: ≤ 2 CPU-h, censor-capped"
  specialised_m2_arm: "≤ 2K systems per m = 3 draw, each ≤ 1 s at t ≤ 6 (devcheck m = 2 rows: 0 s wall at t ≤ 6): < 0.2 CPU-h"
  reference_arm: "re-run of devcheck/m2_t2-10.jsonl and m3_t2-3.jsonl cells for binary identity: < 0.1 CPU-h"
  implementation: "about 0.5 engineer-day (option plus root enumeration)"
controls:
  - "planted m = 3 draws: d_F = ∞ (1 never enters), and each specialised F_2(ρ) with ρ = x(R − P3) of the planted decomposition must be consistent"
  - "the null arm (--controls null --null-chain) on the same cells, to see whether Δ is a support property (Theorem 3 of growth_law)"
  - "a second prime (a P0 cell as in RESULTS_STAGE_C) for t = 4"
risks:
  - "ρ ∉ F_p for some x3. Those pairs are skipped and counted; the max then runs over a subset, which is still a valid lower bound."
  - "Seeds draw a fresh g per t (growth_law Theorem 1 Scope). The ladder compares generic values, not one fixed system."
  - "The t = 6 cell may censor, and only t ≤ 5 decides H_inherit."
lean_obligation: |
  - `closure_image_scalar` and `closure_one_scalar` in
    research/prime_ic_leads_20261110/growth_law/lean/GrowthLaw.lean. They add a third disjunct
    `∃ k, ∀ a, φ (D.mulv x a) = E.smul k (φ a)` to `hmul`, and let generators map to zero.
    Core Lean, no Mathlib.
  - Instantiation on the concrete group algebras is the same missing bridge as Theorem 1.
closest_baseline: |
  - On disk: DERIVED.md D6 (full specialisation, levels off at 5); lfd NOTE.md Lemma 4 /
    Corollary 4b (binary chain, sketch only, never checked); RESULTS_STAGE_C ladders in m at
    t = 2, 3.
  - Literature: the hybrid approach of Bettale–Faugère–Perret, which guesses variables and solves
    subsystems. Here the guess is one summand plus the matching partial sum, and the claim is a
    degree inequality rather than a cost model.
```

---

```yaml
id: KM3-LQR-CLOSURE
title: >
  Quotient by the last link loses nothing at the refutation threshold, d_F^{(f2)} = d_F, and λ₃ is
  the closure pseudo-solution at m = 3 (Conjecture Λ ported to chains)
obstruction_attacked: >
  lower_bound/README.md §8.1–8.2: the m = 2 pseudo-solution λ_f and Theorem 3 need a unique
  multiplier h = g f^{-1}. At m = 3, V_c ⊂ (f1, f2) has no unique multipliers, so neither tool
  applies, and nothing is known about pseudo-solutions for chains.
claim: |
  Define d_F^{(f2)} = least c with 1 ∈ V_c(F) + (f2). This is decided in the finite space
  R/(f2) ≅ A ⊕ A·u of dimension 2K³. Trivially d_F ≥ d_F^{(f2)}.
  (i) d_F^{(f2)} = d_F on every computed m = 3 cell.
  (ii) Whenever 1 ∉ V_c + (f2), the single functional λ₃ (KM3-LQR-NS with ω(x3) = x3) vanishes on
       V_c and λ₃(1) ≠ 0.
  If (i) holds, the m = 3 lower-bound problem is a one-generator problem for f1 in O(Y) = R/(f2),
  where the multiplier h = g·f1^{-1} is unique. The m = 2 Theorem 3 support criterion then applies
  word for word in O(Y).
mechanism: >
  (R/(f2))* is exactly the family {g ↦ Σ_x w(x)⟨u⟩(g f1^{-1}) + w'(x)⟨1⟩(g f1^{-1})}, so (i) asks
  whether some functional of this family is a pseudo-solution. At m = 2 the Python toy evidence (22
  cells, not admitted) had λ_f exact. λ₃ is its chain analogue. It is blind to f2 entirely and
  sees f1-multiples only through the T·u coefficient of the unique O(Y)-multiplier. So
  Conjecture C's support counting would transfer, with 2t bits in z1, z2 plus the z3-twisted
  part.
prediction: |
  - (i) equality on every completed m = 3 cell at t = 2, 3, 4 (2 random draws each, P1). At
    t = 2, 3 this is d_F^{(f2)} = 5 and 6.
  - (ii) λ₃ vanishes on V_c for every c < d_F on those cells.
  - Native m = 2 control: λ_f kills V_c for every c < d_F, and fails at c = d_F, on the devcheck
    seeds: W = 1 at t = 2..9 and the tower at t = 2..9.
discriminating_test: |
  - Engine change, Rust, in src/cryptanalysis/hky_fp_tower.rs: expose the echelon rows of
    `closure`, e.g. `closure_with_rows` or a per-row callback. Today the Echelon is private and only
    dims are returned.
  - New example: map each row g to its grid image in R/(f2), i.e. for every x the class of g(x,u)
    mod f2(x3,u), a vector in F_p^{2K³} (8,192 entries at t = 4).
    - Test (i) incrementally by rank: is π(1) in the span of the π(rows)?
    - Test (ii) by evaluating λ₃ on every row. λ₃ is precomputed as a vector over the monomial
      columns: one 3-D DFT of size K³ per u-power j ≤ c.
  - Cells:
    - m = 3: t = 2, 3 (seconds), t = 4 (minutes), c = 2..d_F.
    - m = 2 control: t = 2..9 (≤ 65 s per closure; devcheck wall times).
  - Reuse: examples/pkm_tower_pilot.rs builders and seed; hky_fp_tower::closure.
falsification: |
  - (i) dies on any cell with d_F^{(f2)} < d_F. The refutation then genuinely uses the bounded
    degree of f2-multiples, the link-quotient family cannot certify d_F there, and a chain
    pseudo-solution must not factor through R/(f2).
  - (ii) dies on any cell with 1 ∉ V_c + (f2) but λ₃(V_c) ≠ 0. The family may still contain a
    witness, which is reported as the rank of the annihilator.
  - The m = 2 control dies (λ_f is not a witness on a native cell with 1 ∉ V_c). That retires the
    non-admitted Python evidence for Conjecture Λ and the claim (ii) analogue at m = 3.
cost_band:
  candidate_arm_m3: "t ≤ 3: < 0.05 CPU-h; t = 4: closure ≤ 0.5 CPU-h per draw plus projection ≤ 0.5 CPU-h"
  control_arm_m2: "t = 2..9, W = 1 and tower, 2 draws: about 0.5 CPU-h"
  reference_arm: "the plain closure (existing dims, d_F) on the same cells, recomputed for identity: included above"
  implementation: "about 1–2 engineer-days (closure row export, projection, DFT)"
controls:
  - "replay: d_F from the modified engine must equal the unmodified engine's d_F on every cell"
  - "swap the base link: quotient by f1 instead (needs x1 ≠ x2 off-diagonal handling; report separately), to test that the choice of f2 is not what makes (i) hold"
  - "null-support chain (--null-chain) on the same cells"
risks:
  - "t = 4 is the largest affordable m = 3 cell here, so at most three t values per draw. That is thin for an asymptotic claim, but it can falsify."
  - "Memory: the projected vectors are 2K³ long. At t = 4 that is fine; t = 5 (65,536 entries times about 10⁵ rows) needs a streaming rank."
lean_obligation: |
  - If (i) is used in a proof: the "multiplier uniqueness in O(Y)" lemma (f1 is a unit of
    R/(f2) for inconsistent draws), and Theorem 3 restated over O(Y) with the induced filtration.
    Both need Mathlib.
  - `d_F ≥ d_F^{(f2)}` is a one-line consequence of V_c ⊆ V_c + (f2).
closest_baseline: |
  - On disk: lower_bound/README.md Prop. 2, Theorem 3 and Conjecture Λ (m = 2, Python evidence
    not admitted).
  - Literature: Alekhnovich–Razborov local pseudo-reduction, i.e. reduction modulo the ideal of a
    satisfiable subsystem S(m) (arXiv:2511.17272 summary). Here S = {f2} is fixed and global, and
    the subsystem is not satisfiable over the grid in the AR sense: f2 alone has solutions u for
    every x3, so the reduction is well-defined.
  - What is new: the explicit trace-form witness, and the dichotomy (i), which can be computed in
    a 2K³-dimensional space.
```

---

```yaml
id: KM-ART-DEGEN
title: >
  Artinian (Gorenstein) degeneration of the top relation, y_{t−1}² = 0: is d_F unchanged, and is
  the socle functional the pseudo-solution?
obstruction_attacked: >
  growth_law README §6 "A lower bound that grows with t" needs a functional correcting the top
  window. Prop. 1 (lower_bound) showed every twist y_{t−1}² = ε ≠ 0 is a rescaling, so the twisted
  route is empty. The ε = 0 fibre is not covered by Prop. 1.
claim: |
  Let R⁰_t = F_p[z1,z2]/(z1^K, z2^K): the same monomial basis, popcount degree and tower rules,
  except that the top rule is y_{t−1}² = 0. For a generator f with f(0) ≠ 0 (a unit of the local
  ring R⁰), let d_F⁰ be its HKY last fall degree.
  (C1) d_F⁰(f,t) = d_F(f,t) on every completed m = 2 cell (inconsistent draws), for the tower
       (S₃) support and the W = 1 box, t = 2..11.
  (C2) If C1 holds: the socle functional σ_f(g) = [T](g·f^{-1} in R⁰), T = z1^{K−1}z2^{K−1}, kills
       V⁰_c whenever 1 ∉ V⁰_c.
  Consequence if C1 holds: R⁰ is a monomial algebra, so every torus scaling z_i ↦ s_i z_i is a
  degree-preserving automorphism. In R only μ_K² and the twisted isomorphisms act. So d_F⁰ on the
  W = 1 box depends on the single invariant κ = c00 c11/(c10 c01), and the proof target becomes a
  one-parameter family.
mechanism: |
  Theorem 2 (top window): the two rings differ only in products y_j · z^e that carry out of bit
  t−1. In R the product wraps to a low-degree monomial; in R⁰ it vanishes. In the infinite
  tower P there is no refutation at all (f vanishes on the curve; Lemma 4). So in both rings
  refutation needs information to be transported to the top window by carry chains, whose cost
  per degree is the period-4 phenomenon. If transport is the bottleneck, the two values agree. If
  the wrap's re-injection at low degree is what refutes, d_F⁰ differs. Note that R⁰ is blind to
  decomposability: planted draws also refute in R⁰. So C1 holding would mean that d_F at random
  targets is a carry-transport property, not a property of the grid.
prediction: |
  - C1: equality on ≥ 90% of completed cells and |d_F⁰ − d_F| ≤ 1 on all of them.
  - Point prediction: equality everywhere for t ≥ 4.
  - C2: σ_f is a witness on every cell with 1 ∉ V⁰_c.
  - Declared alternative, which counts as a falsification of C1: d_F⁰ is constant in t (no
    growth) while d_F grows.
discriminating_test: |
  - Engine change: the pilot builds Kummer rules and checks the tower before measuring. Add
    `--top-const ε` (Rust), which sets the last SquareRule's d to ε, with ε ∈ {1 (reference),
    5 (generic twist), 0 (arm)}.
  - Targets are drawn and classified (inconsistent or planted) with ε = 1 and then reused, so
    grid-based checks are skipped for ε ≠ 1.
  - Cells:
    - m = 2, `--engine hky`, Kummer P1, seed 0x504B4D54;
    - tower arm and `--controls null --null-width 1`;
    - t = 2..9 (cmax 7) and t = 10, 11 (cmax 6, budget 1200 s);
    - 2 random and 1 planted draw per t.
  - C2 needs the closure-row export from KM3-LQR-CLOSURE. σ_f is precomputed per monomial column
    by truncated power-series inversion of f in R⁰.
falsification: |
  - C1 dies if |d_F⁰ − d_F| ≥ 2 on any cell, or if they differ on > 10% of cells, or if d_F⁰ stays
    flat over t = 4..11 while d_F steps at t = 6 and t = 10 (tower) or at t = 4 and t = 8 (W = 1).
  - The ε = 5 control: d_F at ε = 5 must equal the generic value. Prop. 1 says it is the d_F of
    a rescaled generator. A mismatch at ε = 5 is an engine defect, not a result.
cost_band:
  candidate_arm_eps0: "t ≤ 9: ≤ 0.3 CPU-h; t = 10, 11: ≤ 2 CPU-h (two arms × 2 draws, censor-capped)"
  control_arm_eps5: "≤ 0.3 CPU-h (t ≤ 9 only)"
  reference_arm_eps1: "existing rows (devcheck/m2_t2-10.jsonl, nullw1_m2_*.jsonl, stage A); one identity re-run per arm at t = 6: < 0.05 CPU-h"
  implementation: "about 0.5 engineer-day (option plus check bypass); C2 shares KM3-LQR-CLOSURE's row export"
controls:
  - "ε = 1 must reproduce the committed rows bit for bit (same seed and binary)"
  - "ε = 5: native check of Prop. 1 (twist = rescaling)"
  - "planted draws: d_F = ∞ at ε = 1 and finite at ε = 0, a sanity check of the degeneration"
risks:
  - "C1 is not implied by any semicontinuity argument. The closure is an iterated fixpoint, and no direction is proved, so both outcomes are informative and neither is a theorem."
  - "If C1 fails mildly (differences of ±1 at small t only), the reading is ambiguous. Pre-registered: only t ≥ 4 counts for the point prediction."
lean_obligation: |
  - `closure_image_scaled`: hmul disjunct "variable ↦ scalar·variable", core Lean. It gives
    d_F⁰(f(s·z)) = d_F⁰(f), the torus reduction.
  - The κ-reduction of the W = 1 box needs only this plus normalising three coefficients.
  - C2 as a theorem would need the Gorenstein socle-duality statement for R⁰ (Mathlib).
closest_baseline: |
  - On disk: Prop. 1 of lower_bound/README.md (ε ≠ 0 is a rescaling, so the twisted route is
    vacuous); Theorem 2 of growth_law (top window); Prop. 2 λ_f, whose coefficient form
    K²[T](g f^{-1}) has exactly the shape of a socle functional.
  - Literature: Macaulay inverse systems and socle duality for Artinian Gorenstein algebras
    (arXiv:1005.1677). No published use as a polynomial-calculus pseudo-solution was found in
    this session's search; the gap is stated, not claimed as novelty.
```

---

```yaml
id: KM-POPCODE-STEPS
title: >
  Primitive low-popcount multiples of f on the curve (a popcount evaluation code) predict the step
  positions of d_F at m = 2, period 4 included, without running the closure at large t
obstruction_attacked: >
  growth_law README §6: "The period 4 ... is not explained; nor is the offset s_W". Conjecture G is
  fitted only. The closure cannot reach the decisive cells on this host: the m = 2, t = 12,
  c = 6 closure was killed by OOM, and t = 14 is the Amendment 6 C-step prediction.
claim: |
  m = 2, generic f in the width-W box, infinite tower P = F_p[z1,z2] with popcount degree.
  - Let M_d(s) = {P ∈ fP : every exponent < 2^s, deg P ≤ d}.
  - Let E_d(s) = span( M_d(s−1) ∪ { y·g : g ∈ M_{d−1}(s), y a tower variable of level < s, no
    carry out of level s−1, deg(y g) ≤ d } ).
  - Let ν_d(s) = dim M_d(s) − dim E_d(s), and σ_W(d) = max{ s − 1 : ν_d(s) > 0 }, the top level of
    a primitive degree-d multiple.
  Claim: d_F(W, t) = 1 + min{ d : σ_W(d) > t }. Pre-registered values implied by the on-disk step
  positions:
  - σ_1(2), σ_1(3), σ_1(4) = 4, 8, 12;
  - σ_2(3), σ_2(4) = 6, 10, and σ_2(2) ≤ 1;
  - σ_3(4) = 9, and σ_3(3) undefined (no degree-3 multiple).
  - Period: σ_W(d+1) − σ_W(d) = 4.
mechanism: |
  - By Theorem 2 a refutation needs elements whose exponents reach the top window. By Lemma 5,
    high bits cost one degree each unless an all-ones low part carries.
  - A primitive degree-(c−1) multiple spanning levels 0..t is the minimal object that can wrap
    (z^K = 1) into a constant, so refutation at degree c becomes possible exactly when such
    multiples reach level t.
  - Coding view: M_d(s) is the kernel of the evaluation map from the "popcount code" (monomials
    of popcount ≤ d with s-bit exponents) to functions on the curve f = 0.
  - Counting heuristic: Σ_{j≤d} C(2s, j) unknowns against about W·2^{s+1} conditions (Bezout on
    P¹×P¹). Primitive multiples die out at s ≈ 4.5d. This gives the order of magnitude of the
    period: 4 measured, 4.54 asymptotically.
  - Python context, not admitted: dim M_2(s) = 4, 9, 14, 15, 15, 15 for s = 2..7 at W = 1
    (lower_bound §4). So ν_2 is last positive at s = 5, i.e. σ_1(2) = 4, which matches the step
    at t = 4.
prediction: "exactly the pre-registered σ values above; the period is 4 at every width where two σ values are computed"
discriminating_test: |
  - Native Rust, dense elimination mod p (P1, so exponents < p and no Frobenius effect).
    - W = 1: exact via the rational parametrisation z2 = −B/A (lower_bound §0). The columns are
      monomials, mapped to A^{2^s−1}·P(z1, −B/A) of degree < 2^{s+1}.
    - W = 2, 3: evaluation at W·2^{s+1} + 1 distinct F_p-points of the curve. That is exact by
      Bezout on P¹×P¹ for an irreducible curve; irreducibility is checked first.
  - Then the rank of E_d(s) inside M_d(s) in coefficient coordinates.
  - Sizes:
    - W = 1, d ≤ 4, s ≤ 13: 17,902 unknowns × 16,384 conditions (about 1.2 GB as u32; about
      5·10¹² mulmods);
    - W = 2, d ≤ 4, s ≤ 11: 9,109 × 8,192;
    - W = 3, d ≤ 4, s ≤ 10: 6,196 × 6,144.
  - 2 generic draws per W. Reuse the devcheck generators' coefficients (pilot null-width draws at
    seed 0x504B4D54) so the f matches the ladder rows. Candidate routine:
    src/cryptanalysis/diem_quartic.rs:2352 `dense_rank`; check its fitness. emin.c is GF(2)-only
    and cannot be reused.
falsification: |
  - The claim dies on any computed σ that differs from its pre-registered value, e.g.
    σ_1(3) ≠ 8, σ_1(4) ≠ 12, σ_2(3) ≠ 6, σ_2(4) ≠ 10 or σ_3(4) ≠ 9.
  - The finiteness premise dies if ν_d(s) > 0 persists to the largest s computed, i.e. there is
    no σ.
  - A partial match (the period is right but the offsets are off by a constant) is reported as
    such and does not count as support.
cost_band:
  candidate_arm: "W = 1 d = 4: 1–3 CPU-h; W = 2, 3 at d ≤ 4: ≤ 1 CPU-h together; d ≤ 3 cells: minutes"
  reference_arm: "the on-disk step positions (devcheck width table, growth_law §5, falsify/ t = 12 rows): 0 CPU-h"
  validation_arm: "re-derive dim M_2(s) at W = 1 natively and compare to the non-admitted Python values 4, 9, 14, 15, 15, 15: < 0.01 CPU-h"
  implementation: "about 1–2 engineer-days (dense mod-p elimination if diem_quartic's routine does not fit)"
controls:
  - "planted/special f with a factor (κ = 1 at W = 1, f = (1+z1)(1+z2)): σ must differ, as a negative control for genericity"
  - "two primes (P1 and 786433) for one W = 1 cell"
risks:
  - "The alignment rule d_F = 1 + min{d : σ(d) > t} is fitted to the same on-disk steps that make the prediction. Only the uncomputed σ values (σ_1(3), σ_1(4), σ_2(4), σ_3(4)) are genuine tests. σ_1(2) = 4 is from non-admitted Python."
  - "The 'primitive' definition (E_d) is one choice. A mismatch could reflect the definition rather than the mechanism, so the raw dim M_d(s) table is reported as well."
  - "Even a perfect match is a predictor, not a proof. lower_bound §8.2 shows that divisibility alone cannot prove a bound; reachability is needed."
lean_obligation: |
  - None for the empirical correspondence.
  - If the W = 2, 3 exactness is relied on: `bezout_p1p1_divisible` (a bidegree-(a,b) polynomial
    vanishing at more than W(a+b) points of an irreducible bidegree-(W,W) curve is divisible by
    f). Mathlib.
closest_baseline: |
  - On disk: lower_bound/README.md Lemma 4 and the lowmult.py context (Python, W = 1 only, not
    admitted; it concluded "no proof can simply bound the set of low-degree multiples").
  - What is new: primitive multiples as a step-position predictor across widths, at sizes the
    closure cannot reach.
  - Literature: sparse and lacunary polynomials vanishing on curves and unlikely intersections
    (Zannier school, e.g. arXiv:1307.5788). Their results are char-0 finiteness statements with
    no 2-adic popcount structure. Kummer's carry theorem† (ν_2 of binomials counts carries) is
    the arithmetic behind the popcount falls (D2).
```

---

## What these leads do not claim

- None of them is expected to produce an index-calculus speed-up. The only cost readout is the
  component-scope hybrid ratio in `KM3-SPEC-LADDER`, which is not an `S` value and is not
  leaderboard-eligible without the §5a band and the validation gate.
- `KM3-LQR-NS` would bound the NS-in-R degree, not `d_F`. Measured `d_F` (5, 6) is below its bound
  (`2t + 5` = 9, 11).
- A bounded search failing to find a pseudo-solution would not prove that none exists. Every
  falsification above is about the stated object, not about `d_F` in general.
- Every run proposed here is a development check sized for a 4-vCPU, 15 GB container. The route
  (isolab-eks or local), the resume checkpoint and the `bench.record` are the
  experiment-director's call under AGENTS.md §9a and CLAUDE.md.
