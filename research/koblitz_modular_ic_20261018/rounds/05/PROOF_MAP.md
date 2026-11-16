# Round 05 proof map and geometric arguments

All 18 new named theorems in KoblitzRound05 and
KoblitzRound05Quotients are kernel checked and transitively audited. The
matrix decisions use `decide +kernel`, not `native_decide`. Failed build
logs may contain error-recovery sorryAx messages; those attempts are not
proofs. The accepted modules have no such axiom. No full algebraic-geometry
formalization is claimed.

| Claim | Lean coverage | Additional evidence or written bridge |
|---|---|---|
| Off-diagonal Gram entry 20 | hecke_off_diagonal | Equal degrees, deck action and Hecke sum below |
| Polarized composition 32000, both signs | even/odd_polarized_composition; augmentation_square | Actual target polarization and integral adjunction below |
| 11-primary reverse splitting | scalar_bezout; scalar_on_eleven_torsion; reverse_injective_on_eleven; reverse_right_inverse_on_eleven; bezout_reverse_right_inverse | Existence and F2 descent of the correspondence below |
| Five canonical quotient maps | polarized_equivalence_0 through _4 | Integral CM endomorphisms, cyclic shift and natural quotient geometry |
| Ordinary zero support | missing_zero_degree checks degree arithmetic only | Exhaustive factor check, point count, maximal-order argument and finite orbit partitions |
| Supersingular poles | deformation_integral_lift; deformation_binary_identity; supersingular_image_zero | Fine-level local parameter, torsion lifting and automorphism count |
| Symbolic rank 5/588 | No additional Lean rank theorem | Native and separate BigInt elimination; new point images not evaluated |

## 1. Why the Gram matrix is geometric

Work geometrically first. Full determinant-fixed eleven-level structure
modulo simultaneous sign has 660 bases. The map pi:C->X0(49) has degree
660. For a line a in P1(F11), phi_a divides the universal elliptic curve
by that cyclic eleven-subgroup and transports its cyclic49 subgroup.
The intermediate curve X0(539) has two degree-12 degeneracies and the
remaining map from C has degree 55. Thus every phi_a has degree 660.
These are morphisms of smooth proper modular curves, with good reduction
at 2; translations from cusp basepoints do not change their Jacobian maps.

The deck group permutes the twelve maps by its action on lines. It is
two-transitive: after sending one line to infinity, the upper triangular
unipotents act transitively on the other eleven. Native enumeration
independently finds 132 ordered-pair images. Hence every off-diagonal
phi_a*phi_b^dagger is the same endomorphism beta of E49. Interchanging
a,b also gives beta=beta^dagger. E49 is ordinary with imaginary quadratic
endomorphism algebra, whose Rosati-fixed integral endomorphisms are Z;
therefore beta is an integer scalar. The diagonal is [660] by degree.

The twelve maps sum to T11*pi, and T11=[a11]=[4] on the genus-one
Jacobian X0(49). The minimal equation gives eight F11 points, so a11=4;
native enumeration checks this and round 04 checks all supported finite
Hecke sums. Pair the sum with its adjoint:

    12*660+132*beta = 4^2*660, hence beta=20.

