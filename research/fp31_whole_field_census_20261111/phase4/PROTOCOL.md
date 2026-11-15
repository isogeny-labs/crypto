# Phase 4 protocol: does a rho step cost less on some isogenous curves, for the same instance?

Status: **frozen before data** (2026-11-12T03:10Z), after the timing pilots of §8 and before any study data.

Requested by the user on 2026-11-12, paraphrased: across isogenous curves, measure the minimum and maximum cost of
solving one ECDLP instance, holding the same G, the same k and the same Q, to find whether some curve's arithmetic
costs less per step.

Classification: a null-hypothesis characterization of a constant factor. It leaves the IC baseline unchanged and
adds no leaderboard row. Findings are [pcc:unverified].

## 1. Question and expectation

For the same ECDLP instance carried by isogenies to every curve of a class (`G' = φ(G)`, `Q' = φ(Q) = [k]G'`,
the same `k`), does Pollard rho's wall time per counted group operation differ between the curves? Iteration counts
are identical in distribution (phase 2). This phase asks about the cost of one step.

**Expectation: no.**
- **The step formula.** The step is an affine addition, `λ = (y₂ − y₁)/(x₂ − x₁)`. Neither curve coefficient
  `a` nor `b` appears in it. Only doubling uses `a`, and an r-adding walk does not double.
- **No cheaper models.** Montgomery and Edwards models need `4 | #E`, and Hessian models need `3 | #E`; a
  prime-order curve has none of these.
- **Field arithmetic** is the same for every curve over the same `p`.
- **What remains** is value-dependent work, such as the inversion's operand-dependent loop count. It is the same in
  distribution on every curve.

## 2. Prior exploratory evidence: step A (not preregistered, isolation L0)

`fp31-class-census timing` reads phase 2's `isoclass-rho` timing: ns per solve ÷ iterations, for the same
transported instance on every curve. The output is `phase4/step_a_timing.json`.
- **Spread between curves.** The median spread between the fastest and slowest isogenous curve of a class is
  1.035. The same statistic over arms that are the same curve (root, A/A replica, isomorphic models) is 1.026.
- **Uniformity.** The per-class Kruskal–Wallis p-values across isogenous curves are uniform (KS p = 0.96, 53 classes).
- **Why step B.** This was recorded with a 4-thread rayon pool on a shared host. Step B is the isolated,
  preregistered measurement.

## 3. Instrument, population and design (step B)

- **Instrument.** `ecbench run`, isolation L2 required: each run is pinned to a reserved CPU, and ecbench grades
  every row.
- **Population.** The 48 prime-order classes and curves of `phase3/classes.json`: 332 walk curves plus 96
  isomorphic models, with each generator checked as described in phase 3.
- **Spec.** One spec per class, from `fp31-class-census rho-spec --targets 64 --rounds 2 --seed 20261112
  --control rho.negation`:
  - every curve of the class is a workload, with `target_kind: planted_shared`. ecbench then derives the same `k`
    list on every curve from `[r, target_seed, index]`, and `Q' = [k]G'` with `G'` the walk's transported point;
  - arms `plain` (`rho.plain`, the reference), `plain-aa` (`rho.plain` again, A/A) and `control` (`rho.negation`,
    the positive control);
  - `order: shuffle`, a seeded permutation of runs per round; 1 warm-up round.
- **Metric.** `online.wall_ns ÷ cost.total_gae` per run: wall ns per counted group-addition equivalent.

## 4. Commands (frozen, `run_phase4.sh`)

For each class `c` in `phase3/classes.json`:
```
fp31-class-census rho-spec --classes phase3/classes.json --class c --targets 64 --rounds 2 --seed 20261112 --control rho.negation --out phase4/runs/c/spec.json
ecbench run --wait --spec phase4/runs/c/spec.json --out phase4/runs/c/session
ecbench verify --dir phase4/runs/c/session --replay 4 --out phase4/runs/c/audit.json
```
Then `fp31-class-census rho-timing --classes phase3/classes.json --runs phase4/runs --out phase4/analysis.json`.
A class whose session fails is recorded as missing and is not replaced.

## 5. Tests (computed by `rho-timing`, frozen)

- **T1 (primary).** Within each class, a Friedman test across curves on ns/GAE of the `plain` arm, with blocks
  (target, round). This pairs the same `k` on every curve. The 48 p-values are tested for uniformity with
  one-sample KS, at α = 0.05.
- **T2.** The number of classes with Friedman p < 0.05/48 (Bonferroni over classes). Expected: 0.
- **Reported, not tested:**
  - per class, the fastest and slowest curve with their ns/GAE and the max/min ratio, next to the A/A spread;
  - the pooled between-curve SD of true log ns/GAE (variance components), with a one-sided 95% upper bound.
- **Controls:**
  - A/A (`plain-aa ÷ plain`, paired per run): the log ratio is expected to be 0;
  - positive control (`rho.negation ÷ rho.plain`, paired per run): the pilot read 1.073.

## 6. Decision rules

- **Null supported.** T1 has p > 0.05, T2 is 0, the A/A log ratio lies within ±0.01, and the positive control lies
  in [1.03, 1.15] with |z| > 5.
- **Detector failed.** The positive control falls outside [1.03, 1.15] or has |z| ≤ 5. No null is claimed.
- **Effect suspected.** T1 has p ≤ 0.05 or T2 > 0. The flagged curves are reported, and a preregistered follow-up
  on fresh seeds is required before any statement that a curve costs less per step.

## 7. Execution route and isolation

The run is local; this session has no AWS identity, as in phases 2–3. ecbench refuses L2 runs on a busy host, and
grades each row. Rows below L2 are counted and reported.

## 8. Timing pilots (non-study curve, excluded from analysis, not committed)

The pilots used the 16-curve walk of phase 2's pilot curve (draw 100006) plus 2 isomorphic models: 18 workloads.
- **Pilot 1**: 2 targets, 1 round, control `rho.frozen_reference`. 216/216 runs returned the planted k. The control read
  1.014 ± 0.017 (log), so it is **rejected as a positive control**: no resolvable difference.
- **Pilot 2**: 8 targets, 1 round, control `rho.negation`. 864/864 runs returned the planted k, about 17 ms per run, and 28 of
  432 measured rows graded below L2. Results:
  - control 1.073, log SE 0.011 (z ≈ 6.5);
  - A/A −0.009 ± 0.011;
  - per-run log SD about 0.13.

  At 64 targets × 2 rounds, each curve's mean has about 1.1% SE.

Budget: about 3,500 runs per class, about 60 s, and under an hour for 48 classes.

**Binaries:**
- `ecbench` sha256 `86a9421d…` is unchanged from phase 3.
- `fp31-class-census` was rebuilt for phase 4. The phase 3 sources are unchanged, but the phase 3-bound binary hash
  `54c05dc2…` no longer exists on disk. The new hash is in `PROTOCOL.sha256`.
