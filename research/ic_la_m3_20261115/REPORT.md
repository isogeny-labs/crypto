---
title: "Linear algebra for m = 3 rootfind-subtract: incremental-gauss-echelon needs 0.10–0.15× the row operations"
date: 2026-11-15
---

# Linear algebra for m = 3 rootfind-subtract at |F_x| = 2048 and 4096

User request (2026-11-15): "Keep working on open leads". After round 4 of the GPU rootfind work
(`research/ic_gpu_rootfind_pairs_20261115`), linear algebra was the largest part of an m = 3 solve at
|F_x| = 4096: about 0.1–0.34 G `row_ops` a run, unpriced in S (w = 0), plus 0.5–1.5 s of wall.
*Corrected* (validator): in this run's gauss arm at 4096 linear algebra is the largest part of the band
at w = 1 (71–88%) but at w = 0.1 in only 2 of 6 runs, and never the largest phase in wall time here.

**Standing.** `[pcc:unverified]`: one local run in this container (4-core Intel Xeon VM, L0,
`--cpus none --allow-busy`), measured by the session that wrote the protocol; no validator yet. The
`row_ops` are deterministic counts, so the count comparison holds at L0; the wall is indicative only.
S at w = 0 is unchanged by design. No scoreboard or leaderboard row is added. An independent validator
(`VALIDATION_REVIEW.md`) re-verified all 42 certificates with its own code, recomputed every `row_ops`
and wall figure, matched the 24 pairs on every phase but linear algebra, and re-implemented both
solvers (pins equal row for row): **validated, scoped**. Its §5a finding changed the band numbers,
corrected below (*corrected*).

![LA row_ops per solver, [pcc:unverified]](fig_la.svg)

## Protocol

`PROTOCOL.md` and `specs/la.json`, committed and pushed before the run (`9f8feeace6`): the same
`rootfind-subtract:m=3` pipeline (prime-abscissa base of 2048 or 4096 abscissae,
`walk-prime-batch-1024` targets) on the prime_explicit curves of 34, 36 and 38 bits, 2 targets each,
one round, with only the relation matrix varied: `incremental-gauss` (the default every earlier round
used), `incremental-gauss-echelon` (sparse forward elimination) and `structured-gauss` (sparse reduced
echelon, lightest-column pivot); `rho.negation` as the matched reference. The three solvers pin the
target under the same completion rule, so trials, relations, the logarithm and S must agree per
workload.

**Sources checked.** The shared research-kb is not reachable from this container (no
`crypto-autoresearcher` checkout or catalog); recorded as a gap. Repository notes:
`research/ic_gauss_echelon_20261027/README.md` (`incremental-gauss-echelon`: same pins, rank and
logarithm as `incremental-gauss` with 0.59–0.61× its `row_ops` at |F| = 30–42) and
`research/ic_linalg_pipeline_20261019/README.md` (`sparse-cado`, out of scope: it needs the
full-rank completion rule, which changes trials and S).

## Run

`benchrec run` of `ecbench run` (record `benchmarks/records/20261115T1647Z-ic-la-m3.json`, initialised on
the clean commit of the frozen spec), 16:55:20Z–16:58:23Z 2026-11-15; `ecbench verify --replay-all`. Raw
`raw/out_la.tgz` (sha256 `8510cd97…`); tables `RESULTS.md`, `results.json`; `analyze.py`.

## Results [pcc:unverified]

- **Identity:** 42/42 records verified; 42 replays reproduced, 0 problems. All 24 solver comparisons
  (echelon and structured against gauss, per workload): equal trials, relations, recovered logarithm,
  status, S, total GAE and relation-phase additions; 0 mismatches. [pcc:unverified]
- **Prediction 2 held:** `incremental-gauss-echelon` needs 0.10–0.15× the `row_ops` of
  `incremental-gauss` (predicted at most 0.7×); `structured-gauss` 0.25–0.30×.

