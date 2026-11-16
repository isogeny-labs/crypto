# Clearing the cofactor from the IC factor base: A/B on the census V2 curves

Hypothesis test, pre-registered in [PREREG.md](PREREG.md), frozen as `protocol-v2.json` (sha256
`ed317be30768b235d79fc6db814e20dcfc8c77606b12c927cfd88b2b6a4bca9a`, frozen 2026-11-03T02:08:35Z, before any
data) and, for its negative control, `protocol-h1-control.json` (`0740cb81...3f73`). Counted metrics only: the
host is a cloud VM graded L1.

## Result, stated against the frozen rule

| frozen criterion | outcome |
|:--|:--|
| Pass line: lower 95% limit of the S gain G_w above 1 at w = 0, 0.1, 1 | **met**: G_0 = 6.180 [5.915, 6.456], G_0.1 = 6.692 [6.403, 6.995], G_1 = 9.087 [8.674, 9.520] |
| Falsifier: point estimate of G_0 below 3 | **not fired** (6.18; the upper limit 6.456 is also above 3) |
| Control: A/A interval contains 1 | met, but uninformative (see below) |
| Control: h = 1 negative control, G_0 interval inside [0.95, 1.05] | **met**: 0.989 [0.989, 0.989] |
| Control: positive control rho-plain/rho-neg, lower limit above 1.2 | **not met** on the V2 session: 1.227 [1.149, 1.311] (met on the h = 1 session: 1.313 [1.231, 1.401]) |

**By the frozen decision rule the verdict is `invalid_controls`**, because the positive control's lower limit on the V2
session (1.149) is below the pre-registered 1.2. The rule said any failed control voids the verdict. I am not
re-interpreting it after the fact.

What can be said without overriding the rule (descriptive, not a verdict): the measured G_0 and its intervals are
as recorded above; the failed control concerns the rho arms (a negation-fold effect with a point estimate of 1.227,
close to what the control was meant to show, but a wide interval because rho's cost varies run to run), not the two
IC arms being compared; and the 1.2 line was set close to a rho effect that turned out smaller than the sqrt 2 I
assumed. A rerun with a positive-control threshold chosen for rho's actual variance, or a larger rho sample, would
settle it. That is a new pre-registration, not an amendment of this one.

## Measured values (ecbench compare, ratios of totals B/A, 448 matched pairs on 112 workloads each)

| comparison | w = 0 | w = 0.1 | w = 1 |
|:--|--:|--:|--:|
| V2 ic-raw / ic-clear | 6.180 [5.915, 6.456] | 6.692 [6.403, 6.995] | 9.087 [8.674, 9.520] |
| h = 1 ic-raw / ic-clear | 0.989 [0.989, 0.989] | 0.991 [0.990, 0.991] | 0.996 [0.996, 0.996] |
| V2 ic-raw / rho-neg (distance remaining) | 60.6 [56.7, 64.7] | 65.8 [61.6, 70.4] | 91.1 [85.0, 97.6] |
| V2 ic-clear / rho-neg (distance remaining) | 9.80 [9.33, 10.30] | 9.84 [9.36, 10.35] | 10.02 [9.49, 10.58] |

Mean S at w = 0, V2: ic-raw 76.95, ic-clear 12.45, rho-neg 1.27. Mechanism (derived): trials per relation fall from
5433 to 162, a factor of 33.6 against h = 32, as LEAD-19 predicted; at h = 1 the two arms have identical trials and
relations, and the whole 1.1% cost difference is set-up. The cleared base is still about 10 times the matched rho
in counted S: this is a measured improvement over the census `ic` baseline, not a win over rho.

Full numbers, the figure and the PDF: [reports/run-20261103T0220Z/report.pdf](reports/run-20261103T0220Z/report.pdf).

## What is new and what is not

- The treatment is not new code. `prime-projected-abscissa` (`src/cryptanalysis/ic_framework/plugins.rs`,
  `PrimeProjectedAbscissaBase`, tests `projected_prime_tests`, 2 pass) already existed; the brief's `clear=1` option
  was replaced by it. No source changed.
- The same lever was already measured on random-binary curves (LEAD-19; atlas `LEAD-ICB-cofactor-projected-ladder-base`:
  1.29x, 1.58x, 3.36x at h = 2, 2, 6). This study adds the prime family at h = 32. It does not test the fibre-aware
  oracles (`fiber.rs`), which recover the same yield from the oracle side.

## Deviations and problems (all of them)

1. **A/A control is vacuous for noise.** ecbench gives the arms of one (workload, round) the same algorithm seed, so
   `ic-raw-aa` is bit-identical to `ic-raw` (ratio exactly 1.000 [1.000, 1.000]). The protocol described it as having
   its own seeds; that was wrong. It shows determinism, not sampling noise; the interval of the main comparison (workloads
   as clusters) is what carries the sampling uncertainty.
2. **Seed label error, protocol frozen with it.** I labelled `seed_derivation.method` as `explicit_list`; ecbench derives
   seeds itself (`harness_drawn`). `check-export` therefore reports 2240 "execution cell is outside the protocol-derived
   seed table" findings on each export and nothing else (checked: against a scratch copy with only that label corrected
   the sole remaining finding is the intentionally stale hash). The tool refuses to freeze a protocol with a post-data
   amendment, so version 2 (frozen before data) stays the record and the finding is reported, not hidden.
