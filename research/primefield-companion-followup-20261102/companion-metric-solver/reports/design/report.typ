#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Complete weighted-symmetry solve / witness route
2026-11-02 · Derived construction / static checks · Numerical trials: zero

*Domain.* Square distinct-wrap companion source over a caller-certified prime
p>3, with nonzero diagonal metric D and established DA=AᵀD. The public
weighted Gram compiler supplies this invariant. Inverse-metric and local
reciprocal checks alone do not establish global symmetry.

#image("pipeline.svg",width:100%)

*Complete control flow.* Every nonzero source has an invertible principal
scalar or antidiagonal pair under the weighted-symmetry assumption. Simultaneous
permutations and principal Schur reduction preserve the inherited metric.
Full-size pivots use explicit small inverses; zero sources return zero solution
or normalized coordinate witness. No matrix solve oracle is supplied.

*Block cache and lifts.* Leading rows are reconstructed from selected columns
using Aⱼᵢ=dⱼ⁻¹dᵢAᵢⱼ. Leading-supported requests use cached matrix-vector
arithmetic. For A=(E,F;C,H), solution lift uses E⁻¹(b₁−Fz), with
Fz=D₁⁻¹CᵀD₂z; witness lift uses −E⁻ᵀCᵀv. Only C, RHS head, small inverse,
shared root metric and coordinate labels survive in each frame. Full permutation
inversion is explicit. Cache and successful frames release their stored arrays.

*Derived schedule.* Let cut k∈{1,2}, q=n−k>0 and w be transported width.
Search/transport/compression, root checks and common Schur work are separate.
#table(columns:(1.8fr,1fr,1.6fr,1.2fr),inset:4pt,
 [Phase],[Source calls],[Field products],[Field additions],
 [Row setup + validation],[0],[n+2nk],[0],
 [Supported setup / RHS],[0],[nk(2w+5)],[nk(2w+5)],
 [Outer Schur boundaries],[2],[kernel work],[kernel work],
 [Primal lift incl. inverse],[0],[kq+q+2k],[kq+k],
 [Dual lift incl. inverse],[0],[kq+k],[kq+k])

The general uncached block route has 2w+8 source calls over these phases;
this route has two. Scalar and pair pivot sequences differ, so this is a
same-block schedule comparison, not a measured full-run improvement. Zero
lift inputs still pay the displayed arithmetic. Native and field products
remain separate units; transport width growth is still charged.

*Storage.* Each frame has kq+2k field slots and two length-n integer tuples.
The shared root metric costs 2N field slots once. Setup temporarily holds 2nk
border entries alongside captured C. Certificates, root/current sources,
metric gathering, meter blocks and trace history are additional memory;
no total or measured peak-memory claim follows from these slot counts.

*Checks and validation.* Exact AST equality for the preserved zero terminal
and final root solution/witness checks; all Python files parse and dependency
hashes are recorded. No numerical, mutation, heap or performance trial. Full
upstream cohorts must cover pair/scalar/zero terminals, both lift directions,
composed permutations, metric/border corruption, counters and heap aliases.
The execution-cost gate remains in force; feature annihilation stays separate.

*Sources / next.* driver.py, block\_cache.py, block\_frame.py, check\_preparation.py
and DERIVATION.md. Dumas–Pernet2018 (arXiv:1802.10453) and Bostan et al.2017
(DOI 10.1137/16M1062855) are established context, without inherited fast bounds.
Next derive paired transport reuse and provenance-bound metric validation.
