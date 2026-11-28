#!/usr/bin/env python3
"""Ten frozen same-point triples: point-key IC, packed-key IC, and rho."""
from sage.all import *
from sage.groups.generic import discrete_log_rho
import hashlib, importlib.util, inspect, json, platform, resource, signal, time
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
SPEC=json.loads((HERE/'protocol.json').read_text())
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
for key in ('point_candidate','point_runner','point_result','packed_candidate','packed_runner','packed_result','prior_panel_fixtures'):
    assert sha(ROOT/SPEC[key])==SPEC[key+'_sha256']
assert sha(HERE/'fixtures.json')==SPEC['fixtures_sha256']
assert sha(HERE/'make_fixtures.py')==SPEC['fixture_generator_sha256']
assert hashlib.sha256(inspect.getsource(discrete_log_rho).encode()).hexdigest()==SPEC['sage_rho_source_sha256']
def load_module(path,name):
    spec=importlib.util.spec_from_file_location(name,ROOT/path)
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
POINT=load_module(SPEC['point_runner'],'h4_point_candidate')
PACKED=load_module(SPEC['packed_runner'],'h4_packed_candidate')
C_POINT=json.loads((ROOT/SPEC['point_candidate']).read_text())
C_PACKED=json.loads((ROOT/SPEC['packed_candidate']).read_text())
O_POINT=json.loads((ROOT/SPEC['point_result']).read_text())
O_PACKED=json.loads((ROOT/SPEC['packed_result']).read_text())
FIX=json.loads((HERE/'fixtures.json').read_text())
assert len(FIX['targets'])==SPEC['target_count']==10
assert C_POINT['candidate_id']==O_POINT['candidate_id']
assert C_PACKED['candidate_id']==O_PACKED['candidate_id']
assert C_POINT['candidate_id']!=C_PACKED['candidate_id']
assert O_POINT['factor_logs']==O_PACKED['factor_logs']
assert O_POINT['target']==O_PACKED['target']
assert O_POINT['status']==O_PACKED['status']=='complete'
def checkpoint(d): (HERE/'result.json').write_text(json.dumps(d,indent=2,sort_keys=True)+'\n')
def timeout(*_): raise TimeoutError('frozen process or per-arm cap')
signal.signal(signal.SIGALRM,timeout)
result={'schema':'h4-n31-index-variant-panel-result-v1','status':'running',
        'protocol_sha256':sha(HERE/'protocol.json'),'fixture_sha256':SPEC['fixtures_sha256'],
        'candidate_ids':{'point_ic':C_POINT['candidate_id'],'packed_ic':C_PACKED['candidate_id']},
        'host':{'node':platform.node(),'system':platform.system(),'machine':platform.machine(),
                'tier':'ordinary_exploratory'},'triples':[]}
