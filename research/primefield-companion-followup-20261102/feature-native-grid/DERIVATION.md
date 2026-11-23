# Native prescribed-multiplier elliptic source grid

## Preserved construction

The original shared-projective-grid/geometry.py constructs known public multiples
of an input point Q. For h=(r-1)/2, let h=2*b*q+2*s+epsilon, with b,q positive,
0<=s<b and epsilon in{0,1}. It starts H=[(r+1)/2]Q and S=[2b]Q, advances b times
by Q, then q-1 times by S, and appends [s+epsilon]Q when s>0. The exact original
multiplier order is retained. These are prescribed multiplications, not an
unknown-scalar collision or search procedure.

The native port preserves general Jacobian formulas x=X/Z²,y=Y/Z³, doubling,
identity, vertical-pair and equivalent-coordinate exceptional branches. It
validates the original partition, canonical public point and nonsingular curve.
Primality p and the intended subgroup order r are caller preconditions. Only
5<=p<=u64::MAX/2 is implemented; products use u128 reduction. Internal points
are bound to the validated Curve and not accepted as arbitrary public objects.

`grid` returns labels, projective coordinates, affine coordinates and inverse-Z
values. This version exposes a complete individual-grid operation. The original
projective-only entry mode and mixed-stride shared-window updates are not yet
exposed natively; do not substitute repeated individual conversions for the
original shared-window conversion and call the costs equivalent.

## One-inversion conversion

For nonempty finite points, form prefix products of Z, invert their total once,
and traverse backwards to obtain all inverse-Z values. Convert each point using
v²,v³,Xv²,Yv³. Excluding validation and group construction, this is exactly

    (N-1) + 2(N-1) + 4N = 7N-3 field multiplications, one field inversion.

Squares count as products in this expression. Empty batches, infinity and zero-Z
are rejected. Singleton batches work without prefix or reverse-loop products.
The converter preserves the original algorithm; this count is a derivation,
not a newly measured improvement. Allocation, integer work, setup and group
arithmetic remain separate. No wall-time claim or external-best claim is made.

## Executed checks

At source6c5f5b983d6278af220e46f8b76bb3a188f52355, one standalone rustc1.99.1
build and one unit invocation both exit0; all3 methods pass:

- All71 slots of the original three public-generator grids match exactly in
  labels, Jacobian triples, affine pairs and inverse-Z values. The grids contain
  20,23,28 slots for r359,479,683 respectively. Independent affine repeated
  doubling/addition verifies every prescribed multiplier. Its inverse uses
  Fermat powering, independently of production extended Euclid.
- Identity, inverse-point, ordinary doubling/addition, equivalent scaled
  Jacobian representatives, singleton batches and zero-y doubling are checked.
- Invalid curve/point/partition and empty/infinite/zero-Z batch rejection pass.

The affine test reference is restricted to these small fixtures; its direct
u64 products are not an arbitrary-size arithmetic backend. Neither production
upper-bound arithmetic nor all curve points were exhaustively tested. Prime
and subgroup-order validation are inherited caller obligations, not performed
by these constructors. No fresh sign oracle, unknown-scalar recovery cohort,
performance panel, cloud job or workspace-wide Cargo suite was executed.

## Provenance and next integration

Fixtures and slots are jq projections of the original candidate setup receipts
for all three curves, with original files hashed in original-inputs.sha256.
Source and extracted fixtures are locked before execution; raw logs, compiler,
binary hash and source commit are retained. original-geometry.py.txt preserves
the source bytes for review and is not a runtime Python dependency.

Next expose projective-only grid output and mixed-stride shared-window updates,
then join the normalized division/seed recurrences and resultant stage. Preserve
the original full4563-case cohort and all arms. Complete native runtime/source
publication and authoritative admission remain required before that execution.

## Literature

- [EFD short-Weierstrass Jacobian formulas](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html)
  records the coordinate convention and established addition/doubling formulas.
  This port preserves the original formulas; it does not import EFD's fastest
  operation counts or claim formula novelty.
- Original shared-projective-grid/geometry.py and mixed-projective-strides-v2
  source call sites establish the individual-grid versus shared-window boundary.
