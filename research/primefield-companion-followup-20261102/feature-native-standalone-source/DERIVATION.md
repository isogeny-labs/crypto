# A projected character source without a full-value runtime validator

T-1788, 2026-11-07. Producer mathematical derivation and native software checks;
unsigned, not independently verified. [pcc:unverified]

## Domain and new route

Prime field p>3, odd prime subgroup order r>=7, r!=p, gcd(r,p-1)=1.
Write p-1=q*m with m odd, q=2 or4; pi(z)=z^m. Q is in <P>.
The new Source takes public curve/base/order data and a geometry-lane limit.
It computes the projected source directly using the existing half-GCD geometry
residual and a value-only division cache. It does not call the old full-value
source, derivative normalization, transported window or paired validator.
The old implementation is imported only under cfg(test).

The first investigation considered splitting interval products into even and
odd factors. The division chain rule instead yielded the character proof below,
which supports a concrete runtime change now: remove the development reference
from a standalone query path. No new sub-square-root product recursion is claimed.

## Ward correction covariance, without derivative normalization

For a nonzero r-torsion point T, let W_j(T)=psi_j(T), and define

 A(T)=W_(r+2)(T)/(W_(r+1)(T) W_2(T)),
 B(T)=W_(r+1)(T)^2 W_2(T)/W_(r+2)(T).

These denominators are nonzero for prime r>=7. Ward gives
W_(kr+v)(T)=W_v(T) A(T)^(kv) B(T)^(k²).
The division-polynomial chain rule is
W_j([n]T)=W_(jn)(T)/W_n(T)^(j²), for W_n(T)!=0.

For n not divisible by r, apply the two identities to j=r+1,r+2,2:

 W_(n(r+1))(T)=W_n(T) A(T)^(n²) B(T)^(n²),
 W_(n(r+2))(T)=W_(2n)(T) A(T)^(2n²) B(T)^(n²).

Insert these into B([n]T). A and W_(2n) cancel, leaving

 B([n]T) W_n(T)^(r²) = B(T)^(n²).                     (1)

The exponent on W_n is obtained directly as
2-2(r+1)²-4+(r+2)²=-r² in the expression for B([n]T).
No differentiation or division by n is used; characteristic-divisible indices
are allowed. This identifies a point function, rather than assuming that an
arbitrary normalization seed transports correctly.

Set C(T)=pi(B(T))^-1. Since r²=1 modulo q,

 C([n]T)=pi(W_n(T)) C(T)^(n²).                        (2)

Also B(-T)=-B(T): W_(r+1) is even-indexed and changes sign, its square does not;
W_2 changes sign; W_(r+2) is odd-indexed and does not. As m is odd,
C(-T)=-C(T). No radical or derivative seed is needed for C.

## Half-set permutation proves the character

Let h=(r-1)/2 and F(T)=product_{j=1}^h C([j]T). For Q=[k]P!=O, write
[jk] modulo r as epsilon_j*t_j, with1<=t_j<=h and epsilon_j in{+1,-1}.
The t_j permute1..h. Oddness of C therefore implies

 F(Q)/F(P)=product_j epsilon_j.

Multiplying jk=epsilon_j*t_j modulo r and cancelling the nonzero h! gives
product_j epsilon_j=k^h mod r. Euler's criterion identifies this with the
quadratic character of k. The sign ±1 is then represented in F_p, not by
identifying arbitrary F_r residues with F_p elements. Thus the same abstract
sign holds in the target image, and

 F([k]P)/F(P)=Legendre_r(k).                           (3)

Equation (2) expands F(T) as
pi(B(T)^(-sum j²) product_{j=2}^h W_j(T)), the phase-free interval from the
previous round. Its block-pairing proof equates this value with the current
projected geometry residual times its three division factors. Hence (3)
proves the mathematical character law for the new route, without relying on
the old derivative normalization theorem or its runtime computation.

