# Streamed singleton resultants and fused source tail

T-1832 · 2026-11-07. Derived algebraic/operation bounds with exact native producer
checks. No timing measurement, novelty claim or independently signed proof.

## Sources and domain

Read predecessor `feature-native-source-accounting/{DERIVATION.md,lib.rs,polynomial.rs}`.
Primary reference: [FLINT resultant documentation](https://flintlib.org/doc/nmod_poly/index.html#resultant-and-discriminant),
including the root-product convention and Euclidean/half-GCD routes. This is a
standard resultant specialization. KB unavailable (no MCP; repository wrapper
reports missing executable), so exact local code and primary documentation are
the fallback. No assertion about unsearched literature is made.

The polynomial interface retains caller-certified prime p, 3<=p<=u64::MAX/2.
The integrated source retains its stronger p>3, odd prime r>=7, r!=p,
gcd(r,p-1)=1 and word-size bounds. Input root/point slices may contain repeated
values, zero, and noncanonical u64 representatives. The new branch normalizes
operands before subtraction. Empty inputs retain Error::EmptyTree.

## Identity and orientation

For monic f(X)=product_i(X-x_i), g(X)=product_j(X-y_j), the source uses

    Res(g,f)=product_j f(y_j)=product_(i,j)(y_j-x_i).

If the point slice is [y], compute product_i(y-x_i). If the root slice is [x],
compute product_j(y_j-x). Both branches use point-minus-root; no swap sign is
introduced. The first branch handles both singleton inputs. Multiplicities are
retained, and any common root makes the product zero. This gives a full-field
representative of exactly the required resultant, hence the same multiplicative
class in Fp*/(Fp*)^q with zero adjoined. Projection is applied only where the
existing interface requests it. No additive quotient operations are introduced.

When both lengths exceed one, the previous root-product/half-GCD implementation
is unchanged. root_projected_resultant delegates to root_resultant_class and
projects once, ensuring the same specialization in both APIs.

## Exact operation and workspace bound

For N>=1 factors, initialize the accumulator with the first difference and
multiply the remaining N-1 differences. This uses exactly N-1 field products,
N differences, zero field inversions, zero coefficient-polynomial constructions,
and O(1) auxiliary scalar storage excluding the supplied slices and returned
class. It does not stop early on a zero factor, so the product count is fixed
by public lengths even for zeros. Reduction/index/addition/comparison costs are
additional. With reduced operands and p<=u64::MAX/2, each subtraction written
as y+p-x fits u64, and product intermediates fit u128.

The old path constructs degree-N and degree-one polynomials and runs the general
resultant. Its coefficient arrays require at least linear storage; its exact
product/inverse count depends on coefficients and remainder branches. This
round does not invent a fixed old count or claim a measured speed ratio.

## Source fusion

In the source partition h=(r-1)/2=2wk+2s+epsilon, every nonempty tail has the
single public center z=grid.last()[0] and the first s baby x-coordinates.
Previously the baby loop collected these into prefix_x, then constructed a
root polynomial and called root_resultant_class(prefix_x,[z]).

The center is available before this loop. Accumulate the same ordered factors
z-x_j when j<s, initializing from the first factor. The loop invariant is:

    after t prefix entries, tail_resultant=product_(j<t)(z-x_j), 1<=t<=s.

Induction is immediate from the first assignment and subsequent class combines,
each of which multiplies representatives modulo p. On loop exit it equals the
old tail resultant class. The existing prefix division-value pair, tail scaling,
parity signs, final combination and projection are unchanged. For s=0 there is
no tail accumulator; for s>0 the partition guarantees s iterations, so the
Option extraction succeeds. Internal malformed geometry is not a new public API.

Thus the tail polynomial trace term in T-1831 is replaced by exactly (s-1)M,
zero I and s differences, with no prefix_x vector, coefficient polynomial or
second pass over that vector. Other source arrays (babies, centers, grids,
division arena, main resultant and window snapshots) remain. This is a bound
on the removed tail temporary, not on total source memory.

The singleton API can also specialize a main product whose public dimension is
one. General nonsingleton main resultants remain trace-dependent. No full-source
phase dominance or elapsed-time conclusion is inferred.

## Validation and custody

`validation-01` checks the API specialization before source fusion; `recovery-01`
and `normal-build` also precede fusion. `pre-fusion/lib.rs`, generate.py and
generation.json preserve the changed inputs so their receipts remain replayable.
`validation-02`, `recovery-02` and `normal-build-02` bind the final fused source.
No failed attempt was discarded; both versions passed.

The final deterministic native checks contain:
- 8,320 singleton cases across p=5,7,17,41,97,193,257,509; lengths1..65,
  both singleton orientations, repeated/all-equal lists, noncanonical inputs
  and forced common roots. Expected values come from unchanged general
  coefficient resultant plus direct field products/projection.
- 3,852 zero outcomes and 2,276 odd-degree orientation-sensitive controls.
- Audit assertions of exactly N-1 products, zero inversions and zero calls to
  root_product in every singleton call; 128 nonsingleton fallback comparisons.
- 24 empty-input controls and a p=2^61-1/u64-boundary canonicalization control.
- Inherited source1108points/24windows; 160660 division output equalities,
  4100 typed-lifetime plans and8 prepared-source reuse controls.
- Recovery617targets/5929calls/zero unresolved and63 public comparisons/511calls.

Audits compile only with resultant_audit/source_audit. Final normal library
build excludes both and pins seven native dependencies. No external benchmark,
cloud execution, full Cargo-suite rerun or change to the original4563 cohort.
The prior full-suite missing-fixture/deadline result remains in the branch log.

Reproduce from this directory with `python3 generate.py`, then
`python3 validate.py NEW_OUTPUT_DIRECTORY` and
`python3 validate-recovery.py ANOTHER_NEW_OUTPUT_DIRECTORY`.
Use new directories; preserve existing receipts. The normal build command is
recorded in normal-build-02/receipt.json. Generator pins predecessor source;
validation receipts also pin the separate singleton-checks.rs. Warnings are
preserved byte-for-byte. Next examine the nonsingleton resultant remainder trace
and fused data lifetime to remove larger coefficient intermediates.
