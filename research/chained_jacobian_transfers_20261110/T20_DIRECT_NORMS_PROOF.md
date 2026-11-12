# Actual prime-field degree-four covers and T20 norm pair

The two covers below evaluate the existing M=N_C20 R20† directly overF101.
Their balanced-input evaluator uses F101 polynomial/elliptic arithmetic,
without splitting input support. Complete native acceptance is full43/
85436f,under the prior frozen protocolc05a75;all scientific standing is
unsigned. Construction still uses and must charge the existing F101^12
factory. This is a new representation/evaluator of the actual norm pair.
[pcc:unverified]

## Exact curves and functions

Use the original smooth raw curve

    T20: Y²=76z⁶+62z⁵+52z⁴+63z³+69z²+3z+25

and the actual short elliptic curves E2:y²=x³+59x+27 and
F0:y²=x³+44x+21. Define the following polynomials overF101:

| Component | q | n | m | Image of infinity+ |
| --- | --- | --- | --- | --- |
| E2 | z²+38 | 10z⁴+54z²+52z+53 | 2z³+55z²+47z+44 | (10,100) |
| F0 | z²+46 | 71z⁴+96z³+92z²+49 | 5z³+53z²+17z+87 | (71,48) |

The completed covers are pi_i(z,Y)=(n_i/q_i²,Y*m_i/q_i³).
Here infinity+ means Y/z³→50;infinity− uses51. Their images are opposite.
The explicit square normalization Y=50*y gives the monic source
y²=z⁶+46z⁵+6z⁴+50z³+74z²+12z+100. Since50²=76 and50^−1=99,this is a
coordinate isomorphism,not a retwist chosen from cardinalities.
[pcc:unverified]

## Coefficient identities, coprimality and global curve degrees

For each target y²=x³+a*x+b,the exact cleared identity is

    f_raw*m²=n³+a*n*q⁴+b*q⁶.

T20DirectNorms.actual_first_cover_curve and actual_second_cover_curve
prove these literal coefficient identities in every commutative ring
with101=0. They also identify the actual expanded q² and q³ denominators
in the native output. These are symbolic ring proofs,not evaluations at
the101 field elements. The old fixed2^61−1 identity-certificate verifier
is not used to certify a characteristic101 coefficient identity.
[pcc:unverified]

Independent Euclidean reductions give n0=(10z²+78)q0+(52z+18),
q0=(68z+93)(52z+18)+81,and n1=(71z²+96z+58)q1+(28z+7),
q1=(83z+55)(28z+7)+65. The explicit Bezout witnesses are

    (40+64z)n0+(16+58z+4z²+67z³)q0=1,
    (38+50z)n1+(32+17z+77z²+86z³)q1=1.

The two actual_*_cover_coprime_witness Lean theorems prove these identities
under the same characteristic hypothesis. Thus gcd(n_i,q_i)=1 overF101,
and n_i/q_i² has rational-function degree4. Each q has distinct nonzero
roots because q'=2z and its constant term is nonzero. Native existing-poly
gcd checks agree independently with the explicit witnesses.
[pcc:unverified]

The target elliptic curve embeds in the source function field through
the displayed functions and exact curve equation. For nonconstant r,
[k(z):k(r)]=4. The smooth hyperelliptic and elliptic double-cover equations
give [k(T20):k(z)]=[k(E_i):k(x)]=2;the tower law therefore gives
[k(T20):k(E_i)]=4. Characteristic101 does not divide4,so the covers are
separable. Smooth proper completion extends each rational map uniquely.
This is the global degree argument;counting12317 rational images is not
substituted for it. Function-field/Picard bridges remain written.
[pcc:unverified]

At q=0,n is nonzero,so the elliptic image is O. The cleared equation
implies f_raw*m²=n³ is nonzero there;the source is unramified above that
abscissa,and each signed source point has an ordinary completed pole.
No finite point with a nonzero numerator is discarded as undefined.
Infinity follows by leading terms:Y*m/q³→50*lead(m),giving the table.
[pcc:unverified]

## Equality with the actual positive adjoint norm

The actual normalized reverse factors and their cyclic orientation come
from RichelotMaps::new,not a new cubic embedding or selected retwist.
The generic quadratic fibre on C20 has

    U(x)=A1(z)B1(x)+A2(z)B2(x),
    Y*y_C=A1(z)B1(x)(z−x).

