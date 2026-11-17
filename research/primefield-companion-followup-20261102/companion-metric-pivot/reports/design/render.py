"""Render weighted-symmetry primitive derivation; no numerical execution."""
from pathlib import Path
from html import escape
import typst
import fitz
D=Path(__file__).resolve().parent
labels=['Public weighted Gram compiler: D A is symmetric',
        'Find nonzero column; select scalar or 2x2 antidiagonal pivot',
        'Apply the same row / column swaps; permute diagonal metric',
        'Explicit small inverse; reconstruct rows from cached columns',
        'Principal Schur complement inherits the trailing metric']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="800" height="280"><rect width="800" height="280" fill="white"/>'
for i,label in enumerate(labels):
 y=6+55*i
 svg+=f'<rect x="10" y="{y}" width="780" height="38" rx="4" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+24}" text-anchor="middle" font-family="sans-serif" font-size="14">{escape(label)}</text>'
 if i<4:svg+=f'<path d="M400 {y+38} v17 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
(D/'pipeline.svg').write_text(svg+'</svg>')
s='''#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Weighted-symmetry pivot primitives
2026-11-02 · Derived algebra / syntax checks · Numerical trials: zero

*Actual compiler.* Its weighted Gram matrix B is symmetric and
A=D⁻¹JBJ, D=diag(1,γ,…). Thus DA is symmetric. The source constructor
already returns the inverse diagonal. Finite-field weights need not be positive.

#image("pipeline.svg",width:100%)

*Complete local pivot rule.* Find the first nonzero column j. Use its diagonal
if nonzero. Otherwise choose i with Aᵢⱼ≠0 and query column i. The metric gives
dᵢAᵢⱼ=dⱼAⱼᵢ, so the reverse entry is nonzero. Use diagonal i if nonzero;
otherwise the principal pair is (0,u;v,0), with determinant −uv≠0 and inverse
(0,1/v;1/u,0). This costs at most one extra column query. All-zero columns
certify a zero source, with RHS compatibility still to be decided.

*Closure.* Simultaneous permutations give A′=PAPᵀ and D′=PDPᵀ. Partition
A′=(E,F;C,H), D′=diag(D₁,D₂). For S=H−CE⁻¹F,
D₂S=D₂H−FᵀD₁E⁻¹F is symmetric. The child metric is the trailing portion
of D′. It need not remain a geometric progression. Arbitrary separate row
and column swaps do not supply this invariant.

*Border reuse.* A cached column cⱼ gives row entry
A′ⱼᵢ=dⱼ⁻¹ dᵢ cⱼ(i). The helper reconstructs leading rows with zero source
queries. It explicitly allocates rows; no complete memory reduction is claimed.

#table(columns:(1.7fr,2.5fr),inset:4pt,
 [Primitive],[Derived accounting],
 [Build metric / inverse],[1 inversion + 2(n−1) products],
 [Selection metric validation],[n products],
 [Selection queries],[j+1 or j+2; n on zero source],
 [Reciprocal-entry guard],[2 products on offdiagonal branch],
 [Each simultaneous swap],[4 source queries; width +4; 4 products; 2(n+1) additions],
 [Small inverse setup / apply],[cut inversions / cut products],
 [Reconstruct cut rows],[n+2n·cut products including validation; no query])

*Evidence scope.* Syntax and frozen dependency hashes checked; no numerical,
corruption-control or performance trial. Metric inverse checks and the local
reciprocal guard do not certify global symmetry. The invariant must come from
the public compiler or independent source certification. Cached selection has
internal one-use provenance. Existing frozen callers are unchanged.

*Next implementation.* Complete the symmetry-preserving solve/witness driver,
shared weighted block borders and compact block lifts. Preserve zero/singular
outcomes and separate feature-exclusion checks. Charge transport queries,
changing widths, metric storage and indices. Validate both scalar branches,
2×2 pivots, all permutations, inherited Schur metrics and all upstream cohorts
when cost admission permits. No numerical run was launched.

*Sources.* metric.py, selection.py, transport.py, check\_preparation.py and
DERIVATION.md. Dumas–Pernet (2018), arXiv:1802.10453, introduction/Section 2,
establishes exact-field 1×1/2×2 factorization context. Bostan et al. (2017),
DOI 10.1137/16M1062855, provides companion/Schur context. No rank-profile or
fast recursive complexity claim is inherited.
'''
(D/'report.typ').write_text(s)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(D/f'page-{i+1}.png')
print('pages',len(doc))
