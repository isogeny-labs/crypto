# Bounded two-section decomposition for the existing genus-three quotient

## Status and scope

Derived geometric construction, not an executed mapper, timing result, novelty
claim, or full10x result. It replaces a proposed search through multiples by
a bounded zero-dimensional intersection, conditional on implementing global
section equations and the intrinsic difference inverse. The known same-field
quartic/Jacobian identification and polarized etale3-isogeny are premises,
not inferred from this argument. Genus2/4 obligations remain separate.
[pcc:unverified]

Let A=Jac(H), B=Jac(C) over k=Fq, with H hyperelliptic genus3 and C a smooth
nonhyperelliptic plane quartic. Assume char(k) is neither2nor3. Let
psi:A->B be a polarized(3,3,3)-isogeny, so deg(psi)=27 and
psi*ThetaB numerically equals3ThetaA. Let T in A(k) be the same fixed
two-torsion shift used by the existing decoder. Put Sigma=C-C in B and
S={X:psi(X-T) in Sigma}. For nonzero D in A(k) of prime order ell>2592
with ell not dividing6, the fixed F127 case has these hypotheses.
An independent validation of those premises is outside this note.
[pcc:unverified]

## 1. The two sections and the degree

The archived difference proof identifies the reduced Gamma00 divisor with
Sigma, of numerical class2ThetaB. Therefore S is an ample reduced divisor
of class6ThetaA. The equations needed are

    s(X)=0, s(X+D)=0,

where s is the pulled-back, shifted Gamma00 section. Each is class6ThetaA;
the zero loci are S and S-D. The prior map's scalar_g implements only a local
frame of this section, and its rational denominators are not global equations.
[pcc:unverified]

Here S is geometrically irreducible, not merely assumed so. On a smooth
nonhyperelliptic curve, a nonzero difference P-Q has a unique ordered pair:
an alternative would give two distinct linearly equivalent effective degree2
divisors, hence a degree2 pencil, impossible here. The differential of the
difference map has the distinct canonical tangent directions at P and Q and
rank2. Properness, quasi-finiteness away from zero, and normality then identify
Sigma away from zero with (C x C) minus its diagonal. Equivalently one can use
the proper unramified injective map there to obtain this identification.
Thus Sigma is smooth off zero. As a reduced Cartier divisor in smooth B it
is Cohen-Macaulay; the isolated singularity gives R1 and hence normality.
The etale pullback S is smooth away from the27points above zero. An ample
effective divisor on a smooth projective threefold is connected. A reduced
Cartier divisor smooth outside a finite set is normal by the same S2/R1
argument. Connected normal S is consequently irreducible.
[pcc:unverified]

If translation by D stabilized S, it would preserve O(S), putting D in
ker(phi_O(S)). Numerical class6ThetaA gives phi_O(S)=6phi_ThetaA and its
kernel A[6]. The nonzero prime-order D is not in that kernel. Thus S and S-D
are distinct irreducible divisors. Their intersection Y is a nonempty pure
one-dimensional complete intersection. Nonemptiness follows also from its
positive intersection with any ample divisor.
[pcc:unverified]

For the helper use the EXISTING eight quotient sections, not a new source
2Theta basis. They span psi*H0(B,2ThetaB), after the fixed shift. This
subsystem of |6ThetaA| is basepoint-free: 2ThetaB is basepoint-free and
pullback preserves that property. A helper H in this system containing no
component of Y cuts a zero-dimensional scheme Z with

    length(Z)=(6ThetaA)^3=6^3 * ThetaA^3=216*6=1296.

The principal polarization has ThetaA^3=3!=6. This count includes
multiplicities; it neither assumes1296distinct points nor1296rational points.
Using a hypothetical source|2ThetaA| helper would instead give432, and a
moving theta translate216. Those smaller numbers are NOT the degree for
the helper whose eight functions already exist in this implementation.
[pcc:unverified]

## 2. A finite-field helper bound

The number of geometric irreducible components of Y is at most1296: each
has positive integral degree against the ample class6ThetaA and their
weighted degrees sum to1296. Also avoid the at most54geometric points

    E=(T+ker psi) union (T-D+ker psi).

Their avoidance ensures both target differences are nonzero, where the
intrinsic ordered-pair inverse is defined. A helper must not contain any
component and must not pass through any point of E. Each requirement forbids
a proper linear subspace of the8-dimensional helper coefficient space,
even for a component or point not defined over the coefficient field.
Basepoint-freeness makes the corresponding evaluation subspace proper.
[pcc:unverified]

Choose helper coefficients uniformly in F_(q^s)^8, including the zero vector
as a rejected possibility, where Q=q^s>1350. Each proper subspace contains at
most Q^7vectors. Union bound gives failure probability at most

    (1296+54)/Q=1350/Q.

No independence among these events is needed. Accepted helper attempts have
expected count at most1/(1-1350/Q). This bounds HELPER SELECTION ONLY, not
polynomial-system solving or the current decoder's success rate. For q127,
s2 suffices and the lower success bound is14779/16129. A solver must certify
proper finite intersection and avoidance, rather than infer them from a few
evaluations. The proof supplies existence, not an existing certificate routine.
[pcc:unverified]

