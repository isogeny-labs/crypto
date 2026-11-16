# Native mixed-stride window with shared conversion

## Construction and public boundary

This extends an exact byte prefix of the checked native individual-grid core,
without changing that historical component. Prepared public strides borrow their
Curve and retain the prescribed multiplier labels, projective representatives
and affine stride points. Setup converts those strides once; its cost is separate
from the target window. The original even-order partition and point checks remain.

For Q=O, initialize all lanes to O. For Q=P, reuse the prepared projective lanes.
Otherwise generate the original grid for Q projectively, with no conversion.
At each subsequent offset add the corresponding prepared affine stride to each
lane with the original mixed Jacobian-affine formulas. Retain identity snapshots;
flatten only finite lanes in original offset/slot order and convert the entire
flat window once. The all-identity singleton window performs zero conversions.
These are known-multiplier updates, not an unknown-scalar search.

The API accepts public shifted points Q,Q+P,..., never their scalar labels. It
checks curve membership and verifies the prefix projectively, without introducing
an inversion per shift. This validation is additional work beyond the original
window function: one general group update per subsequent offset, curve checks
and coordinate cross-products. It must be charged in a complete comparison.
Case IDs in fixtures equal historical checker scalars but are used only by the
test selector; they are not passed to Prepared or window.

The general/mixed formulas, identity/vertical/doubling/equivalent-coordinate
branches and exact original layout are retained. Prime p and subgroup-order
assumptions remain caller obligations; p<=u64::MAX/2 is the fixed-width limit.
Prepared state is immutable during window calls and tied by a Rust borrow to
its Curve. Output slots are public data, not independently certified objects.

## Accounting

A nonempty target window performs one batch conversion, independent of the
number of shifted points. For N finite flattened lanes, conversion remains
7N-3 field products and one inversion. An empty flat window skips conversion.
This is the same batch algorithm as the original source, not a new measured
speed improvement. Target-independent setup, projective-grid construction,
mixed updates, prefix validation, allocations, snapshots and copies remain costs.
The implementation retains complete snapshots, matching the evidence-producing
source path; it is not a bounded-memory streaming implementation.

The source no longer needs to invoke individual-grid affine conversion for each
shift. Setup's one conversion remains separate. No whole-source cost comparison
is claimed, because division/seed recurrence and the polynomial stage are not
connected yet.

## Executed checks and custody

At source26e63a0422bca5b552f6a9118c12a4cb58007053, one standalone rustc1.99.1
build and one test invocation both exit0. Four methods pass:

- Q=O,P,2P,-P on the original E/F509, r479 fixture: all64 public shift entries
  and1426 finite golden slots are used. Slots match offsets, lane indices,
  public multipliers, Jacobian triples, affine pairs and inverse-Z exactly;
  snapshots agree and identity snapshots contain only O. Initializations match
  identity/prepared-P/generated. Every nonempty window reports one conversion.
- Identity-only singleton skips conversion; empty input, off-curve input and
  a valid-point but incorrect shifted prefix are rejected.
- Repeated prepared-P calls agree without mutating prepared state, and match
  the checked individual-grid output.
- Mixed exceptional cases agree with the previously checked general Jacobian
  addition; zero-y doubling gives O.

These are native geometry software checks against preserved records. No sign
oracle, normalized resultant, scalar decoder, recovery cohort, benchmark or
cloud job runs. No workspace-wide Cargo test pass is claimed. Test clock text
is diagnostic only. Full4563-case validation remains separate and unchanged.

The source/reference/fixture lock is fixed before execution. original-inputs.sha256
binds the original window/mixed source and complete case file. project-fixtures.sh
reproduces the two extracted TSVs. Raw logs, compiler/binary/source receipt and
manifest preserve the single invocation. Historical components stay immutable.

## Next implementation

Connect the normalized seed/progression and resultant stage to these windows
and the checked scalar/jet division cores. Preserve original product-tree and
multipoint-evaluation structure; a direct quadratic cross-product is not an
acceptable substitute for the source algorithm. Then connect the sign-mask
decoder and publish/admit the complete native runtime and original cohort.

## References

- Original mixed-projective-strides-v2/run.py:window and mixed.py:add.
- Checked feature-native-grid core and sealed r479 candidate geometry records.
- [EFD Jacobian formulas](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html):
  established coordinate and mixed-addition formulas, reviewed for the previous
  component. No new formula, fastest-operation or timing claim is made.
