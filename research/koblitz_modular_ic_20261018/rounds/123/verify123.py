#!/usr/bin/env python3
"""Independent degree and leading-edge audit for the x=0 census."""
import hashlib
import json
from pathlib import Path

here=Path(__file__).resolve().parent
rounds=here.parent
p=json.loads((here/'protocol.json').read_text())
inputs={}
for n,h in p['input_sha256'].items():
    path=rounds/n/'result.json'
    assert hashlib.sha256(path.read_bytes()).hexdigest()==h
    inputs[int(n)]=json.loads(path.read_text())
census=json.loads((here/'result.json').read_text())
total=0
for key,stage in p['selected_stage'].items():
    e=inputs[116]['branches'][key]['x_order']
    two=inputs[stage]['branches'][key]
    assert sorted(row['alpha'] for row in two)==[2,3]
    for row in two:
        by={next(v for k,v in c.items() if k.endswith('_exponent')):c for c in row['translated_rows']}
        a,b,c=(by[j]['s_order'] for j in (0,1,8))
        assert a>b>c==0 and (b-c)%7 != 0
        assert row['lower_hull_if_exact'][-1]['s_order'] >= c
        assert by[1]['nonzero_support_below_bound'][0][1] == 1
        assert by[8]['nonzero_support_below_bound'][0][1] == 1
        assert by[0]['nonzero_support_below_bound'][0][1] in (2,3)
    f2=census['base_places'][key]['f2_normalized_places']
    assert sorted((row['residue_degree'],row['ramification_over_base'],row['x_order']) for row in f2)==[(2,1,e),(2,7,7*e)]
    total += sum(row['residue_degree']*row['x_order'] for row in f2)
assert len(census['base_places'])*2 == census['normalized_places_over_f2'] == 8
assert total == census['x_zero_divisor_degree'] == 112
print('CENSUS123_INDEPENDENT_DEGREE_PASS')
