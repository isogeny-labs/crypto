"""Render the derived construction, without importing numerical kernels."""
from pathlib import Path
from html import escape
import typst
import fitz

D = Path(__file__).resolve().parent
labels = [
    'Input: G, K of size n by s over the same prime field',
    'Certified rank factor: G = U R, with rank g',
    'Skinny product: H = K R^T, size n by g',
    'Certified rank factor: H = V S, with rank t',
    'Output: G_new = U S^T, K_new = V; minimal width t',
]
svg = '<svg xmlns="http://www.w3.org/2000/svg" width="800" height="300"><rect width="800" height="300" fill="white"/>'
for i, label in enumerate(labels):
    y = 8 + 58*i
    svg += f'<rect x="15" y="{y}" width="770" height="40" rx="5" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+25}" text-anchor="middle" font-family="sans-serif" font-size="15">{escape(label)}</text>'
    if i < 4:
        svg += f'<path d="M400 {y+40} v18 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
(D/'pipeline.svg').write_text(svg + '</svg>')
source = '''#set page(paper:"a4",margin:17mm)
#set text(size:10.5pt)
= Exact two-sided generator compression
2026-11-02 · Derived construction and syntax-checked source · Zero numerical trials

*Purpose.* Remove redundant columns introduced by pivot and Schur boundary
corrections while preserving the same displacement matrix and prime field.
The construction uses no source-operator query and never forms the full
displacement matrix. It is a classical compression method with explicit
certificates and accounting for the current companion pipeline.

#image("pipeline.svg",width:100%)

*Preservation and minimality.* From G=UR and H=KRᵀ=VS, the returned
pair satisfies (USᵀ)Vᵀ=URKᵀ=GKᵀ. Since U has full column rank,
rank(GKᵀ)=rank(H)=t. Every width-w factor pair has product rank at most w;
therefore t is minimal. This includes cancellations between nonzero factors.
No estimate that t stays small after repeated pivots is assumed.

*Certificate.* Deterministic column elimination retains basis columns,
coordinates, pivot rows and original pivot columns. The selected basis-row
minor is unit lower triangular; the coordinate-column minor is upper
triangular with nonzero diagonal. The independent checker validates both
factor identities, both minor pairs, the intermediate product and outputs.
It imports no producer code. The field primality assumption is caller supplied.

*Derived arithmetic.* Let g=rank(G), t=rank(H), and qG,qH be sums of ranks
before each processed column in the two factorizations. With the existing
Meter convention (including initial-zero dot additions), the producer uses:

```text
products:   n (qG + g + qH + t + s g + g t)
add/sub:    n (qG + qH + s g + g t)
inversions: g + t
queries:    0
```

These are derived schedules, not measured counts. Pivot entry tests are
separate. Setup is classical O(n s²); certificates and copies are additional
storage. A smaller returned width saves work only if later query savings
exceed setup and any required verification cost, in the same counted unit.

*Zero rank and scope.* Width-zero factors are mathematically valid. An
explicit adapter supplies one zero column to legacy positive-width kernels;
encoding width one does not change certified rank zero. No frozen caller is
modified. Compression preserves displacement, not a pivot success guarantee.

*Validation next.* Full rank range, width greater than dimension, zero and
cancelling factors, nonmonotone pivot rows, reconstruction in both directions,
minor and identity mutations, exact counters and unchanged-width controls.
No execution under the currently exhausted daily budget.

*Sources.* compress.py, certificate.py and DERIVATION.md. Bostan et al.
(2017), DOI 10.1137/16M1062855, introduction, printed p. 756 and Section 6,
establishes the structured-matrix context and prior generator compression.
No novelty claim or fast-matrix-multiplication complexity is imported.
'''
(D/'report.typ').write_text(source)
typst.compile(str(D/'report.typ'), output=str(D/'report.pdf'))
doc = fitz.open(D/'report.pdf')
for i, page in enumerate(doc):
    page.get_pixmap(matrix=fitz.Matrix(1.3, 1.3)).save(str(D/f'page-{i+1}.png'))
print('pages', len(doc))
