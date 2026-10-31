# Round 04 proof map and geometric arguments

The pinned Lean 4.34.1/mathlib project now includes
[KoblitzRound04.lean](../../lean/KoblitzRound04.lean). Eleven new named
theorems build and pass the transitive axiom audit. They use only `propext`,
`Classical.choice` and `Quot.sound`, or a subset. Failed proof elaborations
are archived; they are not claimed as proofs. No `sorry`, custom research
axiom or trusted native evaluation is used in the final module.

| Report claim | Lean theorem | Exact scope and missing bridge |
|---|---|---|
| Integral X0(49) coordinate change | `modular49_integral_change` | Universal commutative-ring identity; identifying the modular eta functions uses the published model |
| Characteristic-two specialization | `modular49_binary_change` | Exact identity over every commutative ring of characteristic 2; rational-map chart/gluing remains geometric |
| Natural quotient polarization pullback | `natural_polarization_pullback` | Universal bilinear polynomial identity with the fifth coordinate equal to minus the other four; Picard/polarization identification external |
| Natural quotient kernel | `natural_quotient_kernel` | All elements of every additive commutative group, trace-zero five-tuple; kernel is diagonal five-torsion |
| Rational inverse | `natural_quotient_rational_inverse` | Additive trace/diagonal maps with trace*diagonal=id; on F2 points this follows from five acting as one on E(F2) |
| Integral augmentation has zero sum | `augmentation_sum_zero` | Every five-tuple in every additive commutative group |
| Six-term Frobenius trace is fixed | `trace6_fixed` | Additive endomorphism and actual period-six input; identifying the twisted modular Frobenius is a separate bridge |
| Inverse-Gram coordinate bound | `dual_coordinate_bound` | A real quadratic expression nonnegative for every real t, positive dual norm; positive Gram/inverse specialization supplied by exact certificates and the written argument |
| Quadratic-twist substitution | `binary_twist_substitution` | Characteristic 2, s squared+s=1, arbitrary x,y,a,b; supplies the actual curve-equation substitution |
| Cuspidal projected images vanish | `coprime_cusp_image_zero` | Every group element killed by both 14 and 11; geometric cusp-image orders are the external premise |
| Four-fiber rank ceiling | `fiber_relation_rank_bound` | Every field and every finite matrix factoring through four columns; no empirical rank hypothesis is assumed |

## Quotient basis, positivity and the explicit natural case

For the first graph of each nonempty gluing case let G=K*M. The subgroup
has coordinates (G*z,z)/ell in the product Tate module. Its full inverse
image is the lattice with basis B/ell, B=[[ell*I,G],[0,I]]. This basis
contains both the old lattice and the graph, and its index is ell squared.
The old lattice inclusion is Q=[[I,-G],[0,ell*I]]. Thus Q^t*J'*Q=J_total,
where J'=B^t*J_total*B/ell squared. Kernel isotropy supplies its integrality.
Its determinant is one, and Frobenius is B inverse*F_total*B, checked
integral using F_A*G-G*F_E divisible by ell. These identities, including
F'^t*J'*F'=2J', are exact native certificates, not full Lean matrix proofs.
For supersingular cofactors this presentation is prime-to-two Tate data;
it is not asserted to be an ordinary complex-lattice realization.

For positive real S and a vector z, apply positivity to
z-t*S inverse*e_i. It gives
0 <= z^t*S*z-2*t*z_i+t squared*(S inverse)_ii.
The generic Lean quadratic lemma implies z_i squared <=
(z^t*S*z)*(S inverse)_ii. Here S is twice the real Hermitian Gram, so norm
at most 3 gives the factor 6. BigInt determinant/cofactor and positive-minor
checks certify the bounds. Exact rational LDL search and a separate complete
box enumeration agree: each of the two forms has ten norm-three vectors
and none of smaller positive norm. These computations prove the stated
minimum with the written positivity-to-lattice bridge; no indecomposability
of the final fivefold is inferred.

For the natural graph let A be the kernel of sum:E^5->E after geometric
base change, with its F2 permutation descent. The map
q:A x E->E^5, (a,Q)->a-diag(Q), is surjective as an isogeny, and its kernel
is (diag(T),T), 5T=0. The five-torsion is etale of order 25 in characteristic2.
The abstract additive kernel statement is proved in Lean; the isogeny and
etale assertions use standard abelian-variety geometry. The product
principal polarization pulls back to H=I4+ones4 on A and [5] on E; the
mixed terms cancel since sum(a_i)=0. Its degree is 25 squared, so descent
through the kernel gives the product principal polarization on the quotient.
The target's F2 form is R=Res_(F32/F2)(E), with the descended product
polarization. This describes the complete quotient geometrically, plus an
explicit affine coordinate patch and identity chart. It is not a fully
expanded projective ideal for every mixed chart.

