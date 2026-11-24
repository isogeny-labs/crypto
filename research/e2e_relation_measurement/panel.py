#!/usr/bin/env python3
"""Run a frozen matched panel, preserving every child outcome."""
import argparse, hashlib, json, os, platform, resource, signal, subprocess, sys, time
from pathlib import Path

HERE=Path(__file__).resolve().parent
MODES=('independent','walk','two-group')

def digest(path): return hashlib.sha256(path.read_bytes()).hexdigest()


def resource_metadata():
    """Parent settings inherited by children; not an isolation attestation."""
    cpu = platform.processor() or None
    if sys.platform == 'darwin':
        try:
            value = subprocess.run(['sysctl','-n','machdep.cpu.brand_string'],capture_output=True,text=True,timeout=2,check=True).stdout.strip()
            cpu = value or cpu
        except (OSError, subprocess.SubprocessError):
            pass
    elif sys.platform.startswith('linux'):
        try:
            for line in Path('/proc/cpuinfo').read_text().splitlines():
                if line.lower().startswith('model name'):
                    cpu=line.split(':',1)[1].strip(); break
        except (OSError, IndexError):
            pass
    try: affinity=sorted(os.sched_getaffinity(0))
    except (AttributeError, OSError): affinity=None
    limits={}
    for name in ('RLIMIT_AS','RLIMIT_CPU','RLIMIT_DATA','RLIMIT_RSS','RLIMIT_NPROC'):
        if hasattr(resource,name):
            try: limits[name]=list(resource.getrlimit(getattr(resource,name)))
            except (ValueError,OSError): limits[name]=None
    variables=('OMP_NUM_THREADS','OPENBLAS_NUM_THREADS','MKL_NUM_THREADS','NUMEXPR_NUM_THREADS')
    return {'system':platform.system(),'release':platform.release(),'machine':platform.machine(),
            'cpu_model':cpu,'logical_cpus':os.cpu_count(),'affinity':affinity,
            'parent_limits':limits,'thread_environment':{key:os.environ.get(key) for key in variables},
            'rss_native_unit':'bytes' if sys.platform=='darwin' else 'KiB' if sys.platform.startswith('linux') else 'platform-dependent'}

PHASES = ('generation', 'polynomial_construction', 'groebner_basis', 'root_extraction', 'relation_verification')
CANDIDATE_COUNTS = ('candidates', 'identity', 'filtered', 'duplicate_x')
GENERATION_OPERATIONS = ('target_draws', 'scalar_multiplication_calls', 'point_addition_calls')


def coverage_sum(records, field, key):
    values = []
    for record in records:
        item = record.get(field)
        value = item.get(key) if isinstance(item, dict) else None
        if field == 'observed_timings' and isinstance(value, dict):
            value = value.get('wall_ns')
        if type(value) is int and value >= 0:
            values.append(value)
    return {'observed_attempts': len(values), 'unknown_attempts': len(records)-len(values),
            'observed_sum': sum(values) if values else None}


def coverage_max(records, field, key):
    values=[record.get(field,{}).get(key) for record in records if isinstance(record.get(field),dict)]
    values=[value for value in values if type(value) is int and value >= 0]
    return {'observed_attempts':len(values), 'unknown_attempts':len(records)-len(values),
            'observed_max':max(values) if values else None}


def mode_summary(receipts, mode):
    rows = [r for r in receipts if r['mode'] == mode]
    return {
        'attempts': len(rows),
        'completed': sum(r['result_status'] == 'complete' for r in rows),
        'useful_runs': sum(r['result_status'] == 'complete' and (r['useful_relations'] or 0) > 0 for r in rows),
        'nonzero_b_runs': sum(r['relation_counts'] is not None and r['relation_counts']['nonzero_b'] > 0 for r in rows),
        'nonzero_b_unknown_runs': sum(r['relation_counts'] is None for r in rows),
        'verification_failed_runs': sum(r['result_status'] == 'verification_failed' for r in rows),
        'failed_or_incomplete': sum(r['result_status'] != 'complete' for r in rows),
        'total_process_wall_ns': sum(r['whole_process_wall_ns'] for r in rows),
        'driver_cpu_ns': coverage_sum(rows, 'observed_total', 'cpu_ns'),
        'peak_rss_native': coverage_max(rows, 'observed_total', 'peak_rss_native'),
        'phase_wall_ns': {name: coverage_sum(rows, 'observed_timings', name) for name in PHASES},
        'candidate_counts': {name: coverage_sum(rows, 'candidate_counts', name) for name in CANDIDATE_COUNTS},
        'generation_operations': {name: coverage_sum(rows, 'generation_operations', name) for name in GENERATION_OPERATIONS},
    }


