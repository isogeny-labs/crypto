# Matched public recovery: source review

Tracking: PR #4922. Reviewed on 2026-11-23 UTC before execution.

This is independent source inspection within the same assistant team. The
reviewer did not author the changes or run any build, test, solver or
benchmark. It is not an independent trusted verifier or signed PCC verdict.
Execution evidence is recorded separately. [pcc:unverified]

## Source identities and findings

Initial inspected example SHA-256:
`1b55d3bfe49170b4ec9b847585a4a7191c257027393401438ee2ce40b792021c`.

Corrected example SHA-256, checked before and after follow-up inspection:
`dd965d7116f387ed9cd61cf79fedcee8230af768f3352a503d3da563d69d0868`.

| Finding | Correction inspected | Acceptance purpose |
| --- | --- | --- |
| Accepted F4 tuple was discarded after signed lifting | Store `accepted_assignment` and `accepted_factor_base_indices` inside the callback immediately after acceptance | Preserve the actual polynomial witness rather than reconstructing what it should have been |
| Replay failure returned before storing the candidate relation | Store the tuple, indices, replayed point, replay result and candidate factor row before that branch | Keep a failed replay independently diagnosable |
| Original-only interruption control did not exercise retained preconditioning work | Add one Fresh and one Shared expired-deadline call; finish and print both records before assertions | Check that setup and specialization remain visible even when F4 performs zero runs |
| Plan output did not list all new execution paths | Plan v3 lists matched CLI and the separate full example suite/F17 test | Prevent a CLI-only run from being described as exhaustive F17 acceptance |

The follow-up source inspection confirmed all four changes. The tuple replay
helper checks canonical coordinates, equality to the retained signed base
indices' abscissae, original equations, a finite first-pair sum with abscissa
u, and the signed three-point sum. Both Prime31 signs and all 18 F17 panels
use it. Every F17 recovery panel completes before assertions begin.

## Preparation, inputs and outcomes

- Original constructs no Macaulay object. Fresh calls FreshMacaulay::prepare
  directly. Shared prepares its own common reduction and projections once.
- Every matched arm constructs its family and factor base from public
  parameters inside its enclosing entry. The currently unpriced arithmetic,
  allocation and indexing remain explicit exclusions.
- Every arm uses the same integer construction of the original equations,
  includes those equations alongside any consequences, retains the factor
  domains and leaves the auxiliary domain unrestricted.
- Expected scalars, factor-base logarithms and reference solution sets do not
  enter the public recovery boundary. Canonical-coordinate validation precedes
  point arithmetic. The p and r roles remain distinct.
- The signed generator anchor and each relation row use the existing column
  mapping. For a query Q+[a]G the row is sum(c_j log_G(P_j))-log_G(Q)=a mod r.
  A determined scalar is accepted only after the public group equality.
- Arm collection does not propagate an error that suppresses later arms.
  Found witnesses, finite-chart exhaustion, interrupted traversal, setup errors
  and query-budget exhaustion remain distinct outcomes.

## Accounting and acceptance limits

The decision call to IncrementalGauss::pinned occurs before the final counter
snapshot. Reporting does not call pinned again. Group and field ledgers,
Macaulay logical/kernel products and F4 extraction counters retain their
actual scopes and overlaps. Missing partial work after a preparation error
is unavailable, not zero. Every arm has cost_counter_coverage_complete=false.

The CLI runs two Prime31 target panels and two control panels, retaining all
seven arms in each. It emits full records before a nonzero failed-check exit.
The separate F17 test covers all 18 nonidentity public targets across all
seven arms. Sequential execution is required for interpretable process-global
F4 counters. Passing this source review establishes no test pass, performance
sample, full-chart claim or full-root-library validation.

No unresolved source-review finding remains for the bounded correctness
qualification at the corrected hash. Any changed implementation requires
another source binding and review of the delta before execution.

## Retained-output metadata audit after execution

After the separate immutable runs, the same reviewer checked metadata and
retained output without rerunning scientific computation. The receipt digest
13014e0abf1b1e5333e6d7902bcf819ec1ff158e82c904f0161bc8fbcf27ec81,
all 30 artifact hashes/sizes, all 51 source bindings, equal native manifests,
and both mode-555 executable hashes agree. The raw CLI has 28 retained arms
(26 recovered, one expected cap, one expected interruption); the raw F17 log
exactly matches the extraction of 18 panels and 126 successful arm outcomes.
Both prepared-interruption records retain setup and specialization. No
concrete mismatch was found. This is an output/provenance audit, not a signed
or independent scientific replay. [pcc:unverified]