## Integral modular model and actual isogeny transport

[Kilford--McMurdy, section2](https://arxiv.org/abs/1110.6801) gives the eta
model and degeneracy parameters. Starting from
Y^2+XY=X^3-X^2-2X-1 over the integers, put x=X-2,
D=x^2+7x+7 and y=D*Y+4x^3+22x^2+35x+7. The universal identity in Lean
has the eta residual equal to D squared times the Weierstrass residual.
Both sides have integral coefficients. Specializing at 2 gives the explicit
binary eta map without dividing by 2. D=0 is an exceptional inverse chart,
not a discarded point; the base enumeration retains these points and the
Fricke involution is evaluated by elliptic addition there.

For ordinary underlying j, normalize the elliptic model to
v^2+u*v=u^3+a*u^2+b, b=1/j. Its x-coordinate doubling formula is
u squared+b/u squared. A cyclic odd-order kernel with pair-root polynomial
k has characteristic-two Velu x-map
u+u*k'/k+u squared*(k'/k) squared.
Writing T for the sum of the pair roots, its quotient has a4=T,
a6=b+T; translating v by T normalizes a4 to zero and changes b to
b+T+T squared. The integer-lift certificate records the residual multiple
of 2. This Velu construction and Miller's divisor algorithm are standard
geometric bridges; the present Lean module does not formalize them.

The code reconstructs both degree-seven kernels from the three j-values of
an X0(49) point. Division-polynomial divisibility, separability and doubling
invariance check each kernel. Applying the eleven x-map to these kernels
is implemented as the characteristic polynomial in their quotient algebras.
The transported eleven kernel on the first seven quotient gives an
independent commuting-square equality. The native published-polynomial test,
curve equation and complete Hecke sum are additional finite controls.
The separate maintained library checks basis orders/independence, every
transported target, and T11=[4] for all 24 supported inputs.

## Modular source, canonical Frobenius and the twisted factor

Let C be the determinant-normalized full-eleven-level cover of X0(49).
A point over a field containing mu11 is represented by an ordinary base
point with its cyclic49 subgroup and an eleven-torsion basis (P,Q) of fixed
Weil pairing zeta, modulo simultaneous sign. There are |SL2(F11)|/2=660 such bases at each ordinary nonelliptic base point.

Conjugation by diag(11,1) takes
[[a,b],[c,d]] with c divisible by 49*121 and d=+/-1 modulo 11 to
[[a,11*b],[c/11,d]]. The latter group is Gamma0(49) intersect
+/-Gamma(11). On moduli the conjugation quotients by the first cyclic11
subgroup and transports the coprime49 structure. The quotient subgroup and
the dual generator give the full basis, with the determinant recording its
Weil pairing. This supplies the correspondence with the X_H(5929) model;
the normalization of the determinant specifies its F2 descent, not just its
complex analytic group. This scheme/moduli argument is written, not Lean.

Under F2 Frobenius, canonical normalization is
(P,Q)->(P^2,Q^2/2), where division by 2 means multiplication by 6 modulo 11.
The pairing transforms as zeta->zeta squared and is restored by that factor.
Changes between the chosen universal elliptic twists are explicit y-shifts.
The actual permutation of all 15,840 supported geometric points has20 orbits
of length6 and 524 of length30. No geometric point is omitted from these
selected complete fibers.

Let phi_a be the actual transported X0(49) point associated to the line
P+aQ, including the two fixed lines a=0,infinity. For i modulo 5 set
D_i^+=phi_(2^i)+phi_(-2^i), and
D_i^-=(-1)^i*(phi_(2^i)-phi_(-2^i)). Replace D_i by
5D_i-sum(D_j) to land in augmentation without division. Canonical modular
Frobenius transforms the even tuples by sigma followed by inverse cyclic
shift, and the odd tuples by its negative. All these equalities are checked
at every geometric point, with both trace-zero and order conditions.
Their closed-point traces evaluate actual A5(F2) points.

