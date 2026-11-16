#!/usr/bin/env python3
"""Source-bound structural audit of eight branch-supported sparse cover receipts."""
import hashlib
import json
from pathlib import Path

here=Path(__file__).resolve().parent
source_hash='31538c7d1da93cfacf9bd7a9d931cc389f866247f9d2446681299de682c167f8'
assert hashlib.sha256((here/'screen.sage').read_bytes()).hexdigest()==source_hash
shapes=('L1','L3','L5','L7','LQi','L3Qi','L5Qi','LQi3')
rows=[]
for f in (9735,17371):
    for b in (11,202,38,80):
        d=json.loads((here/f'cell_{f}_{b}.json').read_text())
        assert d['schema']=='ghs.trace-minus14.degree8-branch-sparse/v1'
        assert d['source_sha256']==source_hash
        assert (d['fmask'],d['bmask'],d['trace'],d['target_order'])==(f,b,-14,258)
        assert d['field']=='F3[a]/(a^5+2a+1)'
        assert tuple(d['counts'])==tuple(sorted(shapes))
        assert d['hits']==[]
        for s in shapes:
            c=d['counts'][s]
            expected_assignments=243 if s in shapes[:4] else 405
            expected_candidates=expected_assignments if s in shapes[:4] else 405*121
            expected_controls=0 if s in shapes[:4] else 4050
            assert c['assignments']==expected_assignments
            assert c['logical_candidates']==expected_candidates
            assert c['point_filter_controls']==expected_controls
            assert c['full_equation_hits']==0
            assert c['point_survivors']>=c['degree_eight']>=c['coprime']>=c['f_divides']
        assert (here/f'cell_{f}_{b}.stderr').read_text()==''
        assert d['cpu_seconds']>0 and d['wall_seconds']>0
        rows.append({'fmask':f,'bmask':b,'counts':d['counts'],
                     'cpu_seconds':d['cpu_seconds'],'wall_seconds':d['wall_seconds']})
assert len(rows)==8
print(json.dumps({'schema':'ghs.trace-minus14.degree8-sparse-audit/v1',
                  'audit_source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'source_sha256':source_hash,'representative_cells':8,
                  'exact_equation_pairs_represented':240,'shapes':shapes,
                  'exact_maps':0,'claim_boundary':'eight named degree-eight shapes, one Sage implementation',
                  'rows':rows},sort_keys=True,indent=2))
