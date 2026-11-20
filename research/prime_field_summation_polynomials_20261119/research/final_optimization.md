# Integrated optimization campaign

Date: 2026-11-19 UTC

## Status against the requested leads

| Lead | Status | Integrated evidence |
| --- | --- | --- |
| One root without enumeration | Recorded local self-test pass `[pcc:unverified]` | 900 randomized certificates plus explicit infinity cases; `one-root` CLI |
| Level-batched/projective expansion | Recorded local self-test pass `[pcc:unverified]` | 900 randomized and 31 degeneracy checks; affine and Jacobian paths retained |
| Fixed-field reduction | Recorded local self-test pass for the stated reducers `[pcc:unverified]` | 2,268,486 products and 1,225 inverses; generic fallback retained |
| Reusable workspace/output isolation | Implemented and verified `[pcc:unverified]` | Exact common-path/fallback checks; generation and sort/dedup measured separately |
| Explicit SIMD | Investigated, not implemented | Compiler report shows control flow, prefix dependencies, and throwing paths still block vectorization |
| Fast coefficient materialization | Recorded local self-test pass `[pcc:unverified]` | 135 self-test comparisons and 180 branch benchmark checks through degree 4096 |
| Target-aware search | Recorded local self-test pass `[pcc:unverified]` | 900 hits, 900 misses, nine edge fixtures, sign-mask certificates |
| Curve automorphism folding | Algebraically verified `[pcc:unverified]`; performance-negative beyond tiny degree | 84 controls and 2,600 branch benchmark validations; covariance/invariance gates retained |

The six parallel tracks were combined in source snapshot
`71b769857f99f57e0e3490ef7ced32d43b307a1d`; separate track commit identities
are not preserved by this import. The snapshot includes a recorded combined
ASan/UBSan self-test pass `[pcc:unverified]`. The final receipt is:

```text
PASS self-test s3=900 s4=900 root_methods=900 one_root=900
target_hits=900 target_misses=900 target_edges=9 targeted_degeneracies=31
discriminants=900 coefficient_methods=135 reducer_pairs=2268486
reducer_inverses=1225 automorphisms=84
```

## Serial benchmark boundary

The final benchmarks ran serially on the recorded 5-vCPU AMD EPYC 9V74 host,
using g++ 14.2.0 with `-O3 -march=native`, seed
`6000285944033488934`, and the same curve `(A,B)=(2,3)` unless stated.
Root timings include sorting and deduplication where labeled “distinct.” Point
lifting and relation-system work remain excluded. Values are aggregate means from
one process, not confidence intervals.

## All-roots extraction

Degree 128, 200 validated cases and 20 repetitions:

| Field | Generic factorization | MITM distinct | Workspace distinct | Affine tree distinct | Selected best | Generic / best |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1009 | 4,317.40 us | **10.54 us** | 10.57 us | 12.72 us | MITM | 409.7x |
| 1,000,003 | 12,847.07 us | 10.17 us | **10.03 us** | 10.92 us | workspace | 1281.2x |
| 2,147,483,647 | 14,620.25 us | 11.26 us | 11.18 us | **10.50 us** | affine tree | 1392.6x |

The affine tree is strongest through most small and medium degrees. Small-field
degeneracy changes the crossover, so `auto` uses measured per-field cutovers for
the three campaign fields and defaults to the affine tree elsewhere. Every method
remains explicitly selectable.

Reusable-workspace generation alone took 6.87, 7.05, and 8.25 us at degree 128;
sorting/deduplication alone took 3.36, 2.67, and 2.74 us. Thus output
canonicalization is now a material fraction of matched all-roots time.

## Division-free field reduction

Forced-generic versus automatic reduction at degree 128:

| Field | Compared matched path | Generic reducer | Automatic reducer | Ratio |
| ---: | --- | ---: | ---: | ---: |
| 1009 | MITM distinct | 10.51 us | 10.54 us | 1.00x (no measured gain) |
| 1,000,003 | workspace distinct | 13.49 us | 10.03 us | 1.35x |
| 2,147,483,647 | affine tree distinct | 14.25 us | 10.50 us | 1.36x |

The binary audit confirms specialized hot branches avoid `__umodti3`; only the
generic fallback retains it. The 1009 result is a measured neutral case, not a
failure of correctness.

## Coefficient materialization

At degree 4096, five validated cases and three repetitions:

| Field | Quadratic expansion | Karatsuba subproduct tree | Ratio |
| ---: | ---: | ---: | ---: |
| 1009 | 52.40 ms | 16.13 ms | 3.25x |
| 1,000,003 | 52.53 ms | 16.36 ms | 3.21x |
| 2,147,483,647 | 38.30 ms | 2.85 ms | 13.42x |

Degree 256 remains the conservative universal automatic crossover. The product
tree crosses earlier for the Mersenne field, but a global degree-128 switch would
regress the other measured fields.

## One-root and target-aware search

Representative field `p=1,000,003`, degree 4096, 20 validated cases and five
repetitions:

| Target workload | Full enumeration/membership | Target search | Full / target | One signed sum | Full / one |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1 target, miss | 274.01 us | 29.37 us | 9.33x | 1.39 us | 197.0x |
| 1 target, hit | 281.74 us | 32.28 us | 8.73x | 1.18 us | 237.9x |
| 64 targets, miss | 295.82 us | 210.54 us | 1.41x | 1.71 us | 173.2x |
| 64 targets, hit | 293.08 us | 180.73 us | 1.62x | 1.19 us | 246.3x |

At degree 128, one-target search measured 1.28x–1.48x over full membership;
four or more targets could already lose. Target-aware search is therefore an
application-specific path, not the default all-roots algorithm.

## Automorphism result and negative control

For invariant `j=0` inputs, the orbit evaluator measured 1.32x–1.41x faster `[pcc:unverified]` at
degree 4, but became 3.67x–4.66x slower than MITM at degree 256 despite reducing
256 sign classes to 88 orbits. The `j=1728` evaluator was slower at every measured
degree and 5.69x slower at degree 512. None of 2,600 random branch controls were
invariant.

The algebraic quotient is real, but the current serial-affine representative
evaluation does not convert it into a general root-stage gain. Combining orbit
representatives with level batching remains an open experiment; no end-to-end
speed claim is made.

## Reproduction

```bash
make test
./scripts/run_final_optimization_campaign.sh
sha256sum -c results/final/SHA256SUMS
```

Raw serial CSVs/logs and their manifest are in `results/final/`. Track-specific
reports preserve provisional attempts, neutral results, sanitizer receipts, and
accounting details.
