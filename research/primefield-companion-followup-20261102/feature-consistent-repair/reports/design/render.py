"""Render the exact saved program/check summary; no solve or checker execution."""
from pathlib import Path
from html import escape
import json,typst,fitz
D=Path(__file__).resolve().parent
z=json.loads((D.parent.parent/'certificate/summary.json').read_text())
labels=['Known signature fibers permit 477 representative rows',
        'Factor the 477 x 576 old-feature matrix with exact integer arithmetic',
        'Solve the corrected RHS X - 405 chi(X+15); retain reusable factor',
        'Publish 476 old-feature terms plus one new linear term',
        'Independent checker evaluates the final identity at all 479 points']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="800" height="280"><rect width="800" height="280" fill="white"/>'
for i,label in enumerate(labels):
 y=6+55*i
 svg+=f'<rect x="10" y="{y}" width="780" height="38" rx="4" fill="#edf3f6" stroke="#286e89"/><text x="400" y="{y+24}" text-anchor="middle" font-family="sans-serif" font-size="13">{escape(label)}</text>'
 if i<4:svg+=f'<path d="M400 {y+38} v17 m-4 -5 l4 5 l4 -5" fill="none" stroke="#286e89"/>'
(D/'pipeline.svg').write_text(svg+'</svg>')
s="""#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Explicit corrected-span representation
2026-11-03 · Exact identity certificate · No performance comparison

*Constructed.* The missing old-feature coefficients are now explicit.
Over F479, X equals a cubic polynomial in characters at offsets0..14 plus
405chi(X+15). The program has476 nonzero old-feature terms and one new linear
term. A checker independent of the solver evaluates the identity at all479
points using square-set characters.

#image("pipeline.svg",width:100%)

*Correctness gate.* The two omitted rows173 and440 are known signature
mates of retained rows25 and292. The correction makes the target constant
within each pair. A fit on representatives alone is insufficient: the negative
control fits uncorrected X on all477 representatives but fails globally at
exactly173 and440. The repaired program passes every point.

#table(columns:(2fr,2fr),inset:5pt,
 [Saved exact result],[Count / outcome],
 [Representative rows / old features],[477 / 576],
 [Nonzero old / new terms],[476 / 1],
 [Independent identity points],[479 passed],
 [Known RHS factor-reuse checks],[2 full-field identities passed],
 [Altered programs],[2 rejected],
 [Small linear-algebra checks],[4 passed])

*Backend.* Ordinary rectangular row elimination retains swaps, inverses,
lower multipliers and upper rows for later RHS solves. Free coefficients
are zero. The observed producer rank is477; no new independent rank replay
is claimed. Global program verification establishes this representation
without relying on the old rank premise. Persisted-factor reload has not yet
been separately tested.

*Costs / custody.* Dense exact reference backend: derived factor work
O(nmt), RHS work O(nt+mt), plus feature generation and checking. No compressed
construction or timing gain is claimed. Sources fixed atc311b8a before the
single bounded local invocation: one CPU,512MiB,30seconds,8MiB output cap,
exit0. Saved output1,243,148 bytes. No cloud job, build or package install.

*Next.* Adapt the explicit program to the original point-feature evaluator
and decoder. Preserve original recovery cohorts and controls, and admit a
complete cold/reuse cost comparison before claiming savings. No unknown-scalar
recovery targets were executed in this certificate construction.

*Sources.* Saved feature-span-dual elimination; feature-fiber-repair packet;
NIST DLMF27.9; certificate/program.json, certificate/summary.json and the
construction receipt. Program and reusable factor are preserved by SHA256.
"""
(D/'report.typ').write_text(s)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(D/f'page-{i+1}.png')
print('pages',len(doc))