3. **Freeze flag omitted once.** Version 1 was frozen without `--filed-before-affected-data`; version 2 (amendment A1)
   fixed it before any data. Version 1 files: `frozen-versions/`.
4. **First run attempt refused, no data.** Card 1 stopped at command 5: `host is not quiet (PSI cpu some avg10 5.23 > 5.00)`
   (load from my own compile). Card 2, identical but for its id, produced both sessions. Evidence: `cards/`,
   `cards/attempt1-host-busy/`.
5. **Pre-data evidence is not a git commit.** The binary does not embed its commit, so ecbench records the worktree HEAD
   at run time; sessions had to run at `11173ecd` with this directory untracked. The ordering evidence is the freeze
   timestamp and the S3 checkpoints the run gate pushed (`checkpoints/crypto/claude__ic-cofactor-clear-ab-20261103/`).
6. **Relay not checked verbatim.** The director's assessment found the commands executed as specified, but
   `task_card.py check-relay` was not run (no saved executor text).
7. **Timeout caps on the card** (1,680,000 s per run/verify command) were chosen by the director from the per-execution
   limit, not stated by any source; each execution still had its 600 s limit.
8. **Population overlap.** The V2 curves are the census curves (seen with the raw arm before); targets and seeds are new.
   The relayed Python estimate (post hoc, not seen by this session) is not used.

## What is missing for any leaderboard or benchmarks row

No replay certificate from a second host (only same-host `verify --replay-all`), no validator report, no matched-rho
`VALIDATION.json` with the section 5a band across independent runs. Per the validation gate no row is written; no
leaderboard or `benchmarks/README.md` file is touched.

## Files

`PREREG.md`, `protocol-v2.json`, `protocol-h1-control.json`, `analysis.sh` (frozen, hashed), `specs/`, `cards/` (task
cards, receipts, executor logs), `sessions/{v2,h1}/` (ecbench sessions and `analysis/` outputs; large data files are in the
artifact store, see `.artifacts.json`), `exports/` (experiment.export/v1, large), `reports/run-20261103T0220Z/`
(report, figure, PDF), `frozen-versions/`.
To re-check: pull the data with `python3 tools/artifacts/artifacts.py pull research/ic_cofactor_clear_ab_20261103`, then
`ecbench verify --dir sessions/v2 --replay-all` with the binary of sha256 `b38cffb4...81ec4` built from `11173ecd`.
