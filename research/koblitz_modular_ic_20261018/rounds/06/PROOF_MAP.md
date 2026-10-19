# Round06 proof map and written bridges

The final Lean build and transitive audit cover14 named theorems plus
step.eq_1. Only propext, Classical.choice and Quot.sound occur. Explicit
matrix decisions use decide +kernel. Numerical identity checks supplement
the kernel proofs; they do not formalize geometry.

| Claim | Kernel proof | Written or executable bridge |
|---|---|---|
| Effective descent | descent_order_five | Galois descent of projective E^5 and its maps |
| Quotient polarization | polarization_lift, polarization_pullback; polarized_descent_0,_4,_8 | CM endomorphisms, positivity and principal polarization |
| Split graph kernel | split_kernel | Complete250-point check and explicit basis binding |
| Degree7 cancellation | seven_even_cancel, seven_odd_cancel; degree_seven_equation | Modular forgetful map, local orders and all96 fiber checks |
| Ordinary reverse representative | reverse_ordinary_translation | Elliptic Picard identification and pullback |
| Coordinate-algebra inversion | quadratic_coordinate_norm | Separable degree11 factors and executable Miller checks |
| Reverse degree and Frobenius wrap | reverse_fiber_degree; fifth_line_wrap | Full20-fiber descent and point-algebra verification |

## 1. Quotient and polarization

Use O=Z[w], w²+w+2=0, with its conjugation as Rosati. On E0 let
w=pi and on E1 let w=−pi; pi is the actual coordinate Frobenius.
All k used here are multiples of4, so pi^k=w^k for both signs.
The round05 U matrices are integral automorphisms over F2 carrying each
determinant5 polarization to H=I4+J4. Work first in these coordinates.

Define q_k on A×E by Y_i=a_i−a5 and Z=pi^k a5−epsilon Q.
Its kernel is a1=…=a5=u,5u=0,Q=epsilon pi^k u, so it has25
geometric points and is separable. It is therefore an isogeny. The CM
Hermitian matrix L in Q4 is integral. Its positive leading minors and
determinant1 give a positive principal polarization on the product E^5.
The native Hermitian calculation and the three integral10×10 Lean
certificates verify q†Lq=diag(H,5), T†LT=L and T^5=I. The universal
bilinear identity records the residual5(1+5alpha−p q)sa sb; it vanishes
because Norm(pi^k)=2^k=1+5alpha. Changing epsilon is absorbed by the
elliptic automorphism Q↦epsilon Q, which preserves its polarization.

The source cyclic factor descent induces exactly T_k on q coordinates.
Its coefficients are F2 endomorphisms, hence it commutes with coefficient
Frobenius. The order5 cocycle gives effective descent from F32 to F2
for this projective abelian variety, its polarization and quotient map.
This supplies equations as a split projective model plus descent datum.
Expanding a single F2 projective ideal is a distinct unfinished presentation.

The frozen finite graphs previously had no bound geometric companion basis.
For trace+1 we explicitly use Tplus=[[3,3],[4,1]] mod5, determinant1;
it intertwines companion Frobenius with −w on E1[5]. For trace−1 use
the declared CM basis directly. Q4 retains actual CM basis points and all
25 points of each graph, and checks the common block after the frozen U.
Every graph matches epsilon pi^(−k), k=0,4,8. Thus every specified graph
with this explicit binding is the kernel of the given q_k∘(U,id).
This does not retroactively make the earlier unbound basis canonical.
No determinant19 graph or Jacobian realization follows from this argument.

## 2. The degree7 fiber sum

Write the eta model as

    y²+x y(x²+x+1)+x(x6+x5+x4+x3+x2+x+1)=0.

Substitute t=x4/y and clear denominators. The exact Lean ring identity
gives the monic degree7 equation for x printed in the report. Generically
y=x4/t, so [F2(E49):F2(t)]≤7. On the ordinary elliptic model at
T=(0,1), set v=Y+1. The elliptic equation becomes

    x³+x²+x+x v+v²=0.

