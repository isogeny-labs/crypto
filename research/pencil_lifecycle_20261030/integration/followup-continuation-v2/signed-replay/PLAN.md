# Signed stream replay construction

T-1383, 2026-11-02. Implement bounded JSONL loading with duplicate-object-key rejection, independently expected header binding, strict sequence/event shape and canonical domains, ordered native graph replay and offline prefix information. Source references: ../packets/target-stream-reconciliation/SOURCE-MAP.md; ../signed-stream/PLAN.md; actual SignedGraph and observed collector. Preserve zero-node and [A,-b,-a] convention. Full walk/group-label invariant and source scalar verification remain separate. No timing or scientific cohort execution.

Use the six existing deterministic p739/seed47 controls with cap64/2048 and three backends, source-check successful scalars, and corrupt target/header/order/labels/signs/counters/terminal/limits/JSON keys. One build job, private cache, 3 GiB virtual memory, 180 seconds per command. Preserve failed checks and source identities. Independent review pending; do not promote source fidelity from producer tests.
