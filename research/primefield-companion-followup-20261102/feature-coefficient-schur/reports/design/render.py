"""Render a derivation diagram, without importing arithmetic implementation."""
from pathlib import Path
from html import escape
import typst, fitz
D=Path(__file__).resolve().parent
labels=['Fixed public character basis: m = 1 + q + C(q,2) + C(q,3)',
        'Supply 1 or 2 unused points; require E = F C F^T invertible',
        'Update C and RHS coefficients u; save only F and H for lifting',
        'Supply x with C phi(x) = 0 and h(x) nonzero; seed sparse witness',
        'Lift through frames; independently verify every feature moment']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="800" height="280"><rect width="800" height="280" fill="white"/>'
for i,label in enumerate(labels):
    y=6+55*i
    svg+=f'<rect x="10" y="{y}" width="780" height="38" rx="4" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+24}" text-anchor="middle" font-family="sans-serif" font-size="14">{escape(label)}</text>'
    if i<4:svg+=f'<path d="M400 {y+38} v17 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
(D/'pipeline.svg').write_text(svg+'</svg>')
s="""#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Feature-coefficient Schur and sparse witnesses
2026-11-03 · Derived construction / static audit · Numerical trials: zero

*Closed representation.* The preserved public character features are exactly
all subsets of a q-window of size at most three. Represent the Gram residual by
C and its right-hand side by h(x) = x − phi(x)^T u. No r-row array is allocated.

#image("pipeline.svg",width:100%)

*Exact update.* For supplied pivot rows F, compute W = C F^T, E = F W and
H = W E^(-1). Then C_new = C − H W^T and u_new = u + H h_I. Reject singular
blocks. The basis stays fixed; state is immutable. This is the established
Schur identity specialized to the preserved feature family.

*Sparse certificate.* A supplied terminal point must satisfy C phi(x)=0 and
h(x) nonzero. Normalize its coefficient by 1/h(x). Each frame lifts a tail
moment mu using y_I = −H^T mu and mu_old = mu + F^T y_I. At the root, the
invertible diagonal C implies zero feature moment. A separate checker
recomputes all feature moments and the pairing sum y_x x = 1 from support.
Internal provenance tokens are not a substitute for this final check.

#table(columns:(2fr,2fr),inset:5pt,
 [Derived quantity],[Schedule / storage],
 [State coefficients],[m squared + m field entries],
 [Frame coefficients],[2mk field entries, k = 1 or 2],
 [Pivot products],[2km squared + 2k squared m + 2km],
 [Lift products],[2mk],
 [Accepted pivot coordinates],[At most min(m,r)],
 [Certificate support],[At most m+1])

Pivot counts exclude feature evaluations and small-block inversion. One point
evaluation uses qL+J products, with L the binary-power count for (r−1)/2 and
J the total label incidence. Details and additions are in DERIVATION.md.
Native validation, copies, allocations, failed proposals and final checking
are separate. Storage is O(q^6), and need not be smaller than r.

*Next dependency.* Select useful public pivot/terminal points without a field
sweep; analyze when the residual affine functional lies in the feature span.
The current routines accept supplied points. A feature certificate still needs
its explicit target-extraction connection in a complete driver.

*Evidence.* Source bytes, pattern/power AST and syntax checked statically;
no numerical, corruption, heap or performance trial. New trial execution
requires fresh admission and frozen controls. Earlier cohorts are unchanged.

*Sources.* Preserved direct-cyclic-companion/generators.py; Higham (2023),
“What Is the Schur Complement of a Matrix?”, elimination and block inverses.
No real positive-definiteness assumptions are used in this finite-field work.
"""
(D/'report.typ').write_text(s)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(D/f'page-{i+1}.png')
print('pages',len(doc))
