"""Bounded, supplementary split-composition panel; separate from primary."""
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess
import sys
import time

os.sched_setaffinity(0,{1})
ROOT=Path(__file__).parent
sys.path.insert(0,'/workspace/ecdlp_research/repo_controls')
from controls import verify_witness
from candidate import nmod_poly


def main():
    manifest=json.loads(Path('/workspace/ecdlp_research/prime_fixtures/manifest.json').read_text())
    entries=[e for e in manifest['fixtures'] if e['kind']=='natural' and e['id'].endswith(('_c0_natural_s0','_c0_natural_s1'))]
    entries.append({'path':str(ROOT/'uninformative_first_control.json')})
    truth={r['fixture_id']:r for r in json.loads((ROOT/'audit_results.json').read_text())['records']}
    records=[];outdir=ROOT/'compose_results';outdir.mkdir(exist_ok=True)
    for entry in entries:
        fixture=json.loads(Path(entry['path']).read_text());output=outdir/(fixture['id']+'.json')
        command=[sys.executable,str(ROOT/'compose_candidate.py'),'--fixture',entry['path'],'--output',str(output)]
        before=resource.getrusage(resource.RUSAGE_CHILDREN);start=time.perf_counter()
        completed=subprocess.run(command,capture_output=True,text=True,timeout=40)
        after=resource.getrusage(resource.RUSAGE_CHILDREN)
        if completed.returncode:raise RuntimeError(completed.stderr)
        result=json.loads(output.read_text())
        result['external_process_wall_seconds']=time.perf_counter()-start
        result['external_process_cpu_seconds']=after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime
        result['process_startup_and_serialization_cpu_seconds']=max(0.,result['external_process_cpu_seconds']-result['batch_cpu_seconds'])
        result['pool_generation_cpu_seconds']=fixture['generation']['cpu_seconds']
        result['complete_call_plus_pool_cpu_seconds']=result['solver_total_cpu_seconds']+result['pool_generation_cpu_seconds']
        reference='b016_c1_natural_s1' if fixture['kind']=='recovery_regression' else fixture['id']
        G=nmod_poly(result['gcd_coefficients'],fixture['curve']['p'])
        roots=sorted(row['x'] for row in fixture['pools'][2] if int(G(row['x']))==0)
        if roots!=truth[reference]['explicit_incidence_roots']:raise AssertionError('Composition root parity failed')
        if result['status']=='relation':verify_witness(fixture,result['witness'])
        if fixture['kind']=='recovery_regression' and not result.get('uninformative_matches_skipped'):
            raise AssertionError('Regression did not exercise skipped witness')
        result['root_parity']=True;result['witness_verified']=result['status']=='relation'
        records.append(result)
        print(json.dumps({k:result[k] for k in ('fixture_id','B','status','solver_total_cpu_seconds')}),flush=True)
    report={'schema':'incidence-compose-panel/v1','source_sha256':hashlib.sha256((ROOT/'compose_candidate.py').read_bytes()).hexdigest(),
        'base_source_sha256':hashlib.sha256((ROOT/'candidate.py').read_bytes()).hexdigest(),
        'scope':'Supplement, two natural seeds on one curve per size plus one recovery regression; exact roots versus separately enumerated group oracle; not an independent scaling study.',
        'records':records}
    (ROOT/'compose_panel_results.json').write_text(json.dumps(report,indent=2)+'\n')


if __name__=='__main__':main()
