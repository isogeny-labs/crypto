# Hyperelliptic principal-relation pilots

[Report](REPORT.pdf) and [graph](pilots.svg). All numerical results here are producer
assertions pending independent replay and signed validation [pcc:unverified].

This evidence package preserves five separate records: unrestricted principal relations,
tabulated cubic/quartic oracle component, cold table pipeline, first anchor forest and signed
orbit forest. Do not multiply their ratios or treat cross-run controls as paired measurements.

The signed-orbit pilot completes55 group-gated cases and110 interleaved cold solves, with
all answers checked locally. Ratios of five-seed table/orbit median totals range1.27..2.94.
The table controls are frozen per size from the earlier pipeline screen; all construction,
descents, forest and final solve/verification costs are charged cold. L0 macOS ARM, one host.
This supports further investigation; it does not meet the mathematical10x objective.

Sources and protocols:

- [Principal derivation](../../research/hec_relation_math_20261114/DERIVATION.md)
- [Table protocol](../../research/hec_table_oracle_20261114/PROTOCOL.md)
- [Anchor and orbit protocol](../../research/hec_anchor_relations_20261114/PROTOCOL.md)
- [Orbit raw paired data](../../research/hec_anchor_relations_20261114/run/orbit-paired.txt)
- [Orbit group gate](../../research/hec_anchor_relations_20261114/run/orbit-gates.txt)
- [Summary](../../research/hec_anchor_relations_20261114/run/orbit-summary.json)
- [Report renderer](../../research/hec_anchor_relations_20261114/render_report.py)

External comparison: not run. No fastest-implementation, asymptotic improvement, or novelty
claim. Formal algebra receipts cover the identities named in the report, not the Rust program.
This entry is kept out of the benchmark index and leaderboard until their validation gate is met.
