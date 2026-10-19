# H2 settlement protocol (frozen 2026-10-19, before any run)

Question (lead H2, PR #2170 round 4): on glued genus-2 instances with cofactor
h = 1 (prime-order E/F_{p²}), does large-prime genus-2 index calculus (arm J)
beat negation-map distinguished-point rho (arm R) in a cold, fully priced,
verified comparison?

## Provenance and scope disclosures (fixed now)

- Backend: `examples/h2_glue_cold.rs` is a fork of #2170's
  `examples/n2_routes.rs` (git blob `df0f097b494eb2d87ace262305bffe5be6eacacf`).
  The Jacobian arithmetic (`radd_fast`/`rmul_fast`, reduced sextic-model
  formulas), the glue construction, arm J and arm R are #2170's own. The patch
  (see `n2_routes.patch`) only adds counters (squarings), cold-accounting fields,
  arm parameters (DP density, plain/negation switch), instance probes and the
  `cold` driver mode. It does not change any group law.
- Not #2229's ecbench adapter. #2229's genus adapter accepts only the Scholten
  family and a different Jacobian backend (70M+3S+I); porting the glue into it
  would replace the backend the task says to use. This study therefore re-implements
  #2229's protocol rules (cold one-target accounting, 12 arithmetic price cells,
  native-charge band, A/A, replays, answer verification) in the standalone driver.
  No ecbench session, sealed record or leaderboard row is produced.
- No PMU on this host (`perf` absent): no retired-instruction metric. Wall times
  are descriptive only (shared 4-core VM, threaded); no wall-speed claim.

## Instances

16 glued instances, h = 1 exactly (`Glue::new(bits, 1, rng)`): 8 at log2 #E ≈ 36
and 8 at ≈ 40, seeds `splitmix(0x48320000 ^ bits<<40 ^ inst)`. Every instance
must satisfy checks G-construction (transfer defined, ℓ-torsion nonzero), `n = ℓ` prime.
10 targets per instance (k uniform in [1, ℓ−1]), 160 targets total. Fixed; no
optional stopping, no exclusions.

## Arms (every target gets all arms, interleaved in this order, fresh seeds)

1. `R`   negation-map DP rho on E(F_{p²}), #2170 default DP density.
2. `J`   #2170's `arm_glue` (first-cycle double-large-prime IC, Θ(p) memory).
3. `Rp`  A/A control: algorithm of `R`, independent seed.
4. `Rr`  A/A replay: algorithm and seed of `R`; counters must be equal exactly.
5. `Pl`  positive control: plain rho (no negation map); expected ≈ 1.3–1.5× R's cost.
6. `D<d>` DP rho, negation map, DP density 2^-d, d ∈ {0,2,4,6,8,10,12} (time × memory).

Seeds: `splitmix(inst_seed ^ target<<16 ^ arm_tag)`; `Rr` reuses `R`'s seed.
Counts are therefore reproducible regardless of thread scheduling; a separate
replay pass re-runs 10 % of targets (every 10th) and compares all counters.

## Cold accounting (per target, everything charged)

Arm cost = all field operations (M, S, I) and native operations (hash, table
lookup, sqrt-table lookup, linear algebra mod ℓ) inside the arm, including both
transfers, initial scalar multiplications, failed work and verification of
candidates. Arm J additionally is charged: the glue construction counts after the
order is known and the twist selection (`build`, measured per instance), the
square-root table build (p native operations) and one Legendre test per
degree-2 divisor, priced at one inversion (#2170 H3). Instance generation and
point counting are common fixtures, recorded not charged. Squarings: Fp²
mul = 3M, sqr = 1M + 2S, inv = I + 2M + 2S (#2229's decomposition, E add =
9M + 4S + I).

## Prices

12 arithmetic cells σ = S/M ∈ {0, 0.5, 1} × ι = I/M ∈ {1, 10, 100, 1000}.
Native charge ν ∈ {0, 0.1, 1} per native operation in units of one E(F_{p²})
addition (GAE = 9 + 4σ + ι at that cell), applied to both arms. Cost =
M + σS + ι(I + Legendre) + ν·GAE·native. Ratio = Σ cost(J) / Σ cost(comparator)
over the 160 matched targets (ratio of sums), 95 % interval from a two-level
bootstrap (resample instances, then targets within instance, 4000 resamples,
seed fixed 20261019).

## Comparator and time × memory

Memory unit: 64-bit words stored at the end of the run: DP entry = 4 words (x
two words, a, b), J row = 2 words per stored large-prime entry + 3 (a, b, key).
Equal-memory rho `RM`: for each instance, the D<d> arm with the smallest d whose
mean stored words ≤ J's mean stored words on that instance (rule fixed here, not
chosen on cost). Comparator for the decision: per cell, the cheaper of `R` and
`RM` (so J must beat rho that is allowed J's memory). Memory-capped J (H4,
#2170) is not rerun.

## Controls (pass conditions, fixed now)

- A/A: Rp/R ratio-of-sums 95 % interval contains 1 in all 36 (cell, ν) combinations,
  and |ratio − 1| < 0.03; Rr equals R exactly in every counter on all 160 targets.
- Replay pass: identical counters on the replayed 10 %.
- Positive control: Pl/R interval excludes 1 (> 1) in all 36 combinations.
- Rho calibration: mean R steps / (0.886 √ℓ) in [0.85, 1.25] (#2170 K6 reported
  ≈ 1 with DP overhead).
- Validity: every row verified (kP = Q, k equals the planted scalar). Any
  unverified row invalidates the experiment (reported, not dropped).
- If any control fails, no decision is taken from the run.

## Per-cell verdict and DECISION RULE (frozen)

For each of the 12 arithmetic cells: `rho-wins` if the lower interval end of
J/comparator > 1 at every ν ∈ {0, 0.1, 1}; `IC-wins` if the upper end < 1 at every ν;
otherwise `unresolved` (§5a: orders are ranked only where they hold across the band).

- 12/12 `rho-wins` → **close H2** (negation rho wins; genus-2 IC on h = 1 gluing is
  not a path to beating rho in this cost model).
- Otherwise (any `IC-wins` or `unresolved` cell) → **scope the ≤ 22M + I genus-2
  addition**: compute per-cell break-even Jacobian-addition cost from the data, bound
  what the real-model formulas can reach, build it if the bound permits, and re-run
  under this same protocol (new directory, new freeze). H2 stays open until then.

Reported with every row: distance to the matched rho (ratio), the backend's
measured per-addition (M, S, I), and the break-even addition cost per cell.
