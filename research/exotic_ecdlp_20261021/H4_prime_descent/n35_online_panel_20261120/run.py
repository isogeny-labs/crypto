#!/usr/bin/env python3
"""Frozen secondary n35 fresh-point IC/rho paired online panel."""
from sage.all import *
import hashlib
import importlib.util
import json
import platform
from pathlib import Path
import resource
import signal
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE/'protocol.json').read_text())
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
for key in ('full_runner', 'full_candidate', 'full_result', 'fixtures', 'fixture_generator'):
    assert sha(ROOT/SPEC[key]) == SPEC[key+'_sha256']
module_spec = importlib.util.spec_from_file_location('n35_full_runner', ROOT/SPEC['full_runner'])
full = importlib.util.module_from_spec(module_spec)
module_spec.loader.exec_module(full)
C = json.loads((ROOT/SPEC['full_candidate']).read_text())
O = json.loads((ROOT/SPEC['full_result']).read_text())
FIX = json.loads((HERE/'fixtures.json').read_text())
assert O['status'] == 'complete' and O['factor_logs_verified']
assert C['candidate_id'] == O['candidate_id'] == FIX['candidate_id']
assert len(FIX['targets']) == SPEC['target_count'] == 10
assert FIX['subgroup_order'] == full.R == SPEC['subgroup_order']
def save(obj): (HERE/'result.json').write_text(json.dumps(obj,indent=2,sort_keys=True)+'\n')
def timeout(*_): raise TimeoutError('frozen process or arm cap')
signal.signal(signal.SIGALRM,timeout)
result = {'schema':'h4-n35-online-panel-result-v1','status':'running',
          'protocol_sha256':sha(HERE/'protocol.json'), 'fixtures_sha256':SPEC['fixtures_sha256'],
          'candidate_id':C['candidate_id'],
          'host':{'node':platform.node(),'system':platform.system(),'machine':platform.machine(),
                  'tier':'ordinary_exploratory'},'pairs':[]}
save(result)
process_deadline=time.monotonic()+SPEC['process_cap_seconds']
signal.alarm(SPEC['process_cap_seconds'])
try:
    t=time.perf_counter_ns()
    E,elt=full.build_context()
    base,reps,encoding,G=full.build_base(E,elt)
    assert len(base)==4096 and len(reps)==2048
    assert full.point_record(G)==FIX['generator']
    assert [full.point_record(P) for P in base]==C['record']['factor_base']['encoded_points']
    index=full.build_index(base)
    assert len(index)==O['distinct_pair_sums']==8321091
    logs=O['factor_logs']
    assert len(logs)==len(reps)
    assert all(logs[i]*G==reps[i] for i in range(len(reps)))
    result['shared_setup_ns']=time.perf_counter_ns()-t
    result['shared_setup_peak_rss_bytes']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    save(result)
    def calibration():
        start=time.perf_counter_ns();P=G
        for _ in range(512): P+=G
        assert P==513*G
        return time.perf_counter_ns()-start
    for fixture in FIX['targets']:
        i=fixture['index']
        if time.monotonic()>=process_deadline: raise TimeoutError('frozen process cap')
        Q=E(elt(fixture['target'][0]),elt(fixture['target'][1]))
        assert full.point_record(Q)==fixture['target']
        assert full.digest(fixture['target'])==fixture['target_point_sha256']
        order=SPEC['arm_order'][i]
        row={'index':i,'workload_id':fixture['workload_id'],'target':fixture['target'],
             'target_point_sha256':fixture['target_point_sha256'],
             'order':order,'arms':{},'calibration_pre_ns':calibration()}
        for arm in order:
            remaining=process_deadline-time.monotonic()
            if remaining<=0: raise TimeoutError('frozen process cap')
            signal.alarm(min(SPEC['per_arm_cap_seconds'],max(1,int(remaining))))
            start=time.perf_counter_ns()
            try:
                receipt=full.ic_online(base,reps,encoding,G,index,logs,Q) if arm=='ic' else full.rho_online(G,Q)
                receipt['outer_wall_ns']=time.perf_counter_ns()-start
            except TimeoutError as exc:
                receipt={'status':'censored_timeout','scalar':None,
                         'outer_wall_ns':time.perf_counter_ns()-start,'error':str(exc)}
            except Exception as exc:
                receipt={'status':'error','scalar':None,
                         'outer_wall_ns':time.perf_counter_ns()-start,'error':repr(exc)}
            signal.alarm(0)
            row['arms'][arm]=receipt
        row['calibration_post_ns']=calibration()
        a,b=row['calibration_pre_ns'],row['calibration_post_ns']
        row['calibration_ratio']=max(a,b)/min(a,b)
        row['calibration_pass']=row['calibration_ratio']<=SPEC['calibration_ratio_cap']
        row['scalar_certificate_pass']=all(row['arms'].get(arm,{}).get('status')=='verified' and
            row['arms'][arm]['scalar']==fixture['fixture_scalar'] and
            row['arms'][arm]['scalar']*G==Q for arm in ('ic','rho'))
        row['peak_rss_so_far_bytes']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        result['pairs'].append(row);save(result)
        if row['peak_rss_so_far_bytes']>SPEC['observed_rss_cap_bytes']:
            result['status']='censored_memory_limit';break
    if result['status']=='running':
        result['status']='complete' if len(result['pairs'])==10 and all(x['scalar_certificate_pass'] for x in result['pairs']) else 'incomplete'
except TimeoutError as exc:
    result.update(status='censored_process_timeout',error=str(exc))
except Exception as exc:
    result.update(status='error',error=repr(exc))
    raise
finally:
    signal.alarm(0)
    result['peak_rss_bytes']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    save(result)
print(json.dumps({'status':result['status'],'pairs':len(result['pairs']),
                  'shared_setup_ns':result.get('shared_setup_ns'),
                  'peak_rss_bytes':result['peak_rss_bytes']},sort_keys=True))
