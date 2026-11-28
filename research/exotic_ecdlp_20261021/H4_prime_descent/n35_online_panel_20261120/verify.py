#!/usr/bin/env python3
"""Independent same-point replay for the frozen n35 online panel."""
from sage.all import *
import hashlib
import importlib.util
import json
import math
import random
import statistics
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
SPEC=json.loads((HERE/'protocol.json').read_text())
FIX=json.loads((HERE/'fixtures.json').read_text())
RESULT_PATH=HERE/'result.json'
RESULT=json.loads(RESULT_PATH.read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def digest(obj):return hashlib.sha256(json.dumps(obj,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
for key in ('full_runner','full_candidate','full_result','fixtures','fixture_generator'):
    assert sha(ROOT/SPEC[key])==SPEC[key+'_sha256']
assert sha(HERE/'run.py')==SPEC['runner_sha256']
assert sha(HERE/'protocol.json')==RESULT['protocol_sha256']
module_spec=importlib.util.spec_from_file_location('n35_full_audit',ROOT/SPEC['full_runner'])
full=importlib.util.module_from_spec(module_spec);module_spec.loader.exec_module(full)
prior=json.loads((ROOT/SPEC['full_result']).read_text())
candidate=json.loads((ROOT/SPEC['full_candidate']).read_text())
assert prior['status']=='complete' and prior['factor_logs_verified']
assert prior['candidate_id']==candidate['candidate_id']==FIX['candidate_id']==RESULT['candidate_id']
E,elt=full.build_context()
base,reps,encoding,G=full.build_base(E,elt)
assert len(base)==4096 and len(reps)==2048 and len(RESULT['pairs'])<=10
assert full.point_record(G)==FIX['generator']
assert [full.point_record(P) for P in base]==candidate['record']['factor_base']['encoded_points']
logs=prior['factor_logs']
assert len(logs)==len(reps) and all(logs[j]*G==reps[j] for j in range(len(reps)))
def key(P):
    return 1<<70 if P==E(0) else int(P[0].to_integer()) | (int(P[1].to_integer())<<35)
# Column-major traversal and explicit minimum payload, independent of the measured builder.
pairs={}
B=len(base)
for j,Q in enumerate(base):
    for i,P in enumerate(base):
        k=key(P+Q);v=i*B+j
        old=pairs.get(k)
        if old is None or v<old:pairs[k]=v
assert len(pairs)==prior['distinct_pair_sums']
assert [row['index'] for row in RESULT['pairs']]==list(range(len(RESULT['pairs'])))
verified_ratios=[]
for fixture,row in zip(FIX['targets'],RESULT['pairs']):
    i=fixture['index']
    assert row['index']==i and row['order']==SPEC['arm_order'][i]
    assert row['workload_id']==fixture['workload_id']
    assert row['target']==fixture['target'] and row['target_point_sha256']==fixture['target_point_sha256']
    label=f'h4-n35-panel-target-{i}'
    scalar=int.from_bytes(hashlib.sha256(label.encode()).digest(),'big')%(full.R-1)+1
    assert label==fixture['label'] and scalar==fixture['fixture_scalar']
    target=E(elt(fixture['target'][0]),elt(fixture['target'][1]))
    assert scalar*G==target and digest(fixture['target'])==fixture['target_point_sha256']
    a,b=row['calibration_pre_ns'],row['calibration_post_ns']
    assert a>0 and b>0 and row['calibration_ratio']==max(a,b)/min(a,b)
    assert row['calibration_pass']==(row['calibration_ratio']<=SPEC['calibration_ratio_cap'])
    assert set(row['arms'])=={'ic','rho'}
    ic,rho=row['arms']['ic'],row['arms']['rho']
    if ic['status']=='verified':
        assert ic['scalar']==scalar and ic['scalar']*G==target
        assert sum(ic['phase_wall_ns'].values())==ic['wall_ns']
        rng=random.Random(73035)
        for j,shift_row in enumerate(ic['target_shifts']):
            shift=0 if j==0 else rng.randrange(1,full.R)
            assert shift_row['shift_number']==j and shift_row['shift_scalar']==shift
            shifted=target+shift*G if shift else target
            start=rng.randrange(B)
            first=next((offset for offset in range(B)
                        if key(shifted-base[(start+offset)%B]) in pairs),None)
            if first is None:
                assert shift_row['status']=='exhausted' and shift_row['third_point_attempts']==B
            else:
                assert shift_row['status']=='verified' and shift_row['third_point_attempts']==first+1
                c=(start+first)%B;packed=pairs[key(shifted-base[c])];a0,b0=divmod(packed,B)
                assert ic['witness_indices']==[a0,b0,c]
                assert [full.point_record(base[k]) for k in (a0,b0,c)]==ic['witness']
                assert base[a0]+base[b0]+base[c]==shifted
                coeffs=full.row_for((a0,b0,c),base,encoding)
                assert digest(coeffs)==ic['row_sha256']
                assert (sum(v*logs[k] for k,v in enumerate(coeffs))-shift)%full.R==scalar
                assert j==len(ic['target_shifts'])-1
        assert len(ic['target_shifts'])<=SPEC['target_shift_cap'] if 'target_shift_cap' in SPEC else len(ic['target_shifts'])<=16
    if rho['status']=='verified':
        assert rho['scalar']==scalar and rho['scalar']*G==target
        assert rho['sage_rho_source_sha256']==full.SPEC['sage_rho_source_sha256']
    assert row['scalar_certificate_pass']==(ic['status']==rho['status']=='verified' and ic['scalar']==rho['scalar']==scalar)
    if row['scalar_certificate_pass']:
        verified_ratios.append(rho['wall_ns']/ic['wall_ns'])
assert RESULT['status']==('complete' if len(RESULT['pairs'])==10 and all(x['scalar_certificate_pass'] for x in RESULT['pairs']) else RESULT['status'])
summary={'schema':'h4-n35-online-panel-summary-v1','status':RESULT['status'],
         'pairs_recorded':len(RESULT['pairs']),'verified_pairs':len(verified_ratios),
         'calibration_passes':sum(x['calibration_pass'] for x in RESULT['pairs']),
         'ratio_rho_over_ic':{'per_pair':verified_ratios,
                              'median':statistics.median(verified_ratios) if verified_ratios else None,
                              'geometric_mean':math.exp(statistics.mean(math.log(x) for x in verified_ratios)) if verified_ratios else None,
                              'min':min(verified_ratios) if verified_ratios else None,
                              'max':max(verified_ratios) if verified_ratios else None},
         'shared_setup_ns':RESULT.get('shared_setup_ns'),
         'result_sha256':sha(RESULT_PATH),
         'host_tier':'ordinary_exploratory'}
(HERE/'summary.json').write_text(json.dumps(summary,indent=2,sort_keys=True)+'\n')
print(json.dumps(summary,indent=2,sort_keys=True))
