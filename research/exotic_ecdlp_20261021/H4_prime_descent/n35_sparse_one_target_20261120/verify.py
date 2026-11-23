#!/usr/bin/env python3
"""Independent first-pair, signed-row, target, and rho replay."""
from sage.all import *
import hashlib
import importlib.util
import json
import random
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
SPEC=json.loads((HERE/'protocol.json').read_text())
RESULT_PATH=HERE/'result.json'
RESULT=json.loads(RESULT_PATH.read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def digest(obj):return hashlib.sha256(json.dumps(obj,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
for key in ('full_runner','full_result','fixture','sparse_source','sparse_candidate'):
    assert sha(ROOT/SPEC[key])==SPEC[key+'_sha256']
assert sha(HERE/'run.py')==SPEC['runner_sha256']
assert sha(HERE/'make_fixture.py')==SPEC['fixture_generator_sha256']
assert sha(HERE/'protocol.json')==RESULT['protocol_sha256']
def load(path,name):
    m=importlib.util.spec_from_file_location(name,ROOT/path)
    obj=importlib.util.module_from_spec(m);m.loader.exec_module(obj);return obj
full=load(SPEC['full_runner'],'n35_full_primary_audit')
sparse=load(SPEC['sparse_source'],'n35_sparse_primary_audit')
prior=json.loads((ROOT/SPEC['full_result']).read_text())
fixture=json.loads((ROOT/SPEC['fixture']).read_text())
candidate=json.loads((ROOT/SPEC['sparse_candidate']).read_text())
assert RESULT['status']=='complete' and prior['status']=='complete' and prior['factor_logs_verified']
prefix='IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0h'
assert candidate['candidate_id']==prefix+digest(candidate['record'])[:12]
assert candidate['record']['target_descent']['source_digest']==SPEC['sparse_source_sha256']
assert RESULT['candidate_id']==SPEC['candidate_id']==candidate['candidate_id']
assert RESULT['workload_id']==SPEC['workload_id']==fixture['workload_id']=='W'+digest(fixture['workload_record'])[:12]
assert RESULT['run_id']==candidate['candidate_id']+fixture['workload_id']+'R1'
assert RESULT['curve_id']==candidate['record']['curve']['curve_id']==fixture['workload_record']['curve_id']
assert fixture['target']==SPEC['target']==RESULT['target']
assert fixture['target']!=prior['target']
assert fixture['target_label']==SPEC['target_label']=='h4-n35-sparse-primary-target-0'
scalar=int.from_bytes(hashlib.sha256(fixture['target_label'].encode()).digest(),'big')%(full.R-1)+1
assert scalar==fixture['target_fixture_scalar']==SPEC['target_fixture_scalar']
E,elt=full.build_context()
base,reps,encoding,G=full.build_base(E,elt)
assert len(base)==4096 and len(reps)==2048
assert full.point_record(G)==fixture['workload_record']['G']
assert [full.point_record(P) for P in base]==candidate['record']['factor_base']['encoded_points']
logs=prior['factor_logs']
assert len(logs)==len(reps) and digest(logs)==RESULT['factor_logs_sha256']==prior['factor_logs_sha256']
assert all(logs[j]*G==reps[j] for j in range(len(reps)))
target=E(elt(fixture['target'][0]),elt(fixture['target'][1]))
assert scalar*G==target and full.point_record(target)==fixture['target']
assert digest(fixture['target'])==fixture['target_point_sha256']==RESULT['target_point_sha256']
def key(P):return 1<<70 if P==E(0) else int(P[0].to_integer()) | (int(P[1].to_integer())<<35)
pairs={};B=len(base)
for j,Q in enumerate(base):
    for i,P in enumerate(base):
        k=key(P+Q);v=i*B+j;old=pairs.get(k)
        if old is None or v<old:pairs[k]=v
assert len(pairs)==prior['distinct_pair_sums']
ic=RESULT['ic'];rho=RESULT['rho']
assert ic['status']==rho['status']=='verified'
assert ic['scalar']==rho['scalar']==RESULT['verified_scalar']==scalar
assert ic['scalar']*G==target and rho['scalar']*G==target
assert sum(ic['phase_wall_ns'].values())==ic['wall_ns']==RESULT['online_wall_ns']
assert rho['wall_ns']==RESULT['rho_online_wall_ns']
assert rho['sage_rho_source_sha256']==full.SPEC['sage_rho_source_sha256']
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
        c=(start+first)%B;packed=pairs[key(shifted-base[c])];a,b=divmod(packed,B)
        assert ic['witness_indices']==[a,b,c]
        assert [full.point_record(base[k]) for k in (a,b,c)]==ic['witness']
        assert base[a]+base[b]+base[c]==shifted
        row=full.row_for((a,b,c),base,encoding)
        assert digest(row)==ic['row_sha256']
        signed={}
        for k in (a,b,c):
            col,sign=encoding[base[k]];signed[col]=signed.get(col,0)+sign
        assert ic['signed_coefficients']==[[k,signed[k]] for k in sorted(signed)]
        assert ic['nonzero_signed_columns']==sum(v!=0 for v in signed.values())
        assert all(row[k]==signed.get(k,0)%full.R for k in range(len(reps)))
        assert sum((v*reps[k] for k,v in signed.items()),E(0))==shifted
        assert (sum(v*logs[k] for k,v in enumerate(row))-shift)%full.R==scalar
        assert j==len(ic['target_shifts'])-1
assert len(ic['target_shifts'])<=full.SPEC['target_shift_cap']
validation={'schema':'h4-n35-sparse-primary-validation-v1','status':'passed',
            'result_sha256':sha(RESULT_PATH),'candidate_id':candidate['candidate_id'],
            'workload_id':fixture['workload_id'],'run_id':RESULT['run_id'],
            'target':fixture['target'],'verified_scalar':scalar,
            'target_shift_count':len(ic['target_shifts']),
            'distinct_pair_sums':len(pairs),'factor_logs_verified':len(logs),
            'online_ratio_rho_over_ic':rho['wall_ns']/ic['wall_ns'],
            'host_tier':'ordinary_exploratory'}
(HERE/'validation.json').write_text(json.dumps(validation,indent=2,sort_keys=True)+'\n')
print(json.dumps(validation,indent=2,sort_keys=True))
