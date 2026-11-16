# C1 end to end: standard base against the twisted all-lift line, against strong rho

Preregistered 2026-11-07, before any measured data. First committed in `bfcd18db85`.
Frozen 2026-11-08, after the stopping rule 1 checks and still before any measured data
(the freezing commit is in [`protocol.json`](protocol.json)'s `freeze` block). Machine-readable
form: [`protocol.json`](protocol.json) (`experiment.protocol/v1`, validated and frozen with
`crypto-autoresearcher/tools/experiment_protocol.py`). Instrument: [`specs/`](specs/).
Frozen analysis: [`analyze.sh`](analyze.sh), whose hash is in `protocol.json`.

**Changes between the first commit and the freeze, all made before any measured data:**

- Rule 1 split the stage into two dispatches; see "Arms" below.
- Arm order changed to `alternate_measured`, with 5 measured rounds.
- The arm `rho-strong-l1-aa` was added as the std session's A/A.
- The §5a band now reads the run's own counted calibration ratios (`calibration_ratios`)
  when the pinned table has no row for the curve. Before this change, `ecbench compare` could
  not price the band for any `F2m4` IC record.

## Question

Earlier work measured the twisted line's per-trial relation yield: about 16 times the
standard base's (PR #2704). It also measured the per-trial Macaulay cost: unchanged,
ratio 0.961 [0.928, 0.991]. The question here is whether that turns into a
lower **complete, verified, cold end-to-end** index-calculus cost. The cost
covers relations, linear algebra and the discrete log, charged in ecbench's
counted unit on the same one-target workloads. We also ask how far each arm
stays from the matched rho.

## Curves

`f2m4_a0`: `y² + xy = x³ + b` over the tower `F_{q⁴} = F_q[z]/(quartic)`,
`q = 2^m`. The constructor is `ecbench::xline_bin::find_bin_curve_a0(m, seed = 1,
max_cofactor = 8)`, which is deterministic. It takes the first instance seed with
`#E = h·r`, `r` prime and `h ≤ 8`.

- Stage 1: `m = 5` (`N = 20`), 6 targets.
- Stage 2: `m = 7` (`N = 28`), 4 targets, run only under the stopping rule below.

These are the C1 ladder's generic-`b` family (family B, `a = 0`). The ladder's
`a, b ∈ F_16` family A is not used, for two reasons:

- A1 has no prime factor above 2^9 of `#E` at `k = 5, 7`. A2 has none worth using at `k = 9`.
- A curve defined over `F_16` admits the `F_16`-Frobenius. Its matched rho would then have to
  fold by it (`A = 2k`), and no such walk exists on this tower in ecbench.

Construction rules (`research/subgroup_trap_20261107`), enforced in code:

- `Tr(a) = 0` (`a = 0`).
- `b ∉ F_{q²}`, hence `[K : F_2] = N` for `K = F_2(line, a, b)` on every line.
- `u ∉ F_q`, and `u ∉ b^{1/4}·F_q`.
- Targets are planted in the prime-order subgroup: ecbench `uniform_scalar_sha256_v1`,
  `target_seed = 20261107`.

## Arms (one binary, two sessions per stage)

Stopping rule 1 split each stage into two isolab dispatches with the same workloads
block (6 targets, `target_seed = 20261107`) and the same measurement seed. Their
`(workload, round)` seeds therefore pair across sessions.

- `specs/c1-e2e-m5-std.json`: both rho arms, `ic-std`, and `rho-strong-l1-aa`
  (the A/A of that session).
- `specs/c1-e2e-m5-tw.json`: both rho arms, `ic-tw-a`, `ic-tw-b`, and `ic-tw-a-aa`.

The `m = 7` pair is the same for stage 2. Both sessions use `rounds = 5`, `warmup = 1`
and order `alternate_measured`.

| arm | role | method |
|---|---|---|
| `rho-strong-l1` | reference | `rho.negation_strong`, `lanes=1`, `dp_bits=auto` |
| `rho-strong-l16` | reference | `rho.negation_strong`, `lanes=16`, `dp_bits=auto` |
| `ic-std` | baseline | `ic.pipeline`, `bin-x-line:line=standard`, `gaudry-x-bin:macaulay=26,sparsest=1,charge_unsolved=1` |
| `ic-tw-a` | candidate | the same, `bin-x-line:line=twisted,line_seed=1` |
| `ic-tw-b` | candidate (construction A/A) | the same, `line_seed=2` |
| `ic-tw-a-aa` | control (wall A/A, tw session) | a repeat of `ic-tw-a` |
| `rho-strong-l1-aa` | control (wall A/A, std session) | a repeat of `rho-strong-l1` |

- **Matched rho.** The reference is the cheaper (lower `Σ S`) of the two strong
  negation walks.
- **Curve automorphisms.** The curve has no automorphism beyond negation: `b` is
  generic, so the curve is not defined over a proper subfield.
- **Shared code.** Both IC arms use the same oracle, the same linear algebra
  (`incremental-gauss`) and the same target stream (`walk`). Only `u` differs.

## Metric

- **Primary.** `S = total GAE / √r`, every phase charged, cold. Solver work is
  counted `F_q` multiplications over the curve's `fq_muls_per_add`, exactly as
  `gaudry-d4-bin` and `rho.negation_strong` are charged on this tower. The base's
  lifts are charged at the measured lift price.
- **§5a band.** Every arm's `*_uncharged` counters are listed with their counts and
  priced at `w = 0, 0.1, 1` GAE per operation, on every arm alike, the references
  included.
- **Secondary.** Wall time. It counts as a result only at L2 and outside the A/A
  interval.
- **Counts.** Trials, relations and decompositions per trial are reported.

## Decision rule

The decision comparison runs across the two sessions. It compares operations only:
ecbench refuses wall time across sessions.

```
ecbench compare --dir STD --a ic-std --b ic-tw-a --b-dir TW --weight w    (w = 0, 0.1, 1)
```

It gives the paired totals ratio and its paired bootstrap 95% interval.

- **Improvement.** The upper end of the interval is below 1 at all three weights, and either
  the line A/A interval (`ic-tw-b / ic-tw-a`) contains 1 or the C1 interval excludes the A/A
  interval.
- **Regression.** The lower end of the interval is above 1 at all three weights.
- **Unresolved.** Anything else. A tie is the result if it happens.
- **Incomplete.** Any comparison containing an unverified run.
- **Correction.** Holm over the two decision comparisons. The A/A comparisons decide nothing.
- **Matched rho.** At each `w`, the matched rho is whichever of `rho-strong-l1` and
  `rho-strong-l16` has the lower total priced `S` in the IC arm's own session.
- **Gap to rho.** `S(arm) / S(rho_ref)` per `w`, reported as the distance remaining, not as a
  gate.
- **Wall time.** Compared within a session only, against that session's A/A.

Prediction P1 (out of sample) is a ratio below 1, with a point value in
`[0.1, 0.71]`. The relation phase dominates at toy size: twice the relations
are needed, but the probability that a trial yields a relation is about 6–9
times higher. The 2LP cost model's `2^{-0.5}` is the at-scale value.

## Controls

- **Positive.** The unit test `oracle_finds_every_decomposition_the_pair_table_finds`
  checks the oracle's decomposition set against an independent pair-table
  enumeration on 12 targets per line. Every measured logarithm is verified by the
  runner and by `ecbench verify --replay`.
- **Negative.** The base refuses curves and lines that break the rules. PR #2704's
  `Tr(a) = 1` control is the measured precedent.
- **A/A.** `ic-tw-b` gives the line-to-line spread. `ic-tw-a-aa` gives the wall noise.

## Stopping rules

1. **Before dispatch (satisfied 2026-11-08, before the freeze).**
   - The `xline_bin` unit tests passed.
   - A development smoke per IC arm verified and replayed identically. It ran at `m = 5`
     with `target_seed = 99`, one workload, L0, on the shared preparing container, and is
     labelled not evidence.
   - Its timing:
     - std session: 56 s per workload-round (`ic-std` 57.7 s);
     - tw session: 49 s per workload-round.
   - Projected over one session with 6 workloads × 6 rounds, that is about 3780 s, above
     2700 s. Each stage therefore runs as the two dispatches above: about 2020 s and about
     1760 s on that host.
   - A dispatch that reaches its 3600 s deadline is kept as censored evidence and the stage
     is incomplete. It is never rerun smaller.
2. **Stage 2.** Stage 2 (`m = 7`, std and tw specs) runs only if stage 1 completes and
   an `m = 7` smoke per IC arm, scaled by the stage-1 measured times, projects each dispatch
   below 2700 s. Otherwise it is recorded as not run.
3. **Budget.** USD 10 in total, counting build, failed and interrupted attempts. Stop
   when the budget is reached or the spend is unknown.
4. **Failed runs.** Failed, timed-out and wrong runs are kept. A comparison containing one is
   incomplete.

## Inadmissible

- Reporting the per-trial yield gain as an end-to-end improvement.
- Changing the oracle, line seeds, curves, targets, seeds, unit or accounting after
  data.
- Dropping runs, or shrinking targets or rounds to fit a deadline.
- Wall time below L2.
