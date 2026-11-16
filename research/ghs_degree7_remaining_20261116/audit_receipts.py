#!/usr/bin/env python3
"""Structural audit of all six degree-seven denominator arms on trace +22."""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
OLD = HERE.parent / 'ghs_trace22_cover_screen_20261116'
ARMS = {
    'sparse': (HERE / 'sparse_trace22.sage', lambda n: HERE / f'sparse_{n}.json'),
    'linear': (HERE / 'linear_square_trace22.sage', lambda n: HERE / f'linear_square_{n}.json'),
    'quadratic': (OLD / 'degree7_quadratic_square_trace22.sage', lambda n: OLD / f'screen_{n}.json'),
    'cubic': (HERE / 'cubic_square.sage', lambda n: HERE / f'cubic_square_{n}_v3.json'),
    'two_poles': (HERE / 'two_poles_linear.sage', lambda n: HERE / f'two_poles_linear_{n}.json'),
    'three_poles': (HERE / 'three_poles.sage', lambda n: HERE / f'three_poles_{n}.json'),
}

rows = []
for arm, (source, receipt_path) in ARMS.items():
    source_hash = hashlib.sha256(source.read_bytes()).hexdigest()
    for mask in (68, 84):
        path = receipt_path(mask)
        raw = path.read_bytes()
        d = json.loads(raw)
        assert d['source_sha256'] == source_hash, (arm, mask, 'source hash')
        assert d['bmask'] == mask and d['target_order'] == 222
        assert d['hits'] == [], (arm, mask, 'unexpected hit')
        c = d['counts']
        if arm == 'sparse':
            assert [c[k]['branch_assignments'] for k in ('one','Qi','Qi3','QiQj')] == [243,405,405,270]
            assert all(c[k]['full_equation_hits'] == 0 for k in c)
        elif arm == 'linear':
            assert [c[k]['logical_assignments'] for k in ('H2','H4','H6','QiH2','QiH4')] == [58806]*3+[98010]*2
            assert all(c[k]['full_equation_hits'] == 0 for k in c)
        elif arm == 'quadratic':
            assert c['Hquadratic']['systems'] == 243
            assert c['QiHquadratic']['systems'] == 405
            assert d['allowed_H'] == 58559
            assert all(c[k]['full_equation_hits'] == 0 for k in c)
        elif arm == 'cubic':
            assert c['systems'] == 243
            assert c['zero_dimensional']+c['positive_dimensional']+c['constant_branch_assignments'] == 243
            assert c['censored_systems'] == 0 and d['censored'] == []
            assert c['full_equation_hits'] == 0
        elif arm == 'two_poles':
            assert c['pole_pairs'] == 10 and c['branch_assignments'] == 270
            assert c['denominator_choices'] == 65340
            assert c['logical_candidates'] == 7906140
            assert c['full_equation_hits'] == 0
        elif arm == 'three_poles':
            assert c['denominators'] == 10 and c['branch_assignments'] == 90
            assert c['logical_candidates'] == 643043610
            assert c['bitset_controls'] == 1440
            assert c['full_equation_hits'] == 0
        rows.append({'arm':arm,'mask':mask,
                     'receipt_sha256':hashlib.sha256(raw).hexdigest(),
                     'source_sha256':source_hash,
                     'receipt':str(path.relative_to(HERE.parent)),
                     'cpu_seconds':d.get('cpu_seconds')})

assert len(rows) == 12
print(json.dumps({'schema':'ghs.degree7.trace22-receipt-audit/v1',
                  'scope':'fixed carrier, two source-orbit representatives',
                  'receipt_count':len(rows),'rows':rows,
                  'audit_limit':'source binding and coverage counters; algebraic elimination not reimplemented'},
                 sort_keys=True, indent=2))
