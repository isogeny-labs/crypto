#!/usr/bin/env python3
"""Source-bound structural audit of the trace-minus14 mixed-pole receipts."""
import hashlib
import json
from pathlib import Path

here=Path(__file__).resolve().parent
source_hash='5f6a832d6a72346b34f11b6c9cca40ad81f6e9f2fb85100986f1c6cb097c50b4'
assert hashlib.sha256((here/'screen.sage').read_bytes()).hexdigest()==source_hash
shapes=('LH2','LH4','LH6','LQiH2','LQiH4','L3QiH2')
rows=[]
for f in (9735,17371):
    for b in (11,202,38,80):
        d=json.loads((here/f'cell_{f}_{b}.json').read_text())
        assert d['schema']=='ghs.trace-minus14.degree8-mixed-poles/v1'
        assert d['source_sha256']==source_hash
        assert (d['fmask'],d['bmask'],d['target_order'])==(f,b,258)
        assert d['field']=='F3[a]/(a^5+2a+1)'
        assert set(d['counts'])==set(shapes)
        hit_counts={s:sum(h['shape']==s for h in d['hits']) for s in shapes}
        for s in shapes:
            c=d['counts'][s]
            mixed=s not in shapes[:3]
            assert c['assignments']==(405 if mixed else 243)
            assert c['logical_candidates']==(11859210 if mixed else 58806)
            assert c['point_filter_controls']==(976050 if mixed else 0)
            assert c['full_equation_hits']==hit_counts[s]
            assert c['point_survivors']>=c['degree_eight']>=c['coprime']>=c['f_divides']>=c['full_equation_hits']
        stderr=(here/f'cell_{f}_{b}.stderr').read_text().splitlines()
        assert len(stderr)==5 and all(x.startswith('quadratic_pole=') for x in stderr)
        assert d['cpu_seconds']>0 and d['wall_seconds']>0
        rows.append({'fmask':f,'bmask':b,'counts':d['counts'],
                     'hits':d['hits'],'cpu_seconds':d['cpu_seconds'],
                     'wall_seconds':d['wall_seconds']})
assert len(rows)==8
print(json.dumps({'schema':'ghs.trace-minus14.degree8-mixed-poles-audit/v1',
                  'source_sha256':source_hash,
                  'audit_source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'representative_cells':8,'exact_equation_pairs_represented':240,
                  'shapes':shapes,'exact_maps':sum(len(r['hits']) for r in rows),
                  'claim_boundary':'six named degree-eight shapes, one Sage implementation',
                  'rows':rows},sort_keys=True,indent=2))
