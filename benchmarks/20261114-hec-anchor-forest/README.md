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

## Compact norm quotient addendum

[Compact report](COMPACT-REPORT.pdf), [graph](compact.svg),
[protocol](../../research/hec_anchor_relations_20261114/COMPACT-PROTOCOL.md) and
[raw](../../research/hec_anchor_relations_20261114/compact-run/paired.txt).
Sourceb2fddfd8b2 retains the signed relation stream while using only high norm coefficients
and bounded interpolation arrays. All110 reference/compact gate solves and165interleaved
timed solves pass local checks;22128exhaustive and8000random polynomial cases match reference
and full Newton interpolation. Producer assertions [pcc:unverified].

Reference-orbit/compact median cold ratios range0.83..1.32; the genus2p101 case regresses.
Frozen-table/compact ratios in this separate run range1.22..2.98. No default change and no
10x or novelty conclusion. Do not multiply ratios with earlier experiments. The next research
step is held-out growing-size confirmation, rather than equivalent arithmetic rewrites.

## Held-out dense curves and growing-size screen

[Report](HELDOUT-REPORT.pdf), [graph](heldout.svg), [editable native report source](HELDOUT-REPORT.typ),
[protocol](../../research/hec_anchor_relations_20261114/HELDOUT-PROTOCOL.md),
[raw archive](../../research/hec_anchor_relations_20261114/HELDOUT-RAW.tar.gz) and
[handoff](../../research/hec_anchor_relations_20261114/HELDOUT-STATUS.md).

All20dense fixtures,100group gates and1500timed cells passed local checks. Primary per-target
ratio is fastest of12controls / compact cold total. Five-target curve medians range0.94..3.17,
including a g3p1009 median below parity; individual targets reach4.15. A/A spread reaches3.09.
These are exploratory L0 observations with producer checks [pcc:unverified]. The10x objective
and mathematical novelty remain unestablished. No default/index/leaderboard change.

New scientific execution, custody collection, statistics and report generation are Rust/native
jq/Typst. Historical Python visual-report sources and their frozen evidence remain preserved.
Record20261115T0700Z-hec-heldout-scaling names the archive and independently hashed summary.

Unsigned claim receipt: PCC-20261115-38951c, standing unverified/not_covered. Its native
producer recomputation pins the compact raw observation table; independent signature/replay
is absent. CI acceptance is separate from scientific verification.
