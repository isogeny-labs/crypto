# Kummer reference gate data dictionary

| Artifact | Unit of observation | Main fields |
| --- | --- | --- |
| `results/target-gate-<bits>.jsonl` | one frozen public target, seven coefficient-pair cases | `workload_id`, `workload_sha256`, `algorithm_seed`, `planted`, `source_scalar_verified`, `all_correct`, `cases[]` with `a`,`b`,`compatible_lifts`,`differential_correct`,`signed_recovery_correct`,`label_transfer_correct` |
| `results/model-exceptions-<bits>.jsonl` | one Scholten model, 30 exceptional cases | `p`, `r`, `root_count`, `exception_cases`, `all_correct`, `cases[]` with root, input class degrees, and lift counts |
| `results/summary.json` | native validation aggregate | population and failure counts, histogram of compatible lifts, per-bit counts, explicit `candidate_over_rho: null` |
| `results/price-grid.csv` | one size slice and one of 36 price cells | `S_per_M`, `I_per_M`, `native_GAE_per_logical`; three empty cost/ratio fields mean **not measured**, not zero |
| `results/manifest.json` | one run | source archive hash, revision, commands, populations, output digest paths and status |

All subgroup points are public known-answer fixtures. `K(D)` is represented
by the lexicographically smaller of a Mumford divisor and its negative. The
program's differential operation enumerates signed lifts with Jacobian
addition. Its operation counts and wall time are **not** estimates of a fast
Kummer implementation. `all_correct` is false if any source, transfer,
differential, or signed-recovery check fails.

The 36 prices are 3 squaring weights `{0,0.5,1}` times 4 inversion weights
`{1,10,100,1000}` times 3 logical weights `{0,0.1,1}`. The `all` slice and
seven bit slices each contain 36 explicit missing-cost rows. No ratio is
formed where either paired full-collector or rho cost is absent.
