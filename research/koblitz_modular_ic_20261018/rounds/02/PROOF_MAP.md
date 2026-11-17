# Round 02 proof map

Toolchain Lean 4.34.1; mathlib commit
`d13f23b723b8a846827a245b89c10fc7d3f11612`. The checked module is
[../../lean/KoblitzRelations.lean](../../lean/KoblitzRelations.lean).
Its thirteen declarations have kernel proofs; transitive foundations are
`propext`, `Classical.choice`, and `Quot.sound` only. No native evaluation,
custom axioms, or admitted proofs occur in the final source. Initial failed
builds are retained as failed evidence. `lean4checker` was unavailable.

| Report implication | Lean declaration | Exact scope and remaining bridge |
|---|---|---|
| Orbit recurrences survive scalar rescaling | `recurrence_scale` | Any field and index successor, assuming the displayed recurrence |
| Independent target block retains scaling freedom | `separate_blocks_leave_target_scaling` | Linear kernel statement, with distinctness supplied by the next theorem |
| Different scales of a nonzero target differ | `nonzero_target_scales_distinct` | Field module, nonzero vector; no claim about all modular correspondences |
| Prime-to-r push-pull degree gives injectivity | `push_pull_injective_on_torsion` | Additive homomorphisms, explicit Bezout witness, composition equals degree, r kills both inputs; constructing geometric pull/push is external |
| Zero eigenvalue character pair forces (-p)^D=1 | `eisenstein_zero_necessary` | Commutative ring; both character values have Dth power 1 and epsilon+p*eta=0; identification with cusp/Hecke modules is external |
| Quadratic extension point-order factorization | `quadratic_extension_factorization` | Universal commutative-ring polynomial identity; Frobenius interpretation external |
| Flex divisor relations imply 7D=0 | `klein_flex_relations` | Additive group, E=3D and 3E=2D; deriving divisors of x,y,z is written geometry |
| Klein quotient invariant identity | `klein_invariant_identity` | Universal polynomial identity in a commutative ring of characteristic 2, for all x,y,z; rational-map extension and degree remain geometric |
| Nontrivial additive characters sum to zero | `additive_character_orthogonality` | Finite additive group, nontrivial complex additive character; bit-trace/Walsh implementation correspondence checked by native controls |
| Three-term Frobenius trace is fixed | `frobenius_trace_fixed` | Additive endomorphism F, hypothesis F^3(x)=x; identifying F with curve Frobenius external |
| Frobenius intertwiner respects iterates | `intertwiner_iterates` | Arbitrary functions with intertwining identity; no integral lattice or polarization existence conclusion |
| Nonzero anchored transferred log recovers scalar | `anchored_transfer_recovery` | Field-linear transfer and logarithm, nonzero log of transfer(1); actual geometric maps and correctness of line-derived logs are outside Lean |
| Klein gradient has no common projective zero | `klein_gradient_common_zero` | Field, curve equation and displayed gradient equations; all coordinates must vanish. The projective Jacobian criterion is external |

The older [../../CyclicWindow.lean](../../CyclicWindow.lean) proves the
conditional numerical genus-window implication. This round adds no stronger
formal geometric cyclic-cover theorem.

## Written divisor certificate criterion

The Klein quartic is a smooth plane curve: its three partial derivatives
cannot vanish simultaneously at a projective point. With a zero coordinate,
the gradient equations force all coordinates zero. Otherwise multiply the
three gradient equations by x, y, z respectively. The three curve monomials
are equal and nonzero; their sum in characteristic two is nonzero, a contradiction. It is therefore geometrically
integral of genus three. A homogeneous form of degree m not divisible by its
equation cuts a divisor of degree 4m.

For a degree-zero divisor D supported in the recorded field, the checker
constructs the effective divisor H of a product of split lines. It checks
that a nonzero degree-m numerator, reduced in the standard monomial basis
modulo the curve equation (leading monomial x^3*y), vanishes to at least
H+D at every point. The checked orders sum to 4m; hence equality holds and
its quotient by the denominator has divisor D. Local orders use the implicit
function theorem with a checked nonzero partial derivative. The separate
checker evaluates the supplied coefficients and jets; it never solves the
interpolation system. A rational divisor principal after finite field
extension is already principal over F2: the associated Riemann--Roch space
commutes with field extension and has an F2 generator. This is a written
argument, not a formal Lean theorem.

The coordinate line divisors are
`(x)=3P2+P1`, `(y)=3P0+P2`, `(z)=3P1+P0`,
where P0=(1:0:0), P1=(0:1:0), P2=(0:0:1).
Thus `div(x*z^2/y^3)=7(P1-P0)`.
The difference of two distinct rational points cannot be principal on a
positive-genus smooth curve: its function would give a degree-one map to P1.
Consequently D=P1-P0 has exact order seven. The native interpolation also
checks failure of a function with divisor D, and supplies the witness for 7D.

## Transfer and relation argument

The quotient formulas in REPORT.md are invariant under cyclic coordinate
permutation. The invariant identity shows the image lies on E1. The quotient
has degree three: for a nonexceptional image, the symmetric invariants
recover a cyclic orbit of three points; the independent cyclic quotient
genus computation and explicit fibers provide additional checks. The
native experiment evaluates the three-point fibers for all seven source
points. Its trace map T=sum_(j=0)^2 F^j pull respects addition. Principal
certificates directly show T(kG)=c*k*D, c nonzero for 42 of 56 maps. This
establishes the subgroup action on the complete order-seven fixture group.
It is separate from a general transfer at other n.

Each split line ell supplies the principal divisor of the norm of ell/z.
Write the F2-rational divisor in closed-point orbits, and subtract its degree
times P0. These give relation matrix rows without target labels. Row rank is
checked using u64 reduced elimination and separate BigInt forward elimination.
A single free scale is fixed by D. For each resulting factor-base log l_i,
a separate certificate verifies
`2*(sum_(P in orbit_i) P - deg(orbit_i)*P0 - l_i*D)` is principal.
Thus these logs are correct after projection to the seven-part. The source
transfer is seven-torsion, so the projection loses no scalar information.
The selected map has nonzero base log, and division by it recovers the source
scalar. All seven resulting answers are checked by elliptic multiplication.

## Open formal obligations

A full proof still needs the actual function field and smooth projective
Klein model, divisors/local orders, Riemann--Roch certificate criterion and
base change, quotient/pullback/pushforward, the Picard seven-part, and the
linearity bridge for the instantiated recovery theorem. The experiment does
not encode these definitions by assuming its desired geometric conclusion.
Modularity of the signed-character factor, the Hecke action on cuspidal
divisors, polarized integral gluing and Jacobian realization remain separate
unformalized bridges. GL1 proves finite companion-module calculations only.
