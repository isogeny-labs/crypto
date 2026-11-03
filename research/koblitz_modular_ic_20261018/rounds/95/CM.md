# R95 complete cusp support of the principal-open bridge

This is a divisor deduction from the accepted R27 source branches and
diamond action, R85/R94 field identities, and the smooth elliptic base.
It introduces no new native experiment. Let

    D_X = sum_{e=1,7; j=0,1} e [P[e,j,X]],

for each base cusp X in the orbit O,A,C,D,B. R27's four branches have
ramification indices 1,1,7,7, so each `D_X` has degree 16. On the
elliptic base `v²+v+u³+u=0`, the fiber `u=0` has A=(0,0) and B=(0,1).
The derivative with respect to v is 1, hence u is a uniformizer at
both. At O, `z=1/u`, `V=z²v` obey
`V²+z²V+z+z³=0`; its z-derivative is 1 at (0,0), so V is a
uniformizer and `ord_O(z)=2`. Therefore

    div_E(u) = [A]+[B]-2[O],
    div_X(u) = D_A+D_B-2D_O,
    div_X(z) = 2D_O-D_A-D_B.

The last two equalities use pullback of a principal divisor and the
R27 complete ramification fibers, not a generic degree count. With
`T: O→A→C→D→B→O` and R85's identity `h=z+V=u∘T`,

    div_X(h) = D_O+D_D-2D_B.

In the abstract second copy, `k=z+V+z²=u∘T⁻¹` and

    div_X(k) = D_O+D_C-2D_A.

These are complete divisors because the base divisor is complete and
T is an automorphism of the proper source. Their zero and pole degrees
both equal 32. The per-branch order of h at O,A,C,D,B is
`(e,0,0,e,-2e)` and of k is `(e,-2e,e,0,0)` for both e=1 and e=7.
Thus h is regular on `U0=X\(D_A∪D_B)`, and its zero set there is
exactly D_O∪D_D. The transported chart is
`U1=T⁻¹(U0)=X\(D_O∪D_D)`; the complements are disjoint, so
`U0∪U1=X` and `U0∩U1=D_{U0}(h)`. In the abstract second copy the
overlap is `D(k)`. R94 checks the transition units and R91 checks all
32 generator images on this overlap.

The Lean file `lean/CuspCover95.lean` proves the five-cycle and inverse,
both transported valuation tables, disjoint excluded supports, and the
h-zero support implication. It compiles with Lean 4.34.1/Std; the
transitive axiom audit reports no axioms for the permutation lemmas and
only `propext` for the others. Its inputs are the R27 branch/diamond
facts; Lean does **not** prove the geometric divisor pullback or the
R50 complete-RR normalization lemma. This round closes the cusp-support
bridge for the two-chart cover at the accepted R27/R50 scope. The next
independent tests are the remaining chart-normalization comparison,
direct modular-factor action, and a corresponding ramification/normalization
analysis for the degree-112 extension.

The first Lean draft used `decide` on equalities with a free ramification
index and failed to elaborate. Replacing those two proofs with `simp`
compiled and passed the stated audit. The first PDF render lacked the
`amsmath` import for `\operatorname`; the corrected one-page PDF was
rendered and visually inspected.

Sources checked: R27 `CM.md` and retained LAURENT02/NORM01 evidence,
R50 `RR-ALGEBRA.md` and `NEXT_SOURCE_NORMALIZATION.md`, R85/R94 source
identities, [Stacks principal divisors](https://stacks.math.columbia.edu/tag/02RS),
[proper normal curves and function fields](https://stacks.math.columbia.edu/tag/0BXX),
and [scheme gluing](https://stacks.math.columbia.edu/tag/01JA).
