# Round08 proof map

The new Lean module is KoblitzRound08. It imports the pinned earlier
proofs and adds16 named results without custom axioms or native trust.

| Claim | Kernel-checked statement | Written or native bridge |
|---|---|---|
| Invertible level change | deck_basis_inverse, fixed_line_swap_descent | Actual basis coordinates and normalized Frobenius checked natively |
| Primitive coordinate changes preserve identity | coordinate_change_injective | Native11×11 inverse and every coordinate substitution checked |
| Kummer equation clearing/change | kummer_denominator_identity, kummer_root_change | Explicit coefficient arrays and displayed localization |
| Function multiplication on the binary curve | binary_function_product | Characteristic2 relation, polynomial reduction and rational denominators |
| Miller divisor | miller_divisor_recurrence, miller_divisor_torsion | Line-function divisors; exact chain points, support, jets and origin pole |
| Etale chart | kummer_jacobian_identity, kummer_jacobian_inverse, kummer_root_nonzero, kummer_derivative_nonzero, diagonal_jacobian | Finite-free monic equations over localized base; Kummer/Picard identification written |
| Complete zero-degree sum | complete_new_norm_degrees | Complete modular etale fibers and cusp pole images from earlier written bridge |
| Global chart coverage and gluing | six_chart_cover, transition_cocycle | Actual translated bad sets, exact rational transition identities and verified constant roots |

## 1. Exact joins and relation ranks

The old columns through index587 have degree6,30 or210. A point from
the present supports projects to a modular base point of degree60,
checked in round07. Since60 divides none of those degrees, these supports
cannot meet those columns. M8 columns are then compared directly, and all
subsequent split/etale columns have exact coordinate keys.

For a degree11 field K[t]/g, choose z=x(Pnew). Its powers0 through10
form a basis precisely when the computed matrix is invertible. The code
checks that inverse. Express z^11 in the basis to obtain its monic minimal
polynomial; express y(Pnew),x(Qnew),y(Qnew) in the same basis. Substitution
back into the original algebra verifies every expression. This describes
the actual closed component without choosing a root. Simultaneous sign
changes keep x(Pnew) fixed and change the two ordinates, so taking the
smaller of those two encodings removes exactly that level equivalence.
The modular base point and curve coefficients are also included.

Applying coefficient Frobenius to this presentation gives the Frobenius
component. The new complementary point must also be scaled by2^−j, as
required by determinant-normalized level. The generated keys do this before
the coefficient Frobenius. All closed orbits have60 distinct component
keys: geometric degree60 in the split case and660 in the etale case.
Matching complete key sets verifies an overlap; equality of group images
does not enter the join decision. M10 and M11 have no new joins.

The complete divisors follow the same ordinary prime-to-characteristic
level-cover argument as round07. M10 has five degree13200 zero divisors;
M11 has degree6600. Poles are modular cusps with zero projected11-torsion
images. Every row sum vanishes. BigInt and separate u64 elimination compute
rank12/1308 and rank11 on1165/1160 nonzero-image columns. The finite full
matrix rank is not a new Lean matrix theorem; exact finite calculations
and the earlier generic support-independence lemma are disclosed separately.

## 2. Miller divisor certificate

Write D_n=n[T]−[nT]−(n−1)[O]. The standard line quotient for P,Q has
divisor [P]+[Q]−[P+Q]−[O]. Thus adding its divisor to D_n+D_m gives
D_(n+m). Lean proves this formal-divisor recurrence and the specialization
D_19=19([T]−[O]) when19T=O. Binary19 uses the fixed bits0,0,1,1.
The actual line/tangent points and sums are recorded.

Native function arithmetic represents (A+By)/D and reduces products by
y²=h(x)y+f(x). The universal integer identity binary_function_product
specializes to that reduction in characteristic2. Polynomial gcds cancel
common factors. The final function has leading coefficient1 and pole
order19 at O, using pole orders2 for x and3 for y. The two possible
numerator leading terms have different parity, so cannot cancel.

The denominator factors over the listed subgroup abscissas. On the
ordinary19-torsion x is nonzero, and on the supersingular curve h=1;
therefore x−x(T) is a local uniformizer in every tested affine neighborhood.
Exact implicit-function jets determine the valuations of numerator and
denominator separately through order64. They give one finite zero of
order19 and no finite poles. Together with the order19 origin pole and
the degree-zero divisor law, this proves the recorded divisor. Independent
pointwise Miller evaluations further check the rational expression.