Reduction of its cubic ordinate modulo U retains the whole quadratic
algebra. T20_DIRECT_NORM_DERIVATION.md gives H0/H1 and the symmetric
elliptic sums,including reciprocal-cover conversion. The native code
assembles these exact rational functions using existing poly operations.
The positive reverse adjoint is the negative swapped correspondence;
both elliptic ordinates receive that derived negation. Signs are never
calibrated on scalar samples. [pcc:unverified]

Let N_i(P) be this odd raw fibre norm. For infinity+ rational,setD=N_i(infinity+).
Then2D=N_i(infinity+)−N_i(infinity−) is rational because the inherited
Picard map is defined overF101. Using the known odd elliptic group orders,
take its unique rational half D0 by55 modulo109 or57 modulo113.
The curve/order inputs are checked by the accepted construction and complete
rational enumeration;setup checks rationality of2D,its selected half and
c=D−D0 killed2. Its translation formula is X'=e+(3e²+a)/(X−e),
Y'=−(3e²+a)Y/(X−e)² when c=(e,0). The two cleared translation lemmas
prove the ring algebra;the elliptic chord/completion bridge is written.
[pcc:unverified]

Because N_i induces an F101 Picard map,N_i^sigma−N_i is constant.
At infinity+ this equals D^sigma−D=c^sigma−c. Hence N_i−c descends toF101;
it remains odd since2c=0. Every normalized native coefficient projects
exactly toF101 and yields the displayed functions. A coherent constant
translation cancels on degree-zero divisors,so the induced norm is still
the component of M=N_C20 R20†. It is a positive invariance control.
[pcc:unverified]

Generic fibre equality is an exact function-field identity. Both sides
are completed morphisms on the smooth curve,so it extends through all
branch,degree-drop,infinity and pole limits. Galbraith8.3.8/8.3.10 gives
the induced group homomorphism on Picard classes. This uses the existing
proper correspondence/Picard proof720a59;it is not a finite-set density
argument. [pcc:unverified]

## Complete balanced-input evaluation without support roots

Inputs are canonical tagged monic(u,v,n) overF101 on the normalized
source,deg u≤2,deg v<deg u,n≥0,n+deg u≤2,and v²=f_monic mod u.
These checks match the old consumer Balanced::check contract. The new
thin adapter uses existing Zp/poly operations and introduces no balanced
group engine. Reference classes convert through the inverse projective
Chart;every coefficient descends and every input roundtrips exactly
through an explicit rho50 Chart. This handles the independent infinity
sign convention of the old geometric branch chart.
[pcc:unverified]

In F101[z]/u,remove only actual pole support by gcd with the reduced X
denominator,with full multiplicity. On what remains,both denominators
are units. Evaluate X=A+Bz,Y=C+Dz using invmod. If B≠0,the sum is

    slope=D/B,
    Xsum=slope²−(2A−u1*B),
    Ysum=slope*(A−Xsum)−C.

The linear-image Lean identities prove the symmetric X sum and ordinate
cancellation. The new universal IDC specializes D=slope*B and evaluates
this cancellation at its fixed2^61−1 modulus;its mutation must reject.
The characteristic101 cover equations have the separate exact Lean proof.
[pcc:unverified]

If B=D=0,use the actual elliptic double. If B=0,D≠0,the valid quadratic
curve relation forces2C−u1*D=0;the images are opposite and sum toO.
For repeated support this includes the tangent jet:in that case a
coincident image with zero ordinate is two-torsion and doubles toO.
The ordinary elliptic group/jet interpretation remains written/native;
no denominator is inverted in the exceptional branch.
[pcc:unverified]

Finally add(2n+deg(u)−2)pi_i(infinity+),using the ORIGINAL degree before
pole removal. This is the pushforward of the complete degree-zero
balanced divisor including both infinity signs. The actual evaluator's
type contains Zp,Vec<u64>,Curve<u64>,Pt<u64>;it accepts no extension
context,root cache,class enumeration index or logarithm label.
Extension-field point routines are confined to setup/reference controls.
[pcc:unverified]

## Polarization, actual kernel and DLP recovery