The point-input requirement is Q in <P>. The implementation checks r-torsion;
under the retained coprime domain, the rational r-torsion cannot have rank2
(the Weil pairing would force r to divide p-1). No broader subgroup-membership
claim for arbitrary parameter domains is made.

## Exact implementation

lib.rs imports the existing geometry grid, value-only division module and
feature-native-half-gcd/residual.rs unchanged. It computes the same partition
h=2wk+2s+e and requests only nonzero projected exponents among

 (index,exponent) = (N,4wk+2s+e), (2w,2w*k(k-1)(k+1)/3), (s+e,2s),
 N=(r+1)/2.

Exponents reduce modulo q. The divisible-by3 factor is removed as an integer
before modular multiplication. One cache first validates W_r=0 and then supplies
all needed values; no duplicate validation cache is built. The monomial is
projected, multiplied by the projected geometry result and compared directly
with the stored calibration or its negative. Identity returns0. Any other
normalized character value returns Character; there is no fallback search.

The beta function above belongs to the proof/reference checks; runtime does not
compute beta, a derivative seed or the linear interval product. Their combined
powers cancel in the existing factored geometry expression.

Prime p,r are caller-certified. Backend bounds are p,r<=floor(u64::MAX/2).
Curve membership/nonsingularity, nonzero y, r-torsion, projection domain and
nonzero factors are checked. max_lanes is checked before geometry allocation,
including during calibration. The intended r-torsion point is finite and nonzero;
identity is handled by sign(None). This component currently exposes single-point
raw/sign APIs; it is not a replacement for prepared multi-point transport yet.

The normal-build dependency record has no feature-native-source/,
feature-native-division/, feature-native-normalized-transport/ or
feature-native-half-gcd/transport.rs input. These exclusions are checked against
rustc's dep-info, in addition to the cfg(test) source boundary. Metadata compilation
checks this dependency distinction; it is not a native timing comparison.

## Validation actually executed

validation-01 records an optimized native test build (90s), one selected test
invocation (30s), and a normal metadata/dep-info build (90s), all exit0.
Three methods pass;91 imported methods filtered. Twenty-four transitive Rust/
fixture inputs are hashed before and after. Test binary and normal metadata
hashes, compiler version, commands, outputs and dependency exclusions are recorded.

* All496 nonzero points of F5:y²=x³+2x+1, r7,P=(0,1);
  F7:y²=x³+3,r13,P=(1,2); F509:y²=x³+2x+10,r479,P=(3,68).
  New signs agree with scalar Euler signs and old source signs; raw values agree
  with projected old raw values. Three identity controls return0. Scalars occur
  only in the test harness, never in the query API.
* Equation (1) is checked for all496 nonzero multiples of those bases, using
  independently requested division values at the multiplied point.
* Lane cap, unsupported projection order, non-r-torsion, off-curve and
  noncanonical point controls reject.

No test failed. The test build emits10 unused-code/import warnings and the normal
build12; complete logs retained. Shared field/division/polynomial code means
these tests are not independent implementation review. This is software validation,
not a run of the original4563-case cohort or a timing experiment.

## Sources and next work

KB unavailable (wrapper has no executable; no loaded tools in prior discovery).
Reviewed the exact source-finite-fold derivation, half-GCD residual, original
source, value-only torsion predicate and geometry grid. Read Ward Theorem4 in
Lauter--Stange2008, https://arxiv.org/pdf/0803.0728, and Stange2026 section1
chain rule, https://link.springer.com/article/10.1007/s40993-026-00741-2 .
The half-set sign proof above is the elementary Gauss-lemma argument, written
out to avoid an unproved sign anchor. Standard identities are reused; no
novelty priority claimed. All arithmetic remains in the original prime field.

Next integrate the standalone character into the existing public-point decoder
through a source adapter, preserving its final group equality check and the
original cohort. Prepared-window sharing and complete counted comparison remain
separate tasks. Geometry still materializes w+k-scale points; this round removes
the full-value runtime reference, not that structural product cost.
