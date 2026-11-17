"""Fresh-process, CPU-pinned bounded incidence scaling panel."""
import hashlib
import argparse
import json
import os
from pathlib import Path
import resource
import statistics
import subprocess
import sys
import time

os.sched_setaffinity(0,{1})
ROOT=Path(__file__).parent


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--candidate',type=Path,default=ROOT/'timed_candidate_primary.py',
        help='The published primary panel is reproduced with the preserved timed snapshot.')
    parser.add_argument('--output-stem',default='benchmark')
    args=parser.parse_args()
    manifest_path=Path('/workspace/ecdlp_research/prime_fixtures/manifest.json')
    manifest=json.loads(manifest_path.read_text())
    results=ROOT/'results';results.mkdir(exist_ok=True)
    records=[]
    for entry in manifest['fixtures']:
        for arm in ('horner','blocked'):
            output=results/(entry['id']+'_'+arm+'.json')
            command=[sys.executable,str(args.candidate),'--fixture',entry['path'],
                     '--arm',arm,'--min-batch-wall','0.08','--cpu-cap','30','--output',str(output)]
            before=resource.getrusage(resource.RUSAGE_CHILDREN);start=time.perf_counter()
            try:
                completed=subprocess.run(command,capture_output=True,text=True,timeout=40)
            except subprocess.TimeoutExpired as error:
                completed=subprocess.CompletedProcess(command,-999,
                    error.stdout.decode() if isinstance(error.stdout,bytes) else (error.stdout or ''),
                    error.stderr.decode() if isinstance(error.stderr,bytes) else (error.stderr or ''))
            wall=time.perf_counter()-start;after=resource.getrusage(resource.RUSAGE_CHILDREN)
            (results/(entry['id']+'_'+arm+'.stderr')).write_text(completed.stderr)
            row={'fixture_id':entry['id'],'B':entry['B'],'kind':entry['kind'],'arm':arm,
                 'returncode':completed.returncode,'external_process_wall_seconds':wall,
                 'external_process_cpu_seconds':after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime}
            if completed.returncode==0:
                result=json.loads(output.read_text());row.update(result)
                gen=result['fixture_generation']['cpu_seconds']
                row['pool_generation_cpu_seconds']=gen
                row['complete_call_plus_pool_cpu_seconds']=gen+result['solver_total_cpu_seconds']
                row['process_startup_and_serialization_cpu_seconds']=max(0.0,row['external_process_cpu_seconds']-result['batch_cpu_seconds'])
            else:
                row['status']='censored' if completed.returncode<0 else 'error'
                row['stdout_tail']=completed.stdout[-2000:]
            records.append(row)
            (ROOT/(args.output_stem+'_results.json')).write_text(json.dumps({'schema':'incidence-benchmark/v1','records':records},indent=2)+'\n')
            print(json.dumps({'id':entry['id'],'B':entry['B'],'arm':arm,'status':row['status'],
                              'solver_cpu':row.get('solver_total_cpu_seconds'),'total_plus_pool':row.get('complete_call_plus_pool_cpu_seconds')}),flush=True)
    summary=[]
    for kind in sorted({r['kind'] for r in records}):
      for B in sorted({r['B'] for r in records if r['kind']==kind}):
        for arm in ('horner','blocked'):
            rows=[r for r in records if r['B']==B and r['kind']==kind and r['arm']==arm and r['returncode']==0]
            if not rows:
                summary.append({'B':B,'kind':kind,'arm':arm,'cases':0})
                continue
            summary.append({'B':B,'kind':kind,'arm':arm,'cases':len(rows),
                'relations':sum(r['status']=='relation' for r in rows),'misses':sum(r['status']=='none' for r in rows),
                'median_solver_cpu_seconds':statistics.median(r['solver_total_cpu_seconds'] for r in rows),
                'median_complete_call_plus_pool_cpu_seconds':statistics.median(r['complete_call_plus_pool_cpu_seconds'] for r in rows),
                'max_rss_kib':max(r['peak_process_rss_kib'] for r in rows),
                'quotient_multiplications_per_case':rows[0]['counters']['quotient_multiplications']})
    report={'schema':'incidence-benchmark-summary/v1','source_sha256':hashlib.sha256(args.candidate.read_bytes()).hexdigest(),
            'timed_source':str(args.candidate),
            'manifest_sha256':hashlib.sha256(manifest_path.read_bytes()).hexdigest(),
            'measurement_scope':'CPU1 pinned, single native thread, shared host exploratory. Fresh process per fixture/arm; complete-call repetitions to resolve short CPU intervals. Pool generation separately charged; process startup retained separately.',
            'summary':summary,'claim':'Exact implicit-incidence implementations; no new sub-B^1.5 bound or ECDLP speedup asserted.'}
    (ROOT/(args.output_stem+'_summary.json')).write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report,indent=2))


if __name__=='__main__':main()