checkpoint(result)
process_deadline=time.monotonic()+SPEC['process_cap_seconds']
signal.alarm(SPEC['process_cap_seconds'])
try:
    setup_start=time.perf_counter_ns()
    E,elt=POINT.build_context()
    base,reps,encoding,G=POINT.build_base(E,elt)
    assert len(base)==2090 and len(reps)==1045
    point_index=POINT.build_index(base)
    packed_index=PACKED.build_index(base)
    assert len(point_index)==len(packed_index)==2116611
    logs=O_POINT['factor_logs']
    assert len(logs)==len(reps)
    assert all(logs[i]*G==reps[i] for i in range(len(reps)))
    assert [POINT.point_record(P) for P in base]==C_POINT['record']['factor_base']['encoded_points']==C_PACKED['record']['factor_base']['encoded_points']
    assert POINT.point_record(G)==FIX['generator']
    result['shared_setup_ns']=time.perf_counter_ns()-setup_start
    result['shared_setup_peak_rss_bytes']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    checkpoint(result)
    targets=[E(elt(row['target'][0]),elt(row['target'][1])) for row in FIX['targets']]
    assert all(POINT.point_record(Q)==row['target'] for Q,row in zip(targets,FIX['targets']))
    def coord_hash(P):
        if P==E(0):return 0
        x,y=POINT.point_record(P)
        return ((x*0x9e3779b97f4a7c15)^(y*0xbf58476d1ce4e5b9))&((1<<64)-1)
    def calibration():
        t=time.perf_counter_ns();P=G
        for _ in range(512):P+=G
        assert P==513*G
        return time.perf_counter_ns()-t
    for i,(fixture,Q) in enumerate(zip(FIX['targets'],targets)):
        if time.monotonic()>=process_deadline:raise TimeoutError('frozen process cap')
        row={'index':i,'workload_id':fixture['workload_id'],'target':fixture['target'],
             'target_point_sha256':fixture['target_point_sha256'],
             'order':SPEC['arm_order'][i].split(','),'arms':{},'calibration_pre_ns':calibration()}
        for arm in row['order']:
            remaining=process_deadline-time.monotonic()
            if remaining<=0:raise TimeoutError('frozen process cap')
            signal.alarm(min(SPEC['per_arm_cap_seconds'],max(1,int(remaining))))
            if arm in ('point_ic','packed_ic'):
                module,index=(POINT,point_index) if arm=='point_ic' else (PACKED,packed_index)
                outer=time.perf_counter_ns()
                try:
                    receipt=module.ic_online(base,reps,encoding,G,index,logs,Q)
                    receipt['outer_wall_ns']=time.perf_counter_ns()-outer
                except TimeoutError as exc:
                    receipt={'status':'censored_timeout','scalar':None,'outer_wall_ns':time.perf_counter_ns()-outer,'error':str(exc)}
                except Exception as exc:
                    receipt={'status':'error','scalar':None,'outer_wall_ns':time.perf_counter_ns()-outer,'error':repr(exc)}
            else:
                set_random_seed(95000+i)
                t=time.perf_counter_ns();cpu=time.process_time_ns()
                try:
                    scalar=int(discrete_log_rho(Q,G,ord=SPEC['subgroup_order'],operation='+',hash_function=coord_hash))
                    assert scalar*G==Q
                    receipt={'status':'verified','scalar':scalar,'wall_ns':time.perf_counter_ns()-t,
                             'cpu_ns':time.process_time_ns()-cpu}
                except TimeoutError as exc:
                    receipt={'status':'censored_timeout','scalar':None,'wall_ns':time.perf_counter_ns()-t,
                             'cpu_ns':time.process_time_ns()-cpu,'error':str(exc)}
                except Exception as exc:
                    receipt={'status':'error','scalar':None,'wall_ns':time.perf_counter_ns()-t,
                             'cpu_ns':time.process_time_ns()-cpu,'error':repr(exc)}
            signal.alarm(0)
            row['arms'][arm]=receipt
        row['calibration_post_ns']=calibration()
        a,b=row['calibration_pre_ns'],row['calibration_post_ns']
        row['calibration_ratio']=max(a,b)/min(a,b)
        row['calibration_pass']=row['calibration_ratio']<=1.25
        row['scalar_certificate_pass']=all(row['arms'].get(arm,{}).get('status')=='verified' and
            row['arms'][arm]['scalar']==fixture['fixture_scalar'] and row['arms'][arm]['scalar']*G==Q
            for arm in ('point_ic','packed_ic','rho'))
        row['peak_rss_so_far_bytes']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        result['triples'].append(row);checkpoint(result)
        if row['peak_rss_so_far_bytes']>SPEC['observed_rss_cap_bytes']:
            result['status']='censored_memory_limit';break
    if result['status']=='running':
        result['status']='complete' if len(result['triples'])==10 and all(r['scalar_certificate_pass'] for r in result['triples']) else 'incomplete'
except TimeoutError as exc:
    result.update(status='censored_process_timeout',error=str(exc))
finally:
    signal.alarm(0)
    result['peak_rss_bytes']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    checkpoint(result)
print(json.dumps({'status':result['status'],'triples':len(result['triples']),
                  'shared_setup_ns':result.get('shared_setup_ns'),
                  'peak_rss_bytes':result['peak_rss_bytes']},sort_keys=True))
