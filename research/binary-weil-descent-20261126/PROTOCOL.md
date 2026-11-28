# Protocol: Gaudry–Diem k = 3 Weil-descent oracle (arm D) against matched rho

Study `bwd-20261126/v1`. Frozen before any evaluation data of arm D. The
machine-readable form is [`protocol.json`](protocol.json); its SHA-256 and
those of the harness sources are in [`SEEDS.sha256`](SEEDS.sha256) and in the
commit that freezes them. Seed policy `isogeny-labs/experiment-seeds/v1`.
Status `[pcc:unverified]`, L0, one shared 4-vCPU / 15 GB cloud container.
Single target only (owner instruction): every solve is one cold solve of one
public `Q`; no multi-target amortisation anywhere.

Builds on crypto PR #5253 (`research/binary-composite-rootfind-20261126/`,
study `bcr-20261126/v1`), whose named next step this is.

## Question

On binary composite fields `F_{2^n}`, `n = 3n′`, with the subfield factor
base (points with `x ∈ K′ = F_{2^{n′}}`):

1. What does one target cost with the Gaudry–Diem k = 3 Weil-descent
   decomposition oracle, and what does a whole cold solve cost, in #5253's
   counted unit (Meq), against matched negation-map rho on the same curves
   and targets?
2. Where, if anywhere, could D's cost curve cross rho's?

## Arm D (the instrument)

`harness/src/descent.rs`, every field operation counted through #5253's `CF`:

1. **Symmetric S₄.** For `y² + xy = x³ + ax² + b`,
   `S₄(x₁,x₂,x₃,x_R)` in `e₁, e₂, e₃` (crypto `koblitz_sym_s4.rs`) has `e₁`, `e₂`
   only squared. Put `E₁ = e₁²`, `E₂ = e₂²`, `t = e₃` (in K′ iff the `e_i` are).
2. **Weil descent.** For `(E₁, E₂, t) ∈ K′³`, `S₄ = 0` over `F_{2^n}` is three
   equations over K′. They are used as the three Frobenius-conjugate equations
   `f_j(E₁,E₂,t; b^{σʲ}, x_R^{σʲ})`, `σ = (·)^{q′}`, which have the same
   K′-solutions.
3. **Elimination.** The system is linear in `(E₁², E₂², E₁, E₂, 1)` with
   coefficients in `F_{2^n}[t]` of degrees `(0,0,2,1,4)`. Constant row
   operations remove the square columns from two rows. `E₁` is then eliminated
   with the linear row, and the resultant of the two remaining quadratics in
   `E₂` gives `U(t)` of degree ≤ 20. Every step only multiplies, so `U(τ) = 0`
   at every solution.
4. **Subfield root finding.** `U′ = U / (t^v (t+τ₀)^w)`, where `τ₀` is the
   second root of `α₃ = t(λ+μt)`, a structural factor of U. The K′-roots are
   those of `gcd(U′, t^{q′} + t)`: n′ − k squarings modulo U′ and one
   pseudo-remainder gcd, then the split over K′ (Artin–Schreier for degree 2,
   K′-trace for degree ≥ 3). `t = 0` (a summand is `T`) and `τ₀` are solved
   directly.
5. **Lift.** At every candidate τ the reduced system is solved exactly for
   `(E₁, E₂) ∈ K′²` by case analysis, with no search. Then `e_i = √E_i`, and
   `X³ + e₁X² + e₂X + e₃` must split over K′ (multiplicities included) with
   every root in the factor base.
6. **Degenerate fallbacks.** These are a rank-deficient constant block
   (`x_R²/b ∈ K′`), `U ≡ 0`, or a vanishing quartic in the R2 ≡ 0 branch. All
   are exact counted searches and are flagged in every record. None is
   expected (pre-freeze self-tests: 0 in 2000 targets).

Everything else is #5253's code, included unchanged by `#[path]`: field,
curves, factor base, the A1 relation-target stream (8 arithmetic-progression
lanes), group-law sign resolution of every relation, the A2 incremental
echelon LA mod ℓ, the final `[d]G = Q`, the seed module. So **D consumes the
same ordered target stream as #5253's F and M2 records and, if its
decomposition sets are equal to F's, stops at the same target with the same
relations and the same d.** Its cost differs from F's only in the oracle
(and in having no pair table).

## Instances, references and rungs

**Panel P (n = 18..33): #5253's 64 cases exactly.**

- Curves: `curve|n|ci`. Targets: `target|n|ci|ti`.
- Shape: 4 curves × 3 targets per rung, 2 × 2 at n = 33.
- References reused unchanged from #5253: F and M2 from `data/runs_a2.jsonl`
  (A1 + A2), RHO (16 seeds per target) from `data/runs.jsonl`.
