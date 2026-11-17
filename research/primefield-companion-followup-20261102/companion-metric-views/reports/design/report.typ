#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Immutable inverse-metric views
2026-11-02 · Derived validation reuse / static checks · Numerical trials: zero

*Invariant.* Validate dᵢ·diᵢ=1 once at the root. A view stores distinct root
indices. Permutations reorder indices; trailing views discard a prefix.
Neither changes the field entries, so their inverse identities remain valid.
This does not certify DA=AᵀD; source symmetry remains the compiler/caller's
separate obligation.

#image("pipeline.svg",width:100%)

*Entry checks remain.* Selector and row-cache entry points require the exact
view/root types, same meter, unchanged captured prime, correct dimensions,
canonical integer types and matching root-coordinate values. Wrong order or
altered values are rejected without repeated field products. Frozen Python
objects supply internal provenance, not a public certificate against deliberate
reflection or concurrent mutation.

*Derived product savings.* Let N be root dimension, S the nonzero selector
nodes and C the nonterminal cache nodes, with dimensions nᵥ.
#table(columns:(2fr,2fr),inset:5pt,
 [Previous inverse validation],[N + Σ(S)nᵥ + Σ(C)nᵥ],
 [New inverse validation],[N],
 [Removed field products],[Σ(S)nᵥ + Σ(C)nᵥ],
 [Full scalar descent removed],[N(N+1)−1],
 [Full pair-only descent removed],[N(N+2)/2−2, even N],
 [Initially zero source removed],[0])

Use actual node sums for singular paths. Row reconstruction still uses 2n·cut
products, and all source, transport, pivot and lift arithmetic remains.
No inversion or source query is saved by this component.

*Host work and storage.* Every reuse still checks n coordinate pairs, with
native comparisons and index reads. Label/permutation tests, set construction,
tuple copying and allocation remain. New counters describe those events;
they are not complete CPU or bit-cost measurements. Root metric storage stays
2N field entries. Views share that root and hold index tuples with no parent
chain. Existing gathered arrays and frame storage remain; the current view
adds index metadata. No runtime or total-heap reduction is measured.

*Integration checks.* Four declared substitution audits cover the complete
driver, selector, row helper and block cache. Complete AST comparisons pass;
all Python sources parse. Static inspection finds one root-validation call
site and no field multiplication/inversion/source calls in view derivation
or coordinate matching. These checks do not execute the new implementation.

*Required validation / next.* Preserve the full upstream cohort and pivot
sequences. Cover zero/scalar/pair and singular terminals, both witness/solution
outcomes, nonconstant permuted metrics, wrong values/order/meter/prime,
duplicate/out-of-range/bool indices, frozen writes, counters and heap aliases.
Compare removed products to the actual node-sum formula independently.
Existing cost admission still prevents numerical trials. Next derive safe
selector zero-prefix reuse; its query saving is not included here.

*Sources.* metric\_views.py, selection\_view.py, rows\_view.py,
block\_cache\_view.py, driver\_views.py, integration.json and DERIVATION.md.
The unchanged exact-field scalar/pair elimination context is Dumas–Pernet2018,
arXiv:1802.10453. This representation introduces no new factorization theorem.
