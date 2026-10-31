# Run report: `e7b_k0` (2026-10-18, 16:06:42 to 16:09:45 UTC)

Descriptive record of one `ecbench` session.  Interpretation is in the
round's [README](../../README.md).  PDF: [`report.pdf`](report.pdf).

| item | value |
|:--|:--|
| protocol | [`PROTOCOL.md`](../../PROTOCOL.md), E7b |
| spec | [`specs/e7b_k0.json`](../../specs/e7b_k0.json) |
| session | `ECBS1heff1e81e9e36`, git commit `6574178b6`, `git_dirty` false |
| host | cloud container; isolation earned L2 on 44 records, L1 on 36 (the spec asked for L2) |
| workloads | `icv1-f2m19-t797-b6cf2467` and `icv1-f2m23-t5197-69e76b73` (the a = 0 Koblitz curves over F_2^19 and F_2^23, h = 4), 4 targets × 2 rounds |
| arms | `rho.negation`; blind F4 descent and blind MITM, each on `binary-subspace:dimension=9` with and without `trace_kernel=1` |
| runs | 80 requested, 80 verified |
| audit | [`sessions/e7b_k0.audit.json`](../../sessions/e7b_k0.audit.json): ok, 12 replays reproduced, 0 problems |
| per-arm values | [`TABLES_E7.md`](../../TABLES_E7.md) |

**Measured** (per curve, over 8 runs; relations and trials are summed):

| curve | arm | relations / trials | F4 ops per relation | max degree | S | online wall per run |
|:--|:--|:--|--:|--:|--:|--:|
| F_2^19 | f4 default | 808 / 3,488 | 4,329,947 | 3 | 4.93 | 2.93 s |
| F_2^19 | f4 trace-kernel | 952 / 2,777 | 4,964,166 | 3 | 4.90 | 5.15 s |
| F_2^23 | f4 default | 758 / 47,189 | 4,634,927 | 3 | 5.43 | 7.63 s |
| F_2^23 | f4 trace-kernel | 929 / 29,782 | 3,329,596 | 3 | 3.86 | 6.74 s |

The MITM arms share the bases and give the same relations and trials.  Their
`S` is 195.75 against 160.82 at F_2^19, and 53.13 against 49.69 at F_2^23.
Rho's `S` is 1.72 and 1.18.  Wall times are host observations, not admitted
measurements.

Derived in `TABLES_E7.md`: trace-kernel ÷ default.
- Yield is 1.48 [1.37, 1.60] at F_2^19 and 1.94 [1.77, 2.14] at F_2^23.
- Default ÷ trace-kernel F4 ops per relation is 0.87 and 1.39.

Graph: [`e7b_yield_ops.svg`](e7b_yield_ops.svg), from [`plot.py`](plot.py).
