#!/usr/bin/env python3
"""Audit frozen point-index controls and retain both censored packed arms."""
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
rows=[]
for variant,repetition in P['run_order']:
    path=HERE/f'{variant}-{repetition}.json'
    a=json.loads(path.read_text())
    assert (a['variant'],a['repetition'])==(variant,repetition)
    assert a['protocol_sha256']==sha(HERE/'protocol.json')
    assert a['runner_sha256']==P['runner_sha256']
    assert a['candidate_sha256']==P['candidate_sha256']
    assert a['reference_pair_sha256']==P['reference_pair_sha256']
    if variant=='point':
        assert a['status']=='complete'
        assert a['pair_attempts']==REF['pair_attempts']==4368100
        assert a['stored_sums']==REF['distinct_pair_sums']==2116611
        assert len(a['targets'])==32
        for x,y in zip(a['targets'],REF['targets']):
            for k in ('index','target','attempts','witness_indices','status'):
                assert x[k]==y[k]
    else:
        assert a['status']=='censored_timeout'
        assert a['error']=='frozen 180-second arm cap'
        assert len(a['targets'])==0 and 'index_ns' not in a
        assert a['total_ns']>=180_000_000_000
    rows.append({'variant':variant,'repetition':repetition,'status':a['status'],
                 'index_ns':a.get('index_ns'),'stored_sums':a.get('stored_sums'),
                 'peak_rss_bytes':a['peak_rss_bytes'],'targets_checked':len(a['targets'])})
receipt={'schema':'h4-n31-packed-index-audit-v1','status':'passed',
         'protocol_sha256':sha(HERE/'protocol.json'),'arms':rows}
(HERE/'audit.json').write_text(json.dumps(receipt,indent=2,sort_keys=True)+'\n')
print(json.dumps(receipt,indent=2,sort_keys=True))
