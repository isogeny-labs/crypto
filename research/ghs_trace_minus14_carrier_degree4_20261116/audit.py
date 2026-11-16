#!/usr/bin/env python3
"""Audit corrected trace -14 degree-four receipts and census coverage."""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
source_hash = hashlib.sha256((HERE / 'screen.sage').read_bytes()).hexdigest()
census = json.loads((HERE / '../ghs_odd5_carrier_20261115/cubic_all.json').read_text())
masks = sorted(r['mask'] for r in census['rows'] if r['trace'] == -14)
assert masks == [9735, 17371, 21624, 44227, 64725, 90937, 92153,
                 100118, 124586, 144548, 156795, 159256]
assert sorted(int(p.stem.split('_')[1]) for p in HERE.glob('cell_*.json')) == masks
expected = {'linear_only': 243, 'triple_linear': 243,
            'linear_plus_quadratic': 405,
            'linear_plus_double_nonbranch': 58806}
rows = []
for mask in masks:
    r = json.loads((HERE / f'cell_{mask}.json').read_text())
    assert r['schema'] == 'ghs.trace-minus14.carrier-degree4/v1'
    assert r['source_sha256'] == source_hash
    assert r['fmask'] == mask and r['trace'] == -14
    assert r['source_order'] == 258
    assert r['hits'] == [] and r['survivors'] == []
    assert set(r['counts']) == set(expected)
    for shape, count in expected.items():
        c = r['counts'][shape]
        assert c['assignments'] == count
        assert all(c[k] == 0 for k in ('degree_four_crt', 'coprime_crt',
                                     'polynomial_quotient', 'square_quotient'))
    rows.append({'mask': mask, 'branch_factor': r['rational_branch_factor'],
                 'assignments': sum(c['assignments'] for c in r['counts'].values()),
                 'cpu_seconds': r['cpu_seconds']})
out = {'schema': 'ghs.trace-minus14.carrier-degree4-audit/v1',
       'source_sha256': source_hash, 'carrier_count': len(rows),
       'total_assignments': sum(r['assignments'] for r in rows),
       'cpu_seconds_sum': sum(r['cpu_seconds'] for r in rows),
       'exact_hits': 0, 'rows': rows}
(HERE / 'AUDIT.json').write_text(json.dumps(out, indent=2, sort_keys=True) + '\n')
print(json.dumps(out, sort_keys=True))
