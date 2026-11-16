"""Render the complete candidate reduced-fiber census."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
r = json.loads((h / 'results.json').read_text())
assert [x['status'] for x in r['runs']] == ['accepted', 'accepted']
assert r['candidate_reduced_dimensions'] == [4, 4]
assert r['candidate_prime_degrees'] == [[2, 2], [2, 2]]
assert not r['normality_certified']
data = {'fibers': [{'place': f'y={i}', 'full_dimension': 16,
                    'reduced_dimension': 4, 'prime_degrees': [2, 2]}
                   for i in (0, 1)], 'normality_certified': False}
(h / 'plot-data80.json').write_text(json.dumps(data, indent=2) + '\n')
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="800" height="320" viewBox="0 0 800 320">
<rect width="800" height="320" fill="white"/>
<text x="18" y="31" font-family="sans-serif" font-size="21" fill="#17384c">Rejected R50 candidate: both reduced fibers</text>
<text x="20" y="92" font-family="sans-serif" font-size="18" fill="#17384c">y=0</text>
<rect x="94" y="66" width="160" height="35" fill="#205984"/><rect x="254" y="66" width="160" height="35" fill="#4488ad"/>
<text x="430" y="91" font-family="sans-serif" font-size="16" fill="#17384c">F4 + F4; full length 16</text>
<text x="20" y="164" font-family="sans-serif" font-size="18" fill="#17384c">y=1</text>
<rect x="94" y="138" width="160" height="35" fill="#9b5f2c"/><rect x="254" y="138" width="160" height="35" fill="#c98c51"/>
<text x="430" y="163" font-family="sans-serif" font-size="16" fill="#17384c">F4 + F4; full length 16</text>
<text x="18" y="222" font-family="sans-serif" font-size="15" fill="#435569">Each colored block is one candidate degree-2 prime quotient.</text>
<text x="18" y="250" font-family="sans-serif" font-size="15" fill="#435569">The reduced dimension is 4 at each place; nilpotent length accounts for the rest.</text>
<text x="18" y="278" font-family="sans-serif" font-size="15" fill="#435569">Normality and source-field compatibility remain unproved.</text>
</svg>'''
(h / 'candidate80.svg').write_text(svg + '\n')
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R80: reduced fibers of the rejected R50 normalization candidate
Both $u=0$ candidate quotients have dimension 16 over F2. On each component the radical quotient has dimension 4 and exactly two degree-2 prime quotients. Their degree sum is exhaustive: $2+2=4$ on each component.

#image("candidate80.svg", width: 100%)

The two serial frozen programs passed exit, exact-marker, error, source-byte, and donor checks. These are candidate-algebra factors. The R50 candidate failed its earlier coefficient-descent gate, and neither normality nor source-field compatibility is certified. The next check must compare the four proposed factors with an independent local integral closure.
'''
(h / 'report.typ').write_text(report)
typst.compile(str(h / 'report.typ'), output=str(h / 'report.pdf'))
print(json.dumps({'accepted': 2, 'candidate_prime_degrees': [[2, 2], [2, 2]]}))
