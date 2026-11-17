# Prime-field IC: `stride-renew` relation targets against the walk — protocol

## 1. Question and class

The prime-field `ic.pipeline` baseline draws relation targets with
`targets = walk`, an r-adding walk over 16 jumps `J_j = [a_j]G + [b_j]Q`
with pooled restart offsets. Each jump and each restart offset costs two
scalar multiplications, all of them charged. At 20 bits, N 64, that is
56 scalar multiplications per solve: 981 doublings and about 560
additions of a 7,099 GAE relations phase (*recorded*, one record of
session `ECBS1h33c033ce9c88`, XP-20261029).

`targets = stride-renew` (`src/cryptanalysis/ic_boundary.rs`,
`TargetSource::StrideRenew`) steps `R ← R + S` from `R₀ = [a]G + [b]Q`. It
renews `S ← S + T` after every relation, at one addition each. Set-up is four
scalar multiplications. It has been measured on binary and Frobenius curves
(`research/ecbench_rootfind_speedup_20261014`), but not on the prime-field
m = 2 baseline.

Question: does `stride-renew` lower the whole-pipeline counted cost S of the
prime-field IC baseline at each size, with the factor base and oracle
unchanged, across the §5a band? Class: engineering. Each size is decided on
its own. No exponent claim.

## 2. Arms (one session per size; `spec-P<b>.json`)

| arm | role | targets | decision-bearing |
|:--|:--|:--|:--|
| `walk` | baseline | `walk` (the incumbent) | yes |
| `srenew` | candidate | `stride-renew` | yes (the only candidate) |
| `walk-aa` | control | = `walk` | A/A |
| `rho-neg` | reference | `rho.negation` | matched rho |
| `rho-neg-aa` | control | = `rho-neg` | A/A |

Each IC arm at size `b` uses the incumbent factor base and the incumbent
oracle at that size. Only `targets` differs.

| b | factor base | max_trials |
|--:|:--|--:|
| 20 | `prime-abscissa:size=45` (XP-20261016, the leaderboard baseline) | 1410236 |
| 24 | `prime-abscissa:size=162` | 4243777 |
| 28 | `prime-abscissa:size=407` | 26479085 |
| 32 | `prime-abscissa:size=1024` | 167871873 |

The oracle is `mitm:negation_folded=1`. The other `ic.pipeline` parameters are
the defaults that `ecbench plan` prints.

## 3. Workloads

There are two fresh prime-order curves per size, from `prime_search` with
seeds `20261029<b>1` and `20261029<b>2`. At 20 bits the seeds are
202610292011 and 202610292012, because 2026102921/22 gave curves the
XP-20261029 N 64 study had used.

| b | curves (`ecbench plan`) |
|--:|:--|
| 20 | `icv1-fp20-tm327-45c21191`, `icv1-fp20-tm1549-7ec9131a` |
| 24 | `icv1-fp24-tm2059-5a3b6ab8`, `icv1-fp24-tm5549-c2ef82ac` |
| 28 | `icv1-fp28-t20713-32971d39`, `icv1-fp28-tm21435-0b5ad955` |
| 32 | `icv1-fp32-t10863-8c68282c`, `icv1-fp32-tm62163-8ef9adf3` |

None of these was used by XP-20261016, XP-20261029 (N 64) or the pilot (§8).
Each curve gets 24 public targets (seed `20261029<b>5`). The measurement has
1 warm-up and 6 measured rounds, order `alternate`, seed `20261029<b>9`, L2,
and a timeout of 1800 s. That is 1,680 executions per size and 288 matched
pairs per arm pair.

## 4. Route

isolab-eks via the AWS launcher, one job per size, `c8a.2xlarge`, with a
3,600 s deadline and the image's ecbench. Frozen task cards come from
`experiment-director` and are run unchanged by `executor-mechanical`.
Results are checked against each job's `SHA256SUMS`. A host loss reruns that
size's job whole under a new job name, and the rerun is recorded.

## 5. Unit and pricing

