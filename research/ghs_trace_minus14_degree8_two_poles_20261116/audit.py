#!/usr/bin/env python3
"""Source-bound structural audit of the eight exact two-pole cover receipts."""
import hashlib
import json
from pathlib import Path

here=Path(__file__).resolve().parent
source=here/'screen.sage'
expected_source='b06a004a8654cab2686c1870a311af31abef4083e24289753f86230606e0d75e'
assert hashlib.sha256(source.read_bytes()).hexdigest()==expected_source
rows=[]
for f in (9735,17371):
    for b in (11,202,38,80):
        path=here/f'cell_{f}_{b}.json'
        d=json.loads(path.read_text())
        assert d['schema']=='ghs.trace-minus14.degree8-two-poles/v1'
        assert d['source_sha256']==expected_source
        assert (d['fmask'],d['bmask'])==(f,b)
        assert d['field']=='F3[a]/(a^5+2a+1)' and d['target_order']==258
        assert d['counts']['pole_pairs']==10
        assert d['counts']['shapes']==20
        assert d['counts']['branch_assignments']==540
        assert d['counts']['logical_candidates']==3858261660
        assert d['counts']['bitset_controls']==10800
        assert d['counts']['full_equation_hits']==len(d['hits'])==0
        assert d['counts']['point_filter_survivors']>=d['counts']['coprime']>=d['counts']['divisible']>=d['counts']['polynomial_squares']
        assert d['counts']['polynomial_squares']==0
        stderr=(here/f'cell_{f}_{b}.stderr').read_text().splitlines()
        assert len(stderr)==10 and all(line.startswith('poles=') for line in stderr)
        assert d['cpu_seconds']>0 and d['wall_seconds']>0
        rows.append({'carrier_mask':f,'source_mask':b,'source_sha256':expected_source,
                     'counts':d['counts'],'cpu_seconds':d['cpu_seconds'],
                     'wall_seconds':d['wall_seconds']})
assert len(rows)==8
print(json.dumps({'schema':'ghs.trace-minus14.degree8-two-poles-audit/v1',
                  'audit_source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'representative_cells':8,'exact_equation_pairs_represented':240,
                  'denominator_shapes':['L Qi Qj','L^3 Qi Qj'],
                  'receipts':len(rows),'exact_maps':0,
                  'claim_boundary':'two named degree-eight shapes, one Sage implementation',
                  'rows':rows},sort_keys=True,indent=2))
