"""Render saved exact certificates; no arithmetic constructor imports."""
from pathlib import Path
from html import escape
import json,typst,fitz
D=Path(__file__).resolve().parent
packet=json.loads((D.parent.parent/'repair-certificate.json').read_text())
labels=['Saved duals isolate pairs (25,173) and (292,440)',
        'All 15 old characters agree within each pair: every degree is excluded',
        'New shift 15 separates both pairs; old shift 1 distinguishes the fibers',
        'Two-column projection is invertible; target correction is [405,0]',
        'Next solve old cubic span for X - 405 chi(X+15), then verify coefficients']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="800" height="280"><rect width="800" height="280" fill="white"/>'
for i,label in enumerate(labels):
 y=6+55*i
 svg+=f'<rect x="10" y="{y}" width="780" height="38" rx="4" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+24}" text-anchor="middle" font-family="sans-serif" font-size="13">{escape(label)}</text>'
 if i<4:svg+=f'<path d="M400 {y+38} v17 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
(D/'pipeline.svg').write_text(svg+'</svg>')
s="""#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Character fibers and targeted span repair
2026-11-03 · Derived theorem / exact checks · No performance measurement

*Stronger obstruction.* The saved F479 width-15 duals combine to give
178(e25−e173) and 178(e292−e440), each normalized against X. Independent
square-set evaluation confirms all15 old character values agree within each
pair. Thus no function of those characters can represent X: increasing degree
alone cannot repair this specific width.

#image("pipeline.svg",width:100%)

*Targeted repair.* Let U0=chi(X+15), U1=chi(X+1)chi(X+15). Pairwise
projection gives M=[[2,−2],[2,2]], determinant8, and target pairings[331,331].
The checked correction is [405,0]. Therefore X−405chi(X+15) has zero pairing
with both old difference functionals.

*Rank premise.* The preserved frozen study reports old cubic feature rank477.
The two pair functionals then span the entire left kernel, so the corrected
RHS belongs to the old cubic span. This proves existence of a representation
using old width-15 cubic features plus one new linear character. Its old-feature
coefficients have not yet been constructed. The full old rank is not replayed
here; a Gram inverse cannot be reused without establishing image equality.

#table(columns:(2fr,2fr),inset:5pt,
 [Current exact checks],[Result / scope],
 [Saved dual combinations],[All479 entries checked],
 [Old pair singleton equalities],[30, independently checked],
 [New quotient matrix],[Rank2; residual pairings0,0],
 [Local software suite],[5 methods passed, including subcases/mutations],
 [Historical rank premise],[477 from prior frozen computation])

Software cases also exercise F7 scalar/pair Schur updates and counters,
normalized witness lifts, invalid pivots and frame order, streaming dependencies,
finite caps, and corrupted witnesses/theorem records. Prior static records stay
unchanged. Root-pivot integration still lacks numerical instance validation.

*Costs.* Construction: 4q+4 character evaluations, at most q distinguishing
offset tests, O(q) storage, then ten field products, seven subtractions and one
inversion. Native pow is unmetered. The small-prime reference checker uses
O(r) square-set work. A consistent old-span solve remains the next major step.
No total-time or scalar-recovery improvement is measured.

*Custody / next.* Sources fixed at5ed039e before the bounded local unittest
run: one CPU,512MiB,30seconds,one attempt; exit0. Certificate packet separately
materialized and independently checked. No cloud job, build or budget change.
Next construct and verify old-feature coefficients for the corrected RHS,
then admit a full-cost compiler comparison with all historical controls.

*Sources.* Saved feature-span-dual certificate/rank summary; NIST DLMF27.9;
repair-certificate.json, certificate-check.json and unit-check-receipt.json.
"""
(D/'report.typ').write_text(s)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(D/f'page-{i+1}.png')
print('pages',len(doc))
