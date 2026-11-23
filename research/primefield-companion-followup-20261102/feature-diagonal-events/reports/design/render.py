"""Render inverse-metric provenance and derived schedule; no arithmetic imports."""
from pathlib import Path
from html import escape
import typst
import fitz
D=Path(__file__).resolve().parent
labels=['Square character features: only their zero locations survive',
        'One constant diagonal background C; corrections supported on q points',
        'Scale by gamma^(-z): each pattern offset becomes a constant interval',
        'Two boundary events per interval; one q-entry prefix sum',
        'Diagnose root diagonal zeros without enumerating r coordinates']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="800" height="280"><rect width="800" height="280" fill="white"/>'
for i,label in enumerate(labels):
 y=6+55*i
 svg+=f'<rect x="10" y="{y}" width="780" height="38" rx="4" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+24}" text-anchor="middle" font-family="sans-serif" font-size="14">{escape(label)}</text>'
 if i<4:svg+=f'<path d="M400 {y+38} v17 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
(D/'pipeline.svg').write_text(svg+'</svg>')
s='''#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Compact diagonal of the public feature Gram
2026-11-02 · Derived identities / static checks · Numerical trials: zero

*Actual source.* The original compiler uses weighted character products with
patterns P of size 1, 2 or 3, shifts 0 through ℓ−1, where ℓ=q−max(P).
Because χ(x)² is 1 except at zero, the squared feature only records its zeros.
Distinct offsets ensure at most one zero factor per shifted feature.

#image("pipeline.svg",width:100%)

*Representation.* Let H=Σₚ wₚ(γ^ℓ−1)/(γ−1), C=w₀+H, and z=−k mod r.
Then Bₖₖ=C−Δ_z for z in 0,...,q−1, and Bₖₖ=C otherwise. Each Δ_z sums
wₚ γ^(z−d) over offsets d whose interval [d,d+ℓ) contains z. Scaling by
γ^(−z) makes each interval constant, so endpoint events recover all corrections.

#table(columns:(2fr,2fr),inset:5pt,
 [Derived build cost],[Count],
 [Pattern count M],[1+q(q−1)/2],
 [Offset incidences L],[1+2(q−1)+3(q−1)(q−2)/2],
 [Field products],[3q−1+2M+L],
 [Field additions/subtractions],[2+2M+2L+q],
 [Field inversions],[2],
 [Arithmetic workspace],[O(q), excluding input weights / meter history])

*Root consequence.* For A=D⁻¹JBJ, exceptional diagonal indices are r−1
and 0,...,q−2. If C is nonzero, all bulk diagonals are nonzero, so a root
zero-column prefix has length at most q−1. A zero diagonal is only a necessary
condition for a zero column. These statements do not certify nonsingularity.

*Weight diagnostic.* With other weights fixed, bad nonzero constant weights
are precisely the nonzero values among −H and Δ_z−H. There are at most q+1.
If q is at most r−3, some allowed nonzero weight avoids every diagonal zero.
The code returns this set only; it does not modify or select frozen weights.

*Boundary.* Schur diagonals subtract Bᵢᴵ E⁻¹ Bᴵᵢ. Closure of the compact
root representation under this update is not established. The full matrix,
its inverse and extractor have not been compressed by this diagonal identity.

*Checks / next.* Five Python sources parse; the original constructor copy and
pattern-expression AST match; static inspection finds no field-size sweep,
character exponentiation or source call in the builder. The independent direct
character checker is implemented but unexecuted. Numerical/corruption/heap/
performance trials remain zero. Next bind a root-only pivot proposal to the
constructor and retain fallback; investigate Schur diagonal updates separately.

*Sources.* DERIVATION.md, diagonal.py, independent\_check.py,
reference\_constructor.py; NIST DLMF 27.9 for character values. Original
public feature definition: direct-cyclic-companion/DERIVATION.md.
'''
(D/'report.typ').write_text(s)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(D/f'page-{i+1}.png')
print('pages',len(doc))
