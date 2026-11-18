#!/usr/bin/env python3
"""Document layout only, reading checked metadata. No curve arithmetic."""
from pathlib import Path
import json
s=Path(__file__).resolve().parent;v=json.loads((s/'summary.json').read_text());out=s/'reports';out.mkdir(exist_ok=True)
a,b,c,d=v['rows']
svg=f'''<svg xmlns="http://www.w3.org/2000/svg" width="960" height="275" viewBox="0 0 960 275">
<rect width="960" height="275" fill="#f6f8fc"/>
<g font-family="DejaVu Sans,sans-serif" fill="#17334c" text-anchor="middle">
<text x="480" y="27" font-size="19">Prime degree is not the prime conductor ratio</text>
<rect x="16" y="45" width="450" height="213" rx="8" fill="#e8f0f7"/>
<rect x="494" y="45" width="450" height="213" rx="8" fill="#e5f1e9"/>
<text x="241" y="75" font-size="17">Horizontal retained fixture</text>
<circle cx="110" cy="132" r="39" fill="white" stroke="#50728f"/><circle cx="373" cy="132" r="39" fill="white" stroke="#50728f"/>
<text x="110" y="138" font-size="20">f = {b['source_end_conductor']}</text><text x="373" y="138" font-size="20">f = {b['destination_end_conductor']}</text>
<path d="M154 132H329m-8-6 8 6-8 6" stroke="#50728f" fill="none" stroke-width="2"/>
<text x="241" y="116" font-size="14">degree {b['degree']}</text>
<text x="241" y="202" font-size="16">Conductor ratio {b['conductor_ratio']}</text>
<text x="241" y="231" font-size="13">Fresh point counts + rational three-torsion</text>
<text x="719" y="75" font-size="17">Descending retained fixture</text>
<circle cx="592" cy="124" r="37" fill="white" stroke="#447456"/><circle cx="849" cy="192" r="37" fill="white" stroke="#447456"/>
<text x="592" y="130" font-size="20">f = {c['source_end_conductor']}</text><text x="849" y="198" font-size="20">f = {c['destination_end_conductor']}</text>
<path d="M634 134L807 179m-6-9 6 9-10 3" stroke="#447456" fill="none" stroke-width="2"/>
<text x="732" y="133" font-size="14">degree {c['degree']}</text>
<text x="660" y="209" font-size="16">Prime ratio {c['conductor_ratio']}</text>
<text x="719" y="245" font-size="13">Actual 202-bit field; CM and distinct j</text>
</g></svg>'''
(out/'conductor.svg').write_text(svg)
rows='\n'.join(f'[{r["case"]}], [{r["degree"]}], [{r["source_end_conductor"]}], [{r["destination_end_conductor"]}], [{r["conductor_ratio"]}], [{r["direct_map_inputs_verified"]}],' for r in v['rows'])
text='''#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#set heading(numbering:none)
= Conductor audit of retained explicit maps
Native PARI/GP correctness certification; 18 November 2026.

Four fixed ordinary-curve maps are classified using their actual endpoint
orders. The largest isogeny degree in this panel does not change conductor.
The explicit map on the 202-bit target field changes conductor from one to
two. This is a bounded lower milestone, not the original minimum-80-bit
crossing or an exhaustive record survey. [pcc:unverified]

#image("conductor.svg",width:100%)

#table(columns:(2fr,1fr,0.7fr,0.7fr,0.8fr,0.8fr),inset:5pt,stroke:0.35pt,
 [*Retained fixture*],[*Degree*],[*Source f*],[*Target f*],[*Ratio*],[*Inputs*],
'''+rows+'''
)
All '''+str(v['direct_map_inputs_verified'])+''' retained map inputs match fresh direct classical Vélu sums;
'''+str(v['matched_scalars_verified'])+''' matched known scalar pairs are reverified on both endpoints.
Counts include repeated, kernel and infinity inputs. Six negative controls
reject degree-as-gap, composite degree, wrong kernel/codomain/image and a
supersingular input. Pre/post source/input/executable hashes match.
No logarithm solve, scalar timing or ECDLP performance session ran.
[pcc:unverified]

== What is actually certified
For the two small fields, both endpoint point counts are recomputed. In the
large fields, fresh counts and Frobenius conductors are deliberately absent:
the order argument uses explicit CM endomorphisms, normalized quotient scaling,
ordinary reduction and the degree2 volcano alternatives. The supplied large
ambient cardinalities are not independently recounted. Prime subgroup orders,
generators, disjoint kernels and transported target identities are checked.
[pcc:unverified]

#table(columns:(1.2fr,2.4fr),inset:4pt,stroke:0.35pt,
 [*Requirement*],[*Disposition*],
 [Four retained maps/orders],[Qualified at the labelled fixture scope; GP arithmetic plus the standard order argument],
 [At least 80-bit conductor crossing],[Open: target quotient, non-kernel images and endpoint certificates],
 [Endpoint measurements],[Open: isolated construction/transport/scalar/ECDLP/IC and admitted comparisons],
 [Full costs],[Construction, conversion, failed work, verification, memory, destination and recovery all unmeasured],
)
Producer evidence PCC-20261118-d00763 is unsigned/not covered by a signed
independent verdict; no Lean proof or official ledger promotion is claimed.
The original four maps and previous null conductor fields remain unchanged;
this adds separate certificates. No scientific job is active.
[pcc:unverified]

#pagebreak()
= Why the orders, not the degrees, decide
For ordinary curves, write Δπ = t² − 4p = fπ² DK. Then
Z[π] ⊆ End(E) ⊆ OK, and fE divides fπ. A prime-degree ell edge can change
only the ell valuation of the conductor, by at most one. Thus ell not dividing
fπ forces a horizontal edge. Geometric degree is not an evaluation cost.
[pcc:unverified]

== The degree10009 case
Fresh trace '''+b['trace']+''' gives fundamental field discriminant '''+b['D_K']+'''
and Frobenius conductor '''+b['frobenius_conductor']+'''. Hence actual conductors
are one or three, and degree10009 cannot change them. To determine the orders
exactly, set β = (π − 1)/3. Its integral trace and norm give fundamental DK,
so Z[β] = OK. The full three-torsion is rational iff β is an endomorphism:
use the separable quotient property of [3]. This happens exactly at conductor
one. The three-division polynomial and rational ordinates give '''+str(b['source_rational_three_torsion'])+'''
rational three-torsion points on each endpoint, cross-checked against group
invariant factors. Both conductors are therefore '''+b['source_end_conductor']+'''.
[pcc:unverified]

== The degree2 positive case
On y² = x³ − 140x + 784, take s² = −7 and u = (1 − s)/2. The normalized
quotient with kernel (7 + s, 0), followed by inverse scaling, is an endomorphism
of differential u. Its degree is two and u² − u + 2 = 0. Trace/norm and the
integral trace bound force trace one, hence fundamental discriminant minus
seven and a maximal source order. Splitting of the field prime gives ordinary
CM reduction. [pcc:unverified]

The audited quotient instead uses kernel (−14, 0). Its target has conductor
one or two. The class-number-one maximal order has unique j = −3375; the
target j differs. Consequently the target conductor is two. The independent
direct-map panel checks the actual rational quotient and subgroup transport.
[pcc:unverified]

== The degree5 control and preservation
The pilot small map has equal endpoint j invariants. Its explicit scaling
produces a degree5 endomorphism with absolute trace one and fundamental
discriminant minus19. Both ordinary geometric orders are maximal. This small
map is distinct from the original large pilot kernel. [pcc:unverified]

Each selected subgroup has prime order coprime to the map degree. Thus the
kernel intersection is trivial. Both generators have exact prime order,
and every frozen [k]G and [k]φ(G) is verified. There is no ECDLP eigenvalue,
GLV basis, speedup, cost ranking or timing inferred from these checks.
[pcc:unverified]

== Sources and reproducibility
Galbraith, ANTS submission, introduction and section 2:
#link("https://antsmath.org/ANTSXVI/papers/Galbraith.pdf")[ordinary order sandwich and conductor gap].
Galbraith–Gilchrist–Robert, section 2.1:
#link("https://www.math.u-bordeaux.fr/~damienrobert/pro/publications/articles/climbing_volcanoes.pdf")[prime-degree volcano levels].
The actual reviewed PDF versions/hashes are in sources.json. See DERIVATION.md
for the full kernel/CM/order argument, and qualify-v2.sh for exact native
calls into fresh output paths under the bounded correctness cap. KB unavailable;
no claim of latest literature or an exhaustive world-record search.
[pcc:unverified]

Does degree10009 imply conductor gap10009? No; both actual conductors are three.
Does this meet the requested80-bit criterion? No; the certified ratio in this
fixed panel is two, with no endpoint timings. [pcc:unverified]
'''
(out/'report.typ').write_text(text)