| log2 r | \|F_x\| | gauss row_ops | echelon | structured | LA wall gauss → echelon (indicative) | cost / rho at w = 1, gauss → echelon (*corrected*) |
|--:|--:|--:|--:|--:|--:|--:|
| 33.4 | 2048 | 46.6M | 6.9M (0.15×) | 12.4M (0.27×) | 0.194 → 0.020 s | 168 → 69 |
| 33.4 | 4096 | 108.9M | 11.1M (0.10×) | 27.8M (0.26×) | 0.635 → 0.037 s | 363 → 118 |
| 35.2 | 2048 | 43.6M | 5.8M (0.13×) | 12.9M (0.30×) | 0.214 → 0.026 s | 2 196 → 1 366 |
| 35.2 | 4096 | 293.4M | 41.6M (0.14×) | 72.7M (0.25×) | 1.219 → 0.115 s | 7 492 → 1 966 |
| 37.4 | 2048 | 42.3M | 5.9M (0.14×) | 12.5M (0.30×) | 0.246 → 0.037 s | 836 → 730 |
| 37.4 | 4096 | 340.7M | 46.5M (0.14×) | 99.4M (0.29×) | 1.519 → 0.135 s | 1 392 → 537 |

- **§5a band** (*corrected*). cost_w = total GAE + w × U, with U read as `ecbench band` reads it
  (`band.json`): the counters in `cost.unpriced` from the top-level counters or the phases' native
  maps (rho's `canonicalisations_uncharged` included), plus the counted pair-table inserts. The first
  version read the phases' native maps only, which left rho's unpriced work at zero and gave 1.03–1.87×
  and 1.15–4.46× below. At w = 0 (S) the solvers tie, as they must; at w = 0.1 the echelon arm is
  1.03–1.82× cheaper than the gauss arm, and at w = 1 1.15–3.81× cheaper, largest at |F_x| = 4096.
  Because the arms tie at w = 0, `ecbench band` rates every echelon/gauss pair "unresolved inside the
  band": the order is not a §5a ranking, only a count difference that grows with w. Rho stays ahead at
  every weight. The record's metric `ic_la_echelon_band_w1_r37_f4096` (975 against 2 683) was written
  under the first reading and the closed record cannot be amended; this report's 537 against 1 392
  supersedes it.
- **Distance to rho** (with only two rho runs a curve, so the absolute ratios are noisy; the solver
  ratios on one curve share the same rho and are exact): S is 52–1 230× rho at w = 0; with echelon,
  55–1 254× at w = 0.1 and 69–1 966× at w = 1 (*corrected* from 58–1 359× and 116–3 168×).
- **structured-gauss** (*corrected*, previously only its `row_ops`): it scans 2.7–7.8× as many matrix
  entries as gauss, and its LA wall is 0.50–0.88× gauss's, so its `row_ops` saving does not carry into
  time the way echelon's does.
- **Not computed:** a calibrated-weight (`w_cal`) reading. The validator's illustration, at 0.005–0.013
  GAE per mulmod, puts the echelon advantage at 1.001–1.15×.
- **Wall** (L0, indicative): the echelon arm's linear algebra takes 0.06–0.15× the gauss arm's time.

## What this changes

`incremental-gauss-echelon` is a drop-in for `incremental-gauss` under the target-pinned completion
rule (only there: the pins agree for the target column, not for others): same answer, same S, a
seventh to a tenth of the row operations at these sizes. The linear-algebra time fell to 0.06–0.15×,
part of which comes from a faster multiply kernel rather than fewer operations (*corrected*) [pcc:unverified]. Earlier rounds' GPU arms all ran `incremental-gauss`; at |F_x| = 4096 their 0.5–1.5 s
of linear algebra would fall to roughly a tenth, which is not measured here on the GPU pod.

## Not done / next

- The GPU arms with `linalg=incremental-gauss-echelon` on the pod, to measure the whole-solve effect
  (protocol frozen in `research/ic_gpu_rootfind_la_20261115`; the pod could not be created, account
  balance too low).
- Process (validator): raw data in Git, not S3 (`uploaded: false`; S3 refuses this session's keys); the
  record's PDF field names REPORT.md; the replay-audit command is not in the record; the isolation
  field reads "unstated" (the run was L0).
