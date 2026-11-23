from pathlib import Path
import json,hashlib,subprocess,os,resource,signal,datetime,re
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';initial=json.loads((s/'ROOT_FULL_OUTCOME_380.json').read_text());assert initial['full_suite_qualified'] is False and initial['source_pins_unchanged_after'] is True
ctx=json.loads((s/'ROOT_RECOVERED_UNIT_CONTEXT_350.json').read_text());exe=Path(ctx['executable']);assert hashlib.file_digest(exe.open('rb'),'sha256').hexdigest()==ctx['executable_sha256'];names=initial['observed_failed_test_names'];assert len(names)==2,names
plan={'schema':'root-failed-unit-diagnostic-replay-plan/v1','recorded_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source':'ROOT_UNIT_SOURCE_348.json','initial_failed_partial_run':'ROOT_FULL_OUTCOME_380.json','executable':str(exe),'executable_sha256':ctx['executable_sha256'],'exact_tests':names,'purpose':'Get panic/assert details, compare isolated with original full-run failures. A passing isolated replay does not erase full-run failure or qualify full suite.','flags':['--exact','--test-threads=1','--nocapture'],'cpu':0,'threads':1,'wall_limit_seconds_each':600,'address_space_bytes':12*1024**3,'each_raw_output_file_limit_bytes':16*1024**2,'input_seed_or_assertion_changes':False,'performance_observations_collected':0,'15x_admitted':False};(s/'ROOT_FAILURE_REPLAY_PLAN_384.json').write_text(json.dumps(plan,indent=2)+'\n')
p=repo/'docs/session-logs/2026-11-23.md';p.write_text(p.read_text()+f'''\n## {datetime.datetime.now(datetime.timezone.utc).isoformat()} —380partial terminal receipt;384exact diagnostics

### What was done (commands, files, features)
380terminates with ownedSIGTERM exit-15, no libtest final summary, source348 and exactELFhash unchanged, full_suite_qualified=false.383intentional-stop reason retained; every observed completed-test name is in380outcome.384declares exactly the two reported failed names, same source/executable/inputs/assertions, isolated processes with --nocapture,CPU0/tests1,600s/12GiB/16MiBraw-file bound each.

### Decisions made (and why)
Extract actual assertion/coverage failures before editing. Do not infer root cause from a test name. If isolated passes, keep original full-run failure and investigate order/global-state effects. No retries to manufacture a green full result.

### Open threads (unfinished, pick up next time)
Finish384diagnostics, inspect exact counterexample/line, then choose smallest supported correction or report inherited blocker. Protocol decision remains pending; no actualspeed dataset.

### Errors (what went wrong, how it was handled)
380is incomplete by explicit evidence-directed stop, not wall timeout or successful full suite. Its panic messages were internally buffered, hence isolated diagnostic replay.\n''')
def limits():resource.setrlimit(resource.RLIMIT_AS,(12*1024**3,12*1024**3));resource.setrlimit(resource.RLIMIT_FSIZE,(16*1024**2,16*1024**2))
env=dict(os.environ,RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',RUST_TEST_THREADS='1');rows=[]
for i,name in enumerate(names):
 stdout=s/f'development-01/root-failure-{i}-384.stdout';stderr=s/f'development-01/root-failure-{i}-384.stderr'
 with stdout.open('xb') as out,stderr.open('xb') as err:
  child=subprocess.Popen(['taskset','-c','0',str(exe),name,*plan['flags']],cwd=repo,env=env,stdout=out,stderr=err,preexec_fn=limits,start_new_session=True);timeout=False
  try:code=child.wait(timeout=600)
  except subprocess.TimeoutExpired:
   timeout=True;os.killpg(child.pid,signal.SIGTERM)
   try:code=child.wait(timeout=5)
   except subprocess.TimeoutExpired:os.killpg(child.pid,signal.SIGKILL);code=child.wait()
 raw=stdout.read_text(errors='replace');rows.append({'test':name,'exit_code':code,'wall_timeout':timeout,'summary':re.findall(r'test result: ([^\n]+)',raw),'stdout_sha256':hashlib.sha256(stdout.read_bytes()).hexdigest(),'stderr_sha256':hashlib.sha256(stderr.read_bytes()).hexdigest()})
 (s/'ROOT_FAILURE_REPLAY_OUTCOME_384.json').write_text(json.dumps({'schema':'root-failed-unit-diagnostic-replay-outcome/v1','plan':'ROOT_FAILURE_REPLAY_PLAN_384.json','tests':rows,'initial380failure_preserved':True,'full_suite_qualified':False,'performance_observations_collected':0,'15x_admitted':False},indent=2)+'\n')
 print(json.dumps(rows[-1]),flush=True)
