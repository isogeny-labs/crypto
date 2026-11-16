#!/usr/bin/env python3
"""Audit source binding and completeness of the 24 finite-search receipts.

This verifies run custody and counters; Sage elimination is in search_all.sage.
"""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
F_MASKS = (32280, 35349, 36380, 51009, 67186, 106121,
           112490, 113379, 127268, 168265, 175537, 175771)
B_MASKS = (68, 84)
source_hash = hashlib.sha256((HERE / 'search_all.sage').read_bytes()).hexdigest()
runner_hash = hashlib.sha256((HERE / 'run_all.py').read_bytes()).hexdigest()
manifest = json.loads((HERE / 'manifest.json').read_text())
assert manifest['schema'] == 'ghs.trace22.degree8-quadratic-all-manifest/v1'
assert manifest['status'] == 'complete'
assert manifest['source_sha256'] == source_hash
assert manifest['runner_sha256'] == runner_hash
assert len(manifest['cells']) == len(F_MASKS) * len(B_MASKS)
by_pair = {(r['fmask'], r['bmask']): r for r in manifest['cells']}
assert len(by_pair) == len(manifest['cells'])
assert set(by_pair) == {(f, b) for f in F_MASKS for b in B_MASKS}

rows = []
for f in F_MASKS:
    for b in B_MASKS:
        receipt = HERE / f'allcell_{f}_{b}.json'
        r = json.loads(receipt.read_text())
        m = by_pair[f, b]
        assert r['schema'] == 'ghs.trace22.degree8-quadratic-square-all/v1'
        assert (r['fmask'], r['bmask']) == (f, b)
        assert r['source_sha256'] == source_hash
        assert r['target_order'] == 222
        assert r['field'] == 'F3[a]/(a^5+2a+1)'
        assert r['hits'] == []
        c = r['counts']
        assert c['systems'] == 3**5
        assert c['u_specializations'] == 3**10
        assert c['exact_hits'] == 0
        assert c['zero_polynomial_fallbacks'] >= 0
        assert c['parameter_roots'] >= c['allowed_H'] >= c['degree_eight'] >= c['coprime'] >= c['point_survivors'] >= c['f_divides'] >= c['exact_hits']
        assert m['hits'] == 0 and m['cpu_seconds'] == r['cpu_seconds']
        assert m['status'] in ('complete', 'preexisting')
        if m['status'] == 'complete':
            assert m['returncode'] == 0
            assert (HERE / f'allcell_{f}_{b}.stderr').read_bytes() == b''
            assert '"phase": "complete"' in (HERE / f'allcell_{f}_{b}.stdout').read_text()
        rows.append({'fmask': f, 'bmask': b, 'counts': c,
                     'cpu_seconds': r['cpu_seconds']})

keys = rows[0]['counts']
totals = {k: sum(r['counts'][k] for r in rows) for k in keys}
audit = {'schema': 'ghs.trace22.degree8-quadratic-audit/v1',
         'source_sha256': source_hash, 'runner_sha256': runner_hash,
         'cells': len(rows), 'totals': totals,
         'cpu_seconds_sum': sum(r['cpu_seconds'] for r in rows),
         'runner_wall_seconds': manifest['wall_seconds'],
         'status': 'complete'}
(HERE / 'AUDIT.json').write_text(json.dumps(audit, indent=2, sort_keys=True) + '\n')
print(json.dumps(audit, sort_keys=True))
