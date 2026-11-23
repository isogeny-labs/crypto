#!/usr/bin/env python3
"""Independent exact-witness and scalar replay for n35 dense/sparse checks."""
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
RESULT_PATH=HERE/'result.json'
RESULT=json.loads(RESULT_PATH.read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def digest(obj):return hashlib.sha256(json.dumps(obj,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
for key in ('full_runner','full_candidate','full_result','fixtures','sparse_source','sparse_candidate'):
    assert sha(ROOT/SPEC[key])==SPEC[key+'_sha256']
assert sha(HERE/'run.py')==SPEC['runner_sha256']
assert sha(HERE/'make_manifest.py')==SPEC['manifest_generator_sha256']
assert sha(HERE/'protocol.json')==RESULT['protocol_sha256']
def load(path,name):
    m=importlib.util.spec_from_file_location(name,ROOT/path)
    obj=importlib.util.module_from_spec(m);m.loader.exec_module(obj);return obj
full=load(SPEC['full_runner'],'n35_full_audit')
sparse=load(SPEC['sparse_source'],'n35_sparse_audit')
C0=json.loads((ROOT/SPEC['full_candidate']).read_text())
C1=json.loads((ROOT/SPEC['sparse_candidate']).read_text())
O=json.loads((ROOT/SPEC['full_result']).read_text())
FIX=json.loads((ROOT/SPEC['fixtures']).read_text())
prefix='IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0h'
assert C1['candidate_id']==prefix+digest(C1['record'])[:12]
assert C1['candidate_id']!=C0['candidate_id']
assert C1['record']['target_descent']['source_digest']==SPEC['sparse_source_sha256']
assert C1['record']['implementation']['source_sha256']['sparse_target_check']==SPEC['sparse_source_sha256']
assert RESULT['candidate_ids']=={'dense':C0['candidate_id'],'sparse':C1['candidate_id']}
assert O['status']=='complete' and O['factor_logs_verified']
E,elt=full.build_context()
base,reps,encoding,G=full.build_base(E,elt)
assert len(base)==4096 and len(reps)==2048
assert [full.point_record(P) for P in base]==C0['record']['factor_base']['encoded_points']==C1['record']['factor_base']['encoded_points']
assert full.point_record(G)==FIX['generator']
logs=O['factor_logs']
assert len(logs)==len(reps) and all(logs[j]*G==reps[j] for j in range(len(reps)))
def key(P):return 1<<70 if P==E(0) else int(P[0].to_integer()) | (int(P[1].to_integer())<<35)
pairs={}
B=len(base)
for j,Q in enumerate(base):
    for i,P in enumerate(base):
        k=key(P+Q);v=i*B+j
        old=pairs.get(k)
        if old is None or v<old:pairs[k]=v
assert len(pairs)==O['distinct_pair_sums']
assert len(RESULT['pairs'])<=10 and [x['index'] for x in RESULT['pairs']]==list(range(len(RESULT['pairs'])))
ratios=[]
for fixture,pair in zip(FIX['targets'],RESULT['pairs']):
    i=fixture['index'];target=E(elt(fixture['target'][0]),elt(fixture['target'][1]))
    scalar=int.from_bytes(hashlib.sha256(f'h4-n35-panel-target-{i}'.encode()).digest(),'big')%(full.R-1)+1
    assert fixture['fixture_scalar']==scalar and scalar*G==target
    assert pair['index']==i and pair['workload_id']==fixture['workload_id']
    assert pair['target']==fixture['target'] and pair['target_point_sha256']==fixture['target_point_sha256']
    assert pair['order']==SPEC['arm_order'][i] and set(pair['arms'])=={'dense','sparse'}
    a,b=pair['calibration_pre_ns'],pair['calibration_post_ns']
    assert a>0 and b>0 and pair['calibration_ratio']==max(a,b)/min(a,b)
    assert pair['calibration_pass']==(pair['calibration_ratio']<=SPEC['calibration_ratio_cap'])
    for arm in ('dense','sparse'):
        receipt=pair['arms'][arm]
        if receipt['status']!='verified':continue
        assert receipt['scalar']==scalar and receipt['scalar']*G==target
        assert sum(receipt['phase_wall_ns'].values())==receipt['wall_ns']
        rng=random.Random(73035)
        for j,shift_row in enumerate(receipt['target_shifts']):
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
                assert receipt['witness_indices']==[a0,b0,c]
                assert [full.point_record(base[k]) for k in (a0,b0,c)]==receipt['witness']
                assert base[a0]+base[b0]+base[c]==shifted
                row=full.row_for((a0,b0,c),base,encoding)
                assert digest(row)==receipt['row_sha256']
                assert (sum(v*logs[k] for k,v in enumerate(row))-shift)%full.R==scalar
                if arm=='sparse':
                    signed={}
                    for k in (a0,b0,c):
                        col,sign=encoding[base[k]];signed[col]=signed.get(col,0)+sign
                    assert receipt['signed_coefficients']==[[k,signed[k]] for k in sorted(signed)]
                    assert receipt['nonzero_signed_columns']==sum(v!=0 for v in signed.values())
                    assert sum((v*reps[k] for k,v in signed.items()),E(0))==shifted
                assert j==len(receipt['target_shifts'])-1
    both=all(pair['arms'][arm]['status']=='verified' and pair['arms'][arm]['scalar']==scalar for arm in ('dense','sparse'))
    assert pair['scalar_certificate_pass']==both
    assert pair['same_witness_pass']==(pair['arms']['dense'].get('witness_indices')==pair['arms']['sparse'].get('witness_indices') and pair['arms']['dense'].get('row_sha256')==pair['arms']['sparse'].get('row_sha256'))
    if both:ratios.append(pair['arms']['dense']['wall_ns']/pair['arms']['sparse']['wall_ns'])
summary={'schema':'h4-n35-sparse-check-validation-v1','status':RESULT['status'],
         'pairs_recorded':len(RESULT['pairs']),'verified_pairs':len(ratios),
         'calibration_passes':sum(x['calibration_pass'] for x in RESULT['pairs']),
         'same_witness_passes':sum(x['same_witness_pass'] for x in RESULT['pairs']),
         'candidate_ids':RESULT['candidate_ids'],
         'dense_over_sparse_online_ratio':{'per_pair':ratios,
            'median':statistics.median(ratios) if ratios else None,
            'geometric_mean':math.exp(statistics.mean(math.log(x) for x in ratios)) if ratios else None,
            'min':min(ratios) if ratios else None,'max':max(ratios) if ratios else None},
         'median_target_relation_check_ns':{
            arm:statistics.median(x['arms'][arm]['phase_wall_ns']['target_relation_check'] for x in RESULT['pairs'] if x['arms'][arm]['status']=='verified')
            for arm in ('dense','sparse')},
         'shared_setup_ns':RESULT.get('shared_setup_ns'),
         'result_sha256':sha(RESULT_PATH),'host_tier':'ordinary_exploratory'}
(HERE/'validation.json').write_text(json.dumps(summary,indent=2,sort_keys=True)+'\n')
print(json.dumps(summary,indent=2,sort_keys=True))
