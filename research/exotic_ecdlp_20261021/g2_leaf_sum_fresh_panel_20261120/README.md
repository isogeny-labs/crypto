# Leaf-sum coordinates reduce S3 F4 work across five fixed residuals

The invertible substitution `s = u + v` reduced msolve 0.11.0's total rows reduced from 140,481 to 112,481 (19.9%) and pairs reduced from 70,091 to 56,207 (19.8%) across five preregistered, distinct N31 S235 residuals. Both arms produced identical Boolean root sets after inverse mapping on every residual. Local exploratory wall time totaled 425.001 seconds for the control and 297.460 seconds for leaf-sum, a paired total ratio of 1.429. The five pairwise wall ratios had median 1.380. These are point-decomposition stage diagnostics under the local host envelope.

## Method and results

The frozen [protocol](PROTOCOL.md) selects S235 query indices 2 through 6 from `relations.json`, subtracts the same factor-base point at index 2, and constructs a 30-variable trace-zero S3 system for each resulting point. The q-specific trace relation removes one redundant field equation. Every arm adds 30 Boolean equations; leaf-sum applies the exact invertible change of variables and preserves the Boolean ideal. Inputs for the earlier single residual were reproduced byte for byte before the new inputs were generated. The [preparation receipt](preparation.json), [run receipt](panel_run.json), [validation protocol](validation_protocol.json), and [validation result](validation.json) retain source and artifact hashes.

| Query index | Archived query status | Control roots | Factor-base pairs for fixed singleton | Control wall s | Leaf-sum wall s | Control / leaf-sum | Rows reduced control → leaf-sum | Pairs reduced control → leaf-sum |
| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 2 | exhausted_miss | 0 | 0 | 102.928 | 58.848 | 1.749 | 22,659 → 17,961 | 11,309 → 8,980 |
| 3 | verified_hit | 2 | 0 | 78.855 | 63.584 | 1.240 | 36,108 → 29,284 | 18,011 → 14,627 |
| 4 | verified_hit | 2 | 0 | 75.950 | 56.546 | 1.343 | 35,876 → 29,288 | 17,903 → 14,629 |
| 5 | exhausted_miss | 0 | 0 | 81.159 | 58.815 | 1.380 | 23,169 → 17,981 | 11,556 → 8,988 |
| 6 | exhausted_miss | 0 | 0 | 86.108 | 59.666 | 1.443 | 22,669 → 17,967 | 11,312 → 8,983 |

All ten arms completed with exit code zero inside the frozen 300-second wall, 1-GiB observed process-tree RSS, and 20-MiB output caps. They used the same pinned msolve and FLINT binaries, one solver thread, identical residuals and equation ideals, and alternating arm order. The checked Sage runtime record was saved before the workload. Raw `.ms` inputs, bases, stderr, and stdout are retained under `inputs/` and `runs/`.

The validator parses each completed basis, evaluates every input equation at each root, maps leaf-sum roots through `v = u + s`, and independently enumerates S3 solutions by solving the field equation for every `u` in the 15-dimensional selected subspace. Indices 3 and 4 each have two Boolean roots and a group-point lift summing to the respective residual; neither yields a factor-base pair for the fixed singleton. The archived `verified_hit` labels belong to the original query schedule, which allowed other factor-base triples. Indices 2, 5, and 6 have the unit ideal in both arms. The [validation amendment](VALIDATION_AMENDMENT.md) records the initial overstrong group-membership assertion and its correction.

The next measured step is a Tier 1 paired rerun on a frozen instance with exposed topology, affinity, cgroup and noise receipts. A complete IC candidate comparison would additionally measure its target-dependent query, decomposition, relation checks, descent, and recovery check against same-point rho under the repository measurement contract.
