# Protocol (frozen before any native measurement): the model's K1 factor-base picks, measured by ecbench

Written 2026-10-29. Lane 3 of the relation-yield workflow. Supersedes the "run Python on EKS" plan of BLOCKER.md.
Frozen by `data/PROTOCOL.native.frozen.sha256` (sha256 of this file); byte copy `data/PROTOCOL.native.frozen.md`.
Changes after the freeze are appended below under "Changes after freeze" and never edit the body.

STATUS AT FREEZE: instrument verified by reading code only. No ecbench binary exists in this container (no disk for cargo),
so no spec has been parsed by `ecbench plan` and nothing has been measured. A pilot on EKS is step 0 of the run order.

## Idea
The model only chooses a factor base V (an F_2-subspace of F_{2^n}). The measurement is a standard binary index-calculus
solve. ecbench already accepts an explicit subspace: `binary-subspace:dimension=D,basis=w1;w2;...` (plugins.rs
`BinarySubspaceBase`, full-rank and range checked, polynomial-basis words). So the frozen picks of
`experiments/fb-cost-model-curves` for the a2 = 1 Koblitz curve (K1), l = 3, are passed as explicit bases. No new plug-in is needed.

## Instrument identity (checked by reading code and by `scripts/make_native_specs.py`)
- Curve: ecbench `{"kind":"koblitz","a":1,"n":N}` is y^2+xy = x^3+x^2+1 over F_{2^N}. Its modulus (Rust `find_irreducible_sparse`, smallest
  numeric low part) equals the Python `gf2n.modulus` for N = 17 (0x20009), 19 (0x80027), 23 (0x800021); it differs at N = 13 (0x201b vs 0x2027),
  so N = 13 is excluded (it was also uninformative: h = 106). Hence the basis words denote the same subspace in both codes.
- Bases are read from `fb-cost-model-curves/artifacts/factor_bases/factor_bases.jsonl.gz` by (curve_id, l, family, seed) and are
  checked: 3 words, nonzero, below 2^N, linearly independent.
- Picks are the gated rule (H) and gated ridge (G) picks recorded in `fb-cost-model-curves/results.json`; they are not recomputed.

| N | arm | family, seed | basis | usable points (Python) |
|--:|:--|:--|:--|--:|
| 17 | ic-rule (H) | prefix, 0 | 1;2;4 | 4 |
| 17 | ic-ridge (G) | normal, 402 | 56361;92585;106355 | 14 |
| 19 | ic-rule (H) | geometric, 404 | 387599;219764;450173 | 6 |
| 19 | ic-ridge (G) | random, 401 | 353332;44280;366837 | 12 |

N = 23 (prefix vs normal 403) is deferred: p(decomposable) of about 1e-5 makes a run long, and no pilot exists.

## Hypotheses
- H1: on K1, N in {17, 19}, l = 3, the ridge pick's complete single-target DLP costs less than the rule pick's in the native unit:
  log2(S_rule / S_ridge) > 0, with the 10,000-resample bootstrap CI (workloads resampled, seed 0) excluding 0.
- H0: the log-ratio is 0 or negative.
- Reference for comparison: local Python result for the same cells, `fb-cost-model-curves/e2e_results.json`
  (K1 N = 17: +2.15 bits counted, 12 seeds, post hoc). The native result is a separate instrument, not a replication of its number.

## Primary statistic and decision rule (stated now)
Per N: R = log2( sum S_ic-rule / sum S_ic-ridge ) over (target, round) pairs, `ecbench compare --a ic-ridge --b ic-rule` totals ratio
(AGENTS.md section 8), priced in the section 5a band: every `*_uncharged` counter of every arm, rho included, at 0, 0.1 and 1 GAE per operation.
- CONFIRMED at N if R > 0 with CI lower end > 0 at all three band prices.
- NULL if the CI contains 0 at any band price; REVERSED if the CI upper end < 0.
- Headline: CONFIRMED if both N confirm, PARTIAL if one, else NULL/REVERSED. A run where any IC run fails to verify is `incomplete`, not admissible, and is reported as such.
- Gap to matched rho: log2(S_ic-ridge / S_rho-strong) per band price, reported as distance remaining, not as a pass/fail gate.

## Controls
- Negative / A/A: `rho-strong-aa` (same method as the reference); admitted iff the run script's gate holds (aa_pairs >= 5, aa CI present, L2 for every pair).
  The IC arms have no second A/A copy in this spec (the isolab gate compares reference with control only); counts are deterministic, so the replay
  (`verify --replay-all`) is the A/A for counts.
- Positive control: the IC arm pair differ only in `basis`; the local result predicts R > 0 at N = 17. If native R at N = 17 is not > 0 while
  replay is identical, that is reported as a native NULL, not as an instrument failure.
- Instrument checks: `verify --replay-all` all identical; every run verified (`l2-gate.json` accepted); S3 manifest hashes match; Job and Pod UID equal in receipt and status.

## Run order and limits
0. Pilot (not scored, excluded from every number): the N = 17 spec with `targets_per_curve` 1, `rounds` 5, to measure run time. Needs the same image.
1. N = 17 request, then N = 19 request. Each: c8a.2xlarge Spot, `seconds` 3600, 64 KiB request limit, 4 targets x (5 rounds + 1 warmup) x 4 arms.
2. Cost cap: two jobs of at most 1 h on one c8a.2xlarge Spot node (a few dollars).
3. Image: must be built from a verified crypto main commit that contains the spec (GitHub `isolab-experiment.yml`, `spec_path` at that commit)
   or an existing per-run image digest with `job.build`. See `README_NATIVE.md`.

## Changes after freeze
(none)