These divisor calculations use exact native fields; their complete
finite-field semantics are not formalized in Lean. The field irreducibility
and subgroup support are checked by the native replay and earlier graph
certificate. The certificate hashes bind the actual implementation.

## 3. Identification of the intrinsic quotient chart

Work over K=F2^360, containing μ19. Let B=E^4×E' with its product
principal polarization λ0, let q:B→A=B/Γ, and let λ_A be the principal
polarization already obtained from the isotropic graph. Set

    r = λ0^−1 q^dual λ_A : A→B.

Then r q=diag(H,[19]); both q and r have degree19². Under λ0 and λ_A,
the dual of r is q. Hence the kernel of the dual isogeny is exactly Γ,
as an actual subgroup of B[19]. This uses the retained polarization
pullback and graph certificates, not only a matching dimension or order.

A point T=(T_j) of B[19] represents the degree-zero line bundle
L_T=⊗pr_j^* O([T_j]−[O]) under the product principal identification.
The product F_T=∏ f_Tj trivializes its19th power. The Kummer construction
therefore gives a μ19 torsor with this Picard class. For two generators
of Γ, the two corresponding classes are independent: project a proposed
linear relation to E', where the actual companion points form a basis.
Equivalently, a19th-power relation among F_1,F_2 would make the same
nonzero combination of line bundles principal. Thus the combined cover
is geometrically connected of degree19².

The rigidification matters. Every component function has normalized
leading coefficient1 at O. With local parameters t_j, the unit obtained
from F_i by multiplying by∏t_j^19 has value1 at the product origin.
Since19 is invertible, it has a unique formal19th root with value1.
This chooses a K-rational point above the origin and excludes an
unrecorded constant torsor twist. The resulting connected etale cover
of B is an isogeny with dual kernel Γ. It is therefore r:A→B, with this
origin choice. This is the standard dual-isogeny/Kummer correspondence.

On the open set where every numerator and denominator is a unit, its
intrinsic coordinate algebra is exactly the two displayed monic Kummer
equations. It is finite free of rank361 over that base open set.
The derivative entry z_i^18 D_i has inverse z_i N_i^−1 when
z_i^19 D_i=N_i. Lean proves the exact identity and inverse witness;
the diagonal determinant is a unit. This gives a smooth finite etale
dense chart. The connected function field identifies its prime component
without a numerical choice among roots.

## 4. Six explicit charts cover the global quotient

For each factor choose S and put R=19S, checking19dR≠O for1≤d≤5.
The two torsion points T_1,T_2 and O define its bad set B_j. If
B_j−kR and B_j−lR intersect for k≠l in0..5, then(k−l)R is19-torsion,
contradicting the check. Thus each factor excludes at most one chart
index. The five factors exclude at most five of the six indices. The
Lean six_chart_cover theorem proves precisely this finite implication.
The six translated product opens cover B, and their preimages cover A.

Let g_(T−R,R) be the usual Miller line quotient and let v_R=x+x(R).
Then h=g/v_R has divisor [T−R]−[-R]−[T]+[O]. Hence
f_T(P+R)/(h(P)^19 f_T(P)) is constant. At O, h has leading term t
and f_T has leading term t^−19, both with coefficient1, so the constant
is f_T(R). The native calculation verifies this full rational identity,
not only the divisor argument or point values. It also computes a
19th root of that constant in K and verifies its19th power.

For each generator i and factor j, write rho_ij,k for the recorded
root times h, with rho_ij,0=1. Then F_i(P+kR)/F_i(P) is the19th power
of product_j rho_ij,k(P_j). On an overlap, the Kummer coordinate change is

    z_i,l = z_i,k product_j rho_ij,l(P_j)/rho_ij,k(P_j).

Here P denotes the common unshifted base point; in chart k use its
explicit inverse translation to express P. Ratios telescope, so the
overlap cocycle is exact, as proved in Lean. Each ratio is regular and
invertible on the overlap: its19th power is a unit and the chart is
normal. The base transitions are the explicit elliptic translations
(l−k)R_j. A separate parser checks all200 stored rational functions
against the original functions, plus their inverse identities,100 curve
identities and400 point translations.

The atlas is therefore explicit over K and covers the global rooted
quotient. A closed projective embedding and effective F2 coordinate
descent are still not supplied. The original full F2 quotient-equation
requirement remains partial. This does not identify the polarized
fivefold as a Jacobian.
