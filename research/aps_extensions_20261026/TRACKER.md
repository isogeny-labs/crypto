# APS implementation and validation tracker

This is the durable worklist for the requested PR. Deferred execution is also
tracked by [XP-20261026-aps-complete-cost-panel](../../backlog/XP-20261026-aps-complete-cost-panel/README.md)
and [the accounting problem](../../problems/aps-single-relation-complete-cost.md).
A checked box means the
stated bounded deliverable, not practical or asymptotic validation.

| ID | Mechanism and current coverage | Next acceptance criterion |
| --- | --- | --- |
| APS-B0 | Native random-labeled single-relation baseline; m=3 and paper's split m=4 | Complete replay and costed paper-scale ladder; m=5 remains unimplemented |
| APS-E1 | Fixed Q coefficients; native label checks and coefficient lemmas | Compare useful-relation yield and complete construction/retry cost against random labels |
| APS-E2 | Squarefree cubic-divisor selection and two-pair block selection; exact point-oracle checks and DLP recovery | Matched-base comparison including remainder construction, decoding and relation yield |
| APS-E3 | Verified j=0 automorphism/eigenvalue, label propagation, sparse orbit constraints, tautology rejection and informative DLP recovery | Orbit-aware sampling/rejection accounting; matched random bases, fold-aware rho and existing GLV baseline |
| APS-E4 | Relation-independent O(log^2 s) triple cover, deduplication and shared deadline | Charge every coloring and failed system; test balanced/optimized covering families without inspecting answers |
| APS-E5 | Expanded S4, joined S3, and first-coordinate hybrid solver | Compare actual solver presentations on identical bases; add adaptive branch-variable choice only as a preregistered new arm |
| APS-V1 | Point-oracle and known-answer correctness tests | Standalone transcript deserializer/replayer using an independent arithmetic implementation; no replay certificate is claimed yet |
| APS-V2 | Solver-stage counters explicitly limited in scope | Instrument all phases, add ecbench integration, validate counters against an external backend; freeze the common cost model |
| APS-V3 | Controlled panel design recorded | File valid `experiment.protocol/v1`, freeze analysis/inputs, obtain checkpoint credential, push checkpoint, pass gate, then execute |
| APS-V4 | No performance/scaling conclusion yet | Match strongest rho and external polynomial solver, independent validator/replay receipts, cost bands 0/0.1/1 GAE; only then consider promotion |

## Completed bounded implementation

- [x] Reuse native F4 and curve arithmetic; do not import the earlier Python prototype.
- [x] Preserve all attempts in successful solver transcripts, including empty or incomplete algebra results.
- [x] Verify labels after canonical sign changes and reject overlap, invalid fields, singular curves and wrong point orders.
- [x] Check polynomial solution sets against point addition independently of polynomial construction.
- [x] Recover public tiny DLP fixtures through the algebraic relation path without a generic fallback.
- [x] Implement the five mechanism interfaces above and a public fixture runner.
- [x] Include coefficient lemmas with pinned core-only Lean and an axiom audit.
- [x] Add focused CI for the new code and proofs.
- [x] Locked release example tests: 4 passed; record outcomes in `validation/`.
- [x] Final targeted locked release module tests: 15 passed; receipt in `validation/`.
- [ ] Finish the full library run, still waiting on an unchanged `jv_isogeny_walk` test; record its actual outcome.
- [ ] Publish the regular PR, enable auto-merge, inspect applicable CI and address feedback.

## Follow-up mechanisms retained for implementation

**APS-F1: quotient-ring preconditioning and template reuse.** Reduce the
summation equations modulo membership constraints, cache base-dependent
templates, and compare variable/order choices. Reuse `polynomial_reuse` as
prior art; record cold and warm costs separately. Acceptance: equal verified
solution sets, lower complete cost on the same workloads, all setup charged.

**APS-F2: composed maps and sparse structured bases.** Keep sparse power-map
constraints with explicit curve/rationality conditions, or chains of low-degree
rational/isogeny maps. These mechanisms are already in Petit–Kosters–Messeng
and the factor-base design register. Constructing arbitrary prescribed
coordinates does not give known `(u,v)` labels for free. Acceptance includes
subgroup checks, denominator-pole exclusions, labeling/rejection cost and a
size-matched random control. Not implemented by this PR.

**APS-F3: larger unordered blocks and alternative separating families.**
Generalize squarefree divisor selection beyond cubics, and compare optimized
covering families with the implemented explicit bit construction. Acceptance:
prove or exhaustively verify selection/coverage before timing; account for
duplicates, unbalanced blocks, and every extra solve. Not implemented here.

## Prior-work and atlas handoff

The nearest open local work item is
[`ecdlp_leads_20261018` L4](../ecdlp_leads_20261018/README.md#l4-audit-of-unchecked-prime-field-ic-algorithms).
This PR advances only its APS subitem to tiny native correctness coverage.
It does not close L4 or the other papers in that list.

For the next review in `isogeny-labs/crypto-autoresearcher`, attach this study
and its eventual PR to the relevant `LEAD-ICE-prime-field-semaev-solving-degree`,
`LEAD-ICE-semaev-chain-cost-model`, `LEAD-ICE-rootfind-pairtable-oracle`,
`LEAD-ICP-algebraic-membership-factor-base`, and
`LEAD-ICP-xline-symmetry-special-j` rows. Preserve their existing scoped
closures; they are not general impossibility results about solver presentation.
The sibling repository is read-only in this PR, so its next atlas review is
explicitly still owed.

## Four-lead follow-up (2026-10-28)

The [follow-up study](../aps_followup_20261028/README.md) tracks the requested
implementation of (1) informative-relation streaming and early termination,
(2) membership-domain root extraction plus deadline-aware rational factoring,
(3) a two-parameter line-divisor presentation, and (4) pruning horizontal
orbit components only after checking propagated labels. Exhaustive controls
and point-oracle equality remain acceptance gates; lower complete cost is
still unmeasured. This follow-up does not close APS-V1 through APS-V4.

The formerly pending full-library run from the original implementation has
now finished: 4,835 passed, 0 failed, 120 ignored. Its log SHA-256 is
`6dddc82f8bc16a8e1571943e7f431238f4e5e14de1e312e3b877c7380408e08b`.
This was that original implementation snapshot, not a fresh current-main
or hosted-CI run; see the [completion receipt on PR #2614](https://github.com/isogeny-labs/crypto/pull/2614#issuecomment-6446563321).
The PR merged as `e3adf96bdc4498a97d7227981eab97a7d3dd71af`. Historical
validation files stay unchanged. Current follow-up validation is recorded
in its own study.
