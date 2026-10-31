# Full recovery on one RTX PRO 6000

This study measures a bounded operational frontier for public prime-field
fixtures. It uses GPU factor-base construction and generic meet-in-the-middle
relations, followed by CPU elimination, scalar extraction and fresh certificate
replay. It does not implement the paper's conditional tensor-contraction
backend, establish a mathematical maximum, or compare against matched rho or
the fastest external solver. All clocks are shared-host L0 observations.

**Both fixed public targets now complete through 60-bit fields.** V8 records
12 verified recoveries out of 12 prescribed executions. The 60-bit cold solves
are 770.270416 and 845.179937 seconds. These are bounded observations on two
public fixtures, not a global hardware maximum. The separately checked v9
extension accepts p < 2^62 and records one verified61-bit recovery; the other
prescribed61-bit target hit memory admission. Its62-bit cells were unattempted.

The latest completed panel is described in the [v9 observation report](reports/run-20261030T1502Z/report.pdf).

## Complete run accounting

| Version | Pinned source | Recorded executions | Verified recoveries | Remaining outcome |
| --- | --- | ---: | ---: | --- |
| v1 | `84e11440cc044eb311a2e48254ee15722a6adbaf` | 0 | 0 | Wrapper failure before compilation |
| v2 | `b3e82b3cd1992442819932db292b5c16f68b4b04` | 0 | 0 | Preflight passed; analyzer compilation failed |
| v3 | `614791252805f64a618a8ce0080e8f4f040c3a8f` | 44 | 40 | Four 56-bit timeouts at 300 seconds |
| v4 | `320694bfff1eaed8e2471e9335d9e7cfa94e8ad4` | 20 | 16 | Four 58-bit work-limit failures; four 60-bit cells unattempted |
| v5 | `f6338418572c72e1bf97ad24903cd5efd17cf7da` | 10 | 9 | One 58-bit timeout at 600 seconds; two 60-bit cells unattempted |
| v6 | `5a7bfdb97af8941261a2e66a9008231e52f3f23a` | 0 | 0 | Preflight passed; first solve rejected the requested row cap |
| v7 | `4014622e18caef9a6b6c89d79653e7633f5aaaff` | 12 | 10 | Two 60-bit timeouts at 1,200 seconds |
| v8 | `bbf0f2f51e73afd7e41174094feb2c56a72d3d9d` | 12 | 12 | Complete through both 60-bit targets |
| v9 | `e1a38ce4bc586050c3faa67a3ab79f312eb549b5` | 8 | 7 | One61-bit memory refusal; one61-bit recovery; both62-bit cells unattempted |

The first v6 solve was attempted and failed, despite producing no CSV data
row. Its input, error and entire archive are retained. No launch is silently
reclassified or retried under its original protocol. V1–v9 pods were deleted
and deletion verified before the next task GPU launched.

## Observed comparisons

V4 compares binary search with a high-x prefix index on the same inputs,
host, binary source and limits. Entries below are recorded cold solve seconds;
`x` is an identical indexed A/A invocation. Certificates match byte-for-byte.

| Field bits | Seed | Binary b | Indexed u | Indexed x |
| ---: | ---: | ---: | ---: | ---: |
| 48 | 101 | 8.096602 | 4.833402 | 4.762925 |
| 48 | 202 | 7.799079 | 5.051183 | 5.073777 |
| 52 | 101 | 59.337260 | 38.096282 | 36.137007 |
| 52 | 202 | 65.950374 | 40.096043 | 40.053185 |

The full 56-bit rung then completed with cold times 211.902261–231.374448
seconds. The four 58-bit failures were work admission failures caused by the
old row cap selecting unbalanced splits. They were not exhausted searches.

V5 uses larger explicit caps and compares bounded partial-sum caching with
the uncached index. The cache did not lower either matched 52-bit cold time:

| Seed | Uncached b | Cached u | Cached x |
| ---: | ---: | ---: | ---: |
| 101 | 38.074392 | 39.173036 | 38.890065 |
| 202 | 40.590157 | 42.322563 | 42.127127 |

