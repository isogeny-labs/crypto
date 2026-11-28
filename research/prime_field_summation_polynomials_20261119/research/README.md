# Optimization campaign

The optimization work starts from baseline commit `073bdf3`. Every track must
retain exact correctness checks, commands, seeds, timing boundaries, raw output,
and failed or neutral results.

| Track | Required correctness gate | Performance scope |
| --- | --- | --- |
| Projective expansion tree | Exact finite multiset and infinity count versus baseline across all fixtures | All-roots extraction, with sort/dedup reported separately |
| Fixed-field arithmetic | Edge and randomized equality with generic modular arithmetic | Field operations and matched root extraction |
| Workspace/SIMD | Exact roots including degenerate additions | Allocation, generation, and sorting isolated |
| Product tree | Exact coefficient vector including repeated roots | Coefficient materialization only |
| One-root/target search | Returned value belongs to exhaustive root set; exact miss handling | One-root and target-query workloads, not all-roots output |
| Automorphism quotient | Concrete map, field definition, input/factor-base invariance, orbit accounting | Only compatible invariant workloads |

No track may extrapolate a component ratio into an end-to-end index-calculus or
ECDLP gain. The final integration benchmark must state whether point lifting,
precomputation, coefficient construction, sorting, and verification are included.
