# Double-cover Prym carriers reduce to genus260 or261

2026-11-23. Same ECC2K-130 trace-zero factor A and draft PR5079.
Scope: smooth geometrically connected curves X,D over F2 and a
separable degree2 cover X->D, whose entire connected norm kernel P
is F2-isogenous to A. This is narrower than an arbitrary carrier in
which A is only a factor of a larger Prym. All deductions below are
written and unsigned, pending independent verification. [pcc:unverified]

## Findings

Only two possibilities survive the written arithmetic/geometry
constraints: genus260 over a genus130 base, ramified at one rational
point with ramification invariant1; or genus261 over a genus131 base,
with an etale double cover. Both base and covering curves must be
ordinary. The genus261 base cannot be hyperelliptic. No curve in
either class is constructed here. [pcc:unverified]

These restrictions use the exact Frobenius isogeny class, not the
natural representative's endomorphism ring. They therefore apply
even if P is reached through an isogeny losing maximality atell.
They do not exclude other genus130–300 carriers without this
specific double-cover/Prym structure. [pcc:unverified]

## 1. A local zeta congruence in characteristic2

Write Z_Y(T)=L_Y(T)/((1-T)*(1-2*T)) for a curve Y/F2. Let S be the
set of closed branch points on D and put

    B(T)=product_(v in S) (1-T^degree(v)).

For an unramified closed point of degree d, the local factor of Z_X
is (1-T^d)^-2 when split, or (1-T^(2d))^-1 when inert. These agree
modulo2 and equal the corresponding factor of Z_D(T)^2. At a ramified
point, the unique point above it has the same residue degree; its
local factor is (1-T^d)^-1, giving the extra factor1-T^d relative
to Z_D². Hence, as formal series over F2,

    Z_X = Z_D² * B,
    L_X = L_D² * B/(1-T).

The isogeny J(X)~J(D)*P, with P~A, gives L_X=L_D*L_A. These
polynomials all have constant term1, so cancellation in the formal
series ring is valid. The resulting polynomial identity is

    reduced(L_D) * B = (1-T) * reduced(L_A).

This derivation works for wild degree2 ramification and does not
borrow a characteristic-not2 Prym polarization formula.
[pcc:unverified]

## 2. The reduced numerator forces the branch products

The pinned input gives

    L_A(T)=(1-s131*T^131+2^131*T^262)/(1+T+2*T²),
    s131=-22283658519494248867.

Consequently reduced(L_A)=Phi131=1+T+...+T^130. It is irreducible
over F2 because ord131(2)=130. Direct Rabin polynomial tests and a
separate PARI irreducibility check replay that fact. Thus

    reduced(L_D) * B = (1+T)*Phi131 = 1+T^131.

The right side is squarefree. Every closed branch point contributes
a factor1+T, so there can be at most one closed branch point.
If one is present with degree d, its factor1+T^d divides1+T^131;
the binomial gcd identity forces d|131, hence d=1 or131.
No branch point is also possible. [pcc:unverified]

The degree of the reduced L numerator is the curve's2-rank. We obtain:

| Branch product B | Reduced base numerator | Base2-rank |
|---|---|---|
|1|1+T^131|131|
|1+T|Phi131|130|
|1+T^131|1|0|

## 3. Dimension and wild Riemann–Hurwitz

Put h=g(D), g=g(X). Since dim(P)=130, g=h+130. If the cover is
etale, Riemann–Hurwitz gives g=2h-1, hence h=131 and g=261.
The base2-rank131 makes D ordinary; additivity of2-rank under
isogeny makes X ordinary as well. [pcc:unverified]

If there is one closed branch point of degree d, its geometric
conjugates share an odd positive lower ramification invariant delta.
Set c=(delta+1)/2>=1. Wild Riemann–Hurwitz gives

    g-1 = 2*(h-1)+d*c,
    h+d*c=131.

For d=1, the base2-rank is130, so h>=130. The equation forces
h=130,c=1,g=260. There is exactly one rational branch point with
delta=1. The curves are ordinary. [pcc:unverified]

For d=131, the same equation forces h=0,c=1,g=130. This is a
hyperelliptic curve whose Jacobian is isogenous to A, already
excluded in round05 by N2=2<3. It cannot supply a Prym realization.
These three integer cases exhaust all possibilities; the native and
PARI checks enumerate the same results. [pcc:unverified]

## 4. Polarization and transfer remain substantive conditions

Both remaining cases are covered in characteristic2 by
Achter–Casalaina-Martin, Theorem A and Theorem5.12(a,b): the
Jacobian polarization restricts to twice a principal polarization
on P. For the ramified case, total ramification-divisor degree2
means the single weakly wild point above; this is clarified in
Remark5.14 and Corollary5.16. Thus P must be a principally polarized
representative of A's isogeny class. [pcc:unverified]

