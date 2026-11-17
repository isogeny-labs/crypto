# Scalar-only native division recurrence

## Purpose and interface

The original elliptic sign source often receives its normalization seed from
an outer recurrence. Its subsequent division-value requests need W_n(Q), not
D W_n(Q). `ValueDivision` implements this path without constructing jets or
calling the phase engine. Input remains a public prime-field curve and point;
no target scalar, orbit lookup table or precomputed sign enters the API.

The prior `feature-native-division` module stays immutable and remains the
normalization/derivative engine. There is no implicit promotion or cache sharing
between the two types. A future caller should use value-only mode when its seed
is supplied, and retain the jet engine when it actually needs normalization.
Constructing both duplicates validation/base work and must be charged if done.

## Algebra and cost

The standard even and odd recurrences are unchanged. Scalar powers are expanded:

    W_(2t+1) = W_(t+2) (W_t² W_t) - W_(t-1) (W_(t+1)² W_(t+1))
    W_(2t) = (W_t / W_2) [W_(t+2) W_(t-1)² - W_(t-2) W_(t+1)²].

With inverse W_2 computed once, each newly generated nonbase node uses exactly
six field multiplications and one subtraction after dependency values are
available. Odd: two squares, two cube products, two outer products. Even: two
squares, two cross products, multiplication by inverse W_2 and the final product.
A cache hit performs no recurrence arithmetic. These are algebraic counts of
the explicit expressions, not measured timings or a complete-cost comparison.

Constructor validation, base W_0..W_4 calculations and the inverse are excluded
from the six-per-node count and remain costs. A BTreeMap retains indices and
scalar values, plus allocator/node metadata. Each scalar wrapper also stores
the modulus; one field value does not mean an eight-byte complete cache entry.
There are no derivative values or derivative arithmetic in the production
module. The checked jet module is included only under cfg(test).

New-base powers reuse x²,x³,x⁴,x⁶,a². The backend remains fixed-width:
5<=p<=floor(u64::MAX/2), canonical a,b,x,y, nonsingular curve, point membership,
nonzero y and invertible2y. Primality is a caller obligation. Products reduce
u128; canonical additions/subtractions fit u64. Values may be zero at torsion
indices and are retained normally. Indices use u64; no negative-index API.

## Checks actually executed

At sourceb3b7dd54a375d1d9ec1332aa998d6066588bdb40, rustc1.99.1 compiled one
standalone unit binary and one invocation filtered to value_tests:: passed all
3 selected methods. The3 historical jet tests were filtered out, not rerun.

- All9 original saved setup division values match; repeated requests do not
  add retained indices.
- For the original generators on F353,F509,F701 and their negatives, indices
  0..63,r,(r+1)/2,u64::MAX agree with the preserved checked jet engine. The
  independent antipodal relation W_n(-Q)=(-1)^(n-1)W_n(Q) is also checked,
  with W_0=0 handled explicitly. The maximum-index checks exercise integer
  recursion bounds; they are not cryptographic-size field tests.
- Canonical-input/domain, singular-curve, off-curve and zero-y rejections,
  W_0=0,W_1=1 and W_479(P)=0 pass.

Source/reference/fixture hashes match the precheck lock. Receipt, binary hash,
compiler version and raw logs are preserved. No fresh source signs, scalar
recovery, full cohort, cloud job or timing benchmark ran. Unit-test elapsed text
is diagnostic only. No workspace-wide Cargo test pass is claimed.

## Next integration

Select the value-only engine for supplied-seed calls in the native source;
retain jets for normalization. Port the point-grid and normalized recurrence/
resultant stages and connect the native sign-mask decoder. Keep the original
three curves, all arms and4563 targets. Full validation still needs a complete
native source/runtime manifest and authoritative admission. This unit check
neither replaces nor reduces that cohort.

## Sources and claim classification

- Preserved native jet core and its byte-preserved original Python kernel;
  immutable fixtures from original public setup records.
- [Mathlib EDS documentation](https://leanprover-community.github.io/mathlib4_docs/Mathlib/NumberTheory/EllipticDivisibilitySequence.html):
  standard even/odd relations, reused here. No recurrence novelty claimed.
- Experiment-protocol guidance distinguishes algebraic derivation/software
  checks from comparative measurements. No timing or statistical conclusion
  is made. The referenced controlled-experiment skill path was absent in this
  checkout; isolab-eks explicitly permits local builds and unit checks.