The standard Gauss-sum formula for the weighted degeneracies identifies
these with the primitive order-five/order-ten twists of the conductor49
form, up to Galois conjugacy and the explicitly stated integral scaling.
[Atkin--Li](https://doi.org/10.1007/BF01390245) supplies the newform twisting
framework. The characteristic-zero correspondence and its prime-to2 level
reduction supply the global Picard map. Identification of that map with the
optimal modular quotient, its polarization and saturation at 11 are not
computed here. The nonzero finite images do not establish a reverse
subgroup-to-Jacobian embedding on eleven-torsion.

For the odd sign, choose s in F4 with s squared+s=1 and apply
(x,y)->(x,y+s*x). The Lean substitution proves the twist equation.
The twisted fixed-point condition then makes both coordinates F32-rational.
Their norm to E(F2) vanishes. Since E0(F32) has order 44, E1(F32) has order 22,
and the respective base orders are4 and 2, the norm kernels have order 11.
The maintained library checks every one of the 1,088 recorded projected
points and its eleven-torsion condition. Labels for a planted scalar are
never used in relation construction.

## Genuine principal rows, cusp removal and the rank ceiling

Take a line or vertical function on E1=X0(49) overF64. Its norm toF2 is a
rational function; its full pullback to C has a principal divisor. At each
admitted ordinary base zero the entire degree 660 fiber is represented.
Base points outside the supported set are rejected, even if a partial
projected sum might vanish. The polynomial zero multiplicities determine
the row, with coefficients constant on the four supported closed base
fibers. The separate verifier reconstructs these rows from their functions
and root multiplicities, then checks their point sums using a different
binary arithmetic implementation and rank using BigInt elimination.

Every transported cusp is an X0(49) cusp. These are defined overF8 and
killed by 14 in E1(F8). Consequently a projected closed cusp class in A5(F2)
is killed by 14 and 11 and vanishes; this coprime implication is proved in
Lean. A compatible Tate cusp supplies a degree-zero anchor, with zero image.
The construction of the anchor and all cusp charts is geometric, not
formalized or implemented as arbitrary C divisor-class arithmetic.

Every row factors as W times the four-fiber incidence matrix. The Lean rank
lemma proves its rank is at most4 over every field. The computed rank 4
therefore exhausts this row pattern; collecting more of those pullback
functions cannot supply the missing539 ranks. A level-dependent function
is the next discriminating family, not a supplied cross-row control.

For x(P)+1 the native recurrence gives psi11(1,b) of degree 15, constant
coefficient nonzero and no F64 roots. Each ordinary b-root supplies a
nonzero point P up to sign,11 complementary paired bases and 56 cyclic49
subgroups. This gives full zero degree 9,240, with multiplicities inherited
from the polynomial. None is in the current factor base. Poles also need
resolution. The proposed row is rejected rather than substituted for a
complete principal divisor. This moduli-degree argument is written; the
polynomial and empty-support checks are native exact evidence.

## Identity records and remaining formalization

The loose integer identity certificates cover the integral modular change,
natural polarization pullback and Velu normalization lift. Their identifiers
are listed in ledger.json and cited below after issuance. Each receives 64
issuer points and 128 extra checker points. The first two have direct Lean
source bindings. The third retains its multiple-of-two residual and has no
misleading Lean attachment.

Full moduli/Picard geometry, polarization descent, Weil/Miller correctness,
actual divisor fiber completeness as a scheme theorem, and the reverse
projector/composition remain formalization obligations. The finite tests
and structural Lean statements are not presented as those full theorems.

| Identity certificate | ID | Kernel proof binding |
|---|---|---|
| [Integral modular change](identity-modular49-change.json) | `IDC1h5c93d35226502b13` | Direct universal statement, `modular49_integral_change` |
| [Polarization pullback](identity-polarization-pullback.json) | `IDC1h7a01e3d29f1f37ab` | Direct universal statement, `natural_polarization_pullback` |
| [Velu integer lift](identity-velu-integer-lift.json) | `IDC1h5d4af0a173c60eeb` | No direct Lean attachment; residual 2*(y*T+T squared) is explicit |

Q2 specifies its diagonal graph geometrically. An identification with a
particular stored Q1 first-anti-isometry basis is not included. The global
minimum computations cover the two distinct determinant-19 forms admitting
the tested elliptic cofactor gluing (four sign-specific cases), not all
seven determinant-19 forms retained before that compatibility test.
