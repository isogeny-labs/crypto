"""Render derived border sharing and information bound; no numerical imports."""
from pathlib import Path
from html import escape
import typst
import fitz
D=Path(__file__).resolve().parent
labels=['Acquire existing row tail t and column tail c',
        'Check t = a J c exactly, charging comparison and field work',
        'Pass: retain c and mode / scale; fail: retain both tails',
        'Evaluate primal / dual dot without source queries',
        'Consume frame; preserve original-system verification']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="800" height="280"><rect width="800" height="280" fill="white"/>'
for i,label in enumerate(labels):
 y=6+55*i
 svg+=f'<rect x="10" y="{y}" width="780" height="38" rx="4" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+24}" text-anchor="middle" font-family="sans-serif" font-size="14">{escape(label)}</text>'
 if i<4:svg+=f'<path d="M400 {y+38} v17 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
(D/'pipeline.svg').write_text(svg+'</svg>')
s='''#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Shared border functionals and rank-one limits
2026-11-02 · Derived algebra / static source checks · Numerical trials: zero

*Construction.* Let q=n−1, J reverse q coordinates, and t,c be the cached
row/column tails. Check t=aJc. If it holds, both lift functionals use c:
tᵀz=a cᵀJz and cᵀv is unchanged. If it fails, retain both original tails.
No source query or inversion is introduced. Previous scalar lifts and root
solution/witness checks remain unchanged.

#image("pipeline.svg",width:100%)

#table(columns:(1.3fr,1fr,1.2fr,1.5fr),inset:4pt,
 [Mode],[Border slots],[Primal products],[Successful detection],
 [a=0, t=0],[q],[0],[q comparisons],
 [a=1, t=Jc],[q],[q],[q comparisons],
 [general a, t=aJc],[q+1],[q+1],[q products + comparisons],
 [plain fallback],[2q],[q],[charged prefix scan])

All nonzero dot branches charge q additions; general scaled primal adds one
product after the dot. The general mode is skipped at q=1 because no field
slot is saved. Failed detection retains its overhead. Input validation,
metadata, slicing and allocation are additional host work. No measured
runtime or memory benefit is assigned.

*Positive family.* Put C=Σ gᵢ Zₐⁱ. Then Ce₀=g and its first row is
(g₀,a gₙ₋₁,…,a g₁). Since C commutes with Zₐ,
ZₐC−CZᵦ=(a−b)g eₙ₋₁ᵀ, of rank one for g≠0 and a≠b. This supplies the
shared-border identity. Equality at one node need not survive Schur descent.

*Information bound.* Fix a=1,b=2,g₀=1. There are p^q arbitrary tails c,
all with displacement rank one and nonzero first pivot. Basis queries
distinguish their dot functionals. An exact self-contained encoding answering
all such queries needs at least q log₂(p) bits in the worst case. L field
slots and M metadata states require p^L M≥p^q. External retained data or
recomputation are outside this self-contained model and must be charged.

*Recurrence consequence.* Order-d constant-coefficient recurrences with d
initial values have at most p^(2d) encodings; they cannot cover this family
when 2d is less than q. Small displacement width alone supplies no short border recurrence.
This statement concerns the specified operator/query model, not a lower bound
for elliptic discrete logarithms or a particular elliptic feature family.

*Checks and next work.* Separate frame and solver normalized-AST audits;
independent coefficient checker implemented, unexecuted. No numerical,
mutation, heap or performance trial. Validate all four modes, mismatch
positions, q=1, both directions and counters under the full upstream cohort.
Next seek a recurrence from the actual public feature map and prove its
behavior under pivoting and Schur reduction, or price replay/recomputation.

*Sources.* DERIVATION.md, pair.py, shared\_frame.py, solve\_shared.py,
check\_pair.py and check\_preparation.py. Bostan et al. (2017), equation (2)
and Section 6, DOI 10.1137/16M1062855, fixes the shift/Schur conventions.
'''
(D/'report.typ').write_text(s)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(D/f'page-{i+1}.png')
print('pages',len(doc))
