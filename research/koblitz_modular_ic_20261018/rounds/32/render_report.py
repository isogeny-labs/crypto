#!/usr/bin/env python3
"""Render retained counts/ranks, without research arithmetic."""
from pathlib import Path
import json,html,typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text())
def text(x,y,s,size=18):
 return f'<text x="{x}" y="{y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="#203047">{html.escape(str(s))}</text>'
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="390" viewBox="0 0 1000 390"><rect width="1000" height="390" fill="#f5f8fb"/>',text(28,36,'R32: local incidence rank at every candidate branch',25)]
for y,label in [(90,'degree83, untwisted'),(143,'degree83, diamond twist'),(196,'degree284, untwisted'),(249,'degree284, diamond twist')]:
 s += [text(28,y,label),f'<rect x="330" y="{y-22}" width="225" height="29" fill="#316da4"/>',text(575,y,'rank3; five point algebras',17)]
s += [text(28,306,'Per algebra: all five4x4 minors vanish; forty3x3 minors have an exact unit witness.',17),text(28,342,'Two accepted native runs, one rejected recorder attempt, two pre-CAS refusals retained.',16),text(28,375,'Run IDs: jac83-03 and jac284-02. Tangent diagnostic only; normalization and Picard images remain open.',13)]
(h/'round32.svg').write_text('\n'.join(s+['</svg>'])+'\n')
source='''#set page(paper:"a4",margin:16mm,numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
= Koblitz cover transfers — R32
*Local incidence tangent certificates, with failures retained*

This continuation tests a prerequisite for using R31's surviving degree83
diamond-twist divisors. It differentiates the original five equations in four
variables, then evaluates the complete Jacobian at every retained quadratic
point algebra. No earlier kernel search or point passage is repeated.

#image("round32.svg",width:100%)

At every geometric branch the matrix has rank3, hence the presented tangent
space has dimension1. Complete evaluated matrices and exact unit-ideal lift
witnesses are retained; no factor or denominator is discarded. The zero-minor
family stays proper as a negative control. This diagnostic does not establish
the scheme's local dimension, normalization or proper polarized carrier.

== Failed attempts and exact correction
Two initial invocations were refused before CAS because the resume handoff
lacked required headings. Restoring those headings changed no math or gate.
The first native recorder then failed its explicit witness reconstruction.
Its scalar loop used26 nonzero ideal generators to index a vector with42
slots, truncating coordinates when minors vanished. The runner rejected it
even though Singular returned0 and later printed a terminal marker.

The corrected native matrix product preserves all slots and must equal1
exactly. Both fields then pass. The failed input/log and both pretrial
refusals remain in the raw bundle; no failed run counts as accepted evidence.

== Formal scope and continuation
Two Lean lemmas prove generic unit-identity specialization and a nonzero
minor on every branch. Their assumptions are exact unit identities; native
matrices are not imported into Lean and the local-dimension hypothesis is
not formalized. Pinned Lean4.34.1/mathlibd13f23b and a separate transitive axiom
audit are retained. No proof-assistant geometry claim is made.

Next: establish the required local dimension/standard-smooth presentation,
bind the proper normalization, and evaluate the degree83 twist divisors in
the Jacobian and intended order11 factor. Nonzero formal divisor remains
distinct from nonprincipal class and nonzero modular-factor image.

Reference checked before code: Stacks29.35, Lemma29.35.14
(https://stacks.math.columbia.edu/tag/01V4). Reports are observations only.
No IC timing/rank, attack, global normalization or independent science verdict.
'''
(h/'report.typ').write_text(source);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'),root=str(h))
print('Rendered round32.svg and report.pdf from retained observations.')