- The #5253 binary rebuilt here hashes to its recorded A2 binary
  `bf3f431f…4d52d4011b`.

**Extension E (n = 36, 39, 42, 45, 48), new.**

- Same curve and target derivation: 4 curves × 3 targets, 2 × 2 at n = 48.
- Arms:
  - D;
  - RHO: 16 seeds per target, the #5253 binary unchanged;
  - M2 at n = 36 and 39 only, by the #5253 binary (its table is about F²
    keys; at n ≥ 42 it exceeds the host memory budget);
  - F is not run on E (n = 33 already took 49 min for 4 cases; n = 36 is
    about 8× per case).
- Rho lanes W per E rung: argmin of mean Meq(s=1,u=1) over
  W ∈ {1,2,4,8,16,32,64} on the held-out pilot (`pilot-curve|n|ci`, ci = 0,1,
  16 pilot targets each, `data/pilot_rho_ext.jsonl`), the same rule as #5253.
  The chosen W are in `protocol.json`.

**Resource rule.**

- Each rung runs as its own process under `prlimit --as` = 10 GiB and a
  wall budget (`protocol.json`).
- A killed or failed process is recorded as a resource limit, never as a
  negative result, and the run goes on to the next rung.

## Correctness checks (frozen)

- **Group-law replay.** Every decomposition D returns is resolved into signs
  by the group law (`P₁ ± P₂ ± P₃ = ±R` exactly) before it becomes a row.
  Unresolved rows are counted.
- **D = F.** D's set is compared with #5253's factored oracle F
  (`solve_factored`):
  - every target at n ≤ 24;
  - the first 256 / 128 / 64 targets at n = 27 / 30 / 33;
  - the first 16 targets per case at n = 36 and 39.
- **Brute force.** D's set is compared with the exhaustive group-law set,
  every multiset `{P_i,P_j,P_k}` of base classes with any signs:
  - every target at n = 18 and 21;
  - the first 64 targets per case at n = 24.
- **Logs.** For every solve, `[d]G = Q` is checked inside the counted solve,
  and `d = d_true` by the evaluator (custody as #5253: the solver never sees
  d). Also, D's `(d, targets, relations, columns)` must equal F's A2 record
  on every panel case.
- **Replay.** One representative case (n = 30, ci = 0, ti = 0) is rerun from
  the frozen manifest, and every count is compared bitwise.

## Cost unit and band

- **Unit.** #5253's Meq, unchanged:
  - field m, s at 1;
  - an inversion at its Itoh–Tsujii census;
  - 1 per table map (trace, AS solve, square root, K′-membership), per hash
    probe, and per product or addition mod ℓ.
- **Subfield arithmetic.** D's subfield arithmetic is done on embedded
  elements and counted at full `F_{2^n}` price. A native K′ representation
  would be cheaper per operation; it is not credited.
