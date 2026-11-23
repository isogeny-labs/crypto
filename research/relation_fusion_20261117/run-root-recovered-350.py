from pathlib import Path
import subprocess,json,hashlib,os,re,datetime
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117'
# Called only after348 compiler reports exit0. Refuse to convert build warnings into a pass.
b=(s/'development-01/root-unit-build-348.stderr').read_text();assert 'Finished `release`' in b and 'Executable unittests' in b and 'error:' not in b
src=json.loads((s/'ROOT_UNIT_SOURCE_348.json').read_text());assert all(hashlib.sha256((repo/p).read_bytes()).hexdigest()==h for p,h in src['source_sha256'].items())
match=re.search(r'Executable unittests .*? \(([^\n]+)\)',b);assert match;exe=Path(match[1]);assert exe.is_file(),exe
ctx={'schema':'root-recovered-unit-execution-context/v1','source':'ROOT_UNIT_SOURCE_348.json','plan':'ROOT_RECOVERED_UNIT_PLAN_346.json','executable':str(exe),'executable_sha256':hashlib.sha256(exe.read_bytes()).hexdigest(),'recorded_before_unit_execution':True,'no_run348_exit0':True,'signed_execution_verdict':False,'performance_comparison':False};(s/'ROOT_RECOVERED_UNIT_CONTEXT_350.json').write_text(json.dumps(ctx,indent=2)+'\n')
filters=json.loads((s/'ROOT_RECOVERED_UNIT_PLAN_346.json').read_text())['filters']
cmd=['taskset','-c','0',str(exe),*filters,'--test-threads=1']
env=dict(os.environ,RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',RUST_TEST_THREADS='1')
with (s/'development-01/root-recovered-tests-350.stdout').open('xb') as out,(s/'development-01/root-recovered-tests-350.stderr').open('xb') as err:
 r=subprocess.run(cmd,cwd=repo,env=env,stdout=out,stderr=err)
stdout=(s/'development-01/root-recovered-tests-350.stdout').read_text();summary=re.findall(r'test result: ([^\n]+)',stdout)
result={'schema':'root-recovered-unit-outcome/v1','source':'ROOT_UNIT_SOURCE_348.json','plan':'ROOT_RECOVERED_UNIT_PLAN_346.json','exit_code':r.returncode,'summary':summary,'failed_tests':re.findall(r'^test (.+?) \.\.\. FAILED$',stdout,re.M),'stdout_sha256':hashlib.sha256((s/'development-01/root-recovered-tests-350.stdout').read_bytes()).hexdigest(),'stderr_sha256':hashlib.sha256((s/'development-01/root-recovered-tests-350.stderr').read_bytes()).hexdigest(),'full_root_unit_suite':False,'signed_execution_verdict':False,'performance_comparison':False,'15x_admitted':False}
(s/'ROOT_RECOVERED_UNIT_OUTCOME_350.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result));raise SystemExit(r.returncode)
