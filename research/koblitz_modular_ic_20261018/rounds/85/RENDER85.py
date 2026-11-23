"""Render the exact base-overlap certificate and remaining upper map."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
r = json.loads((h / 'results.json').read_text())
assert [x['status'] for x in r['runs']] == ['accepted', 'accepted']
assert r['runs'][1]['theorems'] == 3 and r['runs'][1]['failing'] == 0
data = {'native': {'open_products': 2, 'transformed_equation_full_lift': True,
                   'corruption_rejected': True},
        'lean': {'theorems': 3, 'failing': 0, 'trusted': 0},
        'upper_rank32_transport_certified': False}
(h / 'plot-data85.json').write_text(json.dumps(data, indent=2) + '\n')
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="800" height="250" viewBox="0 0 800 250">
<rect width="800" height="250" fill="white"/>
<text x="18" y="29" font-family="sans-serif" font-size="21" fill="#17384c">Diamond overlap on the quadratic source base</text>
<rect x="20" y="62" width="285" height="102" fill="#205984"/>
<text x="34" y="88" font-family="sans-serif" font-size="17" fill="white">U0: E(z,V)=0</text>
<text x="34" y="116" font-family="sans-serif" font-size="15" fill="white">h = z+V</text>
<text x="34" y="143" font-family="sans-serif" font-size="15" fill="white">h² + zH = E</text>
<text x="330" y="111" font-family="sans-serif" font-size="32" fill="#435569">↔</text>
<rect x="390" y="62" width="390" height="102" fill="#b87337"/>
<text x="404" y="88" font-family="sans-serif" font-size="17" fill="white">U1: z1=1/h, V1=N/h²</text>
<text x="404" y="116" font-family="sans-serif" font-size="15" fill="white">h1=z1+V1+z1²=H/h²</text>
<text x="404" y="143" font-family="sans-serif" font-size="15" fill="white">On E=0, h1=1/z</text>
<text x="20" y="205" font-family="sans-serif" font-size="15" fill="#435569">Native: full equation lift and corruption control. Lean: 3 audited identities.</text>
<text x="20" y="231" font-family="sans-serif" font-size="15" fill="#435569">Next: transport all 32 upper-field RR generators in both directions.</text>
</svg>'''
(h / 'diamond85.svg').write_text(svg + '\n')
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R85: exact quadratic-base diamond overlap
The frozen native program checks both principal-open polynomial products, a full lift of the denominator-cleared transformed source equation into $(E)$, and rejection of a corrupted numerator. On the overlap where $E=0$ and $z h$ is invertible, these identities give $h_1=1/z$ and the base-coordinate round trip.

#image("diamond85.svg", width: 100%)

The pinned Lean file compiled three characteristic-two identities and its axiom audit found zero failing and zero trusted theorems. This result fixes the base transition for the accepted R50 rank-32 RR algebra. The next check must transport all 32 upper-field generators and prove regularity in both directions.
'''
(h / 'report.typ').write_text(report)
typst.compile(str(h / 'report.typ'), output=str(h / 'report.pdf'))
print(json.dumps({'native_accepted': 1, 'lean_accepted': 1, 'upper_transport_certified': False}))
