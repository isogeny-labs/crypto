# Prime medium-tier rho reference bound: results

Date: 2026-11-08. Lead: `LEAD-RHO-matched-rho-calibration`. Protocol:
`PROTOCOL.md`, frozen as `protocol_sha256 5643e3e7…4cd6` before any data. Every
number here is `[pcc:unverified]`: it comes from the committed tool outputs
named beside it, and no independent validator has re-derived it yet.

## Verdict (PROTOCOL.md §9, applied as written)

**`inconclusive`.** All six audits pass, but the positive control P4 misses
its preregistered threshold in every job.

- **M** is the `rho.negation` medium constant over the √(π/4) floor at
  w = 0, from `docs/bounds/records/prime-medium-rho-neg.json`. It is
  **1.093 [1.033, 1.156]** (95 %, cluster).
- **On M alone, the row is `setup_excess`.** M ≤ 1.10 and its upper end is
  under 1.30. The stricter `within_1.10_strict` line is not met, because the
  upper end is above 1.10.
- **P4 decides the verdict instead.** P4 is the paired `rho-plain / rho-neg`
  S ratio, preregistered as "1.41, interval excluding 1.2 from below". It is
  `sessions/Jn/comparisons/rho-neg__rho-plain.json` at `.ops`. §9 says the
  verdict is `inconclusive` when P4 fails.

| job | workloads | ratio | 95 % interval | excludes 1.2 from below |
|---|---:|---:|---|:--:|
| J1 | 48 | 1.285 | [1.177, 1.404] | no |
| J2 | 16 | 1.286 | [1.094, 1.512] | no |
| J3 | 4 (indicative) | 1.352 | [0.909, 2.009] | no |
| J4 | 4 (indicative) | 1.414 | [0.688, 2.906] | no |
| J5 | 4 (indicative) | 1.663 | [0.986, 2.805] | no |
| J6 | 4 (indicative) | 1.217 | [0.737, 2.011] | no |

The protocol does not say how to pool P4 across jobs, and no tool pooled it.
No pooling was chosen after seeing the data, so the verdict stands at
`inconclusive`.

In J1, the largest job, the control detects `rho-plain` above `rho-neg`, with
an interval that excludes 1. Its lower end of 1.18 falls just short of the
preregistered 1.2.

## Everything else, as measured (counts; no wall time is admissible, §10)

| arm | constant / floor, w = 0 | S, w = 0 | exponent α |
|---|---|---|---:|
| rho.negation | 1.093 [1.033, 1.156] | 0.969 [0.913, 1.026] | 0.497 |
| rho.negation, w = 0.1 | 1.203 [1.136, 1.271] | 1.066 [1.004, 1.129] | 0.497 |
| rho.negation, w = 1 | 2.185 [2.063, 2.310] | 1.937 [1.825, 2.052] | 0.497 |
| rho.plain | 1.433 [1.343, 1.522] | 1.270 [1.194, 1.354] | 0.506 |
| bsgs.negation | 1.111 [1.032, 1.185] | 0.984 [0.916, 1.048] | 0.500 |
| rho.negation_v2 | 1.069 [1.003, 1.138] | 0.947 [0.886, 1.005] | 0.491 |

Every bound is admissible, at level `exponent`.

### The other preregistered predictions

- **P2 holds:** `rho.plain` medium R = S / 1.2533 ≈ 1.014, inside
  [1.00, 1.10].
- **P3 holds:** `bsgs.negation` S has an interval that contains 1.
- **P7 holds:** the A/A paired counts are exactly 1.0000 in every job.
- **P8 fails narrowly at J1** (corrected 2026-11-08, validator finding D1):
  `rho-neg-v2 / rho-neg` is 1.002 [0.934, 1.074]. The interval contains 1,
  but its upper end 1.074 is outside the preregistered [0.93, 1.07]. This
  file first said "P8 holds", which was wrong. P8 does not enter the §9
  verdict.

