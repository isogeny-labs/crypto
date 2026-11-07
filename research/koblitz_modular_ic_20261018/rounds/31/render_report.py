#!/usr/bin/env python3
"""Render retained observations only. No finite-field or research arithmetic."""
from pathlib import Path
import json, html, math
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text())
def text(x,y,t,size=17,color='#203047'):
 return f'<text x="{x}" y="{y}" font-family="DejaVu Sans,sans-serif" font-size="{size}" fill="{color}">{html.escape(str(t))}</text>'
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="710" viewBox="0 0 1000 710"><rect width="1000" height="710" fill="#f5f8fb"/>',text(28,35,'R31: explicit closed points and larger character carriers',25)]
s += [text(28,76,'Closed-point support of the integral odd projector E (no Picard-class claim)',19)]
for y,d,tw in [(120,83,0),(164,83,1),(208,284,0),(252,284,1)]:
 row=next(z for z in r['closed_point_records'] if z['kernel_field_degree']==d and z['twist']==tw)
 s += [text(28,y,f'd={d}, twist={tw}'),f'<rect x="205" y="{y-20}" width="{max(3,row["formal_support"]*22)}" height="26" fill="{"#348457" if row["formal_support"] else "#a66d49"}"/>',text(450,y,f'support {row["formal_support"]}; source degree {row["source_degree"]}')]
s += [text(28,300,'One representative per descent form. All generator choices and all factors checked.',15),text(28,344,'Exact character-cover genus (log scale; not transfer cost)',20)]
for i,z in enumerate(r['carriers']):
 y=380+i*31;w=math.log10(z['character_cover_genus'])*48
 s += [text(28,y,f'n={z["n"]}',16),f'<rect x="115" y="{y-17}" width="{w}" height="21" fill="#316da4"/>',text(505,y,f'g={z["character_cover_genus"]:,}; level {z["level"]:,}',16)]
s += [text(28,690,'Run IDs: color83/284-01, reverse83/284-01, frob83/284-01, cm31-01. Exact observations, no timing comparison.',12)]
(h/'round31.svg').write_text('\n'.join(s+['</svg>'])+'\n')
source='''#set page(paper:"a4",margin:(x:16mm,y:15mm),numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
= Koblitz cover transfers — R31
Exact observations and structural proofs. The research goal continues.

Two remaining ordinary inputs have completely enumerated middles and full
49-level transport checks. Their kernel colors are (3,0) at degree83 and
(4,0) at degree284. The zero second color supplies five rational generator
abscissas, which are now passed into the original Tate/QR incidence equations.

#image("round31.svg",width:100%)

Every generator and both descent forms satisfy all five original residuals,
with all eight registered denominators units, exact order11, ten diamond
iterates, the fifth-iterate sign, and corrupted-nu rejection. These are actual
finite-algebra point certificates. They are not a global map or an evaluated
nonzero Jacobian image.
#pagebreak()
= A surviving divisor candidate and explicit limits
On the original degree83 lane the source points have degree166. The odd
projector E=(1-T⁵)(5-sum T²ⁱ) cancels on closed points. On the diamond twist
the source points have degree83, ten distinct closed diamond translates, and
nonzero signed formal weights. Both degree284 descent forms have degree568
and cancel. Every irreducible factor of every orientation quadratic was
included. Generator labels and orientation choices give repeated candidates,
not independent divisor classes.

The new task is proper divisor-class evaluation on the degree83 diamond
twist. Nonzero formal weights do not prove nonprincipality; nonprincipality
alone would not identify the intended order11 modular factor. Retain the
actual coordinates, Frobenius classes and signed weights through that test.

== Larger character data
The existing one-prime CM construction is extended to n163,233,283,409,571,
with n5,11,83,131 as controls. Full conjugate-root-label bijections certify
the recorded Euler factors; n5/n11 also have dense ring norm controls.
Cyclic fiber cocycles provide exact genus calculations without enlarging
the old ten-million-state materialization cap. Complete materialization is
checked on the smaller controls. Connectedness has retained Bezout data.

The base permutation arrays were checked internally but not serialized as
promised prospectively. Source, cycle histograms and Bezout data remain;
this is a serialization gap and no independent replay certificate is claimed.
Explicit larger curve equations and subgroup transfer remain open.

== Proof and evidence scope
Ten named Lean structural lemmas cover cyclic translation order and periods,
product relabelling, rational Tate identities, QR coefficient changes under
the diamond twist and the sign involution. Compile/audit details are in
lean31-01. These do not formally verify the native finite-field tables,
modularity, normalization, genus geometry or Picard nonprincipality.

Seven native attempts are accepted. All mathematical outputs are native
Rust/Singular; Python only assembles text, indexes metadata and renders.
No old failed source or archive is changed. No EKS/Diem/freeze restart,
new coordination claim, merge or auto-merge occurred. Root-library validation
is not newly certified; the earlier inherited test limitations still apply.

References and precise prospective contracts: REPORT.md, PROTOCOL.md,
REVERSE31-PROTOCOL.md, FROB31-PROTOCOL.md, PROOF_MAP.md and NEXT.md.
No IC timing, rank, key recovery or attack conclusion is made.
'''
(h/'report.typ').write_text(source);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'),root=str(h))
print('Rendered round31.svg and report.pdf from retained exact-output metadata.')