Put M†=R20 P20. The existing identities N_C P20=P20 N_C=[2] and
R†R=RR†=[2] imply MM†=M†M=[4]. Degrees multiply to16. The actual
anti-four graph Q20 is ker(R20 P20);BD Lemma2.6 applies because char101
is prime to4 and identifies the optimal(4,4) splitting. The new explicit
degree-four covers supply its two norm components. Native controls also
construct16 actual classes as M†(A[4]),check all256 sums,killed4 and
exactly4 killed2. Degree16 alone is not used to infer kernel type.
[pcc:unverified]

The conditional IntModule four_norm_compositions theorem proves the
formal composition algebra under these explicit map identities and
scalar-two homomorphism hypotheses. rational_four_recovery proves
9238*[4]x=x under12317-torsion;the constants lemma checks inverse4
82/85 on the109/113 components and the raw square normalization.
Actual group orders,Picard maps and polarization remain written/native
inputs to these faithful statements. [pcc:unverified]

For either odd prime-order component,the norm identity makes M injective
and preserves scalar relations. Public [4^−1]M† recovers every encoded
rational class. Native acceptance checks every12317 rational class,
both actual norm compositions,valid/distinct/rational output pairs,
identity/negation/two generator translations,Frobenius and every109+113
scalar image/exact order. The wrong global sign differs on12316 classes
and agrees only at identity. These are unsigned native observations.
[pcc:unverified]

For the accepted degree49 map Phi49,the scoped comparison is

    M R20 Phi49=[2] N_C20 Phi49=[2]diag(alpha49,id)N_C0.

It holds on every12317 actual C0 class in the native comparison. Public
normalization2 on the odd rational component groups yields the existing
direct norm/isogeny bridge. Such a finite-group inverse scalar is not
claimed as a global inverse isogeny or a change to the global degree.
No additional solving advantage or measured cost follows from this
equality. Construction,descent,conversion,transfer,recovery and external
references still require separate matched cost admission.
[pcc:unverified]

## Evidence and faithful formal boundary

- Native static1bbfec,geometry58b615,whole90601c,complete43/85436f.
  The full suite retains old41/reference/legacy guards/fmt/strictClippy,
  all628 hashes and both old JSON outputs byte-identically. Its complete
  18-file raw tree is one57652-byte standardtar.xz with raw/archiveSHA256,
  every-member hashes and exact roundtrip. Original receipts are retained.
- Rational balanced degrees are[3,236,12078],with118 repeated supports,
  5116 nonsplit supports,two actual pole inputs per cover and238 applicable
  omitted-infinity-correction mismatches per cover. Geometry controls
  include120 rational point classes,six branches,eight actual pole points,
  both infinity signs and128 repeated doubles. No arbitrary F101^12
  Jacobian input domain is inferred from those controls.
- Exact Lean bindings for the initial twelve declarations are in
  evidence/t20-direct-lean-02;the two new Bezout bindings are in
  evidence/t20-direct-degree-03. All twelve earlier declaration digests
  remain unchanged. The first binding attempt5176f3 returned not_covered
  because Lean was absent from that exec's PATH;its original source,
  partial claimc20986,verdict and error remain immutable. It is not a
  mathematical failure or an accepted proof run. Correct-PATH bindings
  and the official axiom audits are separate evidence.

Final full40module proofgate52bbc7 observes238 compiled,212 written and
zero trusted native uses,with all44 hashes unchanged. Its entire171-file
output/input/receipt tree is one39344-byte standardtar.xz with complete
member/raw/archive hashes and exact roundtrip. Algebra02/9214e7 passes
24 tests (old23 retained),eleven IDC outputs,all ten old payloads
byte-identical,one new mutation rejection and all24 guards. The new
universal record is IDC1h6de045a64b58c1c8;its final ProofRef hashes the
14-theorem module499bd624cfcdf09d2980e31bda4d6c628829289bd1c9a8b76dc6889e79889c2d.
The identity ID is unchanged from candidate01 because its statement is
unchanged;only the ProofRef payload changes after proof strengthening.
Candidate01/4771dd and complete40/236 proofgate4a3e09 stay immutable.
[pcc:unverified]

Every item here is unsigned. The source-linked report and exact statement
bindings distinguish formal ring/module identities from the global
curve/Picard/polarization/descent/group-law/jet bridges that remain written
and native. No gate or producer assertion supplies scientific signing.
[pcc:unverified]
