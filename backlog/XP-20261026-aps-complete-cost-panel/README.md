# APS complete-cost comparison — blocked

This card preserves the requested validation beyond the native correctness
implementation in [the study](../../research/aps_extensions_20261026/README.md).
Its five mechanisms and acceptance criteria are in
[TRACKER.md](../../research/aps_extensions_20261026/TRACKER.md); the prospective
controlled panels are in
[PROTOCOL_PLAN.md](../../research/aps_extensions_20261026/PROTOCOL_PLAN.md).

The question is whether any mechanism reduces complete cold verified cost
against the matched APS baseline. A verified reduction chooses the next IC
baseline, even if rho is still cheaper. A null, failed or price-sensitive
result is also recorded and leaves the baseline intact.

## Prerequisites

The [accounting problem](../../problems/aps-single-relation-complete-cost.md)
must be fixed first: full phase counters, native ecbench/export adapter,
independent transcript replay, and reviewed support for the intended parameter
ladder. The fixture runner's F4 counter is insufficient. The future
`aps_complete_cost_panel` binary in `card.json` **does not exist**; this is
not a runnable command or a ready card.

Choose sample sizes and held-out instances, freeze the schema-valid protocol
and analysis, pin a merged code commit, supply a resumable runner/checkpoint
and independent verifier, and pass the S3 checkpoint gate. The upload
credential was absent in the authoring session. No benchmark was launched.

The [panel design](../../research/aps_extensions_20261026/PROTOCOL_PLAN.md)
specifies fixed-base presentation comparisons, distribution-matched label
arms, orbit controls, A/A and positive/negative controls, failures and
deduplication, matched strongest rho and an external solver comparison.
It is not frozen and contains no invented runtime or power estimate.

## Cost derivation and proposed priority

Runtime estimates are **unknown** until a separately preregistered pilot
measures the completed instrument. The card's one CPU, 8 GiB RAM, 10 GiB disk
and four-hour shard timeout are provisional ceilings, not extrapolated costs
or a request to provision a machine. Before `ready`, derive fixed and per-row
cost, total work from the chosen sample count, uncertainty/resolution and
headroom from cited pilot receipts; amend these ceilings if required.

Proposed funding is zero while blocked, following the existing draft
`XP-20261016-prime-base-oracle-tuning` card. Making this card ready requires a
reviewed priority proposal relative to the existing seed-card floor. This PR
does not post an objective, reward, lab task or worker.

## Done means

Every planned row is retained in one `experiment.export/v1` with its raw
records and flow counts, complete exclusive phase costs, independent replay
and metric validation, strongest matched rho at the common price bands,
and same-input external polynomial-solver output equality. The native
measurement and validation gates decide which comparisons are admissible.
Result reports, source-linked graphs and inspected PDFs land back in the
study by PR, including honest null or negative results.

The current tiny public tests are a correctness prerequisite, not this panel's
held-out data, a speed comparison or a reproduction of APS's full paper ladder.
