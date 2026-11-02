# P2 protocol: scaled analogues of ECCp-131's isogeny class (frozen before the full run)

Status: written 2026-11-02 by the P2 builder, before any measurement of the
full study. The smoke run in `smoke/` exercised the harness only; its numbers
are not data of this protocol. Parent design:
[`../DESIGN.md`](../DESIGN.md). Rules followed: AGENTS.md "Standing workflow
rules", section 5a, "IC measurement rules", "Implementation language: no
Python"; skill `controlled-experiment`.

## 1. Question and claim class

Question. Inside one isogeny class whose Frobenius order is the maximal order
(conductor 1, one crater, no vertical levels, every curve of the same prime
order), do the index-calculus parameters of the repository's prime-field
pipeline differ between curves? The scaled analogues stand in for ECCp-131
(prime order, conductor 1, h about 2^63.6), whose class cannot be enumerated.

Claim class: **comparative / statistical** (curve-to-curve variation, "no
effect"). It needs the three controls below and licenses only the sentence in
section 9.

Preregistered prediction: **null.** Within a class only transport cost,
automorphisms and the model presentation can differ
(`research/isogeny_class_difficulty_20261008`). A null closes a lead; it is not
an improvement over the index-calculus baseline (CLAUDE.md success criterion),
and it compares no cross-method cost, so no rho reference or section 5a band
applies (this study ranks curves of one class under one method, never two
methods).

## 2. Population (built by `p2tool build`, recorded in `classes.json`)

Ten classes: two discriminants times five sizes. The discriminants are fixed
across sizes so that size is the only variable between the sizes.

| rule | value |
|:--|:--|
| D | the first fundamental discriminants (by `|D|`) with `D mod 8 != 1`, a prime-order `(p,t)` at every size, and h in [120,200] (class A) or [300,400] (class B) |
| class A | D = -84171 = -3 * 28057, h = 128 |
| class B | D = -414651, h = 300 |
| (p,t) | `t^2 - 4p = D` exactly, p the largest prime below 2^b (inside [0.56,1] * 2^b) such that `n = p + 1 - t` is prime, t > 0 |
| conductor of Z[pi] | 1 (D is fundamental and equals the Frobenius discriminant) |

Why `D mod 8 != 1`: then 2 splits in the CM field and `#E = p + 1 - t` is even,
so no prime order exists. Why `|t|` is near `2 sqrt(p)`: a class number of
20..400 forces `|D| = 4p - t^2` to be tiny, so the Frobenius trace sits at the
Hasse boundary. ECCp-131's trace is generic and its h is 2^63.6; this
difference is a **limit of the analogue, not a feature**, and it is stated
wherever the result is used. Two classes per size with different h guard
against a single-class accident only.

Per class, `classes.json` holds every curve (one representative per j; both
twists are not in the class and the order-`n` twist is taken), with:
- the h roots of the Hilbert class polynomial mod p (PARI 2.17 `polclass`,
  `polrootsmod`; exactly h roots required);
- `ellfromj`, quadratically twisted when needed, then **`#E = n` re-verified
  in Rust without PARI** (n prime, `n G = O` for a point `G != O`, and
  `2n > p + 1 + 2 sqrt p`, so n is the only multiple of ord(G) in the Hasse
  interval); `j(a,b)` re-derived and equal to the root;
- the isogeny edges used: F_p-rational l-isogenies for l in {2,3,5,7,11,13}
  read from `Phi_l(j, Y)` mod p (`polmodular`); every root found must be in
  the class (`off_class_rational_neighbours` = 0: conductor 1 admits no
  descending or ascending rational neighbour), and the class must be connected
  under those edges (`connected_components_under_ells_le_13` = 1);
- `iso_models`: 8 curves per class, chosen by a seeded hash stream, each
  re-presented by a seeded `u`-scaling `(a,b) -> (u^4 a, u^6 b)`,
  `G -> (u^2 x, u^3 y)`; same curve, same j, a different smallest-abscissa
  factor base.

The class is enumerated completely by construction (all h roots), not by a
walk, so no curve of the class is missing.

## 3. Instrument (the same on every arm and every curve)

`ecbench run` of method `ic.pipeline`, workload `prime_explicit`, written by
`p2tool spec`:

- factor base `prime-abscissa:size=N`, the N smallest abscissae of the model
  in `classes.json`; **one rule for all curves**: `N = min(N_y, 1448)` with
  `N_y = ceil(sqrt(y n / 2))` (m = 2, y = 0.02) or `ceil((y n / 4)^(1/3))`
  (m = 3, y = 0.1). N depends on n and the size only, and n is the same for
  every curve of a class, so every curve of a class gets the same N. The cap
  keeps the oracle's pair table, (2N)^2 / 2 entries, within 2^22.
- oracle `mitm:m=2` or `mitm:m=3`;
- `linalg = incremental-gauss-full-rank` and `max_trials = A`, with
  `A = ceil(R / y_eff)`, `y_eff` the predicted yield `2N^2/n` (m = 2) or
  `(4/3) N^3/n` (m = 3, a fit to the calibration runs, within 12%), and `R = min(100, 0.6 N)` expected relations. The
  relation count stays below the column count, so no run completes early:
  **every run is exactly A independent decomposition attempts** and its
  relation count is a fixed-n count (no data-dependent stopping). Every row
  therefore ends `exhausted`; the outcome status is not a failure here.
- targets: `targets: walk` (the default stream), 4 targets per curve
  (8 for m = 3), planted scalars from `target_seed = 20261102` (the harness's
  frozen `uniform_scalar_sha256_v1` law), identical across arms;
- measurement: `order = alternate`, rounds even (4), 1 warm-up, one algorithm
  seed per (workload, round) from `measurement.seed`, **shared by the arms of
  that round** (ecbench pairs arms), different across rounds and targets.

Quantity definitions. An **attempt** is one increment of `targets_tried`; a
**relation** is one increment of `relations_found`. A **cell** is one
(curve, round, target) run; with 4 rounds and 4 targets a curve has 16 cells
(32 for m = 3). Metrics per cell:

1. **yield** = relations_found / targets_tried (primary, a counted quantity);
2. `rel_gae_per_attempt` = relations-phase GAE / attempts (counted group-operation
   work per attempt; `ecbench.gae`, the unpriced native work of section 5a is
   the same on every curve and cannot reorder curves of one class);
3. `online_wall_ns_per_attempt` = `online.wall_ns` / attempts (a clock:
   descriptive unless the row is at the level the spec requires, L2, and read
   only against the in-session A/A interval; AGENTS.md section 10).
4. `oracle_setup` GAE per run (reported in `per_curve`, not tested).

Not measured, and why. **F4 solving degree and Dreg**: the repository's prime
pipeline has no F4 stage (decomposition is by pair-table lookup;
`descent-algebraic`/`f4-f2` is binary-field only, `f4_fp` is exercised only
through the Weil-restriction covers over extensions). A degree measurement for
F_p-points would need a new Semaev system with a degree-N membership
polynomial, whose solving degree is N plus a constant independent of the
curve, so it would not separate curves. It is not measured and not claimed.
**Full-DLP solve time** is measured only by the optional `--kind solve` spec
(default linear algebra, completes when the target is pinned), run at 24 bits
(class cost in section 8) and compared as a descriptive S per curve; it is not
a test of this protocol.

## 4. Controls

- **A/A (noise floor).** Two forms, because ecbench shares the algorithm seed
  of a (workload, round) across arms: (i) the in-session `m*-aa` arm is the
  identical method on the identical seeds. It must reproduce the relation
  counts exactly (a determinism check, reported as `identical/pairs`; any
  mismatch invalidates the session) and its wall differs from `main` only by
  clock noise, which sets the noise floor of the clock metric. (ii) For the
  counted metrics the A/A is the **round-parity split**: rounds 0,2 against
  rounds 1,3 of the same curve (independent algorithm seeds, same FB, same
  targets); `aa_round_halves` reports the spread of the per-curve log ratio.
  Every curve-level difference is read against it. `m*-aa` is dropped from
  runs at 32 bits and above (deterministic duplicate; it only costs time).
- **Isomorphic-model control (per class).** 8 curves in a second model
  (u-scaling). They change the factor base (the smallest abscissae of
  `u^2 x`) and nothing else. Reported as the RMS log ratio of each model to its
  base curve against the A/A round-halves spread; this is the presentation
  noise the curve-to-curve spread must exceed to mean anything.
- **Positive control (power).** Arm `m*-pc` uses `N' = round(N * 1.2^(1/m))`,
  so its expected yield is 1.2 times main (the repository's prime-order mitm
  yield is `2N^2/n` (m = 2) and `(4/3) N^3/n` (m = 3, a fit to the calibration runs, within 12%), measured 0.0205 against 0.0200
  predicted at 24 bits). It runs the same code on the same curves and seeds.
  `analyze` substitutes the pc cells of **one curve at a time** into the main
  data (200 random choices) and reports the fraction of plantings that the
  detectors flag at the 95th percentile of the unplanted permutation null:
  ICC, max |z|, max/min. **The null result is informative only if the power
  of every detector on a planted 20% curve effect is at least 0.9** in that
  class and m (checked per cell, reported in `positive_control`); a cell whose
  power is below 0.9 is reported as "not informative at 20%", never as a null.
- **Negative control / null population.** Under H0 the cells of a class are
  exchangeable across curves (one n, one N, one A, one seed law); the null
  distribution of every statistic is generated by permuting the cells across
  curves (2000 permutations). This is the right null because every nuisance
  that could differ between curves (target, seed, round, order) is already
  crossed with curve and sampled independently; there is no separate
  random-curve population, since an unrelated curve of the same size has a
  different n and different N and would add a factor, not remove one.

## 5. Analysis (`p2tool analyze`, frozen by the binary hash printed in the run)

Per class and m, on the main arm, real curves only:

1. **ICC curve share of variance** (primary): unbalanced one-way random-effects
   ANOVA estimator of `sigma_curve^2 / (sigma_curve^2 + sigma_cell^2)` for each
   metric; the raw estimate may be negative; the clipped value is reported.
   Upper bound: one-sided 95% percentile of a **two-level bootstrap** (curves
   with replacement, cells within drawn curves, 2000 resamples, clipped at 0).
   Permutation p-value: share of 2000 cell permutations with ICC at least the
   observed.
2. **max/min** of the per-curve pooled mean and its permutation null; and
   **max |z|** of a curve against the others, with its permutation null. The
   A/A comparison: `aa_round_halves.max_ratio` and `q95_abs_log_ratio`.
3. The iso and positive controls as in section 4.

Analysis seed 20261102 (splitmix64). Arm labels are not masked: the arms are
roles fixed here (main, aa, pc) and the only comparison between arms is the
frozen pc substitution; no free arm contrast exists to bias.

## 6. Decision rules and multiplicity

Family: primary test = yield ICC permutation p-value, 10 classes x 2 values
of m = 20 tests; Bonferroni alpha = 0.05 / 20 = 0.0025. Secondary metrics
(`rel_gae_per_attempt`, wall) are reported with the same statistics and are
not in the family. Outcomes:

- **Null supported** in a cell: yield ICC permutation p >= 0.0025, bootstrap
  upper bound of the ICC reported (it is the size of curve effect still
  compatible), max |z| and max/min inside their null q95 (or p >= 0.0025),
  iso RMS log ratio of the order of the A/A spread, **and** positive-control
  power >= 0.9.
- **Curve effect suspected** in a cell: p < 0.0025 for the yield ICC (or max |z|),
  and the effect exceeds the iso control (otherwise it is a presentation
  effect of the factor-base rule, not of the curve). It triggers a follow-up
  protocol (not a claim): rerun at fresh seeds, then test which of n, the
  smallest-abscissa structure or the CM field explains it.
- Everything else: reported as inconclusive with the numbers.

## 7. Variable ledger

| factor | treatment |
|:--|:--|
| instrument: commit, `rustc -Vv`, flags, `ecbench` and `p2tool` SHA-256 | fixed; recorded per row (`binary_sha256`) and in the run log |
| host, CPU, SMT, governor | fixed by the route (isolab-eks, L2 required in the spec); measured per row |
| arm order | crossed: `alternate`, balanced rounds, warm-up discarded |
| seeds | derived by ecbench from `measurement.seed` and the workload id; `target_seed` fixed here; iso `u` from a seeded SHA-256 stream; analysis seed fixed; no seed chosen after a look |
| curve model presentation | the `ellfromj` model (twisted if needed) is the main arm; the iso models are the control |
| size, h, D | size varies across cells; D fixed across sizes; class A/B differ in h only by construction |
| N, A | functions of (n, m, cap) only; identical on every curve of a class; the pc arm differs on purpose |
| m | crossed (2 and 3) |
| spot preemption | counts of a re-run row are reproducible from the seed (deterministic); a clock of a preempted or resumed row is invalid and excluded from the wall metric |
| analyst | analysis is the frozen binary; no hand selection of cells |

## 8. Cost and run plan

See `README.md` for the measured per-run times, the CPU-hour estimate per
size and the exact commands. Budgets: `timeout_seconds = 3600` per run;
m = 2 at 36 and 40 bits is optional and costs most; if it is skipped the
protocol is run on the other 16 cells and the skipped cells are reported as
not run, never filled.

## 9. What a result licenses

Exactly: "In the enumerated class (D, h, p, n) the yield of the repository's
prime-abscissa mitm pipeline at m, |F| = N has curve share of variance at most
U (95% bootstrap bound), a max/min curve ratio R against an A/A spread S, and
the detector finds a planted 20% curve effect with power P." It is a
structural statement about the pipeline on these scaled analogues. It is **not**
a statement about ECCp-131 (different t, h and size), not a security
statement, not a speed claim, and not an improvement over the index-calculus
baseline. A null is recorded as a closed lead. No leaderboard or
`benchmarks/README.md` row follows (the validation gate needs a matched rho
and this study has none by design).

## 10. AMENDMENT A1 (2026-11-02, filed before any data): local route, reduced scope

Status: filed before any measurement of this protocol. `runs/` did not exist
when this was filed; `smoke/` and `calibration/` are harness and sizing checks,
not data. Protocol record: `protocol.json` version 2, amendment id
`A1-local-reduced-scope-20261102`.

Why. The isolab-eks route could not be used from the submitting host. The owner
decided (2026-11-02, relayed by the main session): local fallback, reduced
scope, isolation L0 accepted.

What changes.

- **Run scope: 12 cells.** c24A, c24B, c28A, c28B, c32A, c32B for m = 2 and
  m = 3, with the section 8 flags of `README.md` (`--targets 4`, 8 for m = 3 at
  24 bits; `--rounds 4`; `--aa 1` at 24 and 28 bits, `--aa 0` at 32 bits).
- **Deferred, not dropped: 8 cells.** c36A, c36B, c40A, c40B for m = 2 and
  m = 3 stay in the population and in the preregistration; **no data are taken
  for them under this protocol version**. They are reported as "not run (deferred)",
  never filled. Running them later needs a further amendment that says whether
  it precedes their data.
- **Route.** Local backend, isolation L0, one cell at a time under the ecbench
  host lock, smallest cell first; `ecbench verify --replay 12`; per-cell card
  timeout 2 x the README estimate; the instrument is built in a dedicated git
  worktree pinned to the commit that files this amendment.
- **Clock.** At L0 `online_wall_ns_per_attempt` is descriptive only. The counted
  metrics (yield, `rel_gae_per_attempt`, `oracle_setup_gae`) are exact and are
  the only ones that decide.
- **Unchanged.** Question, claims, arms, controls, analysis binary, seeds, N, A, R,
  thresholds, permutation and bootstrap counts, and the decision rule. The
  multiplicity family stays 20 tests, Bonferroni alpha 0.0025: the 8 deferred
  tests are not run and are not removed from the divisor (more conservative).
- **Restated for the run scope only.** `sample_size.instances` 1332 (3 sizes x
  444 curve models; the v1 figure 444 was the per-size count),
  `preflight_runtime.estimated_total_seconds` 67320 (18.7 CPU-hours, README cost
  table, 12 cells), `stopping_rules[0]`, `variable_ledger` factors `size_D_h` and
  `relations_per_run_R`.

Hashes (sha256).

| file | hash |
|:--|:--|
| `versions/protocol-v1.json` (frozen v1, byte-identical to the pre-amendment `protocol.json`) | `6fda9efb1a160d4d71d8bd394e6e2139b1d608c6673fa630c0034536f16a3b0d` |
| v1 canonical freeze hash (`freeze.protocol_sha256`) | `1cab21213fa7eb5562009d924039c7bf17b6c02161499fed8b11bef40c376e9e` |
| `protocol.json` (v2, full file) | `8b6fc89c3c60352b7e5203bcd1120cc10babf28ce91c59b3fce77c12976a14e0` |
| v2 canonical freeze hash (`freeze.protocol_sha256`) | `334f54c93bdb3015f2732d6f9dc6d927a07fa7a2039b2a4d8eeb21db1be08b53` |
| `p2tool/src/analyze.rs` (unchanged, frozen analysis) | `f6cc7286daae471bc40e5860ed8673f1e1676b9fadce3199b8fe67ca8d167678` |

The draft cards `CARD-isogeny-ic-yield-p2-*-20261102-1` (pinned to commit
4c43066b7, binary hashes of a different build) are superseded by the `-2` cards
pinned to the amendment commit; they are kept, never run.
