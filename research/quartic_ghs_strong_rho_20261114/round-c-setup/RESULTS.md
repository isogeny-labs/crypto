# Round C result: exact batched strong-rho setup

> **Superseded for integration and accounting by
> [Round C1](../round-c1-setup/RESULTS.md).** The retained `q=223` path did not
> take the defective fallback, but integration review found that a rare
> identity-walker fallback could restore counters after tentative work. Do not
> integrate the Round C implementation; its raw records remain audit evidence.

Status: deterministic `q=223` correctness/count control passed. The profile,
isolated timing panel, larger cells, matched frontier grid, independent
validation, and PCC were not run. This is a producer-side code/count result,
not a throughput result or leaderboard change `[pcc:unverified]`.

## Outcome

The cold width-2 simultaneous-multiplication setup reproduced the baseline
coefficient arrays, all 32 jump points, all 8 initial walker points and
coefficients, the post-setup RNG state, and every setup native count. The
paired ECDLP control recovered the same scalar after the same walk: 1,487
steps, 76 distinguished points, 0 restarts, 53 look-ahead retries, 2 cycles,
and 10 cycle additions. Every walk and verification field/native count was
identical.

The fully charged setup field vector improved componentwise in its two priced
terms `[pcc:unverified]`:

| setup counter | legacy | batched width-2 | delta |
|:--|--:|--:|--:|
| `fq_mul` | 94,523 | 49,975 | -44,548 |
| `fq_inv` | 2,348 | 93 | -2,255 |
| `fq_add` | 299,272 | 188,646 | -110,626 |
| `f3_mul` | 8,593 | 6,525 | -2,068 |
| `f3_sq` | 3,897 | 1,979 | -1,918 |
| `f3_inv` | 2,348 | 93 | -2,255 |
| `batch_inv` | 0 | 63 | +63 |
| `group_add` | 2,429 | 1,224 | -1,205 |

Thus, for the frozen symbolic setup price
`M-eq(R) = fq_mul + R*fq_inv`, candidate minus baseline is
`-44,548 - 2,255R`, strictly lower for every nonnegative inversion price.
This symbolic result, not local wall time, is the acceptance evidence.

An earlier development preflight used the single-triple local calibration
`R=12.544869175` and gave 123,978.35 versus 51,141.67 M-eq, a **58.75% setup
reduction**. Its explicitly unbound raw record is retained as
`q223-preflight-unbound.jsonl`; it is not the primary source-bound record.
That number is `q=223`/local-calibration only. The exact-bound
rerun's equally non-isolated one-triple calibration moved to `R=27.123133663`
(158,208.12 versus 52,497.45, 66.82%), demonstrating why neither local
percentage is a throughput or frontier result.

## Cold precomputation and phase accounting

The shared table contains `[i]G+[j]Q` for `0 <= i,j < 4`. Its 13 charged
group operations are two doublings (`2G`, `2Q`), two additions (`3G`, `3Q`),
and nine mixed additions. Three complete-addition batches share their
denominators. The exact cold table vector was:

| counter | table |
|:--|--:|
| `fq_mul` | 471 |
| `fq_inv` | 3 |
| `fq_add` | 1,678 |
| `f3_mul` | 61 |
| `f3_sq` | 15 |
| `f3_inv` | 3 |
| `batch_inv` | 3 |
| `group_add` | 13 |

The unchanged target multiplication used 1,210 `fq_mul`, 30 `fq_inv`, 3,826
`fq_add`, 110 `f3_mul`, 50 `f3_sq`, 30 `f3_inv`, and 31 group operations.
After subtracting that target and the cold table, the 32 jumps plus 8 walkers
used 48,294 `fq_mul`, 60 `fq_inv`/batch inversions, 183,142 `fq_add`, 6,354
`f3_mul`, 1,914 `f3_sq`, and 1,180 group operations. Nothing is amortized
outside the setup row.

All non-setup deltas were exactly zero:

| phase | counter vector (both arms) | delta |
|:--|:--|:--|
| walk field | `fq_mul=53825, fq_inv=241, fq_add=195560, f3_mul=7275, f3_sq=1553, f3_inv=241, batch_inv=231, group_add=1551` | all zero |
| verify field | `fq_mul=1210, fq_inv=30, fq_add=3826, f3_mul=110, f3_sq=50, f3_inv=30, batch_inv=0, group_add=31` | all zero |
| setup native | `rng=80, canon=8, scalar_add=12, band_ops=8`; every other field zero | all zero |
| walk native | `rng=0, hash=4534, canon=1497, lookahead_test=1549, dp=1487, table=153, cycle_cmp=355, scalar_add=4496, scalar_mul=1, scalar_inv=1, band_ops=5041` | all zero |

The canonical identity projection of the two raw rho rows appears exactly
twice and has SHA-256
`24235e91f84580fc53aee3e9be4e95c5956195d267ab4a75db6c8ad3d6a655aa`.
That is byte identity after excluding only the arm/setup labels, setup field
counts, and diagnostic wall time. See `validation.txt`.

## Correctness controls

The standalone self-test passed:

- generic field agreement over 200 deterministic inputs;
- batch inversion agreement;
- complete affine batches including identities, doublings, and inverse pairs;
- width-2 simultaneous multiplication against independent legacy scalar
  multiplication for zero/edge coefficients and 64 deterministic pairs;
- point-for-point initial setup, coefficient, native-count, and RNG equality;
- planted-log recovery at widths 1, 8, and 32; and
- a paired width-8 full-trajectory identity control.

The package's 16 request/analyzer unit tests also passed. The isolated sparse
lead did not rerun the known-unrelated repository-wide Cargo suite; Round B's
retained base failure remains the latest full-suite status for this package.

The initial-walker candidate snapshots the RNG and counters. If any tentative
batched walker is the identity, it restores those values and executes the
legacy lane-ordered retry path; it does not reorder random coefficients.

## Provenance and limitations

- Frozen protocol commit: `8a01c131`.
- Candidate implementation commit: `92115033`.
- Exact tested source commit: `f3f2cbcc11ad8ddea9a3c1730144c406bed9fb2b`.
- Baseline revision: `ad2a014c0022cf000845014499a9649b6c2cef2d`.
- Candidate `strong_rho.rs` SHA-256:
  `af664bd2e962c2d8ea1db95d0fda103afdd1489d5c07e1bdaac5c9181b2bfffa`.
- Candidate `main.rs` SHA-256:
  `a217d0e441c1d6f5c10db702f4e154efdaebc69208ed3cc3e15dee516079ec8f`.
- Exact binary SHA-256:
  `552fbf9b173fa4c865491d5876e705d0855c22342134453e9d00c45d3c2dd604`.
- Source-bound `q223.jsonl` SHA-256:
  `918b67928d6ab774569fd24f9834d04d1cd92c74c0793e59e09bcf42a6cf91db`.
- `selftest.txt` SHA-256:
  `82ca2e88124a9341e6d65fe9a52cddc5f47dd90b6cb333210004827c261bba26`.
- Compiler: rustc 1.100.0, LLVM 23.1.5, x86_64 Linux.
- Host: 5 visible Intel Xeon Platinum 8370C cores; not isolated.

The first compile failure and the checkpoint refusal are retained in
`failures.txt`. `scripts/checkpoint.sh gate` exited 2 because no resumable S3
checkpoint existed. Consequently no profile, timing panel, larger diagnostic,
paid run, complete IC/rho comparison, independent validation, PCC, frontier
inference, or leaderboard update was performed. The restricted GHS-weak
family's historical IC-led status is not re-established or changed by this
setup-only control.