Over F_(q^s), a closed point X of Z has residue degree d<=1296, since its
degree contributes to scheme length. Over k its degree e is at most s*d
(it can be smaller). For q127,s2, e<=2592<ell1028893. This is an extension
DEGREE bound, not enumeration of q^e elements. Factoring the finite algebra
must be charged. Nilpotents do not invalidate existence of a residue field.
[pcc:unverified]

## 3. Recover the image and descend it

At any such X, decode the intrinsic oriented pairs for

    U=psi(X-T), V=psi(X+D-T).

Both are nonzero by avoidance of E. A globally consistent choice of isogeny
sign gives V-U=epsilon*psi(D). Thus the two-section decomposition needs no
discrete logarithm of X, no scan index n, and no inverse2n. The source's
fixed differential calibration can resolve the global sign; independently
choosing signs on U and V would invalidate subtraction. If a target
difference class is rational over the residue field, uniqueness of the
ordered pair also makes its endpoints rational over that field.
[pcc:unverified]

For a base-field implementation, trace both decoded pairs over k:

    Tr(V)-Tr(U)=e*epsilon*psi(D).

Each trace is a sum of Frobenius conjugates, representable by degree-e closed
divisors over k, rather than by scanning rational points. Since e<ell,
multiplication by e^-1 modulo ell gives epsilon*psi(D). This avoids assuming
that a randomly selected closed point or its endpoints are k-rational.
Alternatively subtract over the residue field and certify Frobenius-fixed
descent. Either approach still requires actual support/ideal arithmetic.
[pcc:unverified]

## 4. What this does and does not establish

This is a bounded-degree geometric alternative to the current cyclic hitting
search. Given explicit bounded-degree global equations, a complete coordinate
atlas, intrinsic difference inversion and finite-algebra factorization, the
map evaluation can use bounded-degree algebra over finite fields, with no
order-q candidate search. That conditional statement does not measure its
constants, supply those implementations, or imply10x. The bound2592 can be
very expensive and may be much worse than the current F127 mapper.
[pcc:unverified]

The existing transverse-chord gradient/Hessian decoder is ONLY ONE CHART of
the intrinsic inverse. Its bad locus includes curves, not just zero. The
54point avoidance bound must NOT be claimed to avoid every one of its
exceptions. Y could have a component in that bad locus. A full decoder atlas
or explicit proof of avoidance is required; selecting only rational split
transverse pairs would reinstate an unbounded search and invalidate the claim.
Likewise the source Mumford affine chart may miss components. Clearing
denominators without saturation creates false solutions; saturating one chart
does not certify that another chart has no solutions. No experiment is
admitted until these implementation obligations are resolved.
[pcc:unverified]

The full-cost acceptance remains the same public-input requirement:

    Torder+Tkernel+Tsections+Tintersection+Tdecode+Ttrace
      +Trelations+TLP+TLA+Tdescent+Tverify <= Tbest_native/10.

No constants or useful-rank guarantee in this note prove that inequality.
The prior cold failure remains fixed. This construction is genus3-specific;
it does not discharge genus2or4 obligations.
[pcc:unverified]

## Sources, prior art and derivation boundary

Repository: hec_difference13/PROOF.md supplies the reduced difference-divisor
identification and explicitly proposes decomposition into difference points;
hec_kummer13/RESULTS.md supplies the eight descended quotient sections;
hec_coldbuild13/map_dynamic.rs supplies the current cyclic mapper. This note
adds the helper intersection count, finite-extension degree and trace identity
to that earlier decomposition proposal. It is not a novelty claim.
[pcc:unverified]

Primary: Tian, section4.2.3, already lifts general quotient Kummer points via
a polynomial system, so algebraic general-class lifting is established prior
art, not a new possibility asserted here:
https://arxiv.org/html/2007.03172#S4.SS2.SSS3 . Pauly-Previato introduction
equation1.2 supplies the complex Gamma00/difference connection; the repository
contains the separate characteristic127 specialization argument:
https://math.univ-cotedazur.fr/u/pauly/singularities.pdf .
[pcc:unverified]

Milne, Abelian Varieties, I sections5-6 and II Lemma10.3/proof: theorem of the
square, ample intersection/polarization facts, and connectedness of ample
divisors; basepoint-freeness of2Theta also follows by choosing translates
Theta+a,Theta-a avoiding a given point and applying the theorem of the square:
https://www.jmilne.org/math/CourseNotes/AV.pdf . These standard facts are used
as such. The numerical1296/1350/2592 bounds and trace construction are derived
above; no source is claimed to state this exact algorithm or those constants.
[pcc:unverified]

KB lexical query 'Gamma00 difference surface two translates intersection trace
isogeny genus three bounded degree', revision9141e386254340a9bdba3036f2bb706a,
returned no directly relevant hit. The index predates these local studies;
absence is not proof of novelty. Direct primary and current Git sources were
read. Stanford Conrad notes were inaccessible via web; Milne supplied the
replacement accessible reference. No scientific execution or timing performed.
[pcc:unverified]
