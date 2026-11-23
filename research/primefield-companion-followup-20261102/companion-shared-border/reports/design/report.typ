#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Shared border functionals and rank-one limits
2026-11-02 · Derived algebra / static source checks · Numerical trials: zero

*Construction.* Let q=n−1, J reverse q coordinates, and t,c be the cached
row/column tails. Check t=aJc. If it holds, both lift functionals use c:
tᵀz=a cᵀJz and cᵀv is unchanged. If it fails, retain both original tails.
No source query or inversion is introduced. Previous scalar lifts and root
solution/witness checks remain unchanged.

#image("pipeline.svg",width:100%)

#table(columns:(1.3fr,1fr,1.2fr,1.5fr),inset:4pt,
 [Mode],[Border slots],[Primal products],[Successful detection],
 [a=0, t=0],[q],[0],[q comparisons],
 [a=1, t=Jc],[q],[q],[q comparisons],
 [general a, t=aJc],[q+1],[q+1],[q products + comparisons],
 [plain fallback],[2q],[q],[charged prefix scan])

All nonzero dot branches charge q additions; general scaled primal adds one
product after the dot. The general mode is skipped at q=1 because no field
slot is saved. Failed detection retains its overhead. Input validation,
metadata, slicing and allocation are additional host work. No measured
runtime or memory benefit is assigned.

*Positive family.* Put C=Σ gᵢ Zₐⁱ. Then Ce₀=g and its first row is
(g₀,a gₙ₋₁,…,a g₁). Since C commutes with Zₐ,
ZₐC−CZᵦ=(a−b)g eₙ₋₁ᵀ, of rank one for g≠0 and a≠b. This supplies the
shared-border identity. Equality at one node need not survive Schur descent.

*Information bound.* Fix a=1,b=2,g₀=1. There are p^q arbitrary tails c,
all with displacement rank one and nonzero first pivot. Basis queries
distinguish their dot functionals. An exact self-contained encoding answering
all such queries needs at least q log₂(p) bits in the worst case. L field
slots and M metadata states require p^L M≥p^q. External retained data or
recomputation are outside this self-contained model and must be charged.

*Recurrence consequence.* Order-d constant-coefficient recurrences with d
initial values have at most p^(2d) encodings; they cannot cover this family
when 2d is less than q. Small displacement width alone supplies no short border recurrence.
This statement concerns the specified operator/query model, not a lower bound
for elliptic discrete logarithms or a particular elliptic feature family.

*Checks and next work.* Separate frame and solver normalized-AST audits;
independent coefficient checker implemented, unexecuted. No numerical,
mutation, heap or performance trial. Validate all four modes, mismatch
positions, q=1, both directions and counters under the full upstream cohort.
Next seek a recurrence from the actual public feature map and prove its
behavior under pivoting and Schur reduction, or price replay/recomputation.

*Sources.* DERIVATION.md, pair.py, shared\_frame.py, solve\_shared.py,
check\_pair.py and check\_preparation.py. Bostan et al. (2017), equation (2)
and Section 6, DOI 10.1137/16M1062855, fixes the shift/Schur conventions.
