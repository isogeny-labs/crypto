"""Render the precise evidence limit of the first direct reduction."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
r = json.loads((h / 'results.json').read_text())
assert [x['status'] for x in r['runs']] == ['rejected']
assert r['fraction_polynomials_tried'] == 1 and r['positive_memberships_certified'] == 0
data = {'attempt': r['runs'][0]['attempt'], 'status': 'rejected',
        'polynomials_tried': 1, 'positive_memberships_certified': 0,
        'nonmembership_certified': 0, 'normality_certified': False}
(h / 'plot-data84.json').write_text(json.dumps(data, indent=2) + '\n')
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="800" height="160" viewBox="0 0 800 160">
<rect width="800" height="160" fill="white"/>
<rect x="20" y="54" width="190" height="54" fill="#205984"/>
<text x="115" y="87" text-anchor="middle" font-family="sans-serif" font-size="17" fill="white">Source strings mapped</text>
<text x="230" y="87" font-family="sans-serif" font-size="28" fill="#435569">→</text>
<rect x="270" y="54" width="220" height="54" fill="#b87337"/>
<text x="380" y="87" text-anchor="middle" font-family="sans-serif" font-size="17" fill="white">First direct reduction</text>
<text x="510" y="87" font-family="sans-serif" font-size="28" fill="#435569">→</text>
<rect x="550" y="54" width="230" height="54" fill="#9b4a3c"/>
<text x="665" y="87" text-anchor="middle" font-family="sans-serif" font-size="17" fill="white">Inconclusive remainder</text>
<text x="20" y="142" font-family="sans-serif" font-size="14" fill="#435569">No positive membership or nonmembership certificate was produced.</text>
</svg>'''
(h / 'inconclusive84.svg').write_text(svg + '\n')
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R84: corrected direct reduction is inconclusive
The source-polynomial ring-scope repair worked. The first denominator-cleared fraction polynomial reached reduction, but Singular warned that the saved candidate generators are not a standard basis and returned a nonzero remainder. This is not a nonmembership certificate. The strict success gate rejected the run with zero positive memberships.

#image("inconclusive84.svg", width: 100%)

The next verifier needs a coefficient witness from the normal.lib construction or a certified local basis. The accepted rank-32 RR chart offers an independent route to the proper source normalization.
'''
(h / 'report.typ').write_text(report)
typst.compile(str(h / 'report.typ'), output=str(h / 'report.pdf'))
print(json.dumps({'rejected': 1, 'positive_memberships': 0}))
