# Public-target relation interface v2: frozen pre-code plan

Question: can the member/image circuit be composed into a complete public
target relation interface, and can an accepted root be converted to a
signed indexed-family row without canonical point-set deduplication?
This step constructs/checks relations; it supplies no root-discovery solver.

## Source version and public compilation

Preserve membership-circuit-v1 source archive, manifest, qualifier/results,
claims, report and retained executable. Working Rust may change only with a
new source-v2 archive, manifest, pre-call qualifier and affected checks.
Directly pinned Lean645f28 source/configuration must not change.

Compilation receives public field/base/curve parameters, arity, public target
and explicit distinct-index mode. It receives no planted j,t, point witness
or scalar. Add public symbolic role accessors instead of guessing variable
names/slots. Copy each member's equations with an explicit variable offset.

## Complete group composition

For the Curve25519 prime-order image subgroup, convert nonidentity Montgomery
outputs with x=C*u/v, y=(u-1)/(u+1), C²=-(A+2), d=-(A-2)/(A+2).
Target identity is (0,1); other target conversion guards are explicit.
External contracts include a prime field, square -1, nonsquare d, the
standard curve/map relation, and an odd-order image subgroup so retained
nonidentity points have no conversion-chart pole. The actual RFC instance
supplies those group-order assumptions; the constructor does not prove them.

Use RFC8032 section5.1.4 extended coordinates (X,Y,Z,T), XY=ZT,
-X²+Y²=Z²+dT². Use its complete eight-product addition sequence as stated,
including equal/inverse/identity input pairs. Retain nonzero Z by inverse
witnesses and explicit curve/extended invariants. Equality with the public
target uses both X=x_Q*Z and Y=y_Q*Z, never only a quotient coordinate.

Distinct raw indices are the default relation domain. For each pair of
leaves, invert a named norm(z_i-z_j) auxiliary. Nonsquare2 makes this norm
zero exactly at equal raw coordinates. This excludes repeated parameters
without excluding distinct parameters sharing a cofactor image. A separate
general mode allows repeats for complete group-law/cancellation controls;
its outcomes cannot be promoted as distinct-tuple yield evidence.

## Indexed rows and side-phase accounting

Use raw labels (j,t), with 1<=j<=k,0<=t<q, as B=k*q columns. Different
columns may map to the same signed cofactor point, or to identity. Define
their canonical orientation using the smaller raw v root. Such correlations
do not require independent unknown logs in the existing conditional scalar
kernel theorem. The actual group/selection bridge remains an obligation.

Given a checked root, decode j and z from semantic input roles, recover t
from z/h^j in <h^a> using HAC Algorithm3.56, and verify the recovered power.
Use a fixed baby-step memory cap and record it. Compare the native cofactor
point with the canonical orientation; combine coefficients only for equal
raw labels, then independently check the row against the public target.
Do not merge point-image aliases or infer scalar logs.

Side-phase interpretation: even exhaustive q-step index recovery per
successful leaf would cost O(B*q)<=O(B²) for O(B) retained fixed-arity rows,
since q<=B. Standard BSGS improves that to soft-O(B*sqrt(q)) with
O(sqrt(q)) table space. Charge it on successful checked rows, not for free.
These are conditional algorithmic accounting observations, not measured
runtime, a solver bound, or a new overall exponent.

## Fixed controls and resource limits

- Actual cases reuse the eighteen fixed public inputs from v1. Check
  cancellation, doubling and identity intermediates in general mode; six
  distinct raw parameters in default mode; wrong target and repeated-index
  rejection; every-variable mutation of one six-member witness.
- Fixed arithmetic fixture p101,A95,C2,d=-2,h=(17,12),a17,q3,k8.
  Predict #E=104 and verify by all101² affine pairs plus identity. Check
  h^51=1 and h^3,h^17!=1. Enumerate all24 raw parameters and both signs;
  record identity cofactor exclusions rather than changing the fixture.
  Compare all retained ordered distinct raw pairs with every point of the
 13-point image group. At most30000 canonical pair/target checks.
- Test the index adapter on the known actual j/t extremes and on the toy
  family. Bound its baby table by2^20 entries; actual q uses14600 entries.
- Preserve canonical witness trace, full compressed system, inputs, source,
  qualifier, rejection cases and failed development attempts. Focused Rust
  correctness/fmt/Clippy only; no solver invocation, timing comparison,
  admitted benchmark, sampling, EKS spend or broad root-library rerun.

Acceptance: all fixed checks pass and the public-input/domain contracts
remain explicit. A counterexample rejects this encoding or implementation.
Only finite native controls are checked; no full compiler proof is inferred.
Keep the original route criteria, useful-yield, family and full-cost gates.

Sources checked (KB runtime unavailable): RFC8032 section5.1.4, RFC7748
section4.1, HAC section3.6.2 Algorithm3.56, existing Rust point/Fp2 APIs,
existing scalar kernel source and frozen membership/many-coset records.
No novelty claim is made for these established component methods.
