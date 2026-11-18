# Profile-directed monomial enumeration and reducer lead caching

The closed0934 known n16 profile has160575923 guest Ir, leading-monomial scan
self27079477, normal_form inclusive46883266 and a monomial-sort quicksort
inclusive32663999. Inclusive costs overlap; these are not additive phases or
native time fractions. PCC40f7a9/a29e0f. The prior0906 paid15x gate still fails,
with expanded adjusted lower12.792. No speed is predicted. [pcc:unverified]

Sources checked (KB unavailable): exact shared cmp/lead/normal_form/monomials
and quotient change_order, prior Singular dp/grevlex review, closed Callgrind
report and official attribution/format documentation. This is standard order
traversal and immutable-data caching, with no novelty or degree-of-regularity
claim. All existing constructor and full root replay checks must remain.

Candidate1: generate exactly the current monomials(n,d,exact) output order,
without generate-then-sort. Current sort is cmp(b,a), descending grevlex.
For each fixed total degree, enumerate last-variable exponent from0 upward,
then the next-to-last likewise, assigning the remaining degree to the first
variable. Across degrees enumerate high to low unless exact. This partitions
all exponent tuples without duplicates and follows the last-different-exponent
comparison. A differential vector equality test against the legacy enumerator
is mandatory; this explanation is not a formal proof. [pcc:unverified]

Candidate2: during one normal_form call the reducer slice is immutable. Cache
its leading-monomial references once rather than rescan each reducer on every
search and selected-reducer operation. Keep the original reducer order, target
lead selection, inverse/coefficient operations and proof updates. Preserve empty
target and invalid/empty-reducer behavior; do not cache changing target leads.
Require complete remainder AND provenance equality on non-GB/nonmonic reducer
lists in both stated fields, as well as identical native wire outputs.

Keep legacy behavior by default. A private scoped thread-local choice used only
by an explicit native pending-wire factory permits four matched modes: old,
ordered-enumeration only, cached-reducers only, both. Guard restores the prior
choice on success/error/panic and nested calls. No public arbitrary callback
or global setter. This avoids four separate root dependency builds while
preserving one matched graph; newly compiled old mode measures guard/rebuild
cost separately from the frozen previous candidate. No unsafe code needed.

Every mode still performs full native constructor final checks, original and
S-pair witness completion, serialized proof retention and unchanged complete
root consumer before exposing a basis. Original field/admission/proof/work/file/
process caps and old imported/legacy paths remain. Opaque pending factory p31
scope unchanged. New helpers cannot cause a claimed producer basis to bypass
acceptance. No hidden relation/reference/target preprocessing or new seed.

Qualify differential complete monomial vectors including exact/bounded order,
normal-form remainders/proofs and (counter feature) unchanged field operations;
scoped guard restoration including unwind/nesting; all affected feature release
suites/lints. Actual-root API/worker tests and all four native modes through16
against frozen prior complete wire bytes and fresh Singular. Preserve every
failure. No runtime gain from qualification clocks or guest instruction counts.

A separate prospective paid roster must retain all19 previous controls and
append four new matched no-export modes (23 total), exact mode flags and every
control denominator, same complete answers/fees/recovery/caps/CPU reservation.
Publish source/protocol/qualification before any call. Separate A/A/paired
confirmation required; no fresh targets until a development15x gate passes.
LocalL0/checkpoint waiver/zero cloud spend/no unapproved agents persist.