Its derivative with respect to x at T is1. Hence v is a local parameter,
ord_T(x)=2, and the eta y=x²+(x²+x+1)v has order1. Therefore
ord_T(t)=7. The degree bound is attained: t has degree7 and zero divisor
7[T]. Every generic fiber is linearly equivalent to7[T]; under the
elliptic Picard identification its seven points sum to7T=T. This uses
the standard X0(49)→X0(7) forgetful identification of t from the cited
eta formulas. It is written geometry, not a finite-test inference.

The 56 cyclic49 choices group into eight seven-element fibers over a
fixed first7 kernel. Quotienting by eleven commutes with this forgetful
map, so each of the12 target maps also has fiber sum T. All96 sums
are checked in exact F2^210 coordinates. The14-member Frobenius orbits
traverse two such seven-element fibers, with the normalized complement
labels carried along. The direct14-term sum also verifies this for every
column. For the signed tuple, the seven sums cancel pairwise as2T or
T−T; augmentation and the subsequent trace preserve zero. Lean proves
these cancellation implications assuming the two fiber-sum identities.
It does not formalize the modular forgetful map or the local divisor.

## 3. Reverse fiber coordinates and the full divisor

The ordinary reverse11 correspondence has12 base neighbors; each carries
55 determinant-normalized paired bases modulo simultaneous sign. The
old P line is fixed by the dual kernel, while the pairing chooses its
complement. These ordinary fibers are etale; all source49 kernels and
backward targets are explicitly transported and checked. The positive
seed is represented by660 F2^120 points.

For the negative seed let K=F2^120 and h=psi11/psi_known, degree55.
In K[X,Y]/(h,Y²+XY−f(X)) the generic Q has order11. This algebra is
etale because11 is prime to2 and X is nonzero on its support. Inversion
uses the norm of u+vY, namely u²+uvX+v²f. The exact conditional
characteristic2 identity is proved in Lean. The Miller recursion computes
e(G,Q)=u+vY using an auxiliary point avoiding its poles. The recorded
auxiliary points, units and pairing evaluations are executable checks.

For c=1,…,5, put z=zeta^(c^−1). The polynomial gcd of h with
e+e^−1−(z+z^−1) has degree11. On this component set Y=(z+u)/v.
Each factor is irreducible over K, their product is h, and the resulting
point satisfies the curve equation, [11]Q=0 and e(cG,Q)=zeta. Choosing
c modulo sign exhausts paired bases modulo simultaneous sign. Thus the
60 recorded components specify all660 geometric points, rather than an
unresolved extension search. The separate checker recomputes these
equations and pairings in degree11 quotient fields.

On the order11 source generator Pi, coordinate Frobenius acts as[5],
so6Pi=−Frob Pi. In Picard0(E), [6Pi]−[Pi] represents5Pi and thus
may replace [5Pi]−[O] in the round05 adjoint formula. This avoids cusp
fibers while preserving the divisor class. Frobenius on paired bases is
(P,Q)↦(P²,Q²/2). The determinant1 change

    (P,Q)↦(−aQ,a^−1 P+Q)

makes the line Pnew+aQnew equal the old P line. Applying these maps to
both seed formats gives every one of the20 requested pullback fibers.
The stored coordinates satisfy their curves, torsion, pairing and line
conditions; the separate parser checks all7200 curve pairs and line
conditions. Seed Frobenius5 permutations and the20-term permutation
prove that the complete signed divisor descends to F2. There are ten
positive and ten negative fibers, each of degree660.

The forward check invokes the round05 written correspondence identity
Psi r=[32000] and evaluates its target-point expression. It is not an
independent norm computation from the7200 entries. Full Picard reduction,
pointwise forward norms and the odd reverse divisor remain unimplemented.

## 4. Identity certificates and formal scope

identity-split-polarization-lift.json binds the universal bilinear residual
to polarization_lift; identity-degree-seven-equation.json binds the exact
cleared equation to degree_seven_equation. Each issuer checks64 points
and an extra checker checks128, with direct Lean source hashes. The native
record assembler binds all raw outputs and the relevant source files.
Global descent effectivity, positivity-to-polarization, modular moduli,
Picard identification and completeness of divisors are the written bridges
above. No theorem assumes a new research claim as a custom axiom.
