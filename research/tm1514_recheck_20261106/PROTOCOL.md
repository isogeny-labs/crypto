# tm1514 recheck at k = 24: protocol narrative

> Superseded in part by PROTOCOL-AMENDMENT-1.md (counted-only, local L0, replicates A3/B3); where they disagree the amendment governs.

Protocol `XP-20261106-tm1514-recheck`, version 1, status **draft** (not
frozen). The machine-readable record is `PROTOCOL.json`
(experiment.protocol/v1). Where this text and `PROTOCOL.json` differ,
`PROTOCOL.json` governs. Study directory once committed:
`research/tm1514_recheck_20261106/`.

## Question

The earlier five-curve k = 24 session (`research/curve25519_autolab_20261104/multicurve`,
session ECBS1h21e3dc5b9123, claim PCC-20261105-37a78d, standing unverified)
measured the ledger incumbent `inc` against `rho-neg` with 24 solves per arm
per curve. On `icv1-fp24-tm1514-dce9313c` (p = 16776317, the fifth curve of
`spec-k24.json`) it reported an inc/rho-neg ratio of mean S of 1.52 at w = 0
and 0.95 at w = 1. On the four siblings the ratio at w = 0 was 1.73 to 2.10.
This study asks two things:

1. Is 1.52 a small-sample draw, or does tm1514 sit below its four family
   siblings when it is re-measured with many fresh targets?
2. What does one unit of the native work that the §5a band prices at w
   actually cost, relative to one group addition, on these curves and this
   host class?

## Claims and what each outcome licenses

- **C0 (exact computation).** Every measured row recovers the logarithm of
  its public target (`[d]G = Q`, checked by the harness). Twin arms that
  share seeds reproduce their partner's counts exactly.
- **C1 (comparative).** On tm1514 the inc/rho-neg ratio of mean S is, or is
  not, below the pooled ratio of its four siblings, at w = 0, 0.1 and 1.
  Supported: tm1514 is lower than its siblings on this instrument and design.
  That is a curve-level difference within this family, not a statement about
  other curves or sizes. Not supported: the earlier 1.52 is consistent with a
  small-sample draw.
- **C1b (comparative).** The fresh tm1514 ratio at w = 0 is, or is not,
  compatible with the earlier 1.52.
- **C2 (positive control).** rho.plain costs more S than rho.negation on
  every curve, near sqrt 2, so the instrument resolves a known method effect
  at this n.
- **C3 (comparative, timed).** Measured at L2, one unit of each arm's priced
  native work (rho-neg: one negation canonicalisation; inc: one priced
  relations-phase native operation, almost all walk-table lookups) costs
  w_meas group additions, with a 95% interval.
- **C4 (comparative).** At the measured prices, each arm at its own price and
  both at the larger of the two, the tm1514 inc/rho-neg ratio is, or is not,
  above 1.
- **C5 (out of scope).** Nothing here is a security statement about
  Curve25519 or a claim about sizes other than k = 24. A host-class price is
  not a portable constant.

Hypotheses written before data: H1, the 1.52 is a draw (P1: D(0) interval
contains 0 or lies above it; P1b: the fresh R(0) interval lies above 1.52).
H2, the measured price is well below one addition (P2, P3: the 95% upper end
of w_meas is below 0.5 per canonicalisation and per lookup; P4: the
R(w_meas) interval lies above 1). Both hypotheses can fail.

## Arms

All arms run in the same sessions, from one commit and one static musl
`ecbench` binary built by `isolab-experiment.yml`, single-threaded.

| arm | role | method |
|:--|:--|:--|
| `rho-neg` | reference (matched rho) | `rho.negation` |
| `rho-neg-aa` | A/A twin of rho-neg | `rho.negation` |
| `inc` | baseline (ledger incumbent) | `ic.pipeline`, `prime-projected-abscissa:size=2,scan_cap=32`, `wwalk:x=0.9,g=1,grow=2,seed=run`, `stride-generator` |
| `inc-aa` | A/A twin of inc | as `inc` |
| `rho-plain` | positive control, add-price reference for C3 | `rho.plain` |

The five curves are the `prime_explicit` curves of `multicurve/spec-k24.json`,
byte for byte: tm1514 is the subject and tm1578, tm2338, tm3530 and tm7826 are
the negative-control population.

## Controls

- **Negative control.** The four sibling curves of the same family rule
  (largest p = 5 mod 8 below 2^24 and the next primes down, smallest valid A
  up to 4000). The generator was fixed by the earlier study before its data.
  They are matched on field size, family rule, p mod 8, cofactor-8 order
  shape, the target law, the arms, the seed rule and the instrument. Four
  curves cannot rank tm1514 as unusual: the best attainable rank p is 1/5.
  C1 therefore compares tm1514 with the pooled sibling ratio, and the
  between-sibling spread is reported beside it. A population claim would
  need a larger, newly frozen sibling set under a separate protocol.
