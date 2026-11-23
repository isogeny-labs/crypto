# Preregistration: six follow-ups to the SAT-against-F4 panel

Frozen 2026-11-15T06:50Z, before any follow-up data. Owner asked for all six ideas listed
in the 05:30Z reply. Standing: `[pcc:unverified]`, local L0 on a shared 4-vCPU container,
stage diagnostics only (AGENTS.md §2/§5): no end-to-end cost, no rho comparison, nothing
about n = 83 or 131. Counted units (conflicts, solving degree, matrix size, XL columns)
are the results; wall time is a practicality note.

Fact that motivates them (panel of record, m = 7–11, `[pcc:unverified]`): the quadratic
incidence form (3ℓ + 8m unknowns) is undecided by F4 at every m (refused degree-4 matrix
≈ 5.9·10⁵ × 5.7·10⁵ at m = 7) and costs SAT 10³× the conflicts of the cubic norm form
(3ℓ + 2m unknowns), which F4 solves at solving degree 4 at m = 7, 9, 11.

## Instruments

- `examples/rr_quadratic_panel.rs`, new arms `rr4` (α, β eliminated: 3ℓ + 1 unknowns,
  Boolean degree 4, as in `examples/rr_degree_ladder.rs::build_rr`), `s4` (Weil-descended
  S₄, 3ℓ unknowns, degree 6, `koblitz_symmetrised::build_direct_x_system`), `norm4`
  (four summands, f = xy + c₂x² + c₁y + c₀x + c₋ ∈ L(5O), one coefficient pinned by
  f(−R) = 0: 4ℓ + 3m unknowns, degree 4), new solver `xl` (`pq_xl`, fixed degree D).
- `examples/rr_sat_amortise.rs` with an assumptions API added to `cryptanalysis::sat`
  (held to agreement with unit-clause solving on random instances before use).
- CryptoMiniSat 5 command-line binary built from a pinned source release, run on the
  DIMACS/XOR files the panel exports, Gauss–Jordan on and off.
- Same curve (y² + xy = x³ + 1), same seed scheme, same ground truth (base includes T),
  every solution and model verified by group arithmetic.

## F1. Eliminated norm form `rr4` and the ℓ-ladder (idea 1)

Cells: m = 13, ℓ = 3, 4, 5, 6; m = 17, ℓ = 4, 5, 6, 7; arms `norm`, `rr4`, `s4`; solvers
SAT and F4; 4 planted + 4 unplanted draws; 600 s budget.
- P1a. SAT conflicts on `rr4` are below `norm` on the median draw in every cell where both
  decide (fewer unknowns).
- P1b. F4 solving degree on `rr4` rises by about 1 per unit ℓ (the Macaulay ladder's
  ≈ ℓ + 2); on `norm` it stays at 4 while F4 decides.
- P1c. log₂(median SAT conflicts) on `norm` grows with ℓ at a slope below 1 per unit ℓ
  (sub-exhaustive in the summand bits) at m = 13.
- Falsifier F1: if `rr4` SAT conflicts exceed `norm` in ≥ 3 of the 4 m = 13 cells, the
  "fewer unknowns wins" reading is wrong for SAT.

## F2. Guess-and-solve floors (idea 2)

Same draws as the panel's norm arm at m = 11, 13. Sweep k = 0 … 2ℓ guessed summand bits
(the first k of u₀, u₁): total conflicts over all 2^k branches, and branches until the
first SAT branch.
- P2. The minimum of total conflicts over k is at k = 0 (the solver does better than any
  guessing split) at both m. Falsified if some k ≥ 1 is lower on the median draw.
- The k = ℓ point is the guess-one-summand floor and is reported separately.

## F3. Four summands (idea 3)

Cells: m = 13, 15, 17 with ℓ = ⌊m/4⌋ and ⌊m/4⌋ + 1; arm `norm4`; SAT and F4; 4 + 4 draws.
- P3. SAT decides every `norm4` instance at m ≤ 17 within 600 s, and F4 solving degree is
  ≤ 5. No direction predicted for the conflict growth; it is the measurement.

## F4. Amortising across targets (idea 4)

Fixed V per draw, 64 targets (planted and uniform mixed as in the panel), norm arm with
the target bits as variables fixed by assumptions in one solver instance (warm) against a
fresh solver per target with constants (cold).
- P4. Warm total conflicts over the 64 targets are below cold by a factor ≥ 2 at m = 11,
  13. Falsified if the ratio is < 1.2.

## F5. XL at fixed degree (idea 5)

Norm arm at m = 7, 9, 11 (and 13 if memory allows), D = 4 and 5.
- P5. XL at D = 4 recovers exactly the ground-truth solution set (n_solutions equal to the
  group-arithmetic count) at every m where F4's solving degree is 4. Falsified if XL(4)
  needs D = 5 on any draw where F4 reported 4.

## F6. Gauss–Jordan in CryptoMiniSat (idea 6)

The panel's exported CNF files, norm and incidence arms, m = 7–13: CryptoMiniSat with
default options (Gauss–Jordan on) and with Gauss–Jordan disabled; the repository solver's
conflicts from the panel as the third column.
- P6. Gauss–Jordan cuts CryptoMiniSat conflicts by ≥ 10× on the norm arm at m ≥ 11.
  Falsified if < 2×.

## Inadmissible

Changing a system, the seed scheme, or the budget between arms of the same comparison;
counting a timeout as a refutation; reporting any of this as an ECDLP speedup.

## Amendment 3 (2026-11-15T08:58Z, before the affected cells ran)

- **F1, `s4` under F4 at ℓ ≥ 6.** The ℓ = 6 F1 process was killed by the container's
  memory cgroup at both m = 13 and m = 17 on the same step (row 6 of draw 0: `s4`, F4,
  > 10 GB virtual), after `norm` and `rr4` under both solvers and `s4` under SAT had
  completed for that draw. This is recorded as the outcome of that cell ("F4 on S₄ at
  ℓ = 6: killed above 10 GB"). The remaining ℓ = 6 draws (1–7) and all of ℓ = 7 run `norm`
  and `rr4` under SAT and F4 and `s4` under SAT only.
- **F4 sample size.** At m = 11 the warm and cold_sym arms take up to ≈ 150 s per
  unsatisfiable target, about 40 min per draw, so the registered 8 draws × 64 targets at
  m = 11 and m = 13 do not fit the session. Reduced to m = 11: draws 0–3 × 64 targets;
  m = 13: draws 0–3 × 16 targets. Decided after two m = 11 draws had been summarised
  (cold_sym/warm conflicts 1.023 and 1.394; cold/cold_sym 0.019 and 0.021). Those two
  values were seen before this amendment, which is stated here; the threshold of P4 is
  unchanged.

## Amendment 4 (2026-11-15T11:05Z, before the affected cells resumed)

- **F1, `rr4` under F4 at ℓ ≥ 6.** The memory cgroup killed the `norm,rr4` process at
  m = 13, ℓ = 6 on `rr4`/F4 draw 5 (draws 0–4 of that cell had completed as "undecided,
  degree 7 reached, matrix refused") and at m = 17, ℓ = 7 on `rr4`/F4 draw 0. Recorded as
  those cells' outcome ("F4 on rr4 at ℓ ≥ 6: killed above the container's memory on some
  draws"). The remaining draws of both cells run `norm` under SAT and F4 and `rr4` under SAT
  only; `s4` under SAT as before.
- **Container restart at ≈ 10:56Z** killed every queue; all resumed from the next
  unrecorded draw on the identical RNG stream (`--first-draw`), no row repeated.
