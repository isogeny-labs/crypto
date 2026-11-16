"""Render inverse-metric provenance and derived schedule; no arithmetic imports."""
from pathlib import Path
from html import escape
import typst
import fitz
D=Path(__file__).resolve().parent
labels=['Checked zero columns + nonzero diagonal metric imply zero rows',
        'Inspect prefix RHS: nonzero entry gives an immediate witness',
        'Otherwise restrict to trailing T of dimension q = n-j',
        'Restore companion wraps with two generator columns',
        'Solve the smaller system; reinsert zeros and verify at the root']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="800" height="280"><rect width="800" height="280" fill="white"/>'
for i,label in enumerate(labels):
 y=6+55*i
 svg+=f'<rect x="10" y="{y}" width="780" height="38" rx="4" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+24}" text-anchor="middle" font-family="sans-serif" font-size="14">{escape(label)}</text>'
 if i<4:svg+=f'<path d="M400 {y+38} v17 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
(D/'pipeline.svg').write_text(svg+'</svg>')
s='''#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Zero-prefix coordinate deletion
2026-11-02 · Derived identities / static integration · Numerical trials: zero

*Invariant.* With nonzero diagonal D and DA=AᵀD, checked zero columns are
also zero rows. A proper zero prefix of length j gives A=diag(0,T), where
T has dimension q=n−j and inherits the trailing metric.

#image("pipeline.svg",width:100%)

*Displacement formula.* Write S for the q-dimensional nilpotent shift,
e for its first unit vector and f for its last. The trailing block gives
ST−TS=GₜKₜᵀ. Restoring distinct companion wraps yields

G′ = [Gₜ, a e, −b T e],     K′ = [Kₜ, Tᵀ f, f].

Width grows from w to w+2 before recompression. The selector retains its first
nonzero column to supply T e, including secondary-diagonal selection. One
transpose source query supplies Tᵀ f. The identity also holds for q=1.

#table(columns:(2fr,2fr),inset:5pt,
 [Prefix RHS],[Derived branch],
 [Has nonzero entry bₜ],[Witness eₜ/bₜ; zero new source queries],
 [All entries zero],[One transpose query; reduce n to n−j],
 [No zero prefix],[Unchanged paired pivot route])

*Exact reinsertion.* Both solution and dual witness tails lift by zero
padding. Prior Schur frames then unwind normally, and the original root
checks remain. Early witnesses leave the trailing rank unknown: report a
null full rank and only the established pivot-count lower bound.

*Charged work.* Reduction uses one original-width query, q field products
and one subtraction outside it, plus generator copies and allocation.
The next iteration repeats exact compression and selection on the smaller
operator. Retaining the first column adds at most n field entries in the
secondary-diagonal branch. ZeroFrame retains only dimensions, meter and a
consumed flag; padding allocates j zeros and copies q tail entries.
No net runtime or heap improvement is measured. Prefix frequency is unmeasured.

*Checks and next.* Three declared-substitution AST audits pass; six Python
sources parse. A frame AST check excludes source/field-arithmetic calls.
Numerical, corruption, heap and performance validation is unexecuted under
the existing cost gate. Required controls include scalar tails, repeated
deletions, alternating Schur/permutation frames, both RHS outcomes, independent
displacement/lift checks and rank semantics. Next reuse the selected pivot
columns on the reduced operator to avoid repeating search.

*Sources.* DERIVATION.md, deletion.py, selection\_delete.py, driver\_delete.py,
integration.json. Standard cyclic-shift displacement conventions:
Bostan et al.2017, SIAM J. Matrix Anal. Appl.38(3), equation (2).
'''
(D/'report.typ').write_text(s)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(D/f'page-{i+1}.png')
print('pages',len(doc))
