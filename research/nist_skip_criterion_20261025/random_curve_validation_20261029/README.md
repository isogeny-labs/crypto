# Validation requested after the replication manuscript

The user's four validation requirements are retained in PROTOCOL.md.
MECHANISM.md explains the statistic and the missing ECDLP implications.
The existing paper is preserved; this is a separate prospective design.

| Requirement | Current status | Evidence or remaining work |
| --- | --- | --- |
| 10,000 accepted prime-order curves per field | Not run | 60,000 accepted primary draws required; no percentile reported |
| Same seed procedure where possible | Design written | Pinned SHA-1 seed-to-curve map; explicit uniform-seed primary versus drand48 sensitivity distinction |
| Sweep 100 through 5000 | Not run | Existing frozen sweep stops at 2000; new native source will be frozen before execution |
| Fix criterion and cutoff before new data | Written, execution freeze pending | Strict +1 only, cutoff 691, lower direction; already observed targets disclosed |
| Explain mechanism and limits | Documented | MECHANISM.md; no ECDLP attack or historical motive established |
| EKS execution | Blocked | No installed arbitrary-command MCP; portable request/recovery contracts support ecbench rather than the required PARI worker |
| Checkpoint | Pending authorization/inspection | Earlier paper S3 export was rejected; do not export its payload through this study |

No scientific run, new 5000-cutoff result, 10k baseline, canonical protocol
freeze or uploaded checkpoint is implied by these preparation files.

## Native preparation

All four tools compile as C++17 with warnings treated as errors:

- sampler.cpp implements the primary seed map, PARI prime-order search,
  proved primality testing, separate GMP order/point witnesses and journals.
- cutoff_sweep.cpp implements all 4901 integer cutoffs and the expanded
  retrospective label calibration.
- verify_sweep.cpp separately reconstructs the five sweep outputs using PARI
  characters and a brute-force label inventory.
- percentiles.cpp requires all six 10,000-row accepted populations, recomputes
  strict counts using Euler's criterion, and computes ties, marginal
  Clopper--Pearson intervals and Holm correction. CDF/percentile columns are
  fractions from zero to one; multiply by 100 to display percentages.

Run build.sh with a fresh external build directory; PARI_ROOT may select the
tested PARI installation. Each binary has a --self-test mode. Sampler self-tests
additionally take the original inputs.tsv path. These bounded development tests
do not generate new production curves. DEVELOPMENT.md records attempts and gaps.

The primary sampler is implemented but is not production-qualified. A separate
full-journal verifier and interruption-safe contiguous-prefix scheduling are
still required. The matched-drand48 sensitivity worker and baseline percentile
trajectories through 5000 are not implemented. Their pending status preserves
the full requested work; these tools are not a completed experimental result.