The ordinary build therefore retains uncached indexed search; caching is
available explicitly with `make cuda-cached`. At 58 bits, seed 101 completed
in 595.862060 seconds cold and 596.139162 seconds including additional audit.
Seed 202 timed out after 600.006124 seconds. Both admitted their tables, so
this particular failure was a time limit, not memory admission. No values
are pooled across versions or hosts, and no L0 ratio is a certified speedup.

## Wider exact index: complete 60-bit rung

V8 compares the original 2^16-bucket index against an optional 2^24-bucket
index, with exact 16-byte compact rows and identical CPU driver in every arm.
Index setup and storage are included. The new offsets cost at most 128 MiB
plus eight bytes. `make cuda-wide-index ARCH=sm_120` selects the new variant.
The ordinary full-row and compact builds retain their previous index limits.

| Field bits | Seed | Original index b | Wider index u | Identical wider x |
| ---: | ---: | ---: | ---: | ---: |
| 52 | 101 | 29.344188 | 17.810696 | 17.813335 |
| 52 | 202 | 31.542124 | 22.262391 | 22.299082 |
| 56 | 101 | Not prescribed | 99.731624 | Not prescribed |
| 56 | 202 | Not prescribed | 107.053471 | Not prescribed |
| 58 | 101 | Not prescribed | 252.753463 | Not prescribed |
| 58 | 202 | Not prescribed | 273.055707 | Not prescribed |
| 60 | 101 | Not prescribed | 770.270416 | Not prescribed |
| 60 | 202 | Not prescribed | 845.179937 | Not prescribed |

These are cold solve seconds, rounded. The two 60-bit totals including fresh
replay and negative-control auditing are 770.303987 and 845.214673 seconds.
They use 1,471/1,438 relations from 2,208/2,380 attempts. Oracle times are
768.772528/843.662947 seconds; elimination takes 1.108359/1.127755 seconds.
The measured source also includes the separately merged CPU arithmetic changes;
comparisons between v7 and v8 therefore do not isolate the index change.
The within-v8 52-bit comparison does hold the CPU driver fixed. All timings
remain L0 observations, with no certified speedup or extrapolated maximum.

V8 passed 17 Rust tests, full/cached/compact device checks, Clippy, maximum-cap
service headers and full/compact/wide query memcheck, racecheck and synccheck.
The wide index boundary check covers 2^24 offsets, exact lower-bound agreement,
duplicate/parity/infinity cases and existing valid-curve differential fixtures.
Worker/controller exited zero, archive collection and deletion were verified.

## Checked width extension: one61-bit recovery

V9's six52-bit control executions verified. At61bits, seed101 refused the
85,969,611,408-byte admission estimate for1,788,236,560left rows after0.263632s;
seed202 admitted63,161,668,896bytes for1,313,071,104left rows and recovered in
1537.899248s cold,1537.918073s including fresh verification and corruption
rejection. It used1591relations from2432attempts; oracle1536.244543s and
elimination1.360546s. These are L0 observations, not certified performance.
V9's largest complete prescribed rung is52bits; v8's separate complete rung
remains60. Both62-bit cells were unattempted by the frozen stopping rule.

All19Rust tests and the recorded host/device, Clippy, maximum-cap and
full/compact/wide query sanitizer checks passed. The archive digest matched,
worker/controller exited zero, and pod deletion was verified. The exact
[raw export](results-v9/export.json), [coverage](results-v9/coverage.json),
[checker limits](results-v9/CHECKER-NOTE.md) and [protocol](protocol-v9.json)
retain the failure and five canonical rectangular-coverage findings.
V10 was never executed; see [the supersession record](UNEXECUTED-v10.md).
V11 remains a separate frozen test; no pending outcome is counted here.

## Correctness and custody

Each public target Q is obtained by coordinate lifting; its scalar is never
generated or supplied to recovery. Every successful solve writes a complete
certificate, replays it in a fresh CPU process, rejects a scalar-plus-one
corruption and compares certificates across arms wherever the protocol
prescribes multiple arms. The generator/order witness is independently
checked with primality, the curve equation and a unique Hasse-interval
multiple. The original tiny constructor and default column limit remain.

