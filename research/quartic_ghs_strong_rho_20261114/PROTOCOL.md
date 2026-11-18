# Frozen protocol: strong-rho boundary for the GHS-weak plane-quartic family

Protocol status: frozen before paid measurements. Any result produced by this
study is an unverified assertion until an independent validator admits its
artifacts and PCC reports standing `[pcc:unverified]`.

## Question and boundary

Does the end-to-end N4-v2 index-calculus result remain below a matched,
optimized generic rho reference on the same public synthetic
`E0/F_{q^3}` instances and exact targets?

The reference boundary is generic Pollard rho in the order-`ell` subgroup of
the same curve, with the negation quotient. The primary implementation uses
32 additive partitions, 8 lockstep walks, Montgomery batch inversion, a
six-base-product cubic-field multiplication, deterministic fruitless-cycle
look-ahead plus canonical-minimum cycle escape, and known-answer verification.
Setup, walk, collision recovery, and verification are all charged cold. No
oracle, precomputation, or target
generation cost may be omitted.

This boundary is deliberately narrow. It concerns the odd-prime cubic GHS
family constructed by the frozen N4 code. It does not transfer to generic
prime curves, random binary curves, Koblitz curves, standardized curves, or
deployed keys. The experiment solves only planted public known-answer
instances `[pcc:unverified]`.

The historical N4-v2 measurements suggest that the best IC arm is below its
serial negation-map rho reference. That observation is prior evidence, not
the conclusion of this study `[pcc:unverified]`.

## Frozen hypotheses

- **H0 (exception rejected):** at least one admitted grid size has an IC/rho
  95% interval reaching or exceeding one at any native-work band weight.
- **H1 (exception survives):** at every admitted grid size, the 95% upper
  bound for best frozen IC divided by `strong-b8` is below one at all three
  band weights.
- **H2 (rho engineering):** on the diagnostic cells, `strong-b8` has lower
  cold counted cost than frozen `old-neg` at all three band weights. H2 is
  separate from H1 and cannot establish an IC advance.

No acceptance threshold, width, seed, target, size, or pricing rule may be
changed after primary output is inspected. An amendment must be committed,
dated, and identified as a new round.

## Frozen inputs

The grid is the N4-v2 high-range grid:

| k | q |
|--:|--:|
| 12 | 4051 |
| 13 | 8191 |
| 14 | 16363 |
| 15 | 32719 |
| 16 | 65479 |
| 17 | 131071 |
| 18 | 262111 |
| 19 | 524287 |
| 20 | 1048507 |
| 21 | 2097091 |
| 22 | 4194271 |

Each size uses instance seeds 1, 2, and 3. For every `(q, seed)`, the primary
rho repetitions use `target_policy=ic-repeat`: the exact `k` consumed by the
frozen N4-v2 IC unit is reconstructed from the frozen preamble, and every rho
walk solves that same target with an independent deterministic walk seed
`[pcc:unverified]`.

The frozen historical raw archive is
`research/quartic_n4v2_20261020/results/grid-20261025/units.tar.gz`, as
recorded in that directory's artifact manifest. Its full SHA-256 is
`aac9a25ce50ac5d23895f0dba8f907fb223aaac32a2a41391bc590e3c6bcca42`
and the archive is 382,105 bytes. Its durable URI and extraction receipt must
be copied into the result manifest. A local hydrated copy is not evidence by
itself. The pre-migration Git object at
`e8743ca84185389926afca8390f5d17361958df9` may be used only if its full hash
matches the manifest.

The historical IC candidates are `B0`, `S`, `L0`, `L1`, `L`, `D`, `DL1`, and
`DL`. Every successful cold attempt is re-priced; the primary IC row at a
size is the lowest mean among these frozen arms, with no new arm selection or
parameter tuning. Failed attempts remain charged to their unit.

## Arms and sampling

The primary arm is fixed as `strong-b8`. Repetitions per instance are:

| k range | rho repetitions per seed |
|:--|--:|
| 12--18 | 5 |
| 19--22 | 3 |

`strong-b4` and `strong-b16` are sensitivity arms, one repetition per seed at
k = 12, 16, and 18. Neither can replace the primary after results are known. Frozen
`old-neg` is a parity diagnostic, one repetition per seed at k = 12, 16, and
18. Development smoke runs, including width selection work, are excluded.

The analysis forms one rho mean per `(q, seed)` before resampling, so repeated
walks are not treated as additional independent curve instances. Confidence
intervals use 10,000 deterministic clustered bootstrap resamples of the three
seed-level units. Means, medians, all individual observations, and interval
endpoints are retained. At k = 22 the interval is explicitly labelled as a
three-walk interval.

## One unit and complete cost

The shared unit is an `F_q` multiplication-equivalent:

```text
M-eq = fq_mul + r_inv * fq_inv + r_lmul * mod_ell_mul
S    = cold_M-eq / sqrt(ell)
```

