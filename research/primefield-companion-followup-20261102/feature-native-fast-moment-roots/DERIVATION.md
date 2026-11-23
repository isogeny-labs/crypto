# Fast power-sum conversion in the public-point recovery path

2026-11-07, T-1799. Derived algorithm and unsigned native software evidence.
Prime fields only; no timing panel or cloud work.

## Scope and prior work

The saved lifted-sign-moments construction already extracts larger-transcript
counts and first moments with explicit2^J subset expansion. Reimplementing it
would not remove that cost. Here the selected bottleneck is the quadratic
coefficient-by-coefficient Newton conversion in feature-native-inert-seed.
The new adapter changes only how the moment vector becomes a root polynomial;
seed synthesis, sign source, later gcd updates and forward verification persist.

This is established formal power-series arithmetic, not a new identity or
new factorial algorithm. Sources checked (KB): MCP absent; repository wrapper
unavailable. Inspected CONTINUATION lifted-sign-moments, previous native root,
inert-seed and polynomial modules. Read FLINT exp/log/Newton documentation and
its linked source. The current documentation flags generated sections; no
reported tuning thresholds or runtime ratios are adopted. The proof below is
explicit, and exact checks compare with two separate algebraic references.

https://flintlib.org/doc/algorithms/polynomials/newton.html
https://github.com/flintlib/flint/blob/f5c2181a43be/src/nmod_poly/exp_series.c

## Moment identity and characteristic boundary

Let r>=5 be a caller-certified prime and n<r the candidate count. Let
mu_0=n and mu_j=sum_i t_i^j for 1<=j<=n. Define

    q(Z)=product_(i=1..n)(1-t_i Z),
    H(Z)=-sum_(j=1..n) mu_j Z^j/j mod Z^(n+1),
    W(Z)=H'(Z)=-sum_(j=1..n) mu_j Z^(j-1).

Taking the logarithmic derivative of the product gives q'=Wq modulo Z^n,
so q=exp(H) modulo Z^(n+1). Every denominator1..n is a unit because n<r.
The output polynomial is X^n q(1/X), with EXACT length n+1 even when some
roots are zero. Trimming q before reversal would lose those zero factors.
Repeated roots are also valid for this conversion identity. The internal
recovery RootSet still receives distinct roots from an actual candidate set.

The public helper validates prime-word domain, moment length, canonical
residues, count n<r and the degree cap before new allocation. Primality and
provenance of the supplied moments remain caller preconditions. Matching
formal identities does not authenticate an arbitrary caller's moment vector.
The word bound r<=u64::MAX/2 and u128 products match the existing backend.

## Coupled precision doubling

Maintain q correct modulo Z^m, length m, and g=1/q modulo Z^m. Let
N=min(2m,n+1), ell=N-m. The differential residual R=Wq-q' starts at degree
m-1. Since q has degree at most m-1, q' has no coefficient at degree m-1
or higher. Therefore take

    A=coefficients_(m-1..N-2)(Wq),
    V=(g A) mod Z^ell,
    E_i=V_i/(m+i), 0<=i<ell,
    q_new=q+Z^m (q E mod Z^ell).

Indeed eta=H-log(q) starts at degree m, with eta'=R/q. Replacing the full
reciprocal by g changes R/q only from degree2m-1 onward; integration changes
eta only from degree2m onward. Thus Z^m E equals eta modulo Z^N, and
q_new=q(1+eta)=exp(H) modulo Z^N because eta^2 starts at degree2m>=N.

If another stage is needed, q_new*g=1+Z^m D modulo Z^N. Update

    g_new=g-Z^m (g D mod Z^ell).

The geometric inverse identity proves g_new=1/q_new modulo Z^N. Skip this
update at final precision because no later stage consumes the reciprocal.
Initialization q=g=1 modulo Z handles n=0 and starts every other case.

Implementation uses one truncated Wq product, two low products for the
correction, and two more products for a nonfinal reciprocal update. Existing
Karatsuba/truncated multiplication is reused; there is no coefficient-wise
quadratic summation in the new converter and no full inverse-series rebuild.
The initial validation-01 implementation used exp/log with rebuilt inverses;
its passing source is preserved as a snapshot. Validation-02 binds this
coupled final algorithm. The old quadratic adapter remains available as a
reference and is imported for seed APIs; it is not called to initialize roots
by the new recovery path.

## Arithmetic and storage accounting

At each precision, a constant number of polynomial products of O(m) length
plus linear coefficient operations are performed. Under the installed
Karatsuba bound M(m)=O(m^log_2(3)), summing geometrically increasing sizes gives
O(M(n)+n) field work, versus O(n^2) in the coefficient-wise reference. The
inverse table1..n is filled by the standard linear modular recurrence; no
field exponentiation is required by this converter. This is a derived
arithmetic bound, not a measured runtime or a comparison against FLINT.
Small-degree overhead and a practical dispatch threshold are unmeasured.

Retained arrays and precision-stage temporaries remain O(n), plus backend
multiplication workspace. This does not compress the root set: consecutive
triple candidate counts are generally of order r/8. The full moment prefix,
seed setup, source evaluation, later gcds and final scalar-to-point check
remain charged. Diagnostic doubling/Karatsuba/reciprocal-update counts are
coverage data, not full arithmetic counters or performance measurements.

## Composition and evidence

The generator extracts the existing RootSet and RecoverySource declarations
and implementations unchanged, pins source/output hashes, and resolves their
local Consecutive type to the new converter. Its moment acquisition uses the
same capped consecutive seed constructor. Recovery still rejects malformed
coordinates before addition, handles the first-three zero endpoints, enforces
query/degree limits and verifies [candidate]P=Q.

Exact native tests cover zero and repeated roots, all degrees0..r-1 for six
small primes, doubling boundaries127/128/129 and511/512/513, and degree1024
over F65537. References are direct multiplication of linear factors and the
separate coefficient-wise Newton formula. Candidate roots for all88 prior
triple classes are compared with the previous adapter. The20 frozen public
cases at F509, y^2=x^3+2x+10, P=(3,68), r479 compare returned scalar, complete
sign transcript and initial root degree. Endpoint477, wrong-scalar, coordinate,
count/shape/canonical-value, query and degree caps are included.

No original4563-case rerun, timing panel or new cloud submission occurs. This
component reduces conversion arithmetic; larger-transcript aggregation without
field-sized root storage remains the next mathematical construction.
