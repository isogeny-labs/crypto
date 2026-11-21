#!/usr/bin/env python3
"""Independent record and exact first-hit audit of all four completed index arms."""
import hashlib, json
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
P=json.loads((HERE/'protocol.json').read_text())
REF=json.loads((ROOT/P['reference_pair_result']).read_text())
assert sha(ROOT/P['candidate'])==P['candidate_sha256']
assert sha(ROOT/P['reference_pair_result'])==P['reference_pair_sha256']
assert sha(HERE/'run.py')==P['runner_sha256']
assert REF['status']=='complete' and len(REF['targets'])==32
arms=[]
for variant,repetition in P['run_order']:
    a=json.loads((HERE/f'{variant}-{repetition}.json').read_text())
    assert (a['variant'],a['repetition'])==(variant,repetition)
    assert a['status']=='complete'
    assert a['protocol_sha256']==sha(HERE/'protocol.json')
    assert a['runner_sha256']==P['runner_sha256']
    assert a['candidate_sha256']==P['candidate_sha256']
    assert a['reference_pair_sha256']==P['reference_pair_sha256']
    assert a['pair_attempts']==REF['pair_attempts']==4368100
    assert a['stored_sums']==REF['distinct_pair_sums']==2116611
    assert len(a['targets'])==len(REF['targets'])==32
    for x,y in zip(a['targets'],REF['targets']):
        for k in ('index','target','attempts','witness_indices','status'):
            assert x[k]==y[k]
        assert x['lookup_ns']>0
    assert a['index_ns']>0 and a['peak_rss_bytes']>=a['peak_rss_after_build_bytes']>0
    arms.append({'variant':variant,'repetition':repetition,'status':a['status'],
                 'index_ns':a['index_ns'],'peak_rss_bytes':a['peak_rss_bytes'],
                 'stored_sums':a['stored_sums'],'exact_target_outcomes':len(a['targets'])})
point={a['repetition']:a for a in arms if a['variant']=='point'}
packed={a['repetition']:a for a in arms if a['variant']=='packed'}
assert len(point)==len(packed)==2
receipt={'schema':'h4-n31-fast-packed-index-audit-v1','status':'passed',
         'protocol_sha256':sha(HERE/'protocol.json'),'arms':arms,
         'paired_rss_reduction_fraction':{str(i):1-packed[i]['peak_rss_bytes']/point[i]['peak_rss_bytes'] for i in (1,2)},
         'paired_index_time_ratio':{str(i):packed[i]['index_ns']/point[i]['index_ns'] for i in (1,2)}}
(HERE/'audit.json').write_text(json.dumps(receipt,indent=2,sort_keys=True)+'\n')
print(json.dumps(receipt,indent=2,sort_keys=True))
