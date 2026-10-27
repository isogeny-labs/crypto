# Proof map and remaining formalization

The pinned module imports `Std.Tactic` under Lean 4.34.1. Its checked
declaration is `ImageCoordinate.coordinate`; the actual command, compiler
output and axiom audit are preserved in the proof job and this directory.
This is partial formalization of the mathematical discussion, not a formal
verification of the elliptic-curve implementation or the complete algorithm.

| Report statement | Exact hypotheses and evidence | Formal status |
| --- | --- | --- |
| `3*x^2*z = -w^2*(x^3+4*B)` | Arbitrary commutative ring; `w^2+w+1=0`, `y^2=x^3+B`, and `(w-1)^2*x^2*(z+(1+w)*x)=4*y^2`. No division or exceptional-point exclusion is hidden in this conditional statement. | `ImageCoordinate.coordinate`, compiled; standard foundations only; false-theorem control rejected. Exact integer certificate IDC1h7e0104841476f369 also checked. |
| The conditional slope equation describes the actual image point | The finite-field curve law, `P-psi(P)`, nonzero denominators, and agreement with the supplied scalar eigenaction. | Executable checks on every panel generator and factor; the bridge from an abstract elliptic-curve group law to the Lean ring hypotheses is **not formalized**. |
| Image coefficients are known on the declared subgroup | The subgroup has the declared prime order, the explicit action equals lambda there, and `1-lambda` is nonzero modulo that order. | Panel subgroup, action, injectivity and coefficient checks pass. An abstract finite subgroup/action construction is **not defined in the Lean module**. |
| Transporting a complete relation by a nonzero scalar preserves its projective row and span | Residual, anchor, factor coefficients and query coefficients are all transported together over the same prime field. | Written argument and exact-set/rank controls; the project's augmented-row projection and its linear-algebra semantics are **not formalized**. This statement does not identify arbitrary distinct physical decompositions. |
| Uniform-residual first moments agree for equal-size disjoint images | Finite group, uniform residual, three distinct signed L0 factors, disjoint signed image factors; tuples are counted with multiplicity as stated in MATHEMATICS.md. | Written finite counting proof. Signed-factor tuple sets and finite uniform expectations are **not defined in the Lean module**. The coordinate certificate does not certify this probability statement. |
| Translation preserves expected memoryless peel work | Uniform residual and a fixed peel; cost is a deterministic function of the remaining point, including exceptional cases. | Written permutation argument; finite expectations and the cost model are **not formalized**. Adaptive stopping, caches, shared solves and conditioned streams are outside its hypotheses. |

The uncompleted proof obligations are these group/action, projective-row,
finite counting and expectation definitions and their bridges. No theorem
assuming the desired subgroup or rank conclusion is presented in their place.
The complete measured panel remains executable evidence with the limits in
[MATHEMATICS.md](../MATHEMATICS.md) and [BOUNDARY.md](../BOUNDARY.md).
