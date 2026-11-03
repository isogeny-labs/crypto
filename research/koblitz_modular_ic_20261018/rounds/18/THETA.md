# Canonical symmetric characters and the precise F2 cover obstruction

G19 resolves the character ambiguity on the selected diagonal bundle under
the existing symmetric pullback-class bridge. It does not extract sections.
Write F_a for G18's factored rational translation function and c_a its
chosen period-normalizing scalar. The literal factor divisors
(n_i-1)O+T are invariant under inversion, so inversion acts on sections by
I(s)(P)=s(-P). For A_a=c_a F_a(P)t_a,

    I A_a I = Delta_a A_a^(-1),
    Delta_a=c_a² F_a(-P)F_a(P-a).

The function on the right has divisor zero, using the written G18 Miller
and correction-divisor bridge. On the proper geometrically integral product
it is a constant. In characteristic2 it has a unique inverse square root.
Thus b_a=Delta_a^(-1/2) gives the unique symmetric lift b_a A_a. The formula
can instead be applied directly to the raw F_a and avoids a choice among
odd roots when normalizing periods.

Native arithmetic evaluates all four generators at all three retained
tuples. All discrepancy constants agree, the corrected periods equal1,
and all18 pairwise commutator probes per graph pass. Relative to the
checked primitive105th root formed from the three G17 Weil pairings, the
old-to-canonical character exponents are[35,0,50,23] on both graphs0/2.
Their discrepancy exponents are[35,0,5,59], satisfying2k+delta=0 modulo105
and order*k=0. Lean checks these actual congruences and proves square-root
uniqueness, triviality of a self-inverse character and normalization under
a field homomorphism. Twelve standard coefficient-Frobenius controls per
graph verify the corrected scalars square correctly at the squared points.

## Matching the original mixed pullback character

G15's corrected bundle L is symmetric. G16 identifies the diagonal divisor
class with B^*(3L), including its actual Pic2 biases. Rigidify both symmetric
structures at the origin. Any isomorphism of those line bundles then
intertwines inversion: its possible scalar discrepancy is1 on that fibre.
The canonical pullback kernel action is therefore symmetric on the diagonal
representative. Two symmetric actions differ by a character chi with
chi(a)=chi(a)^(-1). Such a character is trivial in characteristic2; the
kernel is odd and étale, so no infinitesimal2-primary ambiguity occurs.
Consequently G19's unique symmetric action is the canonical one under the
written pullback-class identification.

This argument uses symmetric structures, not a guessed trivializing
function. No explicit rational function trivializing the original mixed
divisor has been produced. The bundle/divisor and theta-group geometry
remain written; the scalar uniqueness and finite certificates are in Lean.
The primary symmetric-lift reference is the proof of Robert Theorem1.1; Moonen8.10 identifies a kernel lift with line-bundle descent.
We use the symmetric-lift rule, not an unchecked application of all the
additional hypotheses in Robert's isogeny theorem.

## G20: this diagonal cover does not carry the target rotation

Standard split coefficient Frobenius is different from the actual trace-zero
F2 descent, which includes the augmentation rotation R. Using the actual
integral8 matrices, G20 computes B^(-1)RB with reduced common denominator105.
A row of adj(B) annihilates B modulo81 but does not annihilate RB.
This is an exact obstruction to B A=R B for any integral A. Lean checks
that finite matrix certificate and proves the no-lift implication.

The ordinary elliptic endomorphism order is the full order of discriminant-7;
its action has an integral8 representation. Hence the obstruction rules out
a lift by an E^4 group automorphism, and also a translated descent lift,
whose linear part would have to satisfy the same equation. This excludes
this selected diagonal cover with the specified target rotation. It does
not exclude F2 descent of the quotient, its symmetric bundle or other covers.
The obstruction is stronger and more specific than an unimplemented step.

## G21: an explicit rotation-compatible refinement

The least positive scalar making B^(-1) integral is105. Set C=105B^(-1).
Both B C and C B equal105I, proved by actual integer matrices in Lean.
The native checker also verifies every CM block of C and evaluates both
compositions at four generic tuples on each elliptic sign.
The composite is multiplication105 on the trace-zero source and commutes
with R. It therefore supplies an F2-compatible scalar cover under the
known split trace-zero bridge. It is not another diagonal polarization.

| quantity | exact value |
|---|---:|
|old cover degree|99225|
|refinement C degree|148899515625|
|composite degree105^8|14774554437890625|
|derived source section count|1296068238955079296875|

These are algebraic degrees/dimensions, not execution costs or matrix ranks.
Actual G17 torsion bases give coefficient Frobenius modulo105 as
[[45,62],[14,59]] and[[60,43],[91,46]], each of exact order168.
Tensoring with R gives trace-zero105-torsion Frobenius of exact order840;
all proper prime-divisor tests pass. Lean checks the small matrices, scalar
refinement and rotation, with a general commuting-period lemma. The full
8-dimensional module order is also checked natively. Coordinates over the
compositum F2^840 and its field identification are not materialized.

Open: an explicit mixed refined theta action and coherent F2 invariant basis,
the second19² kernel action in those coordinates, the243 target sections,
proper equations and a saturated identification with the affine quotient.
The scalar refinement avoids the selected-cover lift obstruction but does
not turn the existing factor bases into an evaluated F2 embedding.
