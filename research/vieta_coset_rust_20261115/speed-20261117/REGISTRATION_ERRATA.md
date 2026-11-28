# Frozen seed registration mismatch — retained, not repaired retrospectively

Both protocols and all pre-call configuration/source/binary bytes remain
unchanged. The native instrument followed the published per-configuration
JSON seed, root-splitting seed0x53500000+input index and certification seed42.
The preregistration's `explicit_list` field supplied six configuration seeds.
The canonical helper instead cycles that list for **every arm × instance ×
replicate** execution cell. Those are different rules. This was an authoring
error and was found during post-call export validation, not before freezing.

The final exports retain the actual configuration seed, with numeric instance
indices mapped to the frozen six-configuration order. Physical IDs, target
indices and full scope remain in each raw file. Official file-verifying
`check-export` reports360 primary and60 calibration seed mismatches, and no
other findings. Logs remain under evidence/export-*-check.log. No seed was
relabeled to make that checker pass; no checker/gate/protocol was weakened.

The separate native `run/seed-provenance-audit.json` verifies all26 pre-call
published file hashes and every576 cell/96 input scope against the exact
published configurations. It supports the stated actual workload and absence
of a posthoc workload change. It does not turn the canonical protocol into a
pass, confer independent standing, or replace the failed canonical check.
The existing complete-set replay, count controls and descriptive raw cost
observations remain separately reproducible. They are not presented as fully
protocol-qualified or admitted timing evidence.

The earlier derived-export enum error (`analysis_output` rather than `summary`)
and descriptive-versus-numeric instance-ID error were metadata writer defects.
Their failed logs and local invalid derivative hashes/copies are retained.
Those corrections changed no raw call, seed, target, budget, source or frozen
analysis result. The remaining360/60 seed findings are substantive registration
findings and are deliberately left visible.

For any *future* run, use the schema's `harness_drawn` method with the exact
published deterministic configuration/target/root seed rule, or an explicit
cell-level list ordered exactly as the canonical helper consumes it. A small
native transport/export fixture must pass the canonical seed check before a
new freeze. No new run or retroactive amendment is performed here.
