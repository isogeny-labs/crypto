#!/usr/bin/env python3
"""Source-bound structural audit of rational-branch-swap companion receipts."""
import hashlib
import json
from pathlib import Path

here=Path(__file__).resolve().parent
h='b5b8729e78d98ce4967b3f52a3e3a607dc32069546b715dbd4029d6d8f0a53ed'
assert hashlib.sha256((here/'swap_closure.sage').read_bytes()).hexdigest()==h
rows=[]
for f in (9735,17371):
    for b in (11,202,38,80):
        d=json.loads((here/f'swap_{f}_{b}.json').read_text())
        assert d['schema']=='ghs.trace-minus14.degree8-swap-closure/v1'
        assert d['source_sha256']==h
        assert (d['fmask'],d['bmask'],d['target_order'])==(f,b,258)
        assert d['field']=='F3[a]/(a^5+2a+1)'
        c=d['counts']
        assert c['branch_assignments']==243
        assert c['logical_candidates']==58806
        assert c['coefficient_fallbacks']>=0
        assert c['parameter_roots']>=c['degree_eight']>=c['coprime']>=c['f_divides']>=c['full_equation_hits']
        assert c['full_equation_hits']==len(d['hits'])
        assert (here/f'swap_{f}_{b}.stderr').read_text()==''
        assert d['cpu_seconds']>0 and d['wall_seconds']>0
        rows.append({'fmask':f,'bmask':b,'counts':c,'hits':d['hits'],
                     'cpu_seconds':d['cpu_seconds'],'wall_seconds':d['wall_seconds']})
assert len(rows)==8
print(json.dumps({'schema':'ghs.trace-minus14.degree8-swap-closure-audit/v1',
                  'source_sha256':h,
                  'audit_source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'representative_cells':8,'shape':'L^3(x-c)^4',
                  'exact_maps':sum(len(r['hits']) for r in rows),'rows':rows},sort_keys=True,indent=2))
