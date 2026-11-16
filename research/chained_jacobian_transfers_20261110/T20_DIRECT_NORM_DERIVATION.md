# T20 four-cover norm: prospective exact derivation

2026-11-12,before native implementation. These are written algebra and
Picard deductions to be checked under T20_DIRECT_NORM_PROTOCOL.md,not new
native acceptance or a complete formalized Jacobian theorem. Unsigned.
[pcc:unverified]

## Reverse fibre and two elliptic sums

Use the actual reverse arrays A_i(z),B_i(x) from RichelotMaps,retaining
their cyclic rotation and raw normalization. For T20 point(z,Y),

    U(x)=A_1(z)B_1(x)+A_2(z)B_2(x)=u2*x²+u1*x+u0,
    Y*y_C=A_1(z)B_1(x)(z−x).

Put a=u1/u2,b=u0/u2. Write the cubic on the right asΣC_i*x^i.
Its exact remainder modulo x²+a*x+b is H0+H1*x,where

    H0=C0−b*C2+a*b*C3,
    H1=C1−a*C2+(a²−b)*C3.

Each existing EvenCover has X=alpha*x²+beta,Y_E=mu*y_C on its direct
branch,where alpha=scale²*lead,beta=scale²*shift,mu=scale³*lead.
For generic quadratic fibre use

    m0=−mu*H1/(alpha*a),
    r=m0²/f_T−alpha*(a²−2b)−2beta,
    s_unnegated=(m0*(beta−alpha*b−r)−mu*H0)/f_T.

Then its two-point sum is(r,Y*s_unnegated). The ordinary addition slope
is m0/Y and the symmetric square/ordinate reductions are those of
DIRECT_NORMS_PROOF.md. For the reciprocal branch put a'=a/b,b'=1/b and

    H0'=H0*a'*b'−H1*b',
    H1'=H0*(a'²−b')−H1*a'.

The same formulas in t=1/x apply to Y_E=mu*y_C/x³. The positive reverse
adjoint is the NEGATIVE swapped correspondence,so set s=−s_unnegated
for both components. Normalize polynomial ratios by existing gcd/divrem;
no finite-field interpolation or DLP-dependent sign calibration is used.
Generic rational functions then extend to smooth proper curve morphisms;
actual exceptional limits remain explicit native/written obligations.
[pcc:unverified]

## Descent by a coherent two-torsion translation

Let N(P) be this raw odd elliptic-valued function. It induces the actual
component of M on degree-zero divisors by the completed correspondence
and Galbraith8.3.8/8.3.10. Oddness follows from Y→−Y and the inherited
positive-adjoint sign. Infinity± on raw T20 have Y/z³→±50;both are F101.
Set D=N(infinity+). The difference2D=N(infinity+)−N(infinity−) is rational
because it is M of an F101 divisor class. Let D0 be its unique rational
half,using55 on E2(order109) and57 on F0(order113). Then c=D−D0 is killed2.
[pcc:unverified]

For every Galois element sigma,N^sigma−N is constant because the induced
degree-zero norm is defined overF101. At infinity+ that constant equals
D^sigma−D=c^sigma−c. Hence N'=N−c is F101-defined. Since2c=0,it is also
odd;its coordinates have the form(r(z),Y*s(z)) with F101 coefficients.
The implementation must check this coefficient descent exactly.
[pcc:unverified]

If c=O no translation is needed. If c=(e,0) on y²=x³+A*x+B,let k=3e²+A.
Translation by−c=c gives

    r'=e+k/(r−e),   s'=−k*s/(r−e)².

This follows from the ordinary chord formula and e³+A*e+B=0. The
translated cover induces the SAME Jacobian norm:the constant cancels
on every degree-zero divisor. A coherent translation is therefore an
invariance control,not a false map mutant. Its infinity images are±D0.
[pcc:unverified]

## Prime-field finite algebra and its complete input adapter

For normalized balanced support u=z²+a*z+b reduce the completed cover
coordinates modulo u to X=A+B*z,Y=C+D*z. If B≠0,put m=D/B;then

    Xsum=m²−(2A−a*B),   Ysum=m*(A−Xsum)−C.

If B=0,D=0,the images coincide and the finite pushforward is their
actual elliptic double. If B=0,D≠0,the curve relation modulo u forces
2C−a*D=0;the images are opposite and the sum is O. This includes repeated
jets:in the latter case the repeated image is a two-torsion point and
its double is O. A repeated source branch point with zero ordinate cannot
be a valid smooth-curve Mumford jet. Existing elliptic operations perform
doubling/addition;no new group engine is introduced. [pcc:unverified]

A pole of reduced r is exactly a preimage of elliptic O. Remove only that
support,with full multiplicity,by gcd with its denominator,and require
the remaining r/s denominators to be units. Validity of the cleared curve
equation controls the completed image. Keep the ORIGINAL degree m before
removal. The balanced norm is the finite sum plus(2n+m−2)*D0. This is
pushforward of D_fin+n*infinity+ +(2−m−n)*infinity−−K,whereK is the
canonical infinity pair. Both infinity signs and all multiplicities remain.
[pcc:unverified]

The old odd Chart uses w=L/(z−a0),Z=Y*w³/L,with a0 a geometric branch.
For odd support w²+A*w+B with B≠0,the inverse finite support is

    [(L²+A*L*(z−a0)+B*(z−a0)²)/B],

and the raw ordinate is L*v(w)/w³,then divided by50. Compute it in that
quadratic algebra with existing invmod. When B=0,retain each w=0 as a
signed infinity via rho=v(0)/L²;the remaining support is finite. Repeated
w=0 is twice the same signed infinity. Identity chooses balanced n=1.
No nonidentity rational T20 class can have odd-chart degree1:its unique
effective degree2 representative would contain a0 of exact degree6.
Every converted coefficient must project toF101 and every input must
roundtrip through the explicit rho50 Chart. This is a representation
adapter on an already complete class corpus,not a new enumeration or
balanced group engine. [pcc:unverified]

## Polarization, kernel, recovery and the accepted chain

M=N_C R† and M†=R P20. Existing N_C P20=P20 N_C=[2] and R†R=RR†=[2]
give MM†=M†M=[4]. Degrees multiply to16. The actual Q20 anti-four graph
is the kernel of R P20;the optimal(4,4) interpretation uses BD Lemma2.6
with char101 prime to4. Degree16 alone does not establish kernel type.
The proposed actual A[4] image test gives16 kernel classes,all256 sums,
four killed2 and all killed4. These remain native obligations.
[pcc:unverified]

On J(F101) of order12317,9238*[4] is identity. Component inverse4 constants
are82mod109 and85mod113. Thus M preserves encoded DLP scalars on both
odd prime-order components,with the actual adjoint supplying public
recovery. These facts still need new faithful Lean/native acceptance.
[pcc:unverified]

For the already accepted C0→C20 degree49 map Phi49,

    M R Phi49 = [2] N_C20 Phi49
               = [2]diag(alpha49,id)N_C0.

After public normalization2,the composite is the existing direct norm/
isogeny bridge. This scoped comparison is preserved,not advertised as
an additional solving advantage. A new F101 evaluator could improve the
representation cost of T20 inputs;all complete costs and matched external
references remain unmeasured under this correctness protocol.
[pcc:unverified]
