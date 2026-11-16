#!/usr/bin/env python3
"""Render recorded counters only; no mathematical replay."""
from pathlib import Path
import json
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
h=Path(__file__).resolve().parent
r=json.loads((h/'results.json').read_text());assert all(x['status']=='accepted' for x in r['runs']);c=r['construction_counts']
labels=['Base sections','Quadratic sections','Large-model V','Products V′','Ambient algebra A']
keys=['complete_base_sections','complete_quadratic_sections','large_model_sections','multiplication_space_dimension','closed_point_algebra_dimension']
values=[c[k] for k in keys]
fig,ax=plt.subplots(figsize=(8.2,3.4));bars=ax.barh(labels,values,color=['#35678d','#35678d','#378a72','#378a72','#8a9198']);ax.invert_yaxis();ax.set_xlim(0,850);ax.set_xlabel('Exact dimension over F₂ (not IC relation rank)');ax.bar_label(bars,padding=4);ax.set_title('R35 accepted native algebra and section-space dimensions');fig.tight_layout();fig.savefig(h/'round35.svg');plt.close(fig)
(h/'plot-data.json').write_text(json.dumps({'runs':['algebra35-01','jets35-01','multiply35-01'],'labels':labels,'values':values,'unit':'F2 dimensions','uncertainty':'not applicable: exact deterministic computations'},indent=2)+'\n')
formal=r['formal'];text=f'''#set page(paper:"a4",margin:17mm,numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
= Koblitz cover transfers — R35
*Exact construction observations; no IC performance claim*

The fixed degree-five padding point now supplies 154 complete coefficients,
a 770-dimensional algebra and exact section spaces of dimensions 68,196,324.
All 770 inverse-basis columns are recombined, with the coefficient embedding
tied to the actual source point.

#image("round35.svg",width:100%)

Run algebra35-01 verifies the coefficient lift and full forward/inverse basis.
Run jets35-01 checks all five source equations at every Newton stage, eight
units and the discriminant, all five coefficient conjugates, 68 literal
function-descent comparisons and every retained nine-term regression.

Run multiply35-01 checks all 52,650 unordered products of the 324 sections,
including all products after rank 708 is reached. Scalar field convolution
and binary bit-plane convolution agree on every product, and all 1,024 field
products are independently checked. Every product and pivot is recombined.

Run inflate35-01 constructs all ten 196-dimensional vanishing spaces in the
actual 324-dimensional ambient space, checking all 46,240 products in both
arithmetic implementations and all original/ambient/new-basis recombinations.
Completeness as L(3D0-D) uses the documented proper/local/pole bridge; exact
native rank is not itself a proof of global geometry or a target factor map.

Lean: {formal['audited_declarations']} declarations in {formal['accepted_runs']} accepted pinned runs.
Coefficient-lift identities use explicit characteristic-two, residue-field
and nilpotence assumptions. Native arrays, normalization and Riemann–Roch
are not imported as formal proofs. Every attempt is indexed in results.json.

Next: exact subspace division, flips, addition and equality, then the signed
class using certified deck order and the actual F2 factor / nonzero order-11
action. Full quotient saturation, polarized embeddings, theta, node187 and
larger arbitrary collector supports remain open. No timing comparison,
IC relation rank, scalar recovery or attack claim. Native 4GiB/1200s limits;
Lean4.34.1/mathlibd13f23b, -j1 -M4096/1200s. No independent external replay.

Reference: Khuri-Makdisi math/0409209v3, Representation B and large model;
R34 LOCAL_GEOMETRY, COMPLETE_DIVISORS and NEXT_CONSTRUCTION.
'''
(h/'report.typ').write_text(text);typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print('Rendered report.pdf and exact-dimension SVG from recorded counters.')
