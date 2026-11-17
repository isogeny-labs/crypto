# Universal linear shifted-character constructor and a matching lower bound

## Identity for every odd prime

Let r be an odd prime, h=(r-1)/2, n=h-1, and chi(0)=0 with chi the quadratic
character into F_r. Euler's criterion gives chi(X)=X^h at every X in F_r.
With Delta f(X)=f(X+1)-f(X), finite differences give

    Delta^n X^h = sum_{a=0}^n (-1)^(n-a) binom(n,a) (X+a)^h
                = h! (X+n/2).

Here 1/2 is in F_r. To prove the second equality, each difference lowers
degree by one and multiplies the leading coefficient by the old degree;
the resulting degree-one polynomial has leading coefficient h!. Replacing
X by -X-n reverses the sum and negates it, since n+h=2h-1 is odd.
Its unique antisymmetry center is -n/2, fixing the constant coefficient.
For h=1 this is Delta^0 X=X. All factorials used are nonzero because h<r.

Consequently the universal constructor is

    X = -n/2 + sum_{a=0}^n c_a chi(X+a),
    c_a = (-1)^(n-a) / (h * a! * (n-a)!).

There are exactly h nonzero character coefficients. They depend only on r;
no labels, interpolation system, field sweep or point-to-scalar table is
used to construct them. The finite-difference operator is a compact symbolic
formula, but evaluating it still requires h source-character values here.

## Sharp lower bound within the fixed affine-linear family

Suppose for all X in F_r,

    X = b + sum_{i=1}^q d_i chi(X+a_i),

with distinct a_i, fixed coefficients d_i and a fixed b. Repeated shifts
can be combined and zero coefficients removed. For q=0 a constant cannot
represent X. Assume 0<q<h. Both sides are polynomials of degree at most
h<r after replacing chi(X+a) by (X+a)^h. Equality at all r points therefore
implies equality as polynomials. Their coefficients in degrees
h,h-1,...,h-q+1 (all at least 2) must vanish:

    sum_i d_i a_i^j = 0,  j=0,...,q-1.

The binomial coefficients divided out here are nonzero modulo r since h<r.
This is a square Vandermonde system. Its determinant is the product of all
nonzero differences a_j-a_i, so every d_i=0, contradicting the coefficient
of X. Thus q>=h, and the constructed h-shift identity attains the bound.
The same bound holds for any nonconstant affine target. Public nonzero
input scalings do not evade it: chi(uX+v)=chi(u)chi(X+v/u).

This bound is specific to fixed affine-linear combinations. It says nothing
about nonlinear products, adaptive choice of shifts, other source functions,
or all possible scalar recovery algorithms. In particular it does not bound
the existing cubic-feature decoder. Nor does a feature-dimension count alone
prove a lower bound for the particular target X in that nonlinear span.

## Native construction and cost

lib.rs accepts a caller-certified odd prime 3<=r<=u64::MAX/2 and an explicit
max_terms limit. It rejects h>max_terms before allocation or factorial work.
It computes (h-1)!, one Fermat inverse (verified by multiplication), the
inverse-factorial array by backward recurrence, and all coefficients.
Since h=-1/2 in F_r, its inverse is -2; no second inverse is needed.
Construction takes O(h+log r) field products and O(h) storage, with two
h-entry arrays transiently. The immutable result retains h coefficients
and the constant. The inverse check does not certify primality.

Evaluation requests exactly h ternary signs in ascending offset order,
adds or subtracts their coefficients, and rejects the first invalid sign.
It does not use the 64-offset bitmask representation. Canonical sums stay
below 2r<=u64::MAX-1. Each request's source cost is additional; the existing
public-point source implementation is imported without changes for tests.

For r=479: h=239, so the result has 239 character coefficients plus constant
360. The old compact cubic packet uses 16 character requests and 477 terms.
The new identity resolves construction uniformly in r for the linear family,
while increasing source requests on this fixture. No runtime gain is claimed.
The next structural task is a label-free compact nonlinear constructor, or
an algebraic way to evaluate the aggregate without materializing h signs.

## Exact checks and provenance

One bounded optimized Rust build with overflow-checks=yes and one bounded
invocation pass all 41 tests, none filtered. Four new test methods check:

* Every X in every odd prime field r<=127, and in r=479: 2197 identities
  across 31 prime fields. Native trial division certifies these test primes.
* Independent polynomial expansion of all shifted powers for each of the
  30 odd primes <=127. Every coefficient equals the polynomial X exactly.
* Domain and allocation limits, a composite rejection control (not a primality
  test), and first-invalid-oracle call order.
* All eight saved public-point fixtures on E/F_509 with subgroup order 479,
  using unchanged source windows of length 239. Only Q enters the source;
  expected scalars are checker-only. This is source/constructor integration,
  not an additional production recovery API with its own group verifier.

Sixteen source/data hashes were recorded and verified before build, then
verified after build. Four compiler warnings remain in build.log. The binary
hash, commands and timeout bounds are in receipt.json. No cloud job or timing
panel ran. The original 4563-case cohort and previous decoder evidence remain
unchanged. The general theorem is established by the argument above; finite
tests check the implementation and are not substituted for the general proof.

## Prior work

Finite differences and degree reduction are standard, not claimed novel.
Reviewed the existing feature-coefficient Schur derivation and source before
construction. Read the original Illinois course notes on finite differences
(2026-11-05):
https://courses.physics.illinois.edu/cs418/fa2025/text/finite-differences.html
and NIST DLMF section 26.8 on Stirling numbers and their finite-sum identities:
https://dlmf.nist.gov/26.8.
The explicit proof above avoids relying on a floating-point difference formula
or lifting a numerical conclusion across characteristics. The lower bound is
an elementary Vandermonde argument; no novelty priority claim is made.