- **Positive control.** rho-plain against rho-neg. Expected detection: the
  pooled rho-plain/rho-neg ratio of mean S at w = 0 has a 95% interval whose
  lower end is at least 1.20 (the asymptotic sqrt 2 = 1.414 is diluted by
  fixed set-up and distinguished-point overheads at r near 2^21), and every
  per-curve point estimate is above 1.10. If it misses, the study is invalid.
- **A/A, two kinds.**
  - Seed-independent A/A (noise floor for C1): replicate A against
    replicate B. The specs are identical except `measurement.seed`, so every
    (workload, measured round) has the same target and an independent
    algorithm seed. The analysis bootstraps `ln R_A − ln R_B` per curve.
  - Same-session twins (`rho-neg-aa`, `inc-aa`). Within one session every
    arm of a (workload, round) gets the same algorithm seed, so a twin
    repeats its partner exactly. The twins test determinism (C0) and give
    isolab its wall-clock A/A admission. They are never a variance estimate.

## Seeds

Each seed is `int(sha256(D)[0:16 hex]) mod 2^53` with
D = `crypto:research/tm1514_recheck_20261106:` followed by the suffix:

| suffix | value |
|:--|--:|
| `measurement.seed:A` | 960559345608509 |
| `measurement.seed:B` | 4971825529176011 |
| `workloads.target_seed` | 1406057239440944 |
| `analysis.bootstrap_seed` | 244596609222896 |

They were fixed before any data of this study. None reuses 20261324 or
20261325 of the earlier spec. Per-execution algorithm seeds are derived by
the harness from `measurement.seed`, the workload and the round, and are
recorded in every record.

## Design and sample size

Per replicate: 5 curves × 96 public targets × (1 warm-up + 6 measured rounds)
× 5 arms = 16,800 executions, order `alternate_measured` (spec order on even
measured rounds, reversed on odd), isolation L2. Two replicates give 1,152
measured solves per arm per curve (576 per replicate). The brief's floor was
256 per arm on tm1514.

Power note. The bootstrap resamples targets within each curve, all twelve
(replicate, round) cells of a target together, B = 10,000, percentile
intervals. If per-solve S has a coefficient of variation near 0.5 for each
arm, the standard error of a per-curve mean over 1,152 solves is about 1.5%,
the 98.33% half-width of a per-curve ratio is about 5 to 6%, and the
tm1514-versus-siblings difference resolves about 0.1 in ratio, against an
observed gap of 0.2 to 0.6. That CV of 0.5 was an assumption from rho
theory.

Development estimate, not data: a local, non-isolated L0 run of reduced
copies of both specs (48 targets per curve, 2 measured rounds, 960 measured
solves per arm per replicate pooled over the five curves, frozen seeds
unchanged) gave a per-solve CV of recorded S of 0.48 for rho-neg, 0.50 for
rho-plain and 0.24 for inc. At these CVs and 1,152 solves per arm per curve:

- the standard error of ln R on one curve is about 1.6%;
- the 98.33% half-width of D is about 0.04 in ln;
- the 95% half-width of the tm1514 seed A/A, `ln R_A − ln R_B`, is about
  0.06.

C1 therefore needs |D| above roughly 0.06, which is about 6% in ratio. The
gap the earlier session reported, ln(1.52/1.9), is about −0.22. Scaled from
the development run (192 solves per arm per curve) to the design, the
bootstrap half-widths agree: 0.123 × sqrt(192/1152) ≈ 0.050 for the A/A and
0.096 × 0.41 ≈ 0.039 for D.

## Metrics

Counted metrics decide C0, C1, C1b, C2. Every ratio is a ratio of summed
priced S over (workload, measured round) pairs in which both arms verified:
`priced_S(w) = (G − C + w·(U + n_C)) / sqrt(r)`, computed by
`ecbench::band::priced_s`. Siblings are pooled by summing over all four
curves' pairs.

C3 is timed, and wall time enters only from L2 records of a replicate whose
wall A/A was admitted (`control-comparison.json`: status admitted, aa_pairs at
least 5, no pair below the level):

- `w_meas_canon`: per (curve, replicate), least squares
  `solve_wall_ns = c + t_add · total_gae` over rho-plain rows. The intercept
  absorbs per-solve fixed cost. Then
  `t_canon = Σ(solve_wall_ns − c − t_add · total_gae over rho-neg) / Σ canonicalisations`
  and `w = t_canon / t_add`, pooled over strata. It is repeated with
  `time.hw_solve` cycles where every row has them. Per-step hashing and the
  distinguished-point test are uncounted on both rho arms and are absorbed
  into t_add.
- `w_meas_lookup`: on inc,
  `Σ(relations wall_ns − t_add · relations gae) / Σ(relations-phase natives that the band prices)`,
  divided by t_add of the same stratum. No intercept is fitted there.
- `w_insert` (reported only): the oracle_setup phase over
  `work_native.table_insert_attempts`, which the band does not charge.

## Decision rules (verbatim from PROTOCOL.json `decision_rule.text`)

