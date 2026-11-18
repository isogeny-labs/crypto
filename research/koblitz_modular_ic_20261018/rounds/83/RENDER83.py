"""Render the retained interruption and its exact evidence limit."""
from pathlib import Path
import hashlib
import json
import typst

h = Path(__file__).resolve().parent
r = json.loads((h / 'results.json').read_text())
assert [x['status'] for x in r['runs']] == ['rejected']
assert r['fraction_identities_checked'] == 0
data = {'attempt': r['runs'][0]['attempt'], 'status': 'rejected',
        'source_pins': len(r['input_pins']), 'donor_hashes': r['donor_hashes'],
        'fraction_identities_checked': 0, 'native_log_sha256': hashlib.sha256(b'').hexdigest()}
(h / 'plot-data83.json').write_text(json.dumps(data, indent=2) + '\n')
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="800" height="160" viewBox="0 0 800 160">
<rect width="800" height="160" fill="white"/>
<rect x="20" y="54" width="190" height="54" fill="#205984"/>
<text x="115" y="87" text-anchor="middle" font-family="sans-serif" font-size="17" fill="white">Frozen source + donors</text>
<text x="230" y="87" font-family="sans-serif" font-size="28" fill="#435569">→</text>
<rect x="270" y="54" width="220" height="54" fill="#b87337"/>
<text x="380" y="87" text-anchor="middle" font-family="sans-serif" font-size="17" fill="white">Ring-scope error</text>
<text x="510" y="87" font-family="sans-serif" font-size="28" fill="#435569">→</text>
<rect x="550" y="54" width="230" height="54" fill="#9b4a3c"/>
<text x="665" y="87" text-anchor="middle" font-family="sans-serif" font-size="17" fill="white">Rejected; 0 identities</text>
<text x="20" y="142" font-family="sans-serif" font-size="14" fill="#435569">The strict native-error gate rejected the attempt. No mathematical result was emitted.</text>
</svg>'''
(h / 'failure83.svg').write_text(svg + '\n')
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R83: direct fraction verifier rejected
The frozen attempt passed its source/donor gate, but Singular could not access the source module after switching rings. It printed a terminal marker despite native errors, so the strict gate rejected the run. Zero valid fraction identities were checked.

#image("failure83.svg", width: 100%)

The correction serializes source module polynomials as strings before switching rings; it will run under a fresh frozen protocol.
'''
(h / 'report.typ').write_text(report)
typst.compile(str(h / 'report.typ'), output=str(h / 'report.pdf'))
print(json.dumps({'rejected': 1, 'identities_checked': 0}))