- **Band** (§5a): non-field units priced at 0, 0.1 and 1, and squarings at 1
  and 0.25 (the five #5253 combinations). An order between arms is stated
  only where it holds across the band.
- **Wall time** is recorded and is indicative only.

## Analysis (frozen; `bwd_harness analyze`)

- **Paired ratios per case:** D/rho, F/rho, M2/rho (each over the mean of the
  16 rho runs on the same Q), D/F and D/M2.
- **Per rung:** the geometric mean, with a 95% hierarchical bootstrap
  (curves, then targets, then rho seeds), B = 10000 for the primary key and
  2000 for the band. Stream `boot|bwd|main`.
- **Rho A/A noise floor:** seeds 0–7 against seeds 8–15 on the same Q, per
  rung.
- **Exponents:** OLS slope of log₂ cost on log₂ ℓ per arm, with a bootstrap
  within rung (`boot|bwd|slope|<label>`). Fitted for D (all rungs and
  n ≤ 33), D's relation phase (LA excluded), rho, F (n ≤ 33) and M2.
- **D census per rung:** each phase's Meq, targets/q′, decompositions per
  target (against the prediction below), mean deg U′, oracle Meq per target,
  and LA share.
- **Crossover projection** (a model, labelled as such): see below.
- **Gap:** reported as the remaining distance (ratio and CI), never as pass
  or fail.

## Predictions (written before D's evaluation data)

**Decomposition probability.**

- Let the base have F ≈ q′/2 classes (plus T), and let the group order be
  `N = hℓ ≈ q′³`.
- Unordered triples of classes number about F³/6. Each gives 4 sign classes
  `±P₁±P₂±P₃` up to a global sign.
- A target class `±R` is hit, in expectation, `(F³/6)·4 / (N/2) = 4F³/(3N)`
  times, which is about `(4/3)(1/8) = 1/6 = 1/3!` per target.
- This is constant in q′, whereas pairs-and-solve pays `q′²/8` root finds
  per target. Triples with a summand outside E(F_{2^n}) (abscissa in K′,
  point on the twist) do not give rational sums, except in degenerate cases.
- A solve needs about one relation per column: `T ≈ 6F ≈ 3q′` targets, or
  about 2.6–3 q′ after the echelon's early stop.
- The analysis records the measured decompositions per target next to
  `4F³/(3N)`.

**Per-target oracle cost** `c_D(n′)`:

- descent and elimination are a constant, about 400 Meq;
- the root finding of U′ (deg 16) is about `(n′−3)·(16 s + 15·16 m)` plus
  a gcd of about 500 m;
- the lift is about 300 Meq.

So `c_D ≈ 1000 + 260·n′`, from about 2.5·10³ (n′ = 6) to about 5·10³ Meq
(n′ = 16). It is polynomial in n′ and independent of q′.

**Hypotheses.**

- **H1 (exactness).** On every checked target, D's set equals F's and the
  exhaustive group-law set. On all 64 panel cases D reproduces F's
  `(d, targets, relations, columns)`, and every log is correct.
- **H2 (constant).** The oracle Meq per target is linear in n′, between
  1.5·10³ and 6·10³ over n′ = 6..16, and the mean deg U′ is 16.
- **H3 (gap on the panel).**
  - D/rho lies between 10 and 100 at every rung n = 18..33, so no parity.
  - D/F ≤ 10⁻² for n ≥ 24.
  - D/M2 is below 1 from some rung in 27..33, and above 1 at n = 18.
- **H4 (slopes).**
  - D's relation-phase slope (log₂ cost on log₂ ℓ) is 0.33–0.45, comparable
    to rho's pre-asymptotic 0.37 on the panel.
  - D's total slope rises above 0.5 once the echelon LA, which grows like
    F^γ with γ ≈ 2.5–3, takes over.
  - The LA share is above 50% from n = 42.
- **H5 (crossover), written as a model.** With
  - `D(n′) = κ q′ (α + β n′) + LA(F)` and `rho(n′) = S·√ℓ`,
  - `ℓ ≈ q′³/h` and `F ≈ q′/2`,

  the full-base arm does not cross rho for any n′, because LA is at least
  F² (exponent 2 > 3/2). Then:
  - the relation phase alone crosses at about
    `q′^{1/2} = κ(α+βn′)√h / S`, that is n′ ≈ 22–26 (n ≈ 66–78) at the top
    rung's S;
  - a balanced base of F/λ classes has rho's exponent 3/2, with ratio about
    `√(10 κ c_D h)/S`, of order 50, growing like √n′, so no crossing;
  - only a double-large-prime variant (exponent 4/3) could cross, and none
    is measured here.

**Falsifier.** Parity is reached at a rung if the upper 95% bound of D/rho
is ≤ 1 there. Such a rung must then carry the rho A/A floor and the full
verification before it is stated.

## Crossover formula (used by the analysis)

```text
rho(n′)   = S_top · √(2^{3n′} / h̄)
D_rel(n′) = κ · 2^{n′} · (α + β n′)           κ = targets/q′, α, β fitted on all rungs
LA_e(F)   = 2^{a} F^{γ}                       fitted on rungs with F ≥ 256 (echelon, measured)
LA_w(F)   = 10 F²                             Wiedemann model, mod-ℓ units (not measured)
balanced:  λ = max(1, (LA_w/D_rel)^{1/4}),  D = D_rel·λ² + LA_w/λ²
```

The analysis reports the first n′ ≤ 64 (step 1/4) where each ratio is ≤ 1,
and the minimum ratio of each. These are projections from fitted constants,
not measurements.

## Pre-freeze disclosure

Before this freeze, engineering runs of D touched three evaluation cases,
(n, ci, ti) = (24, 0, 0), (33, 0, 0) and (42, 0, 0). They were used to check
that the code ran end to end and to compare root-finding routes, which were
cost-only choices:

- The Frobenius-conjugate gcd `gcd(U′, σU′)` was dropped because U′ is in
  K′[t] up to a scalar, so the gcd is U′ itself.
- Monic Euclid was replaced by a pseudo-remainder gcd, which saves about 15
  inversions per target.

Their last outputs are kept in `data/prefreeze_dev_runs.jsonl`. They are not
evaluation data and are not used in the analysis. They did inform the panel
range in H3 (D/rho about 30–60 at n = 24 and 33 on those two cases), and this
is said here so the prediction is not read as blind. The rho-lane pilot used
only held-out `pilot-curve` instances.

## Deviations and what is not tested

- Any change after freeze is an amendment frozen before its own data.
  Every record is kept, failures included.
- Not tested:
  - multi-target amortisation (excluded by instruction);
  - double large prime (only modelled);
  - Koblitz rungs (the subfield base projects to O; see #5253);
  - GHS;
  - isolab / L2.
