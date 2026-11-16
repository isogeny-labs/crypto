# Independent finite algorithm coverage audit — unresolved findings

Read-only audit of the root worktree on 2026-10-29. This is a finding report, not a completed coverage review, protocol approval, binary pin or cohort result. Source was read while the root software test was active. No experiments, builds or source edits were performed by this reviewer.

Boundary: validation/accounting/COST_BOUNDARY.md and kernel-primitive-registry.json. Excluded allocator/table internals, CPU instruction census, reporting/serde/replay observer work, harness and infrastructure. Observed semantic categories must be consistent between candidate and reference.

## 1. Actual rho scalar constructor primality is missing

src/cryptanalysis/three_leads.rs:1445 still calls unobserved is_prime(r) in Ring::new. rho_native invokes it at2581, once per rho_native invocation, before RNG initialization and before the first target multiplication. Restarts reuse that Ring; they do not repeat this check. Both plain/negation and transferred rho reach the same invocation. Transferred rho also performs the separately observed Scholten base-field constructor check.

The existing geometry constructor ledger wraps Scholten p only. A correction must add scalar-domain events to the actual Ring call, retain its original is_prime(r) && r>2 order, and avoid adding a second check. Scalar constructor modular multiplications and squares belong to native scalar events, not weighted base M/S. Its standalone reductions must be counted once; embedded product reductions are diagnostic; these powers are not inverse expansions. Existing other callers at1758/2341 should preserve semantics if the helper is instrumented globally, with per-adapter deltas preventing unrelated runs from leaking.

Affected category: outer_and_constructor_validation.

## 2. Candidate associative-container entry semantics omit priced primitives

src/sparse.rs:56-74: successful get, remove and first_key_value return a stored entry but only increment excluded map_calls (and key comparisons where underlying std operations perform them). They do not count an exposed entry visit. In contrast rho's successful collision lookup at three_leads.rs:2631-2633 counts an entry visit and the explicit copied tuple. Candidate residual pivot reads at residual.rs:100-102 and sparse accumulations are actual uses of those missing lookup visits.

sparse.rs:64-66 inserts a new stored map entry without a fresh-element event. tracked_set.rs:38-52 insert, extend and from_iter similarly omit fresh retained set entries. Reference rho explicitly counts new seen entries at three_leads.rs:2665-2666, and candidate vector fresh elements already receive initialization events.

Correction convention: one exposed-entry visit for a successful returned-entry API, none for failed lookup; contains_key is presence comparison, not returned-value traversal. A copied value is an additional event only where the caller actually copies it (e.g. get(...).copied()); it is not inferred for a borrowed reference. One initialization per genuinely new retained map/set entry, not duplicate insertion/replacement. Count set bulk new entries by actual size increase while retaining std bulk implementation. Do not charge allocator nodes, rebalancing, hash buckets, or internal relocation. Existing iterator visits remain one per yielded entry and must not be duplicated at consumers. New map/set initializations can reuse the collection_elements_initialized primitive; separate map_calls remain diagnostic.

Affected category: collection_graph_elimination; this also affects candidate sieve sets.

## 3. Collection predicate coverage remains inconsistent

The selected model already prices candidate polynomial length tests: native_solver.rs:778-780 counts a.len()>=b.len(), and813-814 counts remainder.is_empty(). Corresponding engine pdiv at three_leads.rs:841 evaluates !r.is_empty() && r.len()>=b.len() without predicate events, and pexact:863 evaluates r.is_empty() without one. This is the same finite collection-predicate category, not a request to price every machine branch. Count actual evaluated predicates, respecting && short circuit, as was done for the corrected last()==Some(0) convention.

Residual coefficient-cap predicates are also unobserved: residual.rs:92,103-107,123 compare row length against working/retained limits, while graph canonical cardinality and candidate bucket cardinality predicates are counted. Add those actual collection-length comparisons at their short-circuit sites. Keep reported max/row-update summaries diagnostic instead of pricing summaries on top of underlying work.

Affected categories: field_arithmetic auxiliary polynomial collection operations; collection_graph_elimination. No claim here that all scalar branch decisions need pricing.

## Reviewed areas without an additional concrete finding

The two pencil inverse-expansion loops justify bitlength(modulus-2)+popcount(modulus-2) subtraction per explicit inverse, with checked u128 normalization. Actual MR powers remain noninverse. Outer p/r validation keeps separate domains and short-circuit evidence. Candidate field/arithmetic totals already include setup; setup semantic counts are separately added once. Engine adapter geometry segments are disjoint and their total is not added again. Additional source verification is added once. Calibration preparation plus addition is checked and added once in the explicit calibration-inclusive result; the source addition is also the normalization unit, not silently free calibration. Hash/RNG atomics exclude their internal expansion; candidate walk_label_calls now records actual invocations. Metadata Value node copies are distinct from supplemental byte fields and observer traversal. The three earlier analyzer pairing/null/calibration findings are fixed.

Nine-category disposition: field_arithmetic has predicate consistency finding3; scalar_arithmetic normalization reviewed without new defect; outer_and_constructor_validation blocked by1; setup_transfer no additional finding beyond shared categories; collection_graph_elimination blocked by2/3; hash_rng no additional finding; source_verification no additional finding; metadata_clone no additional finding; calibration no additional finding. These dispositions are scoped review findings, not certificates of exhaustive coverage or permission to emit reviewed_complete.