The push-pull/adjoint interpretation is the standard
[Hecke correspondence construction](https://public.websites.umich.edu/~asnowden/teaching/2013/679/L16.html).
The specific curve equations come from
[Kilford–McMurdy, section 2](https://arxiv.org/pdf/1110.6801).
The Gram derivation above is ours, rather than a quoted identity.

## 2. Target descent, adjoint and saturation

On the determinant-normalized F2 model, Frobenius sends a paired basis
(P,Q) to (P^2,Q^2/2). Consequently
phi_a(z)^2=phi_(2a)(Frob z). With i taken modulo five, the sums at
a=2^i and -2^i transform by cyclic shift. The alternating differences
transform by minus cyclic shift, because 2^5=-1 mod11. Augmentation S
therefore descends the even tuple to A5(E1), and the odd tuple to its
quadratic twist A5(E0). This is an algebraic moduli identity for every
point, rather than an inference from the finite round-04 orbit checks.

Over the algebraic closure, A is sum-zero in E49^5. Its coordinates are
(p1,p2,p3,p4,-sum p_i). Restricting the product polarization gives
H=I4+ones4; this integral polarization has degree 25. If Psi:J(C)->A
is the induced homomorphism, its polarized adjoint is
r=lambda_C^(-1)*Psi^dual*lambda_A. The source polarization is principal,
so r is integral even though lambda_A is not principal. In coordinates
r has coefficients transpose(F)*H. Direct matrix multiplication gives
Psi*r=[32000] for both signs. Lean checks all sixteen entries in each case.

Since gcd(32000,11)=1, this composition gives a left inverse for r over
Z11. Thus r(T11 A) is a direct summand and is saturated at 11. On A[11]
the scalar is already 1. On A[11^k], use a Bezout inverse of 32000 modulo
11^k; the generic Lean lemma checks this implication for any supplied
Bezout certificate. The formula uses no division by 11. No claim is made
that the integral quotient is optimal, that its entire kernel is connected,
or that a divisor-class evaluator has been implemented.

Expanding transpose(F)*H on a five-coordinate sum-zero point gives
5*U^transpose*(P_i). Pulling back the degree-zero divisors of these
ten points gives the reverse-divisor formula in REPORT.md. Pullback
commutes with Frobenius, so this formula descends. For the odd sign,
use the F4 isomorphism to E0 already given in round 04. Each nonconstant
pullback has degree 660, yielding the 6600 bound for each sign part.

The two polarized-adjoint-norm identity certificates bind the universal
quadratic consequences w^t*G*w=32000*v^t*H*v to this Lean source.
Their issuer checks 64 evaluation points and the extra checker 128.
The IDs are IDC1hf4cb17403b4831b1 (even) and IDC1h258cf4d7d3daf45d (odd).
The matrix theorem, rather than a direct formal quadratic-form rewrite,
is the attached Lean statement. Neither certificate proves the modular
scheme or adjunction facts by polynomial testing.

## 3. Canonical quotient equations versus frozen graphs

Let w^2+w+2=0 and O=Z[w]. On E0 take w=Frob2, and on E1 take
w=-Frob2. These are actual integral endomorphisms over F2. The cyclic
shift sigma on the five Weil-restriction factors commutes with O. Thus
every U=sum_(i=0)^3 (a_i+b_i*w)*sigma^i in the records is an algebraic
endomorphism of A over F2. Determinant +/-1 gives an integral O-matrix
inverse, hence an automorphism. The five generated Lean certificates
verify both inverses and the alternating polarization pullback on the
eight-dimensional integral lattice; the Hermitian identities are also
checked entrywise by a separate native path.

Composing the natural quotient with U gives q_U and its explicit kernel
as stated in the report. This proves equations for that graph. A first
frozen anti-isometry with a different common block C is a different
geometric obligation. Checking only determinants, finite isotropy or
the existence of a related canonical graph does not identify its quotient.
The ten explicit block comparisons retain that distinction. In particular,
changing from a trace+1 companion basis to the CM basis requires a
symplectic torsion identification, not a silent integer sign change.

## 4. Ordinary zeros and Frobenius labels

The native bivariate recursion checks psi11 is monic of degree 60 in X.
At X=1 its b polynomial is irreducible degree 15. Its derivative is
b^12*(b+1)^2, coprime to it, so all zeros are simple on the ordinary
moduli curve. Forgetting cyclic49 and the normalized complement is
etale there. There are 56 cyclic49 subgroups and eleven complements
modulo simultaneous sign, giving 15*56*11=9240 geometric zeros.

For the point-rational twist, Frobenius^15 has polynomial
T^2-231*T+32768. Discriminant -77711 is a 7-adic unit, so Z7[pi]
is the maximal unramified quadratic order. A stable rank-one lattice
over this DVR is free. Consequently its action on E[49] is conjugate
to the companion matrix; this would require extra information if the
order were nonmaximal. The lattice/Frobenius framework is standard in
[Waterhouse](https://www.numdam.org/article/ASENS_1969_4_2_4_521_0.pdf).

The enumerated four projective cycles have length 14. On E[11], the
eigenvalues are 1 and -1, distinct; choose Q in the -1 eigenspace and
rescale its pairing. Frobenius^15 sends Q+rP, after determinant
normalization, to Q-rP. Enumerating the 616 product labels gives 44
orbits of length 14 over the b root, hence closed degree 210 over F2.
The checker verifies every transition and the full partition. It does
not produce coordinates for their cyclic49 subgroups or projected images.

## 5. The complete pole stratum and the symbolic row

Use the smooth deformation E_s:y^2+s*x*y+y=x^3 at the unique
supersingular elliptic curve in characteristic two. Its discriminant
is 1+s^3. The Hasse coefficient a1=s is a uniformizer on a fine-level
moduli chart. One can see its nonzero tangent directly: an admissible
coordinate change sends a1 to a1/u in characteristic two, so it cannot
remove a nonzero first-order a1 deformation. The moduli stack is smooth
of dimension one. Prime-to-two level structures are etale over that
chart, so s remains a parameter on C.

Nonzero eleven-torsion sections specialize to finite points of E_0,
since E[11] is etale. The normalization x=s^2*X+1/s,
y=s^3*Y+1/s^3 gives X=(s*x+1)/s^3, whose numerator is a unit. Thus
X(P)+1 has pole order exactly three at every supersingular point.
The integral polynomial identity, including its explicit correction
multiple of 2, and the binary specialization are proved in Lean and
bound to identity-supersingular-deformation-lift.json, ID
IDC1h88fa2cda16d08555. The valuation
argument and moduli-chart smoothness are written, not formalized.

The supersingular curve has 24 automorphisms; see the primary account
by [Zapponi](https://arxiv.org/pdf/1004.3695). Modulo +/-1 the group has
order 12. It acts freely on full11 bases: if an automorphism fixes E[11]
pointwise, its difference from identity has degree at least 121 unless
zero, whereas a difference of two automorphisms has degree at most 4.
The same argument applies when it fixes a basis up to simultaneous sign.
Therefore there are 56*660/12=3080 geometric supersingular points on C.
Their multiplicity-three poles exhaust degree 9240. Monicity in X
implies integrality over F2[b], so no other poles exist at finite b.
This proves the full divisor, not merely equality of its total degrees.

All phi_a targets of this stratum are supersingular modular base points.
The explicit E49 model has six such points, all F4-rational and killed
by 8, checked by the maintained binary library. The closed pole-divisor
trace under the five-coordinate point map is therefore killed by 8.
It lies in A5(F2), killed by 11, so it is zero. The generic coprime
implication is a new Lean theorem. Principal divisors map to zero under
the induced Jacobian homomorphism. Thus the 44 zero columns, each of
multiplicity one, satisfy one projected relation, with pole columns
omitted because their projected images vanish.

Their support is disjoint from all old columns, so a row with 44 ones
there is independent of the old rank-four matrix extended by zeros.
Two native arithmetic paths check rank five. This is a symbolic relation
with a written geometric certificate. New point coordinates and individual
images have not been replayed; the evaluated matrix is still rank 4/544.
