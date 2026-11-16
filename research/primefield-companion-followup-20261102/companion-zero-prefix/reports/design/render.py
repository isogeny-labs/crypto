"""Render inverse-metric provenance and derived schedule; no arithmetic imports."""
from pathlib import Path
from html import escape
import typst
import fitz
D=Path(__file__).resolve().parent
labels=['Search columns in order; retain first nonzero index j separately',
        'Original columns 0 through j-1 have been checked zero',
        'Permute the zero-index set after each simultaneous swap',
        'Materialize missing front columns only when proven zero',
        'Keep generator formulas, pivot order and root verification unchanged']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="800" height="280"><rect width="800" height="280" fill="white"/>'
for i,label in enumerate(labels):
 y=6+55*i
 svg+=f'<rect x="10" y="{y}" width="780" height="38" rx="4" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+24}" text-anchor="middle" font-family="sans-serif" font-size="14">{escape(label)}</text>'
 if i<4:svg+=f'<path d="M400 {y+38} v17 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
(D/'pipeline.svg').write_text(svg+'</svg>')
s='''#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Proven-zero columns in paired transport
2026-11-02 · Derived schedule / static audit · Numerical trials: zero

*Construction.* The first nonzero column index j defines a checked zero
prefix. Record it separately: a secondary diagonal pivot can be chosen at
i greater than j. Preserve the original search and selected pivot.

#image("pipeline.svg",width:100%)

*Proof.* If A eₜ=0 and P swaps two coordinates, then
(P A P)e₍ₚ₍ₜ₎₎ = P A eₜ = 0. Thus the zero-index set follows the same
permutation as the cached column keys. Missing front columns in this set
can be materialized as zero tuples. The unchanged paired formulas still
require the established weighted-symmetry invariant.

#table(columns:(2fr,2fr),inset:5pt,
 [Derived case],[Transport queries removed],
 [Scalar pivot with j greater than 0],[1],
 [Pair pivot with j greater than 0],[2],
 [j equals 0],[0],
 [All-zero source],[No pivot move])

*Width accounting.* Each reused column removes one forward query at that
swap's input width w, or 4w+8 instrumented native products under the preserved
kernel schedule. For two pair swaps the widths are w and w+4: removed native
products total 8w+32. Output width and all outside-query field arithmetic
remain unchanged. Counts are derived; no elapsed-time comparison was run.

*Costs and provenance.* A set retains j indices and is permuted per swap.
Every reuse materializes n zero entries. Native membership, canonical-cache,
allocation and permutation work remains. New counters record these events;
they do not measure total heap or runtime. Source identity, signature, exact
record type, prefix range and one-use checks guard the internal handoff.
They do not independently certify a forged prefix or mutated source.

*Checks.* Three complete declared-substitution AST comparisons preserve the
selector, paired arithmetic and full metric-view driver. Five Python sources
parse. Frozen references remain byte-identical. Dynamic correctness,
corruption, counter, heap and performance validation remains unexecuted under
the existing cost admission gate. No job or new build was launched.

*Next construction.* Use the same zero-coordinate information to derive
coordinate deletion and primal/dual reinsertion, with a compatible displacement
representation. That larger change is not implemented or included in counts.

*Sources.* selection\_zero.py, paired\_zero.py, driver\_zero.py,
integration.json, DERIVATION.md; unchanged finite-field pivot context:
Dumas–Pernet2018, arXiv:1802.10453, introduction and Section 2.
'''
(D/'report.typ').write_text(s)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(D/f'page-{i+1}.png')
print('pages',len(doc))