The round02 necessity argument then applies: any F2-isogeny between
the natural A and this P has degree divisible by
ell=146505763881528721. This is a degree constraint, not a lower
bound on evaluation time. The existence of the double cover would
not by itself give the missing isogeny from the registered source
factor or an evaluated subgroup correspondence. [pcc:unverified]

## 5. Why an etale hyperelliptic base is excluded

Suppose the genus131 base D in the etale case is hyperelliptic.
It is ordinary, so its geometric branch set W has132 points and
all Artin–Schreier poles are simple. Over an algebraic closure write

    D: y²+y=f(x),    f=sum_(w in W) f_w(x),

where f_w has only the pole w. Partition W into two nonempty sets
W1,W2 and put fi=sum_(w in Wi) f_w. The fiber product of the covers
Yi:yi²+yi=fi gives an etale double cover of D: at each pole, on D
one can replace f1 by f2 using y²+y=f1+f2, leaving a regular local
Artin–Schreier equation. Distinct unordered nontrivial partitions
give distinct geometric etale double covers. [pcc:unverified]

There are2^131-1 such partitions, exactly the number of nonzero
classes in H1_et(D,F2), whose dimension is the2-rank131. Therefore
every geometric etale double cover arises this way. Its Klein-four
cover of P1 gives

    P_over_closure ~ J(Y1)*J(Y2),
    g(Yi)=#Wi-1,   g(Y1)+g(Y2)=130.

The decomposition follows by splitting the three nontrivial
characters of the Klein-four action on rational l-adic cohomology,
for any odd l; the third character gives J(D). This does not divide
by2 in characteristic2 coordinate rings. [pcc:unverified]

Frobenius either preserves the two parts or exchanges them. If it
exchanges them, it exchanges the two complementary cohomology
spaces, so its trace on P is0. This contradicts Tr(F|A)=1.
If the parts are preserved, the covers Yi descend to F2 after the
possible constant Artin–Schreier twist: the partial fractions for
each stable subset are rational over F2, and two descents of the
same geometric double cover differ only by that constant twist.
The product decomposition then holds over F2. [pcc:unverified]

A is F2-simple, so both positive-genus factors cannot occur.
One Yi must have genus0 and the other genus130. The latter is a
hyperelliptic curve whose Jacobian is isogenous to A, again
contradicting round05. Thus the genus131 base of an etale
double-cover realization cannot be hyperelliptic. No analogous
bigonal assertion is imported for the ramified genus260 case.
[pcc:unverified]

## 6. Necessary point counts reject two convenient base classes

Write n_m=#D(F(2^m)). Since Tr(F|A)=1 and Tr(F²|A)=3,

    #X(F2)=n1-1,    #X(F4)=n2-3.

In the ramified genus260 case, the single rational branch contributes
one point over F2 and every other rational fiber contributes0 or2.
Thus n1 is even and at least2. Over F4 all rational base fibers
contribute2, except the single branch fiber, giving

    n2 >= 2*n1+2.

In the etale genus261 case, #X(F2) is even, so n1 is odd. The F4
fiber bound gives

    n2 >= 2*n1+3.

These are necessary inequalities, not curve constructions.
[pcc:unverified]

The tempting genus130 base with J(D)~A has n2=2, hence X would have
-1 F4-points; in fact no genus260 curve can have J(X)~A².
The etale base with J(D)~A*E0 has n1=3,n2=5, so X would have2
F4-points, whereas its etale fibers require at least6. It is also
excluded. Other ordinary nonhyperelliptic bases with the required
reduced numerators have not been constructed or excluded here.
[pcc:unverified]

## Status and evidence

The original genus130–300 general existence and computable-map
question remains unresolved. This round reduces the full
double-cover Prym mechanism to two specific cases, and excludes
an etale hyperelliptic base and two convenient formal base classes.
It provides no working curve, subgroup map or attack cost.
[pcc:unverified]

check.cpp/check.txt retain the exact binary irreducibility test,
branch-factor scan, all integer genus cases, Euler-factor identities
and base-count contradictions. Optimized/UBSan output agrees.
replay.gp/replay.txt independently reproduce the reduced numerator,
irreducibility, branch degrees and integer cases. Neither is a
formal or independent signed geometric verification.

Primary reference: Jeffrey D. Achter and Sebastian Casalaina-Martin,
Putting the P back in Prym, Theorem A, Theorem5.12, Remark5.14,
Corollary5.16 and Lemma5.17:
https://www.math.colostate.edu/~achter/math/prym1224.pdf.
The Artin–Schreier branch and2-rank facts are those in round05's
primary references, especially Elkin–Pries ANT7(2013), §1.
General existence context only: Bruce–Li, Theorem A and Proposition2.1,
https://arxiv.org/pdf/1804.11015. Those results do not supply a
genus260/261 realization or an evaluated map for this input.