> C1: compute D(w) = ln R_tm1514(w) - ln R_sib(w) with a paired bootstrap over targets (stratified by curve, replicates A and B pooled as rounds of the same targets), 98.33% interval (Bonferroni over w in {0, 0.1, 1}). Verdict at each w: 'tm1514 lower than siblings' if the interval lies below 0 and |D| exceeds the half-width of the seed-independent A/A interval of ln R (C-aa-seed-inc combined with C-aa-seed-rho); 'tm1514 higher' if above 0 by the same margin; otherwise 'within the sibling ratio (draw not excluded)'. The interval-non-overlap reading of the brief (tm1514's R interval entirely below the pooled sibling interval) is reported as a secondary, more conservative line. C1b: 'the earlier 1.52 was a draw' if the 95% interval of R_tm1514(0) excludes 1.52; 'compatible with 1.52' otherwise. C2: positive control detected if the pooled rho-plain/rho-neg 95% interval has lower end >= 1.20 and every per-curve point estimate > 1.10; otherwise the study is invalid. C3: w_meas with 95% interval per unit; H2's price prediction holds for a unit if the upper end is below 0.5. C4: 'the w = 1 flip does not survive measured pricing' if R_tm1514(w_meas) has a 95% interval above 1 under both pricing variants; 'unresolved at measured pricing' otherwise. The 5a band verdicts at 0 / 0.1 / 1 are always reported; the measured point never replaces them.

Verdict vocabulary: "tm1514 lower than siblings", "tm1514 higher than
siblings", "within the sibling ratio (draw not excluded)", "the earlier 1.52
was a draw", "compatible with 1.52", "positive control failed - invalid",
"measured price below 0.5", "measured price not resolved below 0.5", "the
w = 1 flip does not survive measured pricing", "unresolved at measured
pricing", "invalid_measurement".

How the frozen analysis reads the rule where the text leaves a choice:

- The A/A half-width for C1 at weight w is half the width of the 95%
  bootstrap interval of `ln R_tm1514,A(w) − ln R_tm1514,B(w)`, the same
  resamples as D. The pooled five-curve A/A interval is reported beside it.
- C3's verdict needs at least one replicate with admitted wall. Otherwise C3
  and C4 are `invalid_measurement`.
- A study whose C0 or C2 fails reports every other verdict as
  `invalid_measurement`. The numbers are still written, as descriptive.

## Analysis

The frozen script is `examples/tm1514_analyze.rs` (path and SHA-256 in
`PROTOCOL.json` `analysis_plan.script`). It reads both sessions through the
crate's record types (`runner::read_record_lines`, `Record`, `Session`). It
checks each record's seal, the records hash in `session.json`, session
completeness, the frozen spec ids, the arms' methods and parameters, that A
and B cover the same (workload, round) cells, and that A and B never share an
algorithm seed. It prices through `band::priced_s` and resamples with
`stats::Resampler`.

Arm names are replaced by codes derived from a key (SHA-256 of the protocol
id and both records hashes) before any statistic is computed. Names are
restored only in the output, after every verdict exists. The script writes
`analysis/result.json` and `analysis/summary.txt` and is deterministic for
fixed inputs.

The `ecbench band`, `compare` (including `--b-dir` for the seed A/A) and
`work` outputs listed in `analysis_plan.outputs` are cross-checks, not the
decision.

## Invalidation rules

- A replicate whose audit is not ok, whose replays do not all reproduce,
  whose L2 gate fails, or whose wall A/A is not admitted is invalid for C3
  and reported. Its counts may still serve C1 and C2, but only if the audit
  and replays pass.
- A twin pair whose counts differ on any (workload, round) invalidates C0
  and the session.
- A preempted or killed job has no session and contributes nothing. It is
  reported and charged.
- Exhausted or timed-out rows are kept as right-censored rows and reported
  per arm. A curve whose arm has any censored row is reported with and
  without them and cannot decide C1 alone.

Stopping: the design is fixed at two replicates. There is no third
replicate and no extra targets after a look; a larger run is a new protocol
version. The study stops as invalid if C2 fails, and no further job is sent
once the dollar ceiling in `PLAN.md` is reached. Zero automatic retries.

## Not admissible

- Changing targets, seeds, arms, rounds, weights or thresholds after seeing
  either replicate.
- Dropping a curve, a replicate, a censored row or a failed job.
- Reading wall time below L2, or from a session whose wall A/A was not
  admitted.
- Using w_meas in place of the 0 / 0.1 / 1 band, or pricing the arms at
  different weights without also reporting the common-weight variant.
- Quoting the twins as a variance estimate.

## Out of scope

- No security statement about Curve25519 or any deployed curve. The targets
  are public synthetic targets.
- No claim at any size other than k = 24, and no claim about curves outside
  this family.
- No population claim about tm1514 being unusual. Four siblings cannot
  support one.
- No repricing of the band. Two asymmetries are reported beside C3 and are
  not repriced, because that would be an accounting change:
  - inc's walk-table inserts sit in `work_native`, which the band does not
    read;
  - rho-neg's per-step partition hash, distinguished-point test and table
    work are not counted at all.
- No IC leaderboard change. A re-measurement of the incumbent is not an
  improvement.
