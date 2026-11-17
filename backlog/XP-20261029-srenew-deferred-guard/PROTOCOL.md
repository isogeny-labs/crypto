# Prime-field IC: deferred-guard stride-renew with a smaller factor base — protocol

## 1. Question and class

`targets = stride-renew` probes the repeat guard on every trial: one hash
insert per trial, priced as a lookup. In the stride study
(XP-20261029, `research/ic_stride_renew_prime_20261029`) the mean `srenew`
relations phase at 28 bits recorded 93,544 trials, 93,544
`target_guard_probes`, 93,514 `lookups`, 30 `repeated_targets_skipped` and
203 relations (*recorded*, 288 measured runs).

`targets = stride-renew-deferred-guard` (`TargetSource::StrideRenewDeferredGuard`,
PR #2831) probes only targets that decompose. Its target stream, rows and answer
are those of `stride-renew`. Per non-decomposing repeat it costs one lookup, and
per decomposing repeat it costs one confirming addition. The unit test
in `ic_framework/mod.rs` pins this.

At a fixed N, the change removes about one native operation per trial and no
charged work. A lone comparison therefore ties at w = 0 and reads "unresolved"
under the §5a rule. The counted cost has two terms, both linear in N:
- the oracle set-up, `N(N+1)` additions;
- the relation search, about `r/(c·N)` trials at one addition each, plus the
  native operations per trial.

At w = 0 the optimum N therefore lies below the incumbent at 24–32 bits. The
eager guard's native probe is what pushes the optimum back up as w grows. With
the probe deferred, a smaller N should lower S at every weight.

Question: at each size b ∈ {24, 28, 32}, does `stride-renew-deferred-guard` at a
smaller factor base N_b lower the whole-pipeline counted cost S of the
prime-field IC baseline at that size across the §5a band? The oracle is
unchanged. Class: engineering. Each size is decided on its own. No exponent
claim.

20 bits is not tested. The model below predicts a w = 0 gain of about 1% at the
w = 0 optimum (N ≈ 50 against the baseline's 45), which is below this design's
resolution.

## 2. Arms (one session per size; `spec-P<b>.json`)

The baseline is the current leaderboard baseline at each size:
- 24 and 32 bits: `walk`;
- 28 bits: `stride-renew`, from XP-20261029 (PR #2826).

| arm | role | targets | factor base | decision-bearing |
|:--|:--|:--|:--|:--|
| `walk` | baseline at 24/32 | `walk` | N_inc | yes at 24/32 |
| `srenew` | baseline at 28 | `stride-renew` | N_inc | yes at 28 |
| `dg-small` | candidate | `stride-renew-deferred-guard` | N_b | yes (the only candidate) |
| `dg` | control (factorial) | `stride-renew-deferred-guard` | N_inc | no |
| `srenew-small` | control (factorial) | `stride-renew` | N_b | no |
| `<baseline>-aa` | control | = the baseline arm | N_inc | A/A |
| `rho-neg` | reference | `rho.negation` | — | matched rho |
| `rho-neg-aa` | control | = `rho-neg` | — | A/A |

The arms not named as the baseline (`srenew` at 24/32, `walk` at 28) carry the
role `control` and are descriptive.

| b | N_inc | N_b (candidate) | max_trials (all IC arms) |
|--:|--:|--:|--:|
| 24 | 162 | 130 | 4243777 |
| 28 | 407 | 267 | 26479085 |
| 32 | 1024 | 741 | 167871873 |

How N_b was chosen, before any data:
- **Model.** Take the mean `srenew` phase costs of XP-20261029 at size b. Model
  the set-up as `N(N+1)` and the trials as `T_inc·N_inc/N`. Count 2 native
  operations per trial for the eager guard and 1 for the deferred guard.
- **Choice.** N_b is the integer N that minimizes the worst of the modelled
  ratios `dg-small / srenew(N_inc)` at w = 0, 0.1 and 1.
- **Status.** This is a design-time model, not a measurement.

The oracle is `mitm:negation_folded=1`. The other `ic.pipeline` parameters are
the defaults that `ecbench plan` prints (`linalg = incremental-gauss`).

## 3. Workloads

Each size uses two fresh prime-order curves from `prime_search`, with seeds
`20261029<b>3` and `20261029<b>4`:

| b | curves (`ecbench plan`) |
|--:|:--|
| 24 | `icv1-fp24-t553-4d349218`, `icv1-fp24-tm589-5e500858` |
| 28 | `icv1-fp28-tm13337-3a482fe3`, `icv1-fp28-tm5065-1146b4a5` |
| 32 | `icv1-fp32-t46781-77aa234d`, `icv1-fp32-tm11535-8e4578de` |

None of these curves appears in an earlier study, benchmark or leaderboard file.
The check was a `git grep` over `research/*.md`, `backlog/`, `docs/ic/*.json`
and `benchmarks/*.md`.

Measurement parameters, per size:
- 24 public targets per curve (seed `20261029<b>6`);
- 1 warm-up and 6 measured rounds, order `alternate`, seed `20261029<b>8`;
- L2 isolation and a 1800 s timeout.

That gives 2,688 executions per size and 288 matched pairs per arm pair.

## 4. Route

The image's ecbench predates the new target source, so it cannot be used. The
run uses `isolab-experiment.yml`, dispatched on `main` with `source_revision`
set to the 40-character SHA of the `main` merge commit of the PR that adds this
protocol. That commit carries PR #2831's code.
The workflow builds that commit, publishes a digest-pinned image and launches
one EKS job per size on `c8a.2xlarge` with a 3,600 s deadline.

- **Cards:** frozen task cards come from `experiment-director` and are run
  unchanged by `executor-mechanical`. Each card records the source SHA, the
  image digest and the ecbench sha256.
- **Integrity:** results are checked against each job's `SHA256SUMS`.
- **Host loss:** reruns that size's job whole under a new job name, and the
  rerun is recorded.

## 5. Unit and pricing

`S = total GAE / √r`, cold, whole pipeline. Every arm, rho included, is priced
at w = 0, 0.1 and 1 GAE per unpriced native operation (AGENTS.md §5a), using
`ecbench compare --weight W` and `ecbench band --ranks`. C = 0 on every arm.
Wall time is not a claim.

## 6. Decision rule (per size, fixed now)

`dg-small` is an **improvement** at size `b`, and becomes the prime-field IC
baseline there (targets and factor base), only if all of the following hold:

1. **Instrument:**
   - the session audit is `ok`, with every measured replay identical;
   - the A/A ratios (`<baseline>-aa/<baseline>` and `rho-neg-aa/rho-neg`)
     are exactly 1.000 at every weight;
   - the L2 gate admits every execution;
   - the worker's input spec equals the frozen spec.
2. **Completeness:** all 288 measured `dg-small` runs and all 288 baseline runs
   verified `[d]G = Q`, with 0 censored pairs.
3. **Ratio:** the 95% upper bound of `Σ S_dg-small(w) / Σ S_baseline(w)` is
   below 1 at w = 0, 0.1 and 1. The bound comes from a cluster bootstrap over
   targets, 4,000 resamples, seed 20261002.
4. **Band:** `ecbench band --ranks` gives `dg-small < baseline` on every curve
   of the size.

| outcome at `b` | reading |
|:--|:--|
| 1–4 hold | improvement; `dg-small` becomes the baseline at `b` |
| below 1 at some weights, not at others | unresolved inside the band; baseline kept |
| total falls at w = 0 and rises at w = 1 | relabelling; baseline kept |
| interval includes 1 at every weight | null at this sample size; baseline kept |
| upper bound of `Σ S_baseline / Σ S_dg-small` below 1 at every weight | regression; baseline kept |
| 2 fails | incomplete, never a win |

Every size is always reported with `<baseline>/rho-neg` and `dg-small/rho-neg`
and their intervals. This is the distance remaining.

The factorial arms are descriptive. They attribute the effect, and no decision
reads them:
- `dg / srenew` is the guard alone at N_inc;
- `srenew-small / srenew` is N alone;
- `dg-small / srenew-small` is the guard at N_b.

## 7. Predictions (before any confirmatory data; from the §2 model)

1. **Guard alone at N_inc.** `dg / srenew` is 1.000 at w = 0 within ±0.002, and
   below 1 at w = 0.1 and w = 1: modelled at about 0.95–0.97 and 0.75–0.79.
   - The tolerance at w = 0 allows for one charged addition per decomposing
     repeat.
   - The w = 0 interval may be degenerate.
2. **N alone, with the eager guard.** `srenew-small / srenew` is below 1 at
   w = 0 and above 1 at w = 1: modelled at about 0.83–0.95 and 1.10–1.12. This
   is the relabelling pattern, and the reason a smaller N alone is not a
   candidate.
3. **The candidate.** `dg-small / srenew` is below 1 at every weight. Modelled
   at w = 0 / 0.1 / 1:

   | b | w = 0 | w = 0.1 | w = 1 |
   |--:|:--|:--|:--|
   | 24 | 0.95 | 0.92 | 0.79 |
   | 28 | 0.83 | 0.82 | 0.80 |
   | 32 | 0.89 | 0.88 | 0.80 |

   Against the `walk` baseline at 24/32, multiply by the measured
   `srenew/walk` at that size.
4. **Model risk.** The trial count may not scale as 1/N, for example through
   rank deficiency at small N. If it does not, prediction 3 fails first at
   w = 0 at 24 bits, where the modelled margin is smallest.

## 8. Disclosure

No data has been taken on these curves or with this target source, apart from
the unit tests of PR #2831, which run on roster curves. The N_b values and the
predictions come only from the XP-20261029 records, which are already public.

## 9. Freezing

This file and the three specs are frozen by the commit that adds them. Each
job's request embeds its spec byte for byte, and the spec sha256 is recorded on
the launch card. No amendment is allowed after the first execution.

## 10. Outputs

`research/ic_srenew_deferred_guard_20261029/` holds, per size:
- launch receipts;
- the S3 copy with hashes;
- `compare` JSON at three weights for the decision pair, the factorial pairs,
  `<baseline>__<baseline>-aa`, `rho-neg__rho-neg-aa`, `rho-neg__<baseline>` and
  `rho-neg__dg-small`;
- the band table.

The directory also holds a README with a figure and a PDF. An improvement at any
size goes to the validation gate (an independent validator and VALIDATION.json)
before any leaderboard change.
