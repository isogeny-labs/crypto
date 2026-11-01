# Independent conditional signed-pair review

The three producer declarations freshly compiled in an isolated snapshot. Three producer controls and eight independent reviewer controls compiled; separate readback for all 14 declarations contains only propext, Classical.choice and Quot.sound. No discrepancy was found in the stated abstract conditional uniqueness claim.

PLAN.md was recorded before opening proof bodies. The independent derivation splits the first quotient equality into equal and opposite signs. In the opposite case, the neighbor equality either implies equality directly or forces the anchor to equal its negative, contradicting g+g!=0. Inspection of the producer proof found this exact logic. `filter_unique` composes projection equalities; `zero_anchor_ambiguity` records the degenerate zero-anchor case. This is uniqueness, not existence.

## Independent boundary controls

The reviewer defines actual finite sign-quotient functions q4 and q5 using min(x.val,(-x).val), and proves their full fiber properties by kernel evaluation. Controls show: a nonzero order-two anchor in ZMod4 admits distinct signs matching both projections; one projection alone fails to determine sign; the neighbor separates those signs for anchor1 in ZMod5; a constant projection fails the required fiber property; the main theorem applies to the actual q5 function; and an impossible requested quotient coordinate can have no candidate. All eight declarations passed without native_decide.

## Source correspondence

Snapshot `source/pruned.rs:150-160` (`delta_lift`) first rejects zero inputs, performs chart/radical construction, deduplicates candidates, and filters on j.valid, project_any(d), same_k(d,kq), project_any(anchor+d), same_k(anchor+d,kplus). It returns Some only for one match. The theorem explains uniqueness of the final filter under its premises. It does not establish candidate enumeration completeness, chart validity or radical formulas.

The degree-one path at lines174-181 applies the same two projection checks and excludes duplicate signed candidates. The identity path at lines165-169 instead recognizes a coordinate pattern and checks only the anchor projection against the neighbor. It requires a separate correctness statement for the coordinate-pattern recognition; the filter theorem alone does not verify this shortcut.

`walk_impl_uncached.rs:51-53` implements same_k as all cross-products equal. Zero vectors satisfy that raw predicate against every vector. delta_lift/boundary_lift reject zero supplied vectors, and project_any rejects zero output, but the theorem uses an abstract quotient-valued function rather than this raw predicate. A projective quotient construction, nonzero representative invariant, chart-domain binding and exact fiber theorem remain required. Passing conditional proofs does not establish them.

The anchor non-two-torsion premise is not checked inside these filters. A caller with G nonidentity and exact odd prime order can discharge it, but the source-to-group and caller-to-function binding is separate. No geometric fiber correctness, xadd closure, production totality or compiled Rust equivalence is claimed.

## Trust and publication boundary

Lean4.34.1 builds used fresh private target oleans. The producer imports Product, transitively using retained dependency caches. The adjacent independent-product snapshot's .lean/.olean files were hash-bound before the final replay and verified afterward. Broader Mathlib dependency-union caches were reused; not rebuilt or independently kernel-rechecked. Private producer inputs are hash-bound snapshots, not an immutable published commit. Thus this is a scoped independent source/algebra/replay review, not full formal-review workflow completion.

No scientific execution, curve operations, cohort claims or performance comparisons occurred. Full 1,260-target scope remains unchanged. All artifacts are private for root archival. A jq audit-expression scoping mistake initially failed parsing; corrected audit verified count14 and the allowed axiom set.
