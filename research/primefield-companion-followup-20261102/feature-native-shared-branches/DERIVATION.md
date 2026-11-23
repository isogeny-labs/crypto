# One coefficient traversal across phase branches

T-1873. Producer assertion; independent signed verdict pending. [pcc:unverified]

## Construction

The predecessor processes each phase branch with a separate coefficient reader, even though r,m,e and scale are shared. The new shared_mode takes at most four independently owned ReusedRing contexts. It validates branch count, field and scale, returns immediately for no branches, and otherwise constructs one bounded CoefficientReader. A leaf reads one coefficient and maps that constant into every branch ring. An internal node combines the left and right value vectors componentwise using each ring's cached right-block power. Branches share the coefficient traversal, not polynomial supports, selected roots or power values.

Induction on the common recursion tree proves each vector component equals a separate invocation of ReusedRing::mode. Each branch sees the same coefficients in the same order and performs the same ring products. The coefficient source is target-independent given the mode parameters; sharing it changes neither the constraints nor their roots.

The image adapter first evaluates C0 across all live branches and applies each resulting constraint. These independent state updates commute across branches. It then discards the C0 rings, gathers the survivors, creates fresh ring contexts for their changed supports, and evaluates F1 and F2 through two shared traversals. Each branch keeps its own power cache between F1 and F2. C2 is formed and applied separately on each branch. Zero handling and final lift remain unchanged. Processing all C0 before any C2 preserves successful computations because no branch depends on another branch's state; partial work before an arithmetic error need not have the same ordering as the predecessor.

## Cost and memory accounting

For q>0 live branches and L coefficients in a mode, reader constructions fall from q to1 and reader deliveries from qL toL. For q=0, neither a stream nor coefficients are requested. Per-branch ring arithmetic is unchanged. Counters cover successfully completed shared-mode calls, not every coefficient stream in recovery: initial C0/support construction and later extraction paths remain charged separately.

Reader deliveries are not a count of all scalar recurrence operations. The existing stream still performs its charged initialization, bounded buffering, interval products, inversions and final advancement. The wrapper does not measure elapsed time or scalar-field totals. Shared RecoveryProducts setup remains as in the predecessor.

Up to four evaluator rings and their power caches now coexist. Let K be the sum over active branches of their split-value coefficient bounds, O(sum(degree(B)+degree(S)+1)). Balanced recursion retains O(K log L) partial-value coefficients, plus cached powers bounded conservatively by O(K log² L) from the preceding construction. C2 retains F1 outputs while evaluating F2. Actual Vec capacities, map nodes, support/root copies, temporary products/reductions and output ownership must also be charged. The reported cache and returned-value slot maxima are separate live coefficient-length observations, not peak allocated bytes and not additive measurements of a single instant.

The four-branch guard is explicit. No additional CPU workers or parallel jobs are created. All source grids, order validation, initial support construction, source calls and final public-point checks remain charged. The original4,563cohort and earlier immutable evidence remain unchanged.

## Controls

Five prime fields and widths0..4 compare every returned value against separate reused-power evaluators over unresolved-only, mixed, selected-only and empty supports. The checks assert coefficient deliveries, output cardinality, per-branch coefficient consumption and unchanged scalar/pair product counts. Excess-width, wrong-field and out-of-range-scale inputs are rejected. The zero-width case requests no stream.

The state panel compares canonical triples, branch counts/supports, zero flags and materialized candidates through four radii. The public-point panel compares recover_shared with recover_reused at caps64/16. Its stream counters are collected only from the new adapter and describe completed mode evaluation; equivalent separate counts are q and qL sums on those same branch snapshots. See RESULTS.md and validation01 for observed counts, failures and source pins.

## Sources and next direction

Reviewed feature-native-reused-powers/{reused.rs,reused-image.rs,reused-checks.rs}, the canonical update, and CoefficientReader in reciprocal.rs. Official FLINT quotient-ring conventions: https://flintlib.org/doc/nmod_poly/index.html. Standard componentwise polynomial evaluation is reused, with no novelty claim. KB MCP unavailable and scripts/research-kb status reports no executable; exact predecessor and primary documentation provide the fallback.

Next investigate joint generation of neighboring coefficient modes: their signed binomial coefficients have adjacent-index ratios, potentially avoiding repeated stream initialization and long-stride updates. Endpoint alignment and zero denominators must be resolved before implementation. This would address coefficient arithmetic in addition to the branch duplication removed here. Any timing claim still needs a separately admitted frozen workload with full source/setup accounting.
