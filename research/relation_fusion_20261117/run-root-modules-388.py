from pathlib import Path
import hashlib,json,subprocess,os,resource,signal,datetime,re
r=Path('/workspace/relation-supervisor-live');s=r/'research/relation_fusion_20261117';exe=Path('/workspace/root-fixture-modules-387');sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
source=json.loads((s/'ROOT_FIXTURE_SOURCE_387.json').read_text());assert all(sha(r/k)==v for k,v in source['root_changes'].items())
roster=subprocess.run([str(exe),'--list'],capture_output=True,text=True,check=True).stdout;(s/'development-01/root-fixture-roster-388.stdout').write_text(roster);assert roster.rstrip().endswith('7 tests, 0 benchmarks'),roster
plan={'schema':'actual-root-fixture-module-plan/v1','recorded_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source':'ROOT_FIXTURE_SOURCE_387.json','executable':str(exe),'executable_sha256':sha(exe),'roster_sha256':sha(s/'development-01/root-fixture-roster-388.stdout'),'expected_tests':7,'command':['taskset','-c','0',str(exe),'--test-threads=1','--nocapture'],'cpu':0,'threads':1,'wall_limit_seconds':600,'address_space_bytes':12*1024**3,'each_raw_file_limit_bytes':16*1024**2,'scope':'All original tests in two actual-source modules; not a full root target runtime or rebuild','performance_comparison':False,'15x_admitted':False};(s/'ROOT_FIXTURE_PLAN_388.json').write_text(json.dumps(plan,indent=2)+'\n')
def limits():resource.setrlimit(resource.RLIMIT_AS,(12*1024**3,12*1024**3));resource.setrlimit(resource.RLIMIT_FSIZE,(16*1024**2,16*1024**2))
out=s/'development-01/root-fixture-tests-388.stdout';err=s/'development-01/root-fixture-tests-388.stderr'
with out.open('xb') as o,err.open('xb') as e:
 p=subprocess.Popen(plan['command'],stdout=o,stderr=e,env=dict(os.environ,RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',RUST_TEST_THREADS='1'),preexec_fn=limits,start_new_session=True);timeout=False
 try:code=p.wait(timeout=600)
 except subprocess.TimeoutExpired:
  timeout=True;os.killpg(p.pid,signal.SIGTERM)
  try:code=p.wait(timeout=5)
  except subprocess.TimeoutExpired:os.killpg(p.pid,signal.SIGKILL);code=p.wait()
result={'schema':'actual-root-fixture-module-outcome/v1','plan':'ROOT_FIXTURE_PLAN_388.json','exit_code':code,'wall_timeout':timeout,'summary':re.findall(r'test result: ([^\n]+)',out.read_text()),'stdout_sha256':sha(out),'stderr_sha256':sha(err),'root_source_pins_unchanged_after':all(sha(r/k)==v for k,v in source['root_changes'].items()),'executable_sha256_unchanged_after':sha(exe)==plan['executable_sha256'],'full_root_suite_qualified':False,'performance_comparison':False,'15x_admitted':False};(s/'ROOT_FIXTURE_OUTCOME_388.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
