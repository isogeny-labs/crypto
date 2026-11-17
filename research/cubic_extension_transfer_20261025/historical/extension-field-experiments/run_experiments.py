#!/usr/bin/env sage -python
"""Reproducible bounded sweeps. All experiments run locally without network."""
import argparse
from collections import Counter
from datetime import datetime,timezone
import hashlib
import json
from pathlib import Path
import platform
import subprocess
import sys
from time import perf_counter
from sage.all import version

ROOT=Path(__file__).resolve().parent


def plan(preset,replicates,seed):
    cases=[]
    def add(family,**args): cases.append({'family':family,'args':args})
    ps=[7] if preset=='quick' else [7,11]
    for p in ps:
        for n in range(2,7):
            add('trace_zero_structure',p=p,n=n,enumerate_kernel=p**n<=625)
        for n in [3,5]: add('trace_zero_relations',p=p,n=n)
    for p,m in ([(11,1),(5,2)] if preset=='quick' else [(11,1),(17,1),(23,1),(5,2),(5,3)]):
        add('quadratic_genus_two_transfer',p=p,m=m)
    for p,m in ([(5,1),(5,2)] if preset=='quick' else [(5,1),(11,1),(13,1),(5,2)]):
        add('cubic_genus_three_transfer',p=p,m=m)
    if preset!='quick':
        # A deliberate parameter set with only tiny 2/3-primary factors.
        add('cubic_genus_three_transfer',p=7,m=1)
        cases[-1]['expected_status']='ineligible'
    for p in ([11] if preset=='quick' else [11,13]):
        for n,d in [(4,2),(6,2),(6,3)]:
            add('intermediate_field_controls',p=p,n=n,d=d)
    for n in ([3,5] if preset=='quick' else [2,3,4,5,6]):
        add('binary_ghs_geometry_screen',s=2,n=n,samples=8)
    for family in ['tian_cubic_kernel','tian_cubic_jacobian','tian_cubic_rational_map']:
        add(family)
    if preset=='sweep':
        for p in [17,23,31]:
            for n in [3,5]: add('trace_zero_relations',p=p,n=n)
        for p in [29,41,59]: add('quadratic_genus_two_transfer',p=p,m=1)
        for p in [17,19,23]: add('cubic_genus_three_transfer',p=p,m=1)
    out=[]
    for rep in range(replicates):
        for case in cases:
            raw=json.dumps([case,rep,seed],sort_keys=True).encode()
            digest=hashlib.sha256(raw).hexdigest()
            out.append(dict(case,case_id=case['family']+'-'+digest[:10],
                            seed=int(digest[:8],16),replicate=rep))
    return out


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--preset',choices=['quick','standard','sweep'],default='standard')
    parser.add_argument('--replicates',type=int,default=1)
    parser.add_argument('--seed',type=int,default=20261024)
    parser.add_argument('--timeout',type=float,default=180)
    parser.add_argument('--family',action='append',help='Run only these family names (repeatable)')
    parser.add_argument('--output',default='results/standard')
    parser.add_argument('--plan-only',action='store_true')
    args=parser.parse_args()
    if args.replicates<1 or args.timeout<=0: parser.error('Positive replicates and timeout required')
    cases=plan(args.preset,args.replicates,args.seed)
    if args.family: cases=[c for c in cases if c['family'] in args.family]
    if not cases: parser.error('No experiments selected')
    output=Path(args.output)
    if not output.is_absolute(): output=ROOT/output
    output.mkdir(parents=True,exist_ok=True); (output/'logs').mkdir(exist_ok=True)
    metadata={'started_utc':datetime.now(timezone.utc).isoformat(),'sage':version(),
              'python':platform.python_version(),'platform':platform.platform(),
              'machine':platform.machine(),'preset':args.preset,'replicates':args.replicates,
              'master_seed':args.seed,'per_case_timeout_seconds':args.timeout,
              'source_sha256':{p.name:hashlib.sha256(p.read_bytes()).hexdigest()
                               for p in sorted(ROOT.glob('*.py'))},
              'data_sha256':hashlib.sha256((ROOT/'tian_data.json').read_bytes()).hexdigest(),
              'container_image':'sagemath/sagemath:10.6',
              'container_digest':'sha256:19995db6194f4a4bab18ce9a88556fd15b9ed5e916b4504fefe618a7796ddbdb'}
    (output/'plan.json').write_text(json.dumps(cases,indent=2)+'\n')
    if args.plan_only:
        print(json.dumps(cases,indent=2)); return 0
    rows=[]; started=perf_counter()
    with (output/'results.jsonl').open('w') as stream:
        for i,case in enumerate(cases,1):
            print(f"[{i}/{len(cases)}] {case['family']} {case['args']} ...",flush=True)
            before=perf_counter(); stdout=''; stderr=''
            try:
                proc=subprocess.run([sys.executable,str(ROOT/'case_worker.py'),json.dumps(case)],
                                    cwd=ROOT,text=True,capture_output=True,timeout=args.timeout)
                stdout,stderr=proc.stdout,proc.stderr
                row=json.loads(stdout.strip().splitlines()[-1])
                if proc.returncode and row['status']!='error':
                    row.update(status='error',reason='Worker returned a nonzero exit code')
            except subprocess.TimeoutExpired as exc:
                stdout=exc.stdout or ''; stderr=exc.stderr or ''
                if isinstance(stdout,bytes): stdout=stdout.decode(errors='replace')
                if isinstance(stderr,bytes): stderr=stderr.decode(errors='replace')
                row={'family':case['family'],'status':'timeout','reason':'Per-case time limit',
                     'parameters':case['args'],'seed':case['seed'],'replicate':case['replicate'],
                     'case_id':case['case_id']}
            except Exception as exc:
                row={'family':case['family'],'status':'error','reason':str(exc),
                     'parameters':case['args'],'case_id':case['case_id']}
            row['worker_wall_seconds']=perf_counter()-before
            expected=case.get('expected_status','passed')
            row['expected_status']=expected
            row['expectation_met']=row['status']==expected
            (output/'logs'/f"{case['case_id']}.txt").write_text(stdout+'\nSTDERR\n'+stderr)
            stream.write(json.dumps(row)+'\n'); stream.flush(); rows.append(row)
            (output/'results.json').write_text(json.dumps({'metadata':metadata,'experiments':rows},indent=2)+'\n')
            print(f"  {row['status']} ({row['worker_wall_seconds']:.2f}s)",flush=True)
    metadata.update(completed_utc=datetime.now(timezone.utc).isoformat(),
                    elapsed_seconds=perf_counter()-started,
                    status_counts=dict(Counter(r['status'] for r in rows)),
                    expectations_met=sum(r['expectation_met'] for r in rows))
    (output/'results.json').write_text(json.dumps({'metadata':metadata,'experiments':rows},indent=2)+'\n')
    from report import generate_report
    generate_report(output)
    print(json.dumps(metadata['status_counts']),flush=True)
    print('Report: '+str(output/'report.html'),flush=True)
    return int(any(not r['expectation_met'] for r in rows))


if __name__=='__main__': sys.exit(main())
