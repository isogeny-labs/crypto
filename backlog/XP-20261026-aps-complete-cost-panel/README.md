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

The [2026-10-29 replay continuation](../../research/aps_replay_20261029/README.md)
adds independent BigUint EC/sampling/relation checks and complete-set oracles
for emitted fixed-fixture transcripts. This supplies one replay component;
it does not make this card ready. Full pricing, native ecbench/export records,
pre-transcript abort outcomes, metric validation and the frozen protocol remain
open. The future timed driver must freeze the common online verification
interval and report post-run completeness auditing separately. No cost claim
may be inferred from a replay receipt whose costs are explicitly unattested.

## Four additional comparisons (2026-10-28)

The [follow-up implementation](../../research/aps_followup_20261028/README.md)
provides correctness-tested interfaces for:

1. Exhaustive versus first-informative recovery, keeping the root strategy
   and bases fixed. Retain B=0 candidates, failed branches and incomplete
   searches; a verified early witness is not complete enumeration.
2. Legacy full-field scan versus exact coordinate domains and rational
   factoring on identical equations. Auxiliary and invariant parameters
   remain unrestricted; include factorization and fallback costs.
3. Unordered elementary-symmetric cubic selection versus the two-parameter
   line-divisor selection on identical full bases, including both global
   sign orientations and coordinate deduplication.
4. Unpruned versus saturated line selection on the same certified j=0 orbit
   bases. Independently relabeled orbits are a refusal/correctness control;
   they must not silently enter the pruned arm. Include the inverse variable's
   construction and elimination cost.

These comparisons remain blocked and unfrozen. Automatic approval review
rejected the required source/worktree checkpoint upload as source-code egress
to S3 without explicit user approval; it was not retried. The current EKS
submission contract must also be reconciled with this card's provisional
resource ceilings before execution. No job, funding or resource request is
created by this update, and no practical gain is inferred from the new code.
