from pathlib import Path
import os,sys,json,hashlib,subprocess,signal,time,resource,re
P=Path(__file__).resolve().parent
resource.setrlimit(resource.RLIMIT_AS,(4*1024**3,4*1024**3))
start=time.monotonic();child=None;status='failed';row={}
H=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
binding=json.loads((P/'command.json').read_text());freeze=json.loads((P/'freeze.json').read_text());w=Path(binding['cwd']);out=w/'integration-attempt1';out.mkdir()
def stop(sig,frame):raise RuntimeError('signal '+str(sig))
signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
def current_source_files():return sorted(str(f.resolve()) for root in freeze['source_roots'] for f in Path(root).rglob('*') if f.is_file())
def intact():return current_source_files()==freeze['source_files'] and all(Path(k).is_file() and H(Path(k))==v for k,v in freeze['files'].items())
try:
 assert intact(),'preinput mismatch'
 env={'PATH':'/home/ec2-user/.rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin','LC_ALL':'C.UTF-8',**binding['environment']}
 row={'argv':binding['argv'],'environment':env,'reaped':False}
 with (out/'stdout').open('wb') as so,(out/'stderr').open('wb') as se:
  child=subprocess.Popen(binding['argv'],cwd=w,env=env,stdout=so,stderr=se,start_new_session=True)
  row['pid']=child.pid
  try:row['exit']=child.wait(timeout=max(1,220-(time.monotonic()-start)));row['reaped']=True
  except BaseException:
   os.killpg(child.pid,signal.SIGKILL);child.wait(timeout=3);row['reaped']=True;raise
  child=None
 assert row['exit']==0,row
 raw=(out/'stdout').read_text();name='native_solver::finite_difference_integration_tests::finite_difference_full_producer_pairs'
 assert 'running 1 test\n' in raw and name in raw
 assert re.search(r'test result: ok\. 1 passed; 0 failed; 0 ignored;',raw),'zero/multiple test or failure'
 pairs=[int(x) for x in re.findall(r'\bPAIR (\d+) p=',raw)];assert pairs==list(range(6398))
 summary='SUMMARY pairs=6398 sequences=20207 samples=134067 boundary_transitions=32 rejected_wrong_controls=3 branch_coverage=[22232, 12812, 5366, 267, 753]'
 assert raw.count(summary)==1
 assert intact(),'postinput mismatch';status='passed'
except BaseException as e:error=repr(e)
finally:
 r={'status':status,'error':locals().get('error'),'command':row,'elapsed_seconds':time.monotonic()-start,'frozen_inputs_sha256':H(P/'freeze.json'),'inputs_unchanged':intact(),'one_exact_test_required':True,'scientific_execution':False,'full_producer_crate':True,'output_directory':str(out)}
 (out/'receipt.json').write_text(json.dumps(r,indent=2)+'\n');(P/'receipt.json').write_bytes((out/'receipt.json').read_bytes());print(json.dumps(r));sys.exit(0 if status=='passed' else 1)