The completed v3/v4/v5 device panels passed 13 Rust tests, host/device
arithmetic and relation differentials, complete reduction and replay checks,
Clippy, memcheck, racecheck and synccheck. V6 passed the full/cached/compact
variants and their query checks, then failed at the service-cap interface.
V7 adds boundary tests and actual maximum-cap service-header preflight.
Root crypto/ecbench was not built locally because of the user's disk
constraint. Current-head repository CI is a separate validation record.

- Frozen contracts: [v3](protocol-v3.json), [v4](protocol-v4.json),
  [v5](protocol-v5.json), [failed v6](protocol-v6.json), [v7](protocol-v7.json), [v8](protocol-v8.json).
- Complete exports: [v3](results-v3/export.json), [v4](results-v4/export.json),
  [v5](results-v5/export.json), [v7](results-v7/export.json), [v8](results-v8/export.json). Each includes raw rows, source/binary identities,
  archive-member hashes, certificate audit outputs and deletion receipts.
- Failed launches: [v1](failed-launch-v1/), [v2](failed-launch-v2/),
  [v6](failed-launch-v6/failure.json).
- Full raw certificates and logs: immutable `results.tgz` archives under
  `s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/e2e-20261030-vN/`.
  Exact digests are in each archive manifest. A manifest alone does not replace
  the executed certificate verifier or the archived certificate bytes.

The exports pass schema/flow/file-hash checks. The protocol-aware canonical
checker retains documented rectangular-coverage findings (3 for v3; 5 each
for v4/v5; 4 each for v7/v8). These plans deliberately prescribe fewer arms at larger fields
and stop after the first incomplete rung. Their frozen native analyzers
check the actual nonrectangular cell sets. This is not a full canonical
checker pass or independent scientific validation-gate approval.

Cold solve timing includes target-coordinate generation, process/GPU startup,
curve checks, all trials, base construction, relation search, elimination,
extraction, in-process replay, serialization and teardown. Additional fresh
replay and corruption checks are included in `wall_seconds` and separately
recorded. Fixture generation, compilation and preflight are outside solve
timing and retained in bootstrap records. Admission bytes are conservative
bounds, not measured peak VRAM. CPU isolation and stable GPU clocks are not
certified. EKS was not used; any future EKS image must come from crypto main.

[Inspected seven-page report](reports/run-20261030T0304Z/report.pdf) and
[benchmark browsing copies](../../benchmarks/20261030-explicit-relations-gpu/README.md).

## V11: complete prescribed recovery through 62 field bits

All 12 executions succeeded: eight matched 52-bit controls and both fixed
targets at 61 and 62 bits. Every certificate passed fresh CPU replay and
scalar+1 rejection; the prescribed 52-bit cross-arm certificates match exactly.
The measured source is `d44b59149f468357e86186f956ecb7e323f38915`.

The two 62-bit cold solves took 2139.880951918 and 2187.942594727 seconds; totals
including the separate audit were 2139.908483146 and 2187.971582474 seconds.
For this panel, target-coordinate generation is outside `solver_seconds` and
included in `wall_seconds`, along with fresh replay and corruption rejection.
All clocks remain L0 diagnostics with `wall_valid=false`. Table admission
estimates, including queried CUB scratch, were 48,798,250,775 and 49,539,778,479
bytes; these are not measured peak VRAM. No hardware maximum is established.

[Raw rows](results-v11/rows.csv), [export](results-v11/export.json),
[strict audit](results-v11/frontier-audit.txt), [validation scope](results-v11/CHECKER-NOTE.md),
and [inspected two-page report](reports/run-20261030T1550Z/report.pdf).
All 19 Rust tests and recorded device/header/Clippy/sanitizer checks passed.
Archive integrity and pod deletion were verified; raw evidence was pushed
before rendering. Schema/flow/file hashes pass; five canonical rectangular
coverage findings remain disclosed. The exact staged plan and eight mutation
controls pass the native coverage audit. V13 separately tests persistent
queries and full right-sum reuse; its results are not included here.
