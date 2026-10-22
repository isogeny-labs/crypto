# Review data dictionary

The [protocol](PROTOCOL.md) fixes the quotient recurrence, signed relation
rule, cohort and 36 pricing cells. Every target is joined by `workload_id`
to a same-source rho record from the frozen archive, collected in a separate
process. The native Rust
`src/bin/analyze.rs` recomputes all prices from operation counts, checks
the model and target setup joins, independently replays source-field
`kP=Q`, and verifies every candidate solve.

| File | Unit of observation | Fields |
| --- | --- | --- |
| `results/cold-model-setup.jsonl` | one of 21 curve models | fresh rational-root enumeration, eligible theta orders and selected tuple, five independently derived torsion matrices, separate root/theta/torsion arithmetic and native field counts |
| `results/cold-target-setup.jsonl` | one of 1,260 public source targets | source transfer and source scalar verification, signed anchor/target, initial quotient pair, arithmetic and field counters |
| `results/{uncached,cached}/collector-{bits}.jsonl` | one end-to-end candidate solve per completed arm | source ID, scalar and verification, quotient steps, signed split relations, Recover exceptions, graph operations, Kummer/Recover/Jacobian field counters |
| `results/target-price.csv` | one arm, target and pricing cell | cold/online candidate GAE, rho GAE, both ratios and verification flags |
| `results/price-grid.csv` | one arm, bit slice or full cohort and pricing cell | summed cold/online candidate and rho GAE, ratios, target-level wins/losses |
| `results/summary.json` | whole run | target and relation totals, ratio ranges, applicable cell counts and cost boundary |

`walk_ops` and `recover_ops` count each executed base-field multiplication,
addition and inversion in their independent code paths. The native
`field_counts` count arithmetic used by the Jacobian, splitting test and
source-verification endpoint. The model and target setup have similarly
separated counted phases. A field inversion is priced as an `I` operation;
modular multiplications internal to its Fermat implementation are excluded
from `M` to avoid double counting. General modular multiplication, including
squaring by the same generic routine, is charged as `M`. The `s` count is
reserved for native field operations that call the separate square routine.
The cached arm charges eight initial inversions per target, four in each
independent xADD caller, and uses those inverses for all later xADD calls.

At `native_GAE_per_logical`, candidate logical work is the counted field
additions plus graph operations, one partition/hash event and two scalar
coefficient updates per quotient step, signed-relation dedup probes, and
walk restarts. Rho logical work uses the archive's `_uncharged` categories,
as in the native matched analyzer. The matrix/root setup is charged cold to
*each* target, not amortized across the 60 targets sharing that model.
