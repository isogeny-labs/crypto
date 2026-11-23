"""Render inverse-metric provenance and derived schedule; no arithmetic imports."""
from pathlib import Path
from html import escape
import typst
import fitz
D=Path(__file__).resolve().parent
labels=['Freeze one public input tuple; construct source and compact diagonal',
        'Match root generators; independently check compression identities',
        'Propose a root scalar from the compact diagonal, or use fallback',
        'Query the column and check its predicted nonzero diagonal entry',
        'Consume root binding; retain ordinary Schur selection and root checks']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="800" height="280"><rect width="800" height="280" fill="white"/>'
for i,label in enumerate(labels):
 y=6+55*i
 svg+=f'<rect x="10" y="{y}" width="780" height="38" rx="4" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+24}" text-anchor="middle" font-family="sans-serif" font-size="14">{escape(label)}</text>'
 if i<4:svg+=f'<path d="M400 {y+38} v17 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
(D/'pipeline.svg').write_text(svg+'</svg>')
s='''#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Constructor-bound root diagonal pivots
2026-11-02 · Derived implementation / static checks · Numerical trials: zero

*Construction.* One immutable input-weight tuple feeds the preserved public
source constructor and compact diagonal builder. A binding captures the root
source, generators, metric and field signature. Its lifetime ends at the root.

#image("pipeline.svg",width:100%)

*Equality and entry checks.* Before rebinding across compression, verify the
independent factor certificate. Match the actual new generators to that checked
output and require the original metric coordinates. Query the proposed column;
its diagonal must match the predicted value and be nonzero. Otherwise fail.
This is an implemented runtime gate, not an executed numerical result.

*No false prefix.* RootSelection is a distinct record with an empty zero
prefix and first_column=None. It carries no knowledge of skipped columns.
The paired wrapper changes only the admitted record type. If all compact
diagonals vanish, invoke the ordinary selector. Never reuse the root packet
at a Schur child or after a coordinate permutation.

#table(columns:(2fr,2fr),inset:5pt,
 [Derived root proposal],[Cost / behavior],
 [Nonzero diagonal found],[One full source query + one prediction product],
 [Compact scan],[At most q subtractions and q+1 nonzero tests],
 [All diagonals zero],[Original selector, with proposal overhead],
 [Root compression check],[2ng(s+t) logical products; n(s+2g+t) reductions],
 [Snapshots],[2ns then 2nt field entries, plus transient copies])

Original compiler, compact-diagonal setup, input copies and r−1 forward-metric
products remain charged. Checker integers, Meter field work and native
comparisons are separate units. No runtime or total-heap improvement is measured.

*Changed pivots.* A later scalar may replace the original selector's pair,
changing the downstream pivot tree. Validate final root equations and normalized
witness pairing, not byte equality of nonunique vectors or identical pivots.
Early-witness rank semantics and separate feature-annihilation checks remain.

*Verification / next.* Two complete static AST comparisons cover driver and
paired transport integration; six Python files parse. Defaults and gate ordering
are statically checked. Numerical/corruption/heap/performance trials remain zero
under the existing cost gate. Funded controls include altered generators,
certificates, weights, fields, views and predictions; fallback and zero source;
both outcomes, counters and lifetime. Next derive charged Schur diagonal updates
from accepted pivot columns, with explicit metric and storage accounting.

*Sources.* DERIVATION.md, root\_binding.py, driver\_root.py, root\_move.py,
public\_solve.py and integration.json. Established scalar/pair pivot context:
Dumas–Pernet2018, arXiv:1802.10453, introduction and Section 2.
'''
(D/'report.typ').write_text(s)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(D/f'page-{i+1}.png')
print('pages',len(doc))
