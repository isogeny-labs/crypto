# Primary N83 natural-query external XOR-SAT result

The [frozen protocol](PROTOCOL.md) exported the first all-affine identity
pattern of the same public `a=0` Koblitz N83, canonical six-summand 4+2 query
used in the [hosted native 100,000-conflict pair](../hosted-m6-highcap-20261018/RESULT.md).
The sequential K600 base and Gray-prefix or hash-derived K600 residual pool
were reimported from the exact archived S3 object bytes. Both pre-solve XOR
circuits matched the respective hosted native circuit counts, target,
coefficients, and equal-domain-run ordering.

| Residual pool | Residual signed orbits | Variables | CNF clauses | XOR rows | Formula bytes | CryptoMiniSat wall, local L0 | Outer process wall, local L0 | Outcome |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| Gray-prefix | 97 | 118,930 | 548,356 | 6,059 | 11,653,649 | 630.121 s | 637.474 s | `UNKNOWN_outer_wall_cap` |
| Hash-derived | 600 | 120,942 | 634,872 | 6,059 | 13,174,227 | 630.016 s | 638.011 s | `UNKNOWN_outer_wall_cap` |

CryptoMiniSat 5.14.7, binary SHA-256
`a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac`,
ran with one thread, a 5,000,000-conflict setting, a 600-second internal time
setting, and a 630-second outer watchdog per arm. The outer watchdog killed
both processes with no stdout or stderr; neither solver reported a conflict
count, SAT model, or UNSAT disposition. These are censored one-pattern
observations. In particular, no conclusion about natural relation yield or
the other 15 identity patterns follows. The shared macOS host had load
averages above 50 before both calls, so the local wall times cannot rank the
two factor bases or be compared directly with the hosted Linux native calls.

The independent raw [Gray audit](results/local-20261018/gray_pool-audit.json)
and [hash audit](results/local-20261018/hash_pool-audit.json) passed. They
rechecked exact compressed base objects, source commit and exporter bytes,
public target and coefficients, full XOR-DIMACS row counts, output digests,
outer-cap disposition, and the conservative charge. A changed Gray conflict
cap was rejected in the retained [mutation control](results/local-20261018/gray-mutation-control.json).
Both eight-member raw archives were written once under the requested S3
bucket, with SHA-256 checksums and AES256 server-side encryption. Separate
fresh GETs and an [independent S3 audit](results/local-20261018/s3-audit.json)
verified every archived worker, formula, raw solver file, result, and audit
member. The archive SHA-256 values are
`1beea0fa3d3e1b3f99bb8f0d09a2098dba6afd198b0491e99df43a58bc020616`
for Gray and
`65f3b2e55e3922e61bb04d1de1f0d4f8e9442f42d13ae6b4deab30b072ef559e`
for hash; the exact keys are in the S3 receipts.

The conservative incremental solver envelope is **1,920 seconds**, 960 per
attempted arm, including export, outer solver allowance and replay overhead.
Build and validation work and all earlier attempts remain additional costs;
this envelope is not a cold IC runtime. Release library validation passed
4,085 tests with 117 ignored and zero failures; the touched release example
passed six tests, and the N83 Python discovery suite passed 63. The exporter
source is commit `a2f64e27e1`; the strengthened independent audit and both
clean run receipts bind commit `53b58f6dc6`. Useful natural relation yield,
large-prime closure, rank, linear algebra, target extraction, and matched rho
remain unmeasured on the primary curve. `selected_best_total_runtime` remains
`null`.