`r_inv` and `r_lmul` are the median ratios from 600 interleaved calibration
triples per `(q, seed)`, using the exact frozen N4-v2 kernels. The median of
the three seed calibrations prices every historical and new arm at that size.
It is forbidden to price IC with the historical host's ratios and rho with
the new host's ratios.

The inherited sensitivity band is evaluated at `w = 0, 0.1, 1`:

```text
cold_M-eq(w) = cold_M-eq + w * GAE * band_ops
GAE           = 56 + r_inv
```

For historical IC, `band_ops` is the sum of recorded phase `hash + cmp`. For
historical rho it is `c.hash + c.cmp`. For strong rho it is the sum of setup
and walk canonicalisations, distinguished-point tests, table operations, and
cycle comparisons. Raw hashes implement those abstract operations and are
not charged again. RNG draws, scalar additions, and collision inversions are
retained as explicit diagnostics, matching the frozen treatment of `ladd`
and collision inversion. Base-field additions are counted and not priced.

For strong rho, all setup, walk, and verification field counts are summed;
setup and walk modular scalar multiplications are priced by `r_lmul`. The
last partially consumed lockstep batch is charged in full because its field
work was executed. Wall time, load, CPU model, compiler version, binary hash,
and pod identity are recorded, but standard-isolation wall time is diagnostic
and cannot support a speed claim.

The primary comparison table contains, for every q and arm: successful runs,
cold M-eq at all weights, `S`, `steps/sqrt(ell)`, setup/walk/verify phase
shares, IC/rho ratio, and its clustered interval. It also retains the
superseded historical rho row.

## Correctness and admission gates

All gates are conjunctive:

1. The standalone self-test matches the frozen field operations, point
   addition, batch inversion, and planted logarithms at widths 1, 8, and 32.
2. Every primary record has the pinned merged source revision, expected q,
   seed, subgroup order, and exact historical IC target.
3. Every rho run reports `ok=true`, `k_found=k`, and independently verifies
   `kG=Q` `[pcc:unverified]`. Any failure remains in the raw data and excludes
   that size from H1.
4. At each size, the primary mean `steps/sqrt(ell)` lies in the inherited
   negation-rho interval `[0.75, 1.15]`. A size outside the interval is shown
   but is not admitted to an IC/rho conclusion.
5. Count identities, non-negativity, phase sums, requested repetition counts,
   and JSON schema pass the independent validator. The validator also checks
   that batch width is exactly 8 for every primary row.
6. A duplicate small-cell replay must match all non-timing records and counts
   byte-for-byte after timing/calibration fields are removed.
7. Every raw file and the binary are covered by SHA-256 manifests and the
   benchmark record. Missing, truncated, or mixed-revision artifacts fail the
   round; they are never silently dropped.

H1 is reported as **confirmed on the measured grid** only if every grid size
is admitted and its upper interval is below one at every band weight. If some
sizes are missing or excluded but all admitted sizes pass, the verdict is
**limited**, with the missing sizes named. Otherwise H1 is rejected or
unresolved according to the intervals. No extrapolation can upgrade a
limited verdict.

## Execution route and finite budget

Primary measurements run through IsoLab EKS Spot on the merged source
revision. Four standard-isolation command jobs split k = 12--18, 19--20, 21,
and 22. Each requests at most 4 vCPU and 4 GiB, has zero automatic retries,
and a 3,600-second deadline. Aggregate requested CPU is capped at 16 vCPU.
The pod script runs at most three single-thread seed processes concurrently.

The hard experiment ceiling is USD 5 including failed launches and
infrastructure retries. There are no automatic retries. One manual retry is
allowed only for a documented infrastructure failure, only if the cumulative
estimate remains below the same ceiling, and must use a new job name while
preserving the failed receipt. A computation that reaches the deadline is a
censored result, not evidence of failure or success.

Before submission, record AWS identity, region, cluster readiness, current
daily spend/allowance, active jobs, desired pool CPU/memory, source revision,
protocol SHA-256, image digest, job manifests, and estimated cost. A checkpoint
must be pushed before any paid job. If any preflight or budget gate fails, no
job is launched.

## Artifacts, validation, and publication

Raw pod output is uploaded incrementally beneath a write-once run prefix and
contains host metadata, source/binary hashes, self-test output, calibration,
all JSONL records, exit codes, and `SHA256SUMS`. The repository result round
contains the launch/status receipts, artifact manifest, compact raw data,
analysis JSON/Markdown, validator report, benchmark record, and failure log.

Before any conclusion is cited, create PCC measurement/execution claims with
the raw manifest, analysis, and independent validator as proofs. Producer
self-checks do not establish verified standing. Until PCC standing says
otherwise, every numerical conclusion remains `[pcc:unverified]`.

If the matched reference or whole-pipeline boundary changes, the same result
PR must update and regenerate the IC leaderboard, scoreboard progress/best
panels, lab browser, curve page, benchmark index, and daily progress exporter.
The old rho result remains visible as the before mark. A code/protocol-only PR
does not move those views because it contains no admitted measurement.
