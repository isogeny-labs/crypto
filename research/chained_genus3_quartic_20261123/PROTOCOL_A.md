# Fixed F5 bielliptic plane-quartic source qualification: A

ACTIVE-only4edcdfe9 and draft PR5012 precede this implementation. The
previously published prospective family af2a50 is unchanged. This is a finite
native correctness qualification, with no elapsed-time or DLP advantage claim.
All statements and any later results are unsigned. [pcc:unverified]

For every lexicographic (a,b,c) in {0,1,2,3,4}^3 and kappa in {1,2}, use

```text
C: (kappa Z^2+aX^2+bXY+cY^2)^2=Y(X^3+2XY^2+Y^3).
```

Retain all250 equations, including singular cases; these are equations, not
distinct isomorphism classes or a complete moduli census. Select the first
lexicographic smooth row for each flag below, or preserve an empty outcome.
Never change the field/family/selection after seeing counts. [pcc:unverified]

## Exact source and point-count gates

D is squarefree. Writing v=kappa Z^2+g, the derivative in Z is4 kappa Zv.
A geometric singularity with v=0 would force the binary D and its gradient
to vanish. Thus a singularity must have Z=0, and is exactly a multiple
projective root of h=g^2-D. If a=0, h has degree3 and a simple infinity root;
if a!=0, h has degree4 and no infinity root. Use exact F5 polynomial gcd
of h and h'. Independently compare its degree with nullity of the Sylvester
matrix, reduced by ordinary F5 Gaussian elimination. Require g=0 smooth and
g=x+1 singular, with the actual point (X,Y,Z)=(0,1,0) having zero polynomial
and all partial derivatives. Both kappa values have identical smoothness.
The fixed positive/negative are controls, not acceptance predictions for any
new source candidate. Smooth plane quartics are nonhyperelliptic genus3
curves. This geometry is written, not a generic Mathlib theorem. [pcc:unverified]

Construct F5, F25=F5[b]/(b^2-2), and F125=F5[t]/(t^3+2t+1).
Native checks establish irreducibility (quadratic nonsquare and cubic with
no F5 root), then every addition/multiplication pair against independent
digit arithmetic/Horner multiplication by the field generator. Check every
triple for associative multiplication, associative addition and distributivity;
every nonzero inverse, a^q=a, and Frobenius additivity/multiplicativity.
No Python field, point-count, polynomial or selection calculation is allowed.
[pcc:unverified]

For every smooth row, count all projective points over each of the3 fields:
(1) direct evaluation at Y=1 of every x,z pair plus X=1,Y=0,z; and
(2) independently enumerate elliptic points (x,v), v^2=x^3+2x+1,
then use a precomputed histogram of squares to count solutions of
kappa*z^2=v-g(x), together with the same infinity fibre. The omitted
projective point [0:0:1] has nonzero polynomial. The direct evaluator expands
the square, while the fibre route uses the quotient variable v. Require equal
counts. The elliptic counts must be7,35,112. For each kappa pair, odd-degree
counts sum to14/224, and degree2 counts agree. Print every smooth row's
counts, Weil polynomial, elliptic quotient polynomial, Jacobian order, flags
and mutation result; print every singular row and its gcd degree. [pcc:unverified]

With s_m=5^m+1-N_m, use exact integer Newton identities
a1=-s1, a2=(s1^2-s2)/2, a3=-(s1^3-3s1*s2+2s3)/6;
assert integral divisions and the reciprocal degree6 polynomial
[125,25a1,5a2,a3,a2,a1,1] in ascending order. Require exact monic
integer division by PE=T^2+T+5 on every smooth row. Mutate N3 by+3,
require division failure, then restore pristine coefficients and require
division success again. This is a real altered-count control. [pcc:unverified]

Record separately (i) mod2 Phi3^3=[1,1,0,1,0,1,1] and
(ii) full PE^3=[125,75,90,31,18,3,1]. A full PE^3 source predicts
N1=9,N2=53,N3=84 and #J(F5)=343. A mod2-only candidate is not an
E^3 source. No stable3-dimensional J[2] subspace follows from all simple
Frobenius-module factors having even degree; this excludes a first rational
(2,2,2) factor of [2], not every transfer or computational reduction.
Nontrivial mixed kernels, actual polarization/J[4], quotient-Jacobian/Serre
descent, elliptic DLP preservation and full cold costs remain separate gates.
[pcc:unverified]

## Publication, execution and custody

Publish and fully read back every locked source/protocol/runner/metadata file
and source lock before the first Rust format, compile, Clippy or native call.
Rust1.99.1 edition2021, -Dwarnings, overflow/debug assertions, one worker,
unchanged2GiB address-space/120s per phase. The copied source and final
read-only qualifier have pre/post source/tool/runtime/build guards. Preserve
every first failure and repair in a new immutable version, with its own
protocol/lock and publication. No suppressions or resource increases.
Python is exclusively file/Git/hash/JSON custody. All small raw outputs and
commands are published; whole original ELFs remain custody-pending until
approved-store or CI-artifact retention. No timing comparison. [pcc:unverified]

The original EKS-only tasks, paid STOP, locked crypto-autoresearcher vault,
existing owners, normal CI/merge requirements and uncompiled generic Mathlib
bridge remain binding. This development test does not replace any of them.
The research report/PDF will include the complete finite family and failures.
[pcc:unverified]

## Sources checked (KB)

KB/Conductor unavailable; the repository fallback enters the locked vault
and is not invoked. Exact-source review: prior af2a50 successor, CM19
literature and the distinct finite-plane-quartic qualification described in
PRIMARY_SOURCE_REVIEW.md. No novelty or literature-absence claim. The
primary-source F7 fixture is a separately frozen prospective follow-up only;
it does not change this250-equation F5 gate. [pcc:unverified]
