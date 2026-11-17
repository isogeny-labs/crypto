# Full-D18 phase-first public natural screen

The [frozen protocol](../PHASE_FIRST_NATURAL_PROTOCOL.md) used the complete
primary `a=0` GF(2^83) D18 S3 factor base: 32,643 signed-Frobenius columns
and 5,418,738 point rows. A fresh S3 GET passed the compressed SHA-256,
provider checksum, length, manifest binding and importer replay. Both arms
used clean source commit `43a422ce66ebce82b48af036d15dcbef8a5ac785`,
the same release executable (SHA-256
`9605c08a4aeeb61e6ce3196cb74d5a87ddea9d13f9f1d79a0a4e643d85ac5152`),
the same S3 input, all selectors and Frobenius phases admissible, and the
phase-zero-first search policy. They ran serially on an L0 shared Mac host
without a hard memory cgroup.

| Arm | Terminal receipt | Import | Model | SAT search | Process wall | Supervisor maximum RSS |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| Checked constructed control | `PASS_CONSTRUCTED_CONTROL` | 189.298 s | 0.614 s | 1.388 s, zero conflicts | 193.082 s | 1,817,919,488 bytes |
| One public natural target | `UNKNOWN_wall_cap` | 144.502 s | 0.310 s | censored; 154.117 s elapsed after model construction | 300.025 s cap | 1,996,554,240 bytes |

The constructed arm returned the archived four-point row `[0,166,332,498]`,
rank one and a changed-target rejection. The independent Python auditor
recomputed the original-field group sum and row digest, checked both S3
provider receipts, source and binary identity, exact circuit shapes and raw
supervisor outcomes, and rejected altered-target and altered-row-digest
controls. Its output is [`audit.json`](audit.json); run [`audit.py`](audit.py)
with a fresh base directory and the exact binary to replay it.

The natural arm built 329,726 variables, 2,807,254 clauses and 3,901 XOR rows.
It reached the internal 300-second whole-process cap before the solver
returned; the independent 310-second supervisor cap was not reached. No
natural relation or conflict count is observed in this censored receipt.
The raw [positive](positive) and [natural](natural) directories retain UTC
times, commands, phase files, shapes, stdout/stderr, exit codes and resource
measurements. The archived object and manifest GET metadata are retained here.

The separately [approved source archive](source-archive-custody.json) was
uploaded with an immutable conditional put to S3 and passed a fresh GET,
SHA-256, provider checksum, gzip and full tar-member integrity check. It
snapshots earlier import source commit
`54cbef1a6ce834a77632c1867acc801a31268ca2`; use PR head and the
frozen `43a422...` source commit to reproduce this phase-first pair.

The [validation receipt](validation.json) retains compressed raw logs. The
required two-thread release library run passed 4,165 tests with 117 ignored;
the touched example passed 19 tests, the N83 Python suite passed 63, the
boundary Python suite passed 52, and formatting passed. An initial four-thread
library run had two unrelated JV-cover time-limit failures. Each passed alone
and both passed in the complete lower-concurrency rerun.

This is one bounded natural query, not a measured relation rate or a
factor-base selection. Full rank growth, relation and log collection,
unseen-target descent, complete cold one-target cost and matched same-target
rho are still required. `selected_best_total_runtime` remains `null`.
