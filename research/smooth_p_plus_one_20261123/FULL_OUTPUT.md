# Common full-output consumption protocol

2026-11-24. Local software qualification on public toy fixtures.

## Policy and accounting

The existing first-row policy discards all but the lexicographically first
signed relation from each complete oracle answer. Add an explicit `full-output`
policy: consume returned rows in their existing sorted order until the consumer
recovers the target scalar, an error occurs, or the answer is exhausted. Every
submitted relation passes through the unchanged production equality replay,
canonicalization, deduplication and exact row reduction. No target log or base
log table is supplied. This is standard relation collection, not a new map.

For rows c and c' from one residual R=[a]G+[b]Q, both satisfy
c*x-b*d=a and c'*x-b*d=a; subtracting gives (c-c')*x=0. Thus additional outputs
can supply useful constraints even when they come from the same query. Their
rank and recovery contribution must be determined by the consumer; neither a
row count nor oracle count alone establishes a full-cost advantage.

`full-output` follows optional `base-lookup`, after `account=SEED` or
`account-suite=SEED` and optional `draws=N`. Both earlier formats remain intact.
The new policy uses format 3, with `relation_policy=all_until_recovery` and
explicit full-output counts. Each complete query retains its entire ordered
`returned` set and every `consumed` row/receipt. Unconsumed rows are allowed only
after recovery (or a fatal consumer error, which prevents receipt admission).
The output was already generated, so all its construction/extraction remains
charged even if its final suffix is not consumed. Singleton hits retain their
existing explicit receipts and are counted separately from oracle rows.

Every replay, duplicate and rank update remains within `consume_relation`;
record construction/serialization and other overhead remain within the child
or parent process accounting already specified in COLD_ACCOUNTING.md. No new
solver budget, chart, hidden reference fallback or timing exclusion is added.
The parent checks the exact selected prefix, sorted unique returned set,
per-row/per-draw rank continuity, first recovery stop and cumulative counts.
These consistency checks do not replace independent mathematical replay.

## Fixed controls and falsification

Before collecting new suites, tests qualify full-output Direct recovery with
lookup both off/on, compare the four-leaf-only selected rows with the older
reducer, retain incomplete-oracle/singleton behavior, and mutate the output
prefix, ordering, totals, rank and stopping metadata. Preserve failed tests.
The initial test assumed no singleton hit before recovery under the lookup
policy; that false expectation is retained and corrected separately.

Freeze the source and exact command before four suites, once each, in this order:
original first-row; first-row plus lookup; full-output; full-output plus lookup.
Each suite runs Direct, Refined and ProductionCofactorMsolve in that order, on
F127, depth 4, seed 781815, with 30 maximum draws. Public inputs are
y²=x³+x+7, n=109, G=(1,3), Q=(11,98). Each arm stops at its first consumer
scalar and performs the existing fresh final scalar replay. The first two
suites must match their corresponding base_lookup_v1 mask/row/consumer histories.
They are regressions justified by the source change, not repeated timing trials.

All original bounds remain: three seconds per query, 120 seconds per child,
400 seconds plus five-second grace per suite, one worker, 2 GiB virtual memory,
1 MiB per record and 32 MiB child stdout/stderr. Four suites give a maximum
1,620-second outer allocation. Zero automatic retries; later suites still run
if an earlier suite fails. Fatal children explicitly list unrun arms. Retain all
records, stderr, exits, failures and generated-but-unconsumed rows. Never
overwrite historical results. Parent polling can add up to 5 ms per child;
OS caches are not flushed and virtual memory is a bound, not measured peak use.

A lost output, changed historical control, invalid equality, unexplained early
stop, missing replay/rank work or inconsistent process receipt falsifies this
component. Query counts and rank outcomes are descriptive; record every outcome
without selecting by outcome. No timing ratio or admission is granted here.

## Sources and remaining requirements

Exact sources checked: accounting.rs first-row path, Base.reference's ordered
signed set, and the existing relation_adapter.rs replay/rank interface. The
[prior literature review](REFINEMENT.md#sources-checked), [lookup protocol](BASE_LOOKUP.md)
and [comparison obligations](../../problems/refined-rr5-cold-comparison.md) remain
applicable. KB unavailable: the all-returned-relations search wrapper exits 127,
and no KB MCP tool is loaded; no novelty claim follows from that gap.

Refined retains its narrower distinct-coarse-image chart. The common policy
does not establish equal oracle coverage, complete degree-32/full-base oriented
output, a new low/high-degree mechanism, or an admitted gain. Registered EKS
payload/build and independent standing remain open. Local flow receipts use
the development schema only because the canonical export tooling/schema was
unreadable through the available repository routes. This protocol neither
dispatches a cloud job nor supplies signed validation.
