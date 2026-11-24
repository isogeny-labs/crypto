#!/usr/bin/env python3
"""Run a frozen matched panel, preserving every child outcome."""
import argparse, hashlib, json, os, platform, signal, subprocess, sys, time
from pathlib import Path

HERE=Path(__file__).resolve().parent
MODES=('independent','walk','two-group')

def digest(path): return hashlib.sha256(path.read_bytes()).hexdigest()

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
    source_before=digest(HERE/'measure.py')
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
            receipt={'seed':seed,'mode':mode,'argv':cmd,'status':status,'exit_code':rc,'whole_process_wall_ns':elapsed,'result_status':outcome,'result_sha256':digest(result) if result.exists() else None,'checkpoint_sha256':digest(checkpoint) if checkpoint.exists() else None,'parse_error':parse_error,'useful_relations':data.get('useful_relations') if outcome=='complete' else None,'target':data.get('target') if outcome is not None else None}
            (out/(name+'.receipt.json')).write_text(json.dumps(receipt,indent=2,sort_keys=True)+'\n')
            receipts.append(receipt)
    pairing={}
    for seed in seeds:
        targets=[r['target'] for r in receipts if r['seed']==seed]
        pairing[str(seed)] = None if any(t is None for t in targets) else all(t==targets[0] for t in targets)
    source_after=digest(HERE/'measure.py')
    summary={'target_pairing_by_seed':pairing,'measure_sha256_at_start':source_before,'measure_sha256_at_end':source_after,'measure_source_unchanged':source_before==source_after,'plan_sha256':digest(out/'plan.json'),'measure_sha256':digest(HERE/'measure.py'),'panel_sha256':digest(Path(__file__)),'platform':platform.platform(),'python':sys.version,'runs':receipts,'per_mode':{mode:{'attempts':sum(r['mode']==mode for r in receipts),'completed':sum(r['mode']==mode and r['result_status']=='complete' for r in receipts),'useful_runs':sum(r['mode']==mode and (r['useful_relations'] or 0)>0 for r in receipts),'failed_or_incomplete':sum(r['mode']==mode and r['result_status']!='complete' for r in receipts),'total_process_wall_ns':sum(r['whole_process_wall_ns'] for r in receipts if r['mode']==mode)} for mode in MODES}}
    (out/'summary.json').write_text(json.dumps(summary,indent=2,sort_keys=True)+'\n')
    print(json.dumps(summary['per_mode'],sort_keys=True))

if __name__=='__main__': main()
