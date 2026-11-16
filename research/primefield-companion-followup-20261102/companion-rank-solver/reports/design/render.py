"""Render derived solve/witness route; imports no numerical component."""
from pathlib import Path
from html import escape
import typst
import fitz

D = Path(__file__).resolve().parent
labels = [
    'Compress generators exactly in the same prime field',
    'Zero source: b=0 gives x=0; otherwise w=e_i/b_i',
    'Nonzero source: query columns and move a scalar pivot to (0,0)',
    'Invert that scalar; construct Schur generators and reduced RHS',
    'Repeat, then lift through frames and undo the correct permutations',
    'Check A x=b or A^T w=0 and w^T b=1 on the original source',
]
svg = '<svg xmlns="http://www.w3.org/2000/svg" width="800" height="355"><rect width="800" height="355" fill="white"/>'
for i, label in enumerate(labels):
    y = 8 + 58*i
    svg += f'<rect x="15" y="{y}" width="770" height="40" rx="5" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+25}" text-anchor="middle" font-family="sans-serif" font-size="14">{escape(label)}</text>'
    if i < 5:
        svg += f'<path d="M400 {y+40} v18 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
(D/'pipeline.svg').write_text(svg+'</svg>')
source = '''#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Structured solve or normalized witness
2026-11-02 · Derived implementation; syntax checked · Zero numerical trials

*Construction.* For a square companion displacement Z_a A−A Z_b=GKᵀ
over a caller-certified prime field, with a≠b, combine exact compression,
deterministic scalar pivots and Schur lifts. No supplied matrix solve is used:
every leading solve multiplies by an explicitly inverted nonzero scalar.

#image("pipeline.svg",width:100%)

The zero-source branch terminates at its level and proceeds to unwinding;
only the nonzero branch continues through pivoting and Schur reduction.

*Column transport.* For u=e_i−e_j and Q=I−uuᵀ, represent AQ by

```text
Gc = [G  | A u | -A Z_b u]
Kc = [QK | Z_b^T u | u]
```

Two forward queries add two generator columns. Existing row transport adds
two transpose queries when nontrivial. For B=PAQ, solutions map as x=Qy;
left-null witnesses map as w=Pᵀv. RHS maps as Pb. Confusing these two
permutation paths would invalidate certificates.

*Proof.* Distinct wraps make the square Sylvester operator invertible, so
zero displacement means A=0. A nonzero matrix has a nonzero queried entry.
After transport, that entry is an invertible scalar E. Block elimination
gives rank(A)=1+rank(S), and Schur residual and pairing identities transport
either answer. Each step reduces dimension by one. Control flow uses an
explicit frame stack, not recursive Python calls. The final root equation
is checked with a charged original-source application.

*Charged schedule.* A node searches j+1 columns through its first nonzero
column j. With compressed width t and nontrivial column/row swaps c,p,
transported width w=t+2c+2p. Schur setup uses 2w+6 source queries and
2w+4 scalar solves. RHS reduction and unwinding each add one query and one
scalar solve. Compression setup and one inversion per pivot are additional.
The scalar terminal omits Schur work. Root witness verification also charges
a dot product. All field arithmetic uses the existing Meter.

*Scope and next checks.* Singular compatible systems may return zero on free
coordinates. An inconsistent system yields Aᵀw=0 and wᵀb=1; feature
annihilation Fᵀw=0 remains separate and explicitly unchecked. Derived rank
is not independently certified at runtime. Frames and compression traces
can require substantial aggregate storage; scalar depth and width growth
do not inherit balanced-inverse bounds. No speed measurement is claimed.

Next freeze full pivot, singular-system, both-transport-direction, corruption,
rank-reference and exact-accounting checks, preserving upstream cohorts.
Numerical execution remains behind the recorded cost gate.

*Sources.* solve.py and DERIVATION.md; local pivot, compression, recursive
and Schur-lift primitives. Bostan et al. (2017), Section 6, printed pp. 764–765,
DOI 10.1137/16M1062855: established structured rank-profile/solve context.
Its randomized preconditioning and fast complexity are not assumed here.
'''
(D/'report.typ').write_text(source)
typst.compile(str(D/'report.typ'), output=str(D/'report.pdf'))
doc = fitz.open(D/'report.pdf')
for i, page in enumerate(doc):
    page.get_pixmap(matrix=fitz.Matrix(1.3, 1.3)).save(str(D/f'page-{i+1}.png'))
print('pages', len(doc))
