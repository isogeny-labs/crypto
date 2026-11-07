# C1 end to end: standard base against the twisted all-lift line, against strong rho

Preregistered 2026-11-07, before any measured data. Machine-readable form:
[`protocol.json`](protocol.json) (`experiment.protocol/v1`, validated with
`crypto-autoresearcher/tools/experiment_protocol.py validate`; its `freeze`
block records the freezing commit). Instrument: [`specs/`](specs/).

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

## Arms (one spec, one binary, one Job)

| arm | role | method |
|---|---|---|
| `rho-strong-l1` | reference | `rho.negation_strong`, `lanes=1`, `dp_bits=auto` |
| `rho-strong-l16` | reference | `rho.negation_strong`, `lanes=16`, `dp_bits=auto` |
| `ic-std` | baseline | `ic.pipeline`, `bin-x-line:line=standard`, `gaudry-x-bin:macaulay=26,sparsest=1,charge_unsolved=1` |
| `ic-tw-a` | candidate | the same, `bin-x-line:line=twisted,line_seed=1` |
| `ic-tw-b` | candidate (construction A/A) | the same, `line_seed=2` |
| `ic-tw-a-aa` | control (wall A/A) | a repeat of `ic-tw-a` |

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

Run `ecbench compare --a ic-std --b ic-tw-a` over all `(workload, round)` pairs. It
gives the paired totals ratio and its paired bootstrap 95% interval, at each `w`.

- **Improvement.** The upper end of the interval is below 1 at all three weights,
  and `ic-tw-b / ic-tw-a` (the line A/A) contains 1.
- **Regression.** The lower end of the interval is above 1 at all three weights.
- **Unresolved.** Anything else. A tie is the result if it happens.
- **Incomplete.** Any comparison containing a run that did not verify.
- **Correction.** Holm over the two decision comparisons.
- **Gap to rho.** `S(arm) / S(rho_ref)` at each `w`, reported as the distance
  remaining, not as a gate.

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

1. **Before dispatch.**
   - The `xline_bin` unit tests must pass.
   - One development smoke solve per IC arm at `m = 5` must verify, and its
     per-execution time must be measured. The smoke is labelled not evidence.
   - If the projected stage-1 time is above 2700 s, run each IC arm as its own
     dispatch of the same workloads, seeds and rounds, with both rho arms and the
     A/A arm. Never reduce targets or rounds.
2. **Stage 2.** Stage 2 (`m = 7`) runs only if stage 1 completes and its measured
   times project stage 2 below 2700 s per dispatch. Otherwise it is recorded as not run.
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
