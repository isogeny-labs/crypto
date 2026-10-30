# Fixed-base arithmetic with cached field inverses

## What changed

FixedPair construction and evaluation use the existing affine group routine
when the Curve already owns a complete field-inverse table. This is the existing
p <= 4096 policy; it does not depend on fixture identity or target scalar.
Radix-16 tables and digit traversal remain identical. Infinity, inverse and
doubling cases remain handled by add_valid. Larger fields retain the previous
projective table construction, batch normalization and projective evaluation.

The cache is still constructed per Curve and charged to setup. This adds no
persistent cache, heap allocation or public API. The bounded-field constructor
uses stack rows and avoids projective normalization scratch vectors. Both
complete independent certificate replays rebuild their own fixed-base tables;
no table or validation fact is shared between collection and replay. No scalar
search, random draw, relation ordering or certificate layout changes.

## References and rationale

Reviewed the existing inverse-cache records and the primary
[Explicit-Formulas Database short-Weierstrass formulas](https://www.hyperelliptic.org/EFD/g1p/auto-shortw.html).
The existing affine formula divides by a coordinate difference (or 2y for
doubling), while projective formulas avoid inversions through additional field
products and eventual normalization. A complete inverse table changes that
tradeoff: the existing affine division is a lookup and multiplication. This
candidate reuses that routine; it introduces no new group formula or novelty
claim. Whether it reduces complete latency must be measured.

## Validation and limits

The new test compares every fixed-base table entry for every target in the
19- and 199-element groups against the preserved projective path, forced by
clearing a cloned Curve's inverse table. It exhausts all scalar pairs in the
19-element group and checks radix boundaries and endpoints in the larger group,
comparing both paths to binary affine scalar multiplication without the cache.
Identity targets and identity first bases are included. Existing wide-field
and archived-certificate tests remain applicable.

No timing has been collected for this change. Round18 remains frozen at the
preceding hash-enumeration revision and its preserved binaries; do not replace
those binaries or attribute that panel's results to this candidate. Any later
performance comparison must keep all three fixtures and the same complete-cost
boundary. The further10x goal and largest feasible CPU scale remain unproven.

## Ownership and next steps

T-959 reserves the shared source and this directory. All 38 standalone release tests, strict all-target Clippy and formatting pass.
The complete example builds; binary/source hashes are in validation.json. Root
crypto/CUDA is not built locally under the user's disk constraint. Changes to
FixedPair affect arithmetic used by both collection and replay, while each
replay independently reconstructs tables and validates every relation.