`S = total GAE / √r`, cold, whole pipeline. Every arm, rho included, is
priced at w = 0, 0.1 and 1 GAE per unpriced native operation (AGENTS.md
§5a), using `ecbench compare --weight W` and `ecbench band --ranks`. C = 0
on every arm. Wall time is not a claim.

## 6. Decision rule (per size, fixed now)

`srenew` is an **improvement** at size `b`, and becomes the target source of
the prime-field IC baseline there, only if all of the following hold:

1. **Instrument:** the session audit is `ok`, with every measured replay
   identical. `walk-aa/walk` and `rho-neg-aa/rho-neg` are exactly 1.000 at
   every weight. The L2 gate admits every execution. The worker's input
   spec equals the frozen spec.
2. **Completeness:** all 288 measured `srenew` runs and all 288 `walk`
   runs verified `[d]G = Q`, with 0 censored pairs.
3. **Ratio:** the 95% upper bound of `Σ S_srenew(w) / Σ S_walk(w)` is below
   1 at w = 0, 0.1 and 1. The bound comes from a cluster bootstrap over
   targets, 4,000 resamples, seed 20261002.
4. **Band:** `ecbench band --ranks` gives `srenew < walk` on every curve of
   the size.

| outcome at `b` | reading |
|:--|:--|
| 1–4 hold | improvement; `stride-renew` becomes the baseline target source at `b` |
| below 1 at some weights, not at others | unresolved inside the band; walk kept |
| total falls at w = 0 and rises at w = 1 | relabelling; walk kept |
| interval includes 1 at every weight | null at this sample size; walk kept |
| upper bound of `Σ S_walk / Σ S_srenew` below 1 at every weight | regression; walk kept |
| 2 fails | incomplete, never a win |

Every size is always reported with `walk/rho-neg` and `srenew/rho-neg`
and their intervals. This is the distance remaining.

## 7. Predictions (before any confirmatory data)

1. At 20 bits, `srenew/walk` is below 1 at every weight. Its size is
   uncertain: the pilot gave about 0.74 at w = 0 (§8), and the scalar
   multiplications alone predict about 0.85–0.90.
2. The gain shrinks with size, because the scalar-multiplication share of the
   relations phase falls as the number of trials grows faster than log r. By
   32 bits it is under 5%, and the interval may contain 1 (null).
3. The unpriced counters per trial (guard probe, lookup, step) are roughly
   unchanged, so the ratio at w = 1 is close to the ratio at w = 0.

## 8. Disclosure: feasibility pilot run before this draft

Before freezing, a local feasibility pilot checked that `ic.pipeline`
accepts `targets = stride-renew` and `stride` with the prime m = 2 oracle.
- **Spec:** `pilot-spec.json`.
- **Session:** `ECBS1hceb511b50e2d`, local ecbench `bf5eec6d`, L0, `--cpus none`.
- **Curve:** one throwaway curve, `icv1-fp20-tm1247-8feb8b3e` (seed 424242), not used here.
- **Size:** 6 targets × 2 rounds, 12 pairs.
- **Result:** all 54 runs verified. `stride-renew/walk` was 0.738 [0.666, 0.819] at w = 0 and 0.784 [0.692, 0.888] at w = 1. `stride/walk` was 1.57 at w = 0 and 1.86 at w = 1.

The pilot chose the candidate (`stride-renew` rather than `stride`). It is
not part of any decision. No other data on these curves or seeds exists.

## 9. Freezing

This file and the four specs are frozen by the commit that adds them. Each
job's request embeds its spec byte for byte, and the spec sha256 is recorded
on the launch card. No amendment is allowed after the first execution.

## 10. Outputs

`research/ic_stride_renew_prime_20261029/` holds, per size:
- launch receipts;
- the S3 copy with hashes;
- `compare` JSON at three weights for `walk__srenew`, `walk__walk-aa`, `rho-neg__walk`, `rho-neg__srenew` and `rho-neg__rho-neg-aa`;
- the band table.

The directory also holds a README with a figure and a PDF. An improvement
at any size goes to the validation gate (an independent validator and
VALIDATION.json) before any leaderboard change.
