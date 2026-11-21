# Dual-isogeny reused-setup comparison, 2026-11-18

These are browsing copies from the [study](../../research/isogeny_torsion_fibers_20261012/dual_projected_base_20261118/RESULTS_V1.md). The study holds the frozen protocol, public inputs, native source, raw records, independent replay receipts, and SHA-256 manifest. This is a **secondary amortization diagnostic**; its per-target batch quotient is excluded from the single-target ECDLP ranking. The primary [cold study](../../research/isogeny_torsion_fibers_20261012/dual_projected_base_20261115/RESULTS_V1.md) charges setup on every fixed-target solve.

The 64-target, three-round panel completed all 18 children on an Apple M4 Pro shared host (L0). In the counted **group-operation component** of whole-batch work, mean `S_group` was 1,714.111 for direct dual, 1,714.442 for direct inverse Vélu, and 6.654 for same-target signed rho. The inverse/dual ratio was 1.000193. A complete portable `S` and isolated wall-time ratio remain unset.

- [Run report PDF](RUN_REPORT_V1.pdf)
- [Results PDF](RESULTS_V1.pdf)
- [SVG graph](S_GROUP_GRAPH_V1.svg) and [150 dpi PNG](S_GROUP_GRAPH_V1.png)

These copies have the same SHA-256 digests as the study originals. The full-library validation gate failed on a duplicate test declaration already present in the base revision; the [validation receipt](../../research/isogeny_torsion_fibers_20261012/dual_projected_base_20261118/VALIDATION_V1.md) records the failure. This entry is pending index admission and does not assert a complete ECDLP speed ratio.
