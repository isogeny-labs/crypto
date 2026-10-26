# Round10 proof map and written geometric bridges

## Lean scope

KoblitzRound10.lean adds23 named results to the pinned project. The
actual build and transitive audit pass;the full gate checks146 project
theorems with zero native trusts. Only the
standard propext,Classical.choice and Quot.sound foundations are admitted;
no custom axiom,sorryAx or trusted native decision is used.

| Named results | Exact scope |
|---|---|
| inverse_witness_unique,inverse_witness_cancel | Ring inverse uniqueness and cancellation |
| inverse_ideal_cancel,saturation_powers | Ideal cancellation with a unit witness;all powers under an explicit regularity hypothesis |
| determinant_inverse_witness | Matrix inverse implies determinant-unit identity |
| coefficient_bilinear_expansion | Finite bilinear expansion over any commutative base ring |
| dual_number_square,square_sum_residual | Characteristic2 nilpotent square and universal integer residual |
| component_sequence_unique | A recursively linked component sequence is determined by its first component |
| homogeneous_saturation_certificate,naive_projective_false_point,saturated_relation_on_affine_chart | Exact membership certificate,false projective point and h≠0 implication |
| product_cusp_degree,coupled_pole_degrees,sum_difference_map_degrees | Integer arithmetic for the recorded degrees |
| radial_line_derivative,product_divisor_sum | Conditional characteristic2 derivative and formal divisor addition |
| collision_derivative_residual,collision_derivative_factorization,collision_resultant_factorization | Universal residual,char2 derivative factorization and degree96 factorization |
| ordinary_transport_determinant,ordinary_transport_finite_chart,ordinary_transport_other_chart | Universal determinant and both explicit ZMod49 projective charts |

The homogeneous certificate and the collision derivative residual receive
source-hash-bound IDC certificates,each64 issuer and128 extra evaluations.
Those are identities,not certificates for the whole descended scheme or
for the Euclidean resultant implementation. Native matrix/field semantics
and global geometry are not imported as axioms into Lean.

## F2 polynomial systems and scheme interpretation

For any F2-algebra A,let K_A=A[z]/(z^360+z^5+z^3+z^2+1).
It is free on1,z,...,z^359. The multiplication tensor is explicitly
specified by the719 reduced powers. Comparing coefficients makes each
operation block a list of360 scalar polynomials overF2. Constants and
addition are linear. A multiplication output satisfies

    out_i+sum_(j,k with z^(j+k) coefficient_i=1) left_j*right_k=0.

An inverse output uses the same tensor with right=out and constant
coefficient1. Coefficient Frobenius is the A-linear map sending z^j to
z^(2j);it fixes A. Its equations are linear in the input variables. It
is not absolute squaring in K_A. The64 dual-number controls test this
ring distinction. The native field/bit reductions and23 structural Lean
lemmas are separate evidence layers.

The graph equations uniquely determine all operation coordinates from
the twelve input blocks whenever each inverse exists. Conversely,any
solution of the graph equations supplies precisely those inverses and
operations. Hence the graph is an effective affine presentation of the
rational formulas on the domain they specify,over arbitrary A,not only
over finite fields. The original seven curve/Kummer residuals and twelve
equalizer residuals are all explicit indexed outputs in the program.

For a denominator d and witness u,the multiplication matrices satisfy
M_d*M_u=1. Their determinants show Norm(d)*Norm(u)=1. Thus the full graph
ideal is saturated by D,the product of the norms of all denominator inputs.
The norm is the constant coefficient of the product of all360
coefficient-Frobenius conjugates,so D is an explicit polynomial circuit
using the retained tensor and inverse-node indices. The ideal-cancellation
and determinant statements are Lean checked;their interpretation as these
actual free-algebra multiplication matrices is a written bridge.

This does not provide an eliminated generating set for I:D^infinity in
the original4320-variable ring. It supplies its localized graph through
inverse witnesses. Eliminating the witness/operation variables would give
an affine closure ideal;one must retain the open condition to identify
the localized model. No elimination or small invariant presentation is
claimed to have run.

