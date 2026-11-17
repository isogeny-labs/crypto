# Local terms remove intermediate Miller singularities

Producer derivation and software checks; not independent verification. [pcc:unverified]
Task T-1784, 2026-11-07. Builds on feature-orbit-divisor-compression without
changing its immutable claim or evidence. Prime fields only.

## Regularized subgroup update

Use the preceding notation: H=<P> of odd prime order r!=p, p>3,
K(X)=product_{j=1}^{(r-1)/2}(X-x(jP)), F=K(x), and
c_T=2y(T)K'(x(T)) for nonzero T in H. Let u_T(Q)=t(Q-T), t=-x/y.
Then F(T+R)=c_T t(R)+O(t(R)^2). For nonzero S,T,T+S in H, taking
leading terms in the prior rational translation identity gives

    c_(T+S) = c_S c_T / f_(r,-S)(T).                    (1)

The normalized Miller function has divisor r[-S]-r[O], so its value at T
in this domain is nonzero. Intermediate line zeros can still occur. Equation
(1) uses regularized values, not pointwise cancellation of zeros of F.
Taking (1) at T and U gives a subgroup version of the anchor-free cross ratio:

    c_(T+S)c_U / (c_T c_(U+S)) = f_(r,-S)(U)/f_(r,-S)(T).

No outside point is required. If c_S and c_T are supplied, one Miller value
updates c_(T+S). This does not produce either supplied anchor from nothing.
These c values are kernel local coefficients, not automatically the existing
division-jet normalization seed; an equality between those objects is not assumed.

## What a self-zero evaluation reveals

At Q=-S+R, F(Q+S)=t(R)^(-(r-1))(1+O(t(R))), while
F(Q)=c_(-S)t(R)+O(t(R)^2). Hence

    f_(r,-S)(-S+R) = -c_S^2 t(R)^r + higher terms,     (2)

because c_(-S)=-c_S. The evaluator can therefore obtain c_S^2 in a
logarithmic-length Miller chain, without constructing K. Recovering the
oriented c_S remains a sign problem. For the existing image p-1=q*m,
m odd, q=2 or4, pi(-1)=-1. Thus the two square roots have opposite projected
images. Equation (2) alone is not a public odd-weight sign anchor.

## Native local arithmetic

local.rs represents a nonzero rational function locally by (integer order,
nonzero leading coefficient). Products add orders and multiply coefficients;
quotients subtract orders and divide coefficients. No series of length r is
materialized, even at a zero of order r. Checked i128 stores the order.

At a finite evaluation point T with y(T)!=0, use u=x-x(T). Expanding the curve,

 y=y0+y1*u+y2*u²+y3*u³+...,
 y1=(3*x0²+a)/(2*y0),
 y2=(3*x0-y1²)/(2*y0),
 y3=(1-2*y1*y2)/(2*y0).

For the nonvertical line y-y(A)-lambda*(x-x(A)), select the first nonzero
coefficient among its value, y1-lambda, y2, y3. No line has intersection
multiplicity above3 with a nonsingular cubic. This also follows here because
if y2=0 then y3=1/(2*y0)!=0. A vertical line has order1/coefficient1 when
x(T)=x(A). At infinity, a nonvertical line has order-3/coefficient-1 and
a vertical line has order-2/coefficient1.

Normalize each chord quotient to leading coefficient1 at infinity: negate
nonvertical line/vertical quotients; keep vertical opposite-point factors.
Identity-point additions use1. Binary Miller accumulation then maintains
normalization throughout. The final point is checked to be O.

The finite u coefficient converts to the translated invariant parameter by
multiplication by (2*y(T))^order (or inverse power for negative order). Since
u=2*y(T)*t(Q-T)+higher terms, this is exactly the leading-term conversion.

The native endpoint accepts curve, public base point, r, and public evaluation
point, with no scalar labels or subgroup enumeration. Arithmetic is fixed-width:
caller-certified prime 3<p<=u64::MAX/2 and caller-certified odd prime r in the
same word bound, r!=p. Curve coefficients/point membership/nonsingularity are
checked; base y!=0 and r-torsion are checked. Finite y=0 evaluation is rejected;
O uses its own parameter. Utility plus/negative/invariant methods assume validated
points (the Miller entrypoint validates its inputs). This is a research component,
not an arbitrary-big-integer cryptographic backend.

Binary Miller has O(log r) chord steps and O(1) stored field elements. This
implementation uses Fermat inversions; field-operation cost includes their
O(log p) products. No timing, total-source speedup or secret recovery is measured.

## Software checks actually run

validation-01 records a bounded optimized rustc test build (90s cap, overflow
checks) and test invocation (30s cap), both exit0, no compiler warnings. Four
methods pass. The receipt pins source, binary, commands, and output hashes.
The preserved log's harness elapsed time is not a benchmark.

* Kernel derivative reference: enumerates H only in cfg(test), forms its distinct
  x roots and computes the derivative at a root by the product of differences.
  It never invokes Miller to produce an expected coefficient. All nonzero S,T
  are used for E/F5:y²=x³+1,r3; E/F5:y²=x³+2x+1,r7;
  E/F7:y²=x³+3,r13. For E/F509:y²=x³+2x+10,P=(3,68),r479,
  eight fixed S and twelve fixed T indices are used, excluding T=-S in (1).
  Each S checks normalization at O and self-zero order/coefficient (2).
  No fixture-dependent values or indices enter the evaluator.
* Explicit F5 reference f_(3,(0,4))=-(y+1) checks ordinary values and the
  order3 local zero. Separate line contacts exercise multiplicities2 and3.
* F7 primitive checks verify normalization and exercise simultaneous numerator
  and denominator zeros, retaining a nonzero leading coefficient after cancellation.
* Reject finite y=0 evaluation and a base whose r-multiple is not O.

The kernel-product reference uses the same field/group primitives, so this is
an algebraically different software reference, not independent implementation
review. Original4563-case cohort, source runtime and old artifacts are unchanged.
No experiment panel, cloud job or scalar recovery ran.

## Sources and next work

KB unavailable: prior discovery found no loaded tools; wrapper status again
reports no executable. Reviewed prior orbit-divisor-compression derivation,
feature-native-division and value-division contracts. The latter have a different
normalization seed and must not be conflated with c_T. Read Miller1986 Algorithm1
https://crypto.stanford.edu/miller/miller.pdf and Lynn's chord recurrence
https://crypto.stanford.edu/pbc/notes/elliptic/tate.html . Local line expansions
above follow directly by coefficient comparison. No novelty priority claimed.
The referenced isolab-eks skill path is absent in this sparse checkout; no cloud
work attempted. This invocation is a bounded local software unit check.

Next: use the regularized function to evaluate paired local-coefficient ratios,
then derive the exact divisor of the source's partial geometry product. Seek
support cancellation that preserves its odd-weight sign. The self-zero square
is available now, but does not close the sign branch.
