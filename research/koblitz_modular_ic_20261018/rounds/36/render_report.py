#!/usr/bin/env python3
"""Render receipt counters only; no mathematical replay."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text());c=r['class_tests']
labels=['Self equality','Source vs zero','Negative vs zero','Sum with inverse vs zero'];keys=['self_equality_dimension','source_versus_zero_dimension','negative_versus_zero_dimension','add_to_inverse_versus_zero_dimension'];values=[c[k] for k in keys]
fig,ax=plt.subplots(figsize=(8,3.2));ax.scatter(values,range(4),s=100,color='#31756a');ax.set_yticks(range(4),labels);ax.invert_yaxis();ax.set_xticks([0,1]);ax.set_xlim(-.15,1.15);ax.set_xlabel('Exact equality-witness dimension over F₂');ax.set_title('group36-01: actual class-arithmetic controls');ax.grid(axis='x',alpha=.25);fig.tight_layout();fig.savefig(h/'round36.svg');plt.close(fig)
(h/'plot-data.json').write_text(json.dumps({'run':'group36-01','labels':labels,'values':values,'unit':'F2 dimensions','uncertainty':'not applicable: exact deterministic computation'},indent=2)+'\n')
text='''#set page(paper:"a4",margin:17mm,numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
= Koblitz cover transfers — R36
*Exact class-operation observations; no IC performance claim*

Two accepted frozen native runs implement exact subspace division with all
original generators in the 770-dimensional closed-point algebra. They use
the actual 196-dimensional divisor space inside V of dimension324.

#image("round36.svg",width:100%)

quotient36-01 verifies the zero control, a 68-dimensional flip using a section
that vanishes on9Q, and exact recovery of the original196-space by flipping
back with the same section. The section is not inverted in the local algebra.
An actual nonmember of the multiplication target is found and rejected.

group36-01 tests self equality (witness dimension1), source against zero (0),
constructs its negative, checks the negative against zero (0), and verifies
that Addflip(source,negative) is equivalent to zero (1). Thus this one actual
source class has direct native class-arithmetic nonprincipality evidence.

All12subspace divisions retain residual maps, kernel coordinates, reconstructed
candidates and rank-nullity checks. All construction products and final
memberships against every original generator are checked with two different
arithmetic implementations. No uncertified deflation or random IGS is used.

The geometric interpretation uses the retained proper/local/pole and complete
Riemann–Roch bridge. Native arrays and these group algorithms are not formally
verified in Lean or independently replayed externally. R35 coefficient-lift
Lean lemmas are separate. Resource limits remain4GiBvirtual/1200s per native
invocation; no timing comparison, IC relation rank or scalar recovery.

Next: the actual signed combination in certified deck order and its intended
F2 modular-factor / nonzero order11 action. Full quotient saturation, proper
polarized embedding, theta and arbitrary/larger collector supports remain.

Reference: Khuri-Makdisi math/0409209v3, algorithms3.10,3.14,3.16 and remark3.17.
See REPORT.md, prospective protocols and retained exact receipt/certificates.
'''
(h/'report.typ').write_text(text);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'));print('Rendered R36 report and equality-control plot.')
