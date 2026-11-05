# Projective fixed-base CPU candidate

Fixed-base P/Q evaluation previously inverted after almost every affine sum.
Accumulate in Jacobian coordinates and normalize once at the public result.
Build each radix-16 window in Jacobian coordinates and batch-normalize its
entries, including the next window base, with one inversion per window.
Solve and each replay still rebuild their own tables inside the complete clock.
The public affine Curve::mul/add reference and certificate semantics are unchanged.

Source reviewed before implementation: [Explicit-Formulas Database, Jacobian
short-Weierstrass formulas](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html),
especially madd-2004-hmv, general doubling and affine scaling. Identity, equal
and inverse points are dispatched explicitly because mixed-add formulas are
not complete. This is standard coordinate arithmetic, with no novelty claim.

All 27 standalone release tests and strict Clippy pass. A new test checks every
pair of points of the order-199 fixture using nonunit Jacobian scales, mixed
identity batches and doubling; a y=0 control checks order-two doubling. The
existing fixed-pair test compares against independent affine binary multiplication
on small and 40-bit curves, including zero scalars, identity/equal targets and
4096 seeded pairs per target. Archived full certificates still match exactly.

The next frozen screen retains baseline2bd4588fba60, all three fixtures, full
setup/replay/serialization, A/A and batch-1 controls. No timing result is implied
by these correctness tests. Projective evaluation and table construction are a
bundle; their separate contributions are not identified by that comparison.
