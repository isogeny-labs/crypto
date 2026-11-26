# Incremental column maintenance in `eliminate` (2026-11-25)

Change: `src/cryptanalysis/large_prime_filter_mst.rs`, `eliminate`. The MST
filter's outer loop rebuilt the column map from every row, rescanned every
row for the atom total and the weight/source caps, and rebuilt the row
vector after each pivot: O(rows x pivots). It now keeps a slab of rows with
append-only ids, one id-ordered member set per large column, a
`(degree, column)` priority set, a running atom total and a running count
of rows over the caps. All three tree strategies use it. [pcc:unverified]

**Exactness.** The original rebuilt vector is "survivors in order, then the
new edge rows in order", which is the slab's live rows in id order. So
member order (Prim vertex numbering and ties), column choice (first of the
priority set), refusal points and output order are unchanged.

**Differential check.** `lp_mst_dump.rs` (here) was built against the
pre-change and post-change source in two separate crates and run on 4000
seeded streams (6 primes, 2..121 rows, tight caps and budgets in part,
hot target column in a fifth of them), the M31 613-row stream, its
512-row prefix, and the four synthetic streams below; three strategies each.
Both builds print the same 12,018 lines, byte for byte (8,418 completed runs,
3,600 refusals), digest `12dc7dad43b412a8314d415b741405f6ad4d2e7276291068e6322b81265168ce`.
Mutations that the differential catches: taking the largest-degree column,
reversing member order, not removing a row's atoms, dropping the
over-cap refusal. One surviving mutation (not counting new rows as over the
caps) cannot change the result: new rows are selected tree edges, and every
strategy admits only edges within the weight and source caps.

**M31 613-row stream, MST stage** (callgrind instructions per run, same
method as `../CALLGRIND.txt`; wall medians of 201 interleaved samples):

| arm | before | after | ratio | wall before | wall after |
|---|---:|---:|---:|---:|---:|
| exhaustive | 29.43 M | 7.71 M | 0.262 | 3.34 ms | 0.88-1.07 ms |
| overlap-sensitive | 32.75 M | 11.02 M | 0.337 | 3.79 ms | 1.28-1.53 ms |
| shared-key | 29.21 M | 7.42 M | 0.254 | 3.43 ms | 0.90-1.08 ms |

**Scaling, exhaustive arm, synthetic streams** (`SYNTHETIC_SHA256SUMS`;
generator: rows `n`, LP ids below `n/2`, 0-2 large primes, 1-4 small
columns below 4096 plus the target column with one of 10 coefficients,
Python `random.Random(n)`), wall median of 7:

| rows | trees | before | after | ratio |
|---:|---:|---:|---:|---:|
| 500 | 143 | 2.44 ms | 0.75 ms | 3.2x |
| 1000 | 289 | 10.78 ms | 1.64 ms | 6.6x |
| 2000 | 516 | 38.28 ms | 3.03 ms | 12.6x |
| 4000 | 1054 | 168.74 ms | 6.68 ms | 25.3x |

The old loop grows about 4x per doubling (quadratic), the new one about 2x.
Wall times: one shared 4-vCPU x86_64 container, informational.
