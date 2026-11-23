# Bounded Boolean candidate continuation, revision 2

This runnable standalone component reuses SearchV2 and the independent search-result checker. Session exclusively borrows one owner, immutable ordered original equations and one absolute deadline across every next/exclude operation. It does not admit a new original worker, create a new global budget or change native phase transitions.

## Exact continuation

next returns a checked Boolean candidate and retains its native trajectory and independent proof audit. Repeated next calls preserve that candidate until an explicit exclude(record_id). Record IDs are caller data, not proof that a signed lift was exhausted. Only an authentic owning adapter may attach the classified signed-lift exhaustion established against its original CheckedMap, signed domain and target.

For a partition fixing the first k variables and a retained candidate x, exclusion creates one child for each j=k..n-1. That child fixes indices below j to x and index j to !x[j]. These children are pairwise disjoint, and their union is exactly the parent partition minus x. The original equations are preserved in order in every child, followed by explicit single-variable prefix constraints. The construction avoids exponentially expanded point-blocking polynomials. Witnesses, parent/child links, prefixes and caller exclusion IDs remain owned.

SearchV2 solves each child under the same cumulative wrapper and original owner. A refuted partition needs a successful independent arithmetic/structural search audit. The terminal outcome is RemainingBooleanModelsExhaustedUnderRecordedExclusions. It means no Boolean assignments remain after the recorded exclusions; it never reports the original polynomial system as unsatisfiable merely because candidates were excluded. This core does not establish signed-lift exhaustion or group relations.

## Independent coverage replay

The separate audit checks the exact original+prefix equation list in every partition, rooted parent links, every complete disjoint complement split against its retained exact witness, each caller exclusion record's presence and every terminal partition status. It independently replays native search proofs with the existing successful-result checker. Missing children, altered prefixes, altered conditional generators and missing exclusion records are rejected. Exclusion IDs remain assumptions; the authentic consumer must verify their scientific provenance separately.

## Resources, failures and retained state

Finite limits cover total partitions, cumulative work/bytes, native SearchV2 and proof audit. At most n-k new partitions are admitted per excluded witness. Prefix storage is at most O(partitions*n), before equation/native-proof storage, all charged before allocation. Every rebuilt source, comparison, branch loop and proof checker envelope passes through the one cumulative owner. Local wrapper charges never reset across next/exclude calls. The separate independent auditor accepts an explicit finite verification allowance; a real integration must supply the same original phase owner and remaining deadline.

Every refusal is sticky and makes the outcome Unknown. Previous candidates and native/audit states remain retained, including partially built or splitting partitions. These are diagnostic artifacts, not a resumable checkpoint: temporary copied polynomial scratch may drop on failure, and no retry method clears failure. The implementation retains original admitted charges without refund.

Revision 2 adds with_candidate_owner: a pending candidate and the unchanged absolute deadline are borrowed alongside the SAME exclusively held owner. Parent/deadline checks run immediately before and after the callback. CandidateCall retains its returned value even if the final check refuses; that refusal also sticks in the session, makes its outcome Unknown and forbids subsequent exclude/next. No callback executes without a pending candidate or after a terminal state. The callback must admit its own progress allocation, work and group additions through the original owner. Its costs belong to that parent's ledger, not to the generic wrapper's reservation counter; this API does not pretend to account for arbitrary callback operations. The genuine integration must keep lift in Matrix while continuing, and transition to extraction only after accepting a signed relation; that phase transition is not implemented in this generic component.

## Validation

Compiled with rustc +1.98 --edition=2021 --test harness.rs, compilation bounded by timeout 180. Ran timeout 60 ./tests --test-threads=1. All 26 tests passed: ten continuation controls plus sixteen unchanged donor controls. New controls enumerate eight distinct models, compare several ideals with independent truth-table solutions, verify exact coverage, reject mutated split/source/exclusion records, preserve paused candidates, retain limits/deadline/refusal states, exercise variable index129 and sweep every owner refusal boundary of a completed one-variable enumeration. Three new callback controls verify owner address identity, exact candidate/deadline forwarding, parent-ledger callback charges, no invocation before a candidate or after terminal, and returned-value retention plus forbidden exclusion after postcheck refusal. Signed-lift certification is not mocked or claimed by these tests; exclusions are explicitly caller actions.

No full Cargo build, scientific generation, cloud operation, production edit or frozen-packet mutation occurred. The genuine owning adapter and central evidence/publication path still require integration.