def main():
    p=argparse.ArgumentParser(); p.add_argument('--plan',required=True); p.add_argument('--out',required=True)
    args=p.parse_args(); plan_path=Path(args.plan); plan=json.loads(plan_path.read_text())
    if set(plan) != {'common_args','seeds','timeout_seconds'}: p.error('plan requires common_args, seeds and timeout_seconds')
    common=plan['common_args']; seeds=plan['seeds']; timeout=plan['timeout_seconds']
    if not isinstance(common,list) or not all(isinstance(x,str) for x in common): p.error('common_args must be strings')
    if any(x.split('=')[0] in ('--mode','--seed','--output','--checkpoint') for x in common): p.error('common_args cannot override mode, seed or output')
    if not isinstance(seeds,list) or not seeds or any(type(x)!=int for x in seeds) or len(seeds)!=len(set(seeds)): p.error('seeds must be distinct integers')
    if not isinstance(timeout,(int,float)) or not 0 < timeout <= 86400: p.error('invalid timeout')
    out=Path(args.out); out.mkdir(parents=True,exist_ok=False)
    (out/'plan.json').write_bytes(plan_path.read_bytes())
    receipts=[]
    resource_at_start=resource_metadata()
    source_before=digest(HERE/'measure.py')
    accounting_before=digest(HERE/'accounting.py')
    for seed in seeds:
        for mode in MODES:
            name=f'seed-{seed}-{mode}'; result=out/(name+'.result.json'); checkpoint=out/(name+'.checkpoint.json')
            cmd=['sage','-python',str(HERE/'measure.py'),*common,'--mode',mode,'--seed',str(seed),'--output',str(result.resolve()),'--checkpoint',str(checkpoint.resolve())]
            started=time.perf_counter_ns()
            try:
                proc=subprocess.Popen(cmd,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
                try:
                    stdout,stderr=proc.communicate(timeout=timeout); status='exited'
                except subprocess.TimeoutExpired:
                    os.killpg(proc.pid,signal.SIGKILL); stdout,stderr=proc.communicate(); status='timeout'
                rc=proc.returncode
            except OSError as exc:
                stdout=b''; stderr=str(exc).encode(); status='launch_error'; rc=None
            elapsed=time.perf_counter_ns()-started
            (out/(name+'.stdout')).write_bytes(stdout); (out/(name+'.stderr')).write_bytes(stderr)
            data=None; parse_error=None
            if result.exists():
                try: data=json.loads(result.read_text())
                except (ValueError,OSError) as exc: parse_error=str(exc)
            valid = isinstance(data,dict) and data.get('schema')=='e2e-relation-measurement/v1' and data.get('config',{}).get('seed')==seed and data.get('config',{}).get('mode')==mode
            outcome=data.get('status') if valid and status=='exited' and rc==0 else None
            if data is not None and not valid: parse_error='invalid schema, seed or mode'
            snapshot=None
            if checkpoint.exists():
                try:
                    candidate=json.loads(checkpoint.read_text())
                    if isinstance(candidate,dict) and candidate.get('schema')=='e2e-relation-measurement/v1' and candidate.get('config',{}).get('seed')==seed and candidate.get('config',{}).get('mode')==mode:
                        snapshot=candidate
                except (OSError,ValueError):
                    pass
            observed=data if outcome is not None else snapshot
            receipt={'seed':seed,'mode':mode,'argv':cmd,'status':status,'exit_code':rc,'stdout_sha256':hashlib.sha256(stdout).hexdigest(),'stderr_sha256':hashlib.sha256(stderr).hexdigest(),'whole_process_wall_ns':elapsed,'result_status':outcome,'result_sha256':digest(result) if result.exists() else None,'checkpoint_sha256':digest(checkpoint) if checkpoint.exists() else None,'parse_error':parse_error,'useful_relations':data.get('useful_relations') if outcome in ('complete','verification_failed') else None,'relation_counts':data.get('relation_counts') if outcome in ('complete','verification_failed') else None,'candidate_counts':observed.get('counts') if observed is not None else None,'observed_timings':observed.get('timings') if observed is not None else None,'observed_total':observed.get('total') if observed is not None else None,'generation_operations':observed.get('generation_operations') if observed is not None else None,'observation_source':('result' if outcome is not None else 'checkpoint' if snapshot is not None else None),'target':data.get('target') if outcome is not None else None}
            (out/(name+'.receipt.json')).write_text(json.dumps(receipt,indent=2,sort_keys=True)+'\n')
            receipts.append(receipt)
    pairing={}
    for seed in seeds:
        targets=[r['target'] for r in receipts if r['seed']==seed]
        pairing[str(seed)] = None if any(t is None for t in targets) else all(t==targets[0] for t in targets)
    source_after=digest(HERE/'measure.py')
    summary={'resource_metadata_at_start':resource_at_start,'resource_metadata_at_end':resource_metadata(),'target_pairing_by_seed':pairing,'measure_sha256_at_start':source_before,'measure_sha256_at_end':source_after,'measure_source_unchanged':source_before==source_after,'accounting_sha256_at_start':accounting_before,'accounting_sha256_at_end':digest(HERE/'accounting.py'),'accounting_source_unchanged':accounting_before==digest(HERE/'accounting.py'),'plan_sha256':digest(out/'plan.json'),'measure_sha256':digest(HERE/'measure.py'),'panel_sha256':digest(Path(__file__)),'platform':platform.platform(),'python':sys.version,'runs':receipts,'per_mode':{mode:mode_summary(receipts,mode) for mode in MODES}}
    (out/'summary.json').write_text(json.dumps(summary,indent=2,sort_keys=True)+'\n')
    print(json.dumps(summary['per_mode'],sort_keys=True))

if __name__=='__main__': main()
