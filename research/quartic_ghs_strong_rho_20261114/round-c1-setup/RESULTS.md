# Round C1 result: fail-closed batched jump setup

Status: corrective deterministic `q=223` correctness/count control passed.
This supersedes Round C for integration and setup accounting. It is a
producer-side code/count result, not an isolated throughput result, frontier
result, or leaderboard change `[pcc:unverified]`.

## Correction outcome

The corrected candidate batches the 32 jump constructions with the cold
width-2 table and constructs every initial walker through the legacy
lane-ordered `new_walker` retry function. It contains no tentative walker
batch, counter rollback, or RNG rollback. The only direct `Walker`
construction is inside `new_walker`, keeping initialization centralized for
later hot-loop cached state.

The candidate reproduced `ma`, `mb`, all jump points, every initial walker
point/coefficient, the post-setup RNG state, and all native counts. Both arms
recovered scalar 1,765,657 after the identical walk: 1,487 steps, 76 DPs, 0
restarts, 53 look-ahead retries, 2 cycles, and 10 cycle additions. Walk and
verification field/native counts were identical.

## Fully charged setup counts

| counter | legacy | corrected jump batch | delta |
|:--|--:|--:|--:|
| `fq_mul` | 94,523 | 58,528 | -35,995 |
| `fq_inv` | 2,348 | 511 | -1,837 |
| `fq_add` | 299,272 | 210,274 | -88,998 |
| `f3_mul` | 8,593 | 6,948 | -1,645 |
| `f3_sq` | 3,897 | 2,346 | -1,551 |
| `f3_inv` | 2,348 | 511 | -1,837 |
| `batch_inv` | 0 | 33 | +33 |
| `group_add` | 2,429 | 1,455 | -974 |

The cold table is included and unchanged: 13 group operations in three
complete-addition batches, costing 471 `fq_mul`, 3 `fq_inv`, 1,678 `fq_add`,
61 `f3_mul`, 15 `f3_sq`, 3 `f3_inv`, and 3 `batch_inv`.

For `M-eq(R)=fq_mul+R*fq_inv`, candidate minus baseline is
`-35,995 - 1,837R`, strictly lower for every `R >= 0`. The exact-bound run's
non-isolated, single-triple local calibration (`R=18.328733094`) maps that to
137,558.87 versus 67,893.98 M-eq, a 50.64% reduction. That percentage is a
`q=223` local-calibration illustration only, not throughput evidence. Round
C's earlier 58.75% illustration is superseded by this corrected setup.

## Identity evidence

All non-setup deltas were zero:

| phase | counter vector in both arms |
|:--|:--|
| walk field | `fq_mul=53825, fq_inv=241, fq_add=195560, f3_mul=7275, f3_sq=1553, f3_inv=241, batch_inv=231, group_add=1551` |
| verify field | `fq_mul=1210, fq_inv=30, fq_add=3826, f3_mul=110, f3_sq=50, f3_inv=30, batch_inv=0, group_add=31` |
| setup native | `rng=80, canon=8, scalar_add=12, band_ops=8`; all other fields zero |
| walk native | `rng=0, hash=4534, canon=1497, lookahead_test=1549, dp=1487, table=153, cycle_cmp=355, scalar_add=4496, scalar_mul=1, scalar_inv=1, band_ops=5041` |

The canonical identity projection appears exactly twice and retains SHA-256
`24235e91f84580fc53aee3e9be4e95c5956195d267ab4a75db6c8ad3d6a655aa`.

The exact source-bound standalone self-test passed complete group-law cases,
width-2 point equality, setup state/RNG equality, planted widths 1/8/32, and
the paired full-trajectory control. All 16 request/analyzer unit tests passed.

## Provenance and limits

- Corrective protocol commit: `e18a281f`.
- Corrected implementation and exact tested source:
  `0e92c5352f9674881e41cefaa9e6a76b08bdb2d8`.
- Superseded Round C source: `f3f2cbcc11ad8ddea9a3c1730144c406bed9fb2b`.
- Original baseline: `ad2a014c0022cf000845014499a9649b6c2cef2d`.
- Corrected `strong_rho.rs` SHA-256:
  `86b3d617ae66d3251abe75d833c8a5ba290050f7b396d52876a7c33cc10b72a8`.
- Corrected `main.rs` SHA-256:
  `6e49221ad570fa62aa244b89e8e32f6cb8ac0562b8be59c53e3c9fc9ce482dda`.
- Exact binary SHA-256:
  `42e0db0c97e16c8dba0532064cb4cd348f156feb6471f69d6c1e644021939926`.
- `q223.jsonl` SHA-256:
  `06ce317a80499ac2c62eff7b236d4035b2cc214eeee5dfd884b902d119a27df5`.
- `selftest.txt` SHA-256:
  `0c80c2938735e7a9cddd85cbca9baceb3b69ff3479f745c8dc2bdffcec6eea10`.

The checkpoint gate again exited 2 because no resumable S3 checkpoint was
present. No profile, isolated timing, larger diagnostic, matched IC/rho grid,
independent validation, PCC, frontier inference, or leaderboard update was
performed. Round C's raw evidence remains for audit but its implementation
must not be integrated.
