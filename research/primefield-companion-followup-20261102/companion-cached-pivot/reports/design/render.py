"""Render a derived schedule; no numerical source imports."""
from pathlib import Path
from html import escape
import typst
import fitz

D = Path(__file__).resolve().parent
labels = ['Search establishes zero columns 0,...,j-1 and retains c_j',
          'j=0: identity transport, unchanged zero extra queries',
          '0<j<n-1: query only c_(j+1); derive both corrections',
          'j=n-1>0: derive both corrections from cache and zeros',
          'Continue unchanged row transport, Schur reduction and lifting']
svg = '<svg xmlns="http://www.w3.org/2000/svg" width="800" height="300"><rect width="800" height="300" fill="white"/>'
for i, label in enumerate(labels):
    y = 8 + 58*i
    svg += f'<rect x="15" y="{y}" width="770" height="40" rx="5" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+25}" text-anchor="middle" font-family="sans-serif" font-size="14">{escape(label)}</text>'
(D/'pipeline.svg').write_text(svg+'</svg>')
source = '''#set page(paper:"a4",margin:17mm)
#set text(size:10.5pt)
= Reuse pivot-search queries in column transport
2026-11-02 · Derived operation schedule · Syntax/static checks only

*Construction.* The scalar-pivot search already queries columns until the
first nonzero c_j=Ae_j. Retain that column and the established zero prefix
to avoid duplicate work in the two-query column-swap primitive.

#image("pipeline.svg",width:100%)
The three middle boxes are mutually exclusive branches. All continue to
the same row-transport and solve/witness path.

*Identity.* For j>0 and u=e_0−e_j, A u=−c_j. The second generator
correction −A Z_b u equals c_(j+1)−c_1 for an interior column and
−c_1+b c_0 for the last column. Here c_0=0, and c_1 is cached when j=1
or already known zero when j>1. Thus the interior branch needs only the
following column; the last-column branch needs no query. For n=2,j=1,
both corrections are the same cached negative column.

*Derived transport counts, excluding query internals.*
#table(columns:(2fr,1fr,1fr,1fr,1.4fr),inset:5pt,
  [Branch],[Old queries],[New queries],[New products],[New add/sub],
  [j=0],[0],[0],[0],[0],
  [j=1 interior],[2],[1],[0],[2n+1],
  [j>1 interior],[2],[1],[0],[n+1],
  [Last j>0],[2],[0],[0],[n+1])

For every old nonidentity branch, explicit scalar work was two products
and n+1 add/sub operations. The first interior branch now adds n
subtractions. Search query count and entry-test order are unchanged.
Copying the canonical wrap b eliminates multiplication by one. Generator
width remains s+2, including a zero correction when applicable.

*Integration and provenance.* A separate solve_cached.py changes only
the search return and transport call, plus one import. Normalized AST
comparison checks every other executable statement against the preserved
original solver. The internal cache is tied to the source object/signature
and is consumed once. It retains one length-n vector. In-place source
mutation and forged cache objects are outside this internal contract; the
cache is not an independently verified public certificate.

*Scope.* Nonsingular nodes have j=0, so this optimization targets singular
intermediate systems with zero-prefix columns. Query vectors differ from
the reference: native bit-length volumes and runtime must be evaluated,
not inferred from call counts. No numerical or performance experiment ran.

*Next checks.* All branch boundaries, n=2, wrap values, pivot rows,
wrong-source/reuse/malformed cache controls, exact scalar/query counters,
reference generator equality and both solve/witness outcomes. Preserve
the full original cohort; charge cached vectors and audit storage.

*Sources.* cached_transport.py, solve_cached.py, check_preparation.py and
DERIVATION.md; original companion-rank-solver column-swap identity.
Bostan et al. (2017), Section 6, DOI 10.1137/16M1062855 supplies prior
structured-solve context; no randomized or fast bound is imported.
'''
source = source.replace('_', r'\_')
(D/'report.typ').write_text(source)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,page in enumerate(doc):
    page.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(str(D/f'page-{i+1}.png'))
print('pages',len(doc))
