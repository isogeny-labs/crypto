"""Render inverse-metric provenance and derived schedule; no arithmetic imports."""
from pathlib import Path
from html import escape
import typst
import fitz
D=Path(__file__).resolve().parent
labels=['Validate canonical root metric / inverse once; freeze tuple values',
        'Derive checked permutation and nonempty trailing views',
        'Match selector / cache arrays to the inherited root coordinates',
        'Reuse inverse identities without multiplying each pair again',
        'Keep all pivots, source queries, lifts and root verification unchanged']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="800" height="280"><rect width="800" height="280" fill="white"/>'
for i,label in enumerate(labels):
 y=6+55*i
 svg+=f'<rect x="10" y="{y}" width="780" height="38" rx="4" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+24}" text-anchor="middle" font-family="sans-serif" font-size="14">{escape(label)}</text>'
 if i<4:svg+=f'<path d="M400 {y+38} v17 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
(D/'pipeline.svg').write_text(svg+'</svg>')
s='''#set page(paper:"a4",margin:17mm)
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
'''
(D/'report.typ').write_text(s)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(D/f'page-{i+1}.png')
print('pages',len(doc))
