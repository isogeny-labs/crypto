# Phase 3 protocol: does index-calculus relation yield depend on the isogeny class or the curve, over F_{2^31−1}?

Status: **DRAFT, not yet frozen** (2026-11-12). It is frozen, and this line changed, once §8 is filled from the
timing pilot and before any study data.

Requested by the user on 2026-11-12. Of the next phases offered after phase 2, the user chose "IC yield across
classes". This protocol follows `../phase2/PROTOCOL.md` and reuses the P2 instrument of
`research/isogeny_class_ic_yield_20261102/p2_prime_analogues` unchanged.

Classification: a null-hypothesis characterization (controlled-experiment skill). It does not change the IC
baseline, has no rho row and adds no leaderboard or `benchmarks/README.md` row. Findings are [pcc:unverified].

## 1. Question

Rho cost depends only on `r` and `Aut(E)` (phase 2). Index-calculus relation yield is different: it depends
on the equation, because the factor base is the set of points of smallest abscissa. Over the 48 prime-order
classes of phase 2, does the yield per decomposition attempt, normalized by its prediction, depend on any of:
- (a) the class, through class size `H`, conductor `f`, number of endomorphism rings or `|D_K|`;
- (b) the curve inside a class (the verified isogenous curves of the phase-2 walks)?

The preregistered expectation is no for both. P2 found no curve effect inside two conductor-1 classes at
24–32 bits, and this protocol extends that across 48 classes and every ring structure.

## 2. Instrument and metric

- **Instrument**, unchanged from P2:
  - `p2tool spec`: fixed-|F| rule. The factor base is `prime-abscissa:size=N` on every curve of a class, with
    N a function of `n` and the target yield only.
  - `ecbench run`: arm `ic.pipeline`, oracle `mitm:m=2` or `mitm:m=3`, linalg `incremental-gauss-full-rank`,
    so each run makes exactly its fixed number of attempts.
  - `ecbench verify --replay`.
  - `p2tool analyze`: per class, it gives the ICC with a permutation p, max |z|, the planted +20% positive
    control, round-halves A/A and the isomorphic-model RMS.
- **Primary metric:** for each curve, relations ÷ attempts ÷ the predicted yield. The prediction is
  `2N²/n` for m = 2 and `(4/3)N³/n` for m = 3. Relations and attempts are exact counts.

## 3. Population

- The 48 prime-order classes (S1, S2, S3) of `../phase2/manifest.json`, with every curve measured in phase 2:
  332 curves, each reached by an `isoclass-rho` walk that checked every edge. Phase 2's R and J0 classes are
  not prime-order, which the P2 instrument requires, so they are excluded.
- **Generator.** Each curve's generator is the instance point `P` that `isoclass-rho` carried to it. Before
  use, `fp31-class-census ic-classes` checks that `P` is on the curve, that `n·P = O`, and that the j-invariant matches.
- **Isomorphic models.** 2 u-scaled models per class (96 in total), seeded by `ic-classes --seed 20261112`.
- **Input file:** `phase3/classes.json`, with its sha256 in `PROTOCOL.sha256`.

## 4. Per-class commands (frozen arguments, `run_phase3.sh`)

For every class `c` and `m ∈ {3, 2}`, m = 3 first:
```
p2tool spec --classes phase3/classes.json --class c --m m --y Y --rounds 2 --targets 2 --aa 0 --isolation L1 --seed 20261112 --out runs/c_mM/spec.json
ecbench run --wait --allow-busy --spec runs/c_mM/spec.json --out runs/c_mM/session
ecbench verify --dir runs/c_mM/session --replay 4 --out runs/c_mM/audit.json
p2tool analyze --classes phase3/classes.json --class c --m m --session runs/c_mM/session --out runs/c_mM/analysis.json
```
- Y is the target yield, 0.02 for m = 2 and 0.1 for m = 3, as in P2.
- At n ≈ 2^31 the table cap makes |F| = 1448 for m = 2 in every class, so the predicted yield differs between
  classes by less than 10⁻⁴.
- Relations per run are min(200, 0.6|F|) = 200, giving 2 rounds × 2 targets × 200 = 800 relations per curve.
- A class whose session fails is recorded as missing and is not replaced.

## 5. Tests (computed by `fp31-class-census ic-analyze`, frozen)

For each m, on the 48 classes. The family has 7 tests per m and 14 overall, at Bonferroni α = 0.05/14:
- **T1** Kruskal–Wallis across classes, on each curve's yield ÷ predicted.
- **T2–T5** Spearman correlation of the class pooled yield ÷ predicted against `log2 H`, `log2 f`, number of
  rings and `log2 |D_K|`. The p-values come from 20,000 seeded permutations.
- **T6** Within class: each class's p2tool ICC permutation p. Classes with at least 2 curves are tested for
  uniformity with one-sample KS.
- **T7** Within class: a binomial over-dispersion test. The curves' relation counts are compared with each
  class's pooled rate, using a Pearson χ² pooled over classes.

**Controls**, reported from p2tool per class and pooled:
- **Planted +20% arm:** factor base `N·1.2^{1/m}`. The median pc/main ratio is expected to be 1.2, and its
  detection rate is reported. This is the detector check.
- **Isomorphic models:** the RMS log ratio. This is not an A/A control: a u-scaling changes which points have
  small abscissa, so it measures presentation noise.
- **Round-halves A/A:** the SD of the log ratio.

## 6. Decision rules

- **Null supported (per m).** All of T1–T7 have p > 0.05/14, and the planted ratio's mean over classes falls
  in [1.12, 1.28].
- **Detector failed.** The planted ratio falls outside [1.12, 1.28]. No null is claimed.
- **Effect suspected.** Any of T1–T7 has p ≤ 0.05/14. The effect is reported as suspected; a preregistered
  follow-up on fresh seeds is required before any claim.

## 7. Execution route and isolation (deviation from the default, recorded)

As in phase 2 §7: there is no AWS identity in this session, and the EKS dispatch only runs ecbench specs from
`main`. So this runs locally under `--allow-busy`, and ecbench grades isolation per row. Relation and attempt
counts are exact and do not depend on host load. Wall times are descriptive.

## 8. Timing pilot and frozen budget

TO BE FILLED FROM THE PILOT, then this file is frozen and hashed.
