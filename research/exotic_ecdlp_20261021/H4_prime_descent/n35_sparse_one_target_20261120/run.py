#!/usr/bin/env python3
"""Primary one-target sparse IC and exact same-point Sage rho pair."""
from sage.all import *
import hashlib
import importlib.util
import json
import platform
from pathlib import Path
import resource
import signal
import time

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
SPEC=json.loads((HERE/'protocol.json').read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def load(path,name):
    m=importlib.util.spec_from_file_location(name,ROOT/path)
    obj=importlib.util.module_from_spec(m);m.loader.exec_module(obj);return obj
for key in ('full_runner','full_result','fixture','sparse_source','sparse_candidate'):
    assert sha(ROOT/SPEC[key])==SPEC[key+'_sha256']
full=load(SPEC['full_runner'],'n35_full_primary')
sparse=load(SPEC['sparse_source'],'n35_sparse_primary')
prior=json.loads((ROOT/SPEC['full_result']).read_text())
fixture=json.loads((ROOT/SPEC['fixture']).read_text())
candidate=json.loads((ROOT/SPEC['sparse_candidate']).read_text())
assert prior['status']=='complete' and prior['factor_logs_verified']
assert fixture['workload_id']==SPEC['workload_id']=='W'+full.digest(fixture['workload_record'])[:12]
assert candidate['candidate_id']==SPEC['candidate_id']
assert fixture['target']==fixture['workload_record']['target']==SPEC['target']
assert fixture['target_point_sha256']==full.digest(SPEC['target'])
assert fixture['target']!=prior['target']
assert fixture['target_fixture_scalar']==SPEC['target_fixture_scalar']
assert prior['curve_id']==candidate['record']['curve']['curve_id']==fixture['workload_record']['curve_id']
def save(d):(HERE/'result.json').write_text(json.dumps(d,indent=2,sort_keys=True)+'\n')
def timeout(*_):raise TimeoutError('frozen process or rho cap')
signal.signal(signal.SIGALRM,timeout)
result={'schema':'h4-n35-sparse-one-target-result-v1','status':'running',
        'protocol_sha256':sha(HERE/'protocol.json'),
        'candidate_id':candidate['candidate_id'],'curve_id':prior['curve_id'],
        'workload_id':fixture['workload_id'],
        'run_id':candidate['candidate_id']+fixture['workload_id']+'R1',
        'target':SPEC['target'],'target_point_sha256':full.digest(SPEC['target']),
        'host':{'node':platform.node(),'system':platform.system(),'machine':platform.machine(),
                'tier':'ordinary_exploratory'}}
save(result)
signal.alarm(SPEC['process_cap_seconds'])
try:
    t=time.perf_counter_ns()
    E,elt=full.build_context()
    base,reps,encoding,G=full.build_base(E,elt)
    assert len(base)==4096 and len(reps)==2048
    assert full.point_record(G)==fixture['workload_record']['G']
    assert [full.point_record(P) for P in base]==candidate['record']['factor_base']['encoded_points']
    index=full.build_index(base)
    assert len(index)==prior['distinct_pair_sums']==8321091
    logs=prior['factor_logs']
    assert len(logs)==len(reps) and all(logs[j]*G==reps[j] for j in range(len(reps)))
    result['replay_setup_wall_ns']=time.perf_counter_ns()-t
    result['factor_logs_sha256']=full.digest(logs)
    result['replay_setup_peak_rss_bytes']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    assert result['factor_logs_sha256']==prior['factor_logs_sha256']
    if result['replay_setup_peak_rss_bytes']>SPEC['observed_rss_cap_bytes']:
        result['status']='censored_memory_limit';save(result)
    else:
        Q=E(elt(SPEC['target'][0]),elt(SPEC['target'][1]))
        assert full.point_record(Q)==SPEC['target']
        assert SPEC['target_fixture_scalar']*G==Q
        result['target_fixture_verified']=True
        save(result)
        ic=sparse.ic_online(full,base,reps,encoding,G,index,logs,Q)
        result['ic']=ic
        result['online_wall_ns']=ic['wall_ns']
        save(result)
        if ic['status']!='verified':
            result['status']='censored_target_shift_cap'
        else:
            signal.alarm(SPEC['rho_cap_seconds'])
            try:
                rho=full.rho_online(G,Q)
                result['rho']=rho
                result['rho_online_wall_ns']=rho['wall_ns']
                assert rho['scalar']==ic['scalar']==SPEC['target_fixture_scalar']
                assert rho['scalar']*G==Q
                result['verified_scalar']=ic['scalar']
                result['status']='complete'
            except TimeoutError as exc:
                result['status']='censored_rho_timeout'
                result['rho']={'status':'censored_timeout','error':str(exc)}
            finally:
                signal.alarm(0)
except TimeoutError as exc:
    result.update(status='censored_process_timeout',error=str(exc))
except Exception as exc:
    result.update(status='error',error=repr(exc));raise
finally:
    signal.alarm(0)
    result['peak_rss_bytes']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    save(result)
print(json.dumps({k:v for k,v in result.items() if k not in ('ic','rho')},sort_keys=True,indent=2))