Let U be the original Kummer chart with the further displayed generic
line denominators inverted. Apply restriction of scalars to its affine
inverse-witness presentation. The equalizer is X=theta(sigma_coeff X),
where theta is the actual round09 rational lift on point coordinates.
After base extension to K,the algebra K tensor_F2 K splits into360
copies through its embeddings. The equalizer becomes a cyclic chain of
rational graphs between these conjugate copies. Because all inverse
witnesses are required to exist in K_A,every component lies in the
necessary domain. The360-fold cocycle from round09 closes the chain.
Projection to one component therefore identifies it with the intersection
of all360 translated opens. The inverse is the uniquely specified orbit
of graph maps. These substitutions hold over all base algebras and give
the scheme isomorphism;the native fixtures merely check its application.

The intersection is a nonempty stable affine open in the separated proper
quotient (a finite intersection of affine opens of a separated scheme is
affine). Affine Galois descent is effective,as in
[Stacks0244](https://stacks.math.columbia.edu/tag/0244). The emitted quotient
ring gives a closed embedding of this descended open into the stated
F2 affine space. It is smooth of dimension5 under the retained connected
Kummer/isogeny identification. A full Lean construction of these schemes
and that descent equivalence has not been supplied.

## Projective saturation control

The repaired prototype ideal is generated by xy-zh,xu-h²,zu-yh.
Under lex order x>z>y>u>h,the leading monomials are xy,xu,zu.
All three S-pairs reduce to zero in native F2 polynomial arithmetic.
Buchberger's criterion proves this is a Groebner basis. Its initial
monomials omit h:if h*f belongs to the ideal,the normal form of h*f is
h times the normal form of f,so the latter is zero. Therefore h is regular
modulo the ideal and it is h-saturated. Localization at h identifies this
ideal with the original two generators by the explicit membership
certificate. It is exactly their h-saturation. Lean checks the certificate,
the false point and the conditional regularity-to-power implication;
the concrete Buchberger/normal-form bridge remains native/written.

The full quotient has an explicit homogenized block system,but its
h-saturation has not been computed. No argument equates its projective
closure with the proper polarized abelian quotient. The prototype shows
why a general claim from naive homogenization would be invalid;it is not
a counterexample exhibited on the full quotient's own homogenized ideal.

## Cusps,ramification and complete product divisor

Use the eta coordinate x of X0(49),which is also the retained minimal
elliptic x coordinate. Its divisor is2(T)-2(O). The six other cusps have
x a nontrivial7th root and y=(x+1)/(x²+x+1) overF8. This follows both
from the frozen eta equation and its cusp divisor;see
[Kilford–McMurdy,section2](https://arxiv.org/pdf/1110.6801).

On a Tate chart with q=s^11,full-eleven bases have exponent vectors
(b,d) in F11² minus zero,modulo sign. There are60. A line is the
canonical multiplicative subgroup precisely when its exponent vanishes.
Quotient by that line sends q to q^11;quotient by a different line sends
q to zeta^k*q^(1/11). Hence its ramification on C is121 or1. Five labels
are canonical for each line. Base-cusp widths are1 or49. Both are coprime
to11 and do not change these relative exponents. The counts give
5*121+55=660 for each map. This is a Tate-moduli calculation,not a
ramification conclusion inferred just from equality of degrees.

At a middle cusp,a cyclic49 generator can be written using a primitive
49th root and q^(1/7). The multiplicative quotient multiplies its
cyclotomic label by11;the other quotients multiply it by11^(-1) modulo7.
On the chosen cusp x coordinates these are x^4 and x^2. All actual F8
points and corresponding function values are checked. At equal
ramification,the distinct zeta phases yield distinct leading powers for
the exponents1,2,3,5 used here. The440 finite leading-coefficient checks
cover every pair of distinct phases. The general Tate/integral-moduli
framework is described by
[Cais–Conrad,section2](https://math.stanford.edu/~conrad/papers/J5final.pdf).

At O,the poles of x,y have orders2,3. If one tested line is canonical,
the two ramification exponents are121,1;otherwise they are1,1.
This gives pole orders242/2,363/3,244/4 and365/5 for x-sum,y-sum,
x-product and cross-coordinate functions respectively. At T,x has order2
and y+1 has order1. At middle cusps,x is a uniformizer and the exact
implicit y series is checked through order8. These calculate every
cuspidal valuation. Three middle cusps also satisfy y=x,producing the
additional30 cross-function zeros of order1. All other residual degrees
remain noncuspidal without an ordinary/supersingular split.

For x0*xinf,pulling back2(T)-2(O) gives the complete displayed divisor.
Every preimage of either cusp is a cusp,with the sixty labels above.
Frobenius on the determinant-normalized level is(P,Q)->(P²,Q²/2);
on the Tate exponents this gives(b,d)->(b,d/2). Both O and T are rational.
The enumerated cycles give22 closed divisor points. At O every target is
O,and at T every target is the two-torsion point T;the even and odd
paired target differences both vanish. Thus the projected images are
zero already geometrically. No new nonzero-image rank follows.

## Collision screen and supersingular diagonal gap

Differentiating the frozen sparse mod2 Phi11 gives

    dPhi/dY=X^3*Y²*((1+X^4)Y^4+X^3Y^3+X^4)².

For X nonzero,Phi(X,0)=X^12 excludes Y=0. A repeated eleven-neighbor
j value therefore requires the displayed quartic and Phi to have a
common root. Native Euclidean resultant arithmetic overF2(X) computes
X^96+X^88;all denominators cancel. Trial division exhausts it as
X^88*(X+1)^8. Separate small-field gcd controls cover2046 X values.
Lean proves the characteristic2 derivative and factorization identities;
it does not formalize this resultant algorithm. The only ordinary source
candidate is j=1,whose maximal CM order has discriminant-7.

Its two horizontal norm11 maps are alpha=1+2w and conjugate(alpha)=3-2w,
w²-w+2=0. In the recorded basis their determinant on a vector(x,y) is
-8*(x²+xy+2y²). A primitive vector modulo49 has one of the two enumerated
projective forms;Lean excludes zero in both. Thus those two quotients
cannot have the same transported cyclic49 subgroup. The correspondence
between maximal-order torsion and this split matrix representation is
written,not inferred from determinant matching alone.

For the supersingular curve,E_ss:y²+y=x³,the24 automorphisms have the
explicit u,r,t formulas recorded in the raw artifact. The selected I,J
satisfy I²=J²=-1 and IJ=-JI. Each of the24 Hurwitz norm1 units is bound
to an actual automorphism by checking2*unit against its integer quaternion
numerator on225 points. The difference is a sum of at most six degree1
automorphisms,with degree≤36;225 distinct zeros force it to vanish.
Canceling[2] in the endomorphism domain proves the identities. Actual
Frobenius is -J+IJ. The maximal Hurwitz order interpretation is described
in [Elkies's notes](https://abel.math.harvard.edu/~elkies/M223.24/index.html).
The finite checks and this global degree bridge are explicitly separate.

At11 and49,the chosen matrices give the split two-dimensional torsion
modules of that maximal order. Their identification with actual torsion
requires the standard freeness/uniqueness theorem over each split local
matrix algebra;no coordinates for full torsion bases are emitted.
The288 norm11 elements give12 kernels. Two source-level lines in an
SL2(F11) basis select the corresponding isogenies;applying their49
matrices and comparing modulo the actual units tests equality of the
transported modular targets. The660 bases modulo sign have55 free unit
orbits,leaving55*56=3080 modular point labels. Four hundred match.
Normalized Frobenius using the actual -J+IJ puts them into40 cycles of
length10. These are complete set-theoretic moduli labels under the stated
bridge,but not explicit torsion-point coordinates or a divisor with
known multiplicities.

Target stabilizers give only multiplicity lower bounds. Here their sum
is400,whereas1280 minus the420 cuspidal contribution is860. The condition
for degree exhaustion fails. A total460 intersection multiplicity remains
to be computed at these labels. The antidiagonal part is separate and
uncomputed. Neither a complete x0+xinf row nor a rank increase is claimed.

The abstract11-module basis fixes some primitive Weil-pairing value.
If it differs from the earlier chosen value by a power r,rescale the
first basis vector by r^(-1). This diagonal right action preserves both
lines0 and infinity and commutes with the normalized Frobenius right
action diag(1,1/2). It therefore gives an F2 isomorphism between the two
determinant components preserving the two functions tested here. No
uncomputed symplectic identification is used to compare other line maps
or the earlier projected column images. An explicit torsion-coordinate
adapter would still have to determine r and verify the pairing itself.