### R(N) fit (`analysis/rfit-rho-neg.json`; figure `analysis/rho-neg-R-of-N.{svg,pdf,png}`)

- **Model A** (`1 + c/N^β`, N = √r) fits: χ² = 13.2 on 14 dof, p = 0.51,
  β = 0.58 [0.21, 1.89].
- **Model B** (`R∞ + c/N^β`) gives R∞ = **1.084 [1.020, 1.142]**, p = 0.96.
- **Means:** the weighted mean R is 1.083 (se 0.029) over the 10 medium
  curves, against 1.380 (se 0.069) over the 6 toy curves.

These fits sit beside the verdict and do not override it. Model B's R∞
interval excludes 1, and P6 (R∞ inside [1.00, 1.10]) is met on the point
value only.

## What it means for the IC gaps (derived, conditional on the numbers above)

Toy-tier prime rho sits about 1.38× the floor, and medium-tier rho about
1.08–1.09×. A prime IC gap normalised to a toy rho is therefore understated
by roughly 1.27× (1.38 / 1.09) relative to a medium-tier rho, which is the
lead's "gaps grow". Gaps measured against a same-session matched rho are
unchanged.

The verdict is `inconclusive`, so this is reported and not promoted to a
leaderboard row. It also needs a `VALIDATION.json` with a replay
certificate, a validator report and a matched-rho reference before any row.

## Execution record and caveats

- **Jobs:**
  - J1: 1,680 executions.
  - J2: 560 executions.
  - J3 to J6: 140 executions each.
  - Every job ran once, on c8a.2xlarge under L2, with the frozen image
    `isolab/tasks@sha256:292bdc3d…cae5`. The ecbench sha256 is
    `a18af09f…d3b9`, and the source is `dbd587a01a`.
  - Every pod exited 42 (L2 wall gate).
  - Every session was complete and audited: `verify --replay-all`, 0
    problems, every replay reproduced.
  - All six are admitted for counts under §10, so no wall time is reported.
- **Provenance:** the sessions embed `git_commit e374755b`. That is
  `GITHUB_SHA` of the image workflow run on `main`, which ecbench falls back
  to when `CRYPTO_BUILD_GIT_COMMIT` is unset. The build checked out
  `dbd587a01a` (`receipts/image-publication.json`), and the binary sha256
  matches.
- **Launch history:** cards -1 to -6 are in `cards/`, with assessments.
  Execution was interrupted three times by host problems:
  - aws-cli v1 ignored `AWS_REGION`;
  - the executor's 600 s shell limit cut a run short;
  - an idle container killed a detached client.
  No job was relaunched; each interrupted launch was finished with
  `wait_launcher_result.sh`. The benchmark record has no launch entries for
  J2 and J4, which were lost when their clients were killed.
- **Export:** the `experiment.export/v1` export was not produced. In the
  director's trial, `from-ecbench` + `check-export` on J3 gave 132 findings.
  The protocol plans 2,400 rows in one export, and the tool exports one
  session at a time. That is a tooling gap, recorded here.
- **rfit binary:** sha256 `7457cb7c…` was built before a layout-only rustfmt
  of `examples/prime_medium_tier_rho_rfit.rs`.

## Follow-up: P4 confirmatory study (2026-11-08)

A separate confirmatory protocol was frozen before any data: `research/prime_medium_rho_p4_confirm_20261108/` (protocol sha256 `26fd5c57…`). It ran ten jobs and 2,160 pairs.

- **Frozen verdict: `inconclusive`.** Gate G3, the positive control, failed. `bsgs.textbook/negation` is designed to read 1.5 and read 1.464, with a 99% interval of [1.438, 1.494].
- **Primary estimate, descriptive only:** the pooled `rho-plain / rho-neg` ratio is 1.362 [1.320, 1.405].
- **Detail:** see that study's `RESULTS.md`. Its exploratory analysis traces the positive-control shortfall to an unusually high draw of planted targets. That explanation is a hypothesis.

`[pcc:unverified]`
