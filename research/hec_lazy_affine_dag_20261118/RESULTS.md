# Deferred affine expansion

All four solvers returned the planted scalar on all 44 fixtures. Each scalar passed multiplication on the original curve. The two graph arms matched the prior schedule on accepted rows, anchors and outcome. The process exited successfully after 176 counter captures and 2,112 timed calls.

Each speedup divides the corresponding comparator's median wall time by the deferred solver's median on the same input. The ranges span fixtures within each genus. Links are totals across that genus.

| Genus | Fixtures | Original / deferred | Prior / deferred | Eager / deferred | Original / deferred REDC count | Links expanded, eager → deferred |
|---|---:|---:|---:|---:|---:|---:|
| 2 | 24 | 1.261–2.444× | 1.197–1.588× | 0.984–1.128× | 1.212–1.696× | 4,940 → 1,596 |
| 3 | 12 | 1.744–3.098× | 1.115–1.649× | 0.979–1.064× | 1.547–2.429× | 914 → 625 |
| 4 | 8 | 2.652–5.171× | 1.085–1.355× | 0.999–1.096× | 2.320–4.516× | 112 → 106 |

The graph solver with deferred expansion beat the prior schedule on every fixture in this run. Deferring expansion by itself produced smaller, mixed timing changes. Genus two skipped 3,344 link expansions, but the largest wall improvement over eager expansion was 1.128×. The 10× full-solver goal remains open. These are same-host measurements on the registered curves and targets. The benchmark does not estimate an asymptotic exponent.

RUN.txt identifies the remotely verified input commit. INPUT.sha256 and HASH-CHECK.txt cover the build and source inputs. CONTROL.txt contains counter vectors and individual timings. COMPARISON.tsv contains fixture medians and ratios. Git holds the experimental record during the S3 outage.
