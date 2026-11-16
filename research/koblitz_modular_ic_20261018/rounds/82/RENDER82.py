"""Render the retained interruption and its exact evidence limit."""
from pathlib import Path
import hashlib
import json
import typst

h = Path(__file__).resolve().parent
r = json.loads((h / 'results.json').read_text())
assert [x['status'] for x in r['runs']] == ['interrupted']
assert r['fraction_identities_checked'] == 0
data = {'attempt': r['runs'][0]['attempt'], 'status': 'interrupted',
        'source_pins': len(r['input_pins']), 'donor_hashes': r['donor_hashes'],
        'fraction_identities_checked': 0, 'native_log_sha256': hashlib.sha256(b'').hexdigest()}
(h / 'plot-data82.json').write_text(json.dumps(data, indent=2) + '\n')
svg = '''<svg xmlns="http://www.w3.org/2000/svg" width="800" height="160" viewBox="0 0 800 160">
<rect width="800" height="160" fill="white"/>
<rect x="20" y="54" width="190" height="54" fill="#205984"/>
<text x="115" y="87" text-anchor="middle" font-family="sans-serif" font-size="17" fill="white">Frozen source + donors</text>
<text x="230" y="87" font-family="sans-serif" font-size="28" fill="#435569">→</text>
<rect x="270" y="54" width="220" height="54" fill="#b87337"/>
<text x="380" y="87" text-anchor="middle" font-family="sans-serif" font-size="17" fill="white">Global std(norid)</text>
<text x="510" y="87" font-family="sans-serif" font-size="28" fill="#435569">→</text>
<rect x="550" y="54" width="230" height="54" fill="#9b4a3c"/>
<text x="665" y="87" text-anchor="middle" font-family="sans-serif" font-size="17" fill="white">Interrupted; 0 identities</text>
<text x="20" y="142" font-family="sans-serif" font-size="14" fill="#435569">The strict marker gate rejected the attempt. No mathematical result was emitted.</text>
</svg>'''
(h / 'interruption82.svg').write_text(svg + '\n')
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R82: global fraction verifier interrupted
The frozen attempt passed the source/donor gate and entered the full 22-variable standard-basis step. The analyst interrupted it at the previously observed bottleneck. The raw Singular log is empty, the terminal marker is absent, and zero source-fraction identities were checked.

#image("interruption82.svg", width: 100%)

The retained failure motivates a fresh protocol using direct positive membership reductions without a full global standard basis. A nonzero remainder from those reductions would remain inconclusive until separately certified.
'''
(h / 'report.typ').write_text(report)
typst.compile(str(h / 'report.typ'), output=str(h / 'report.pdf'))
print(json.dumps({'interrupted': 1, 'identities_checked': 0}))
