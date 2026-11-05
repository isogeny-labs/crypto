"""One frozen ordinary integration check; never invoke without root release."""
from pathlib import Path
import os,sys,json,hashlib,subprocess,signal,time,resource,re,stat,traceback
P=Path(__file__).resolve().parent
start=time.monotonic(); deadline=start+230
resource.setrlimit(resource.RLIMIT_AS,(4*1024**3,4*1024**3))
resource.setrlimit(resource.RLIMIT_FSIZE,(64*1024**2,64*1024**2))
H=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
binding=json.loads((P/'command.json').read_text());freeze=json.loads((P/'freeze.json').read_text());w=Path(binding['cwd'])
out=w/'attempt1';out.mkdir();child=None;row={};status='failed';pending=[];peak=0;cleanup=[]
def stop(sig,frame):
 pending.append(sig)
 if len(pending)==1: raise RuntimeError('signal '+str(sig))
signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
def members(pgid):
 found=[]
 for f in Path('/proc').glob('[0-9]*/stat'):
  try:
   fields=f.read_text().rsplit(')',1)[1].split()
   if int(fields[2])==pgid:found.append(int(f.parent.name))
  except (OSError,ValueError,IndexError):pass
 return sorted(found)
def intact():
 files=sorted(str(f) for root in freeze['source_roots'] for f in Path(root).rglob('*') if f.is_file())
 return files==freeze['source_files'] and all(not Path(k).is_symlink() and Path(k).is_file() and H(Path(k))==v for k,v in freeze['files'].items())
def sample_target(directory):
 # Cargo atomically renames/unlinks temporary files. Only ENOENT is benign.
 total=0
 try:
  with os.scandir(directory) as entries:
   for entry in entries:
    try:
     info=entry.stat(follow_symlinks=False)
     if stat.S_ISDIR(info.st_mode):total+=sample_target(entry.path)
     elif stat.S_ISREG(info.st_mode):total+=info.st_size
     elif stat.S_ISLNK(info.st_mode):raise RuntimeError('unexpected target symlink')
    except FileNotFoundError:continue
 except FileNotFoundError:return total
 return total
def clean():
 if not row.get('pid'):return
 pid=row['pid']
 if members(pid):
  try:os.killpg(pid,signal.SIGKILL)
  except ProcessLookupError:pass
 if child is not None:
  row['cleanup_observed_exit']=child.wait(timeout=3)
 end=time.monotonic()+2
 while members(pid) and time.monotonic()<end:time.sleep(.05)
 row['reaped']=child is None or child.poll() is not None
 row['group_remaining']=members(pid)
 if row['group_remaining']:raise RuntimeError('owned process group unresolved')
try:
 assert intact(),'preinput mismatch'
 env={'PATH':'/home/ec2-user/.rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin','LC_ALL':'C.UTF-8',**binding['environment']}
 row={'argv':binding['argv'],'environment':env,'reaped':False}
 with (out/'stdout').open('wb') as so,(out/'stderr').open('wb') as se:
  mask=signal.pthread_sigmask(signal.SIG_BLOCK,{signal.SIGTERM,signal.SIGINT})
  try:
   child=subprocess.Popen(binding['argv'],cwd=w,env=env,stdout=so,stderr=se,start_new_session=True)
   row['pid']=child.pid
  finally:signal.pthread_sigmask(signal.SIG_SETMASK,mask)
  while child.poll() is None:
   if time.monotonic()>start+210:raise RuntimeError('compiler/test work deadline')
   size=sample_target(w/'target')
   peak=max(peak,size)
   if size>binding['monitored_target_bytes']:raise RuntimeError('sampled target output limit')
   time.sleep(.1)
  row['exit']=child.wait();row['reaped']=True
 clean();assert row['exit']==0,row
 raw=(out/'stdout').read_text();name='progress_native::finite_difference_integration_tests::finite_difference_progress_route_pairs'
 assert 'running 1 test\n' in raw and name in raw
 assert re.search(r'test result: ok\. 1 passed; 0 failed; 0 ignored;',raw),'zero/multiple test or failure'
 assert [int(x) for x in re.findall(r'\bPAIR (\d+) p=',raw)]==list(range(6398))
 summary='SUMMARY pairs=6398 sequences=20207 samples=134067 boundary_transitions=32 rejected_wrong_controls=3 branch_coverage=[22232, 12812, 5366, 267, 753]'
 assert raw.count(summary)==1
 assert raw.count('DIAGNOSTIC_CONTROLS passed: exact three emissions, enabled gate, noop, eight milestones, actual counter payload')==1
 assert intact(),'postinput mismatch'
 assert not pending and time.monotonic()<deadline,'final deadline/signal'
 status='passed'
except BaseException as e:
 error=repr(e);failure_traceback=traceback.format_exc()
finally:
 try:clean()
 except BaseException as e:cleanup.append(repr(e));status='failed'
 try:unchanged=intact()
 except BaseException as e:unchanged=False;cleanup.append(repr(e))
 if not unchanged or pending or time.monotonic()>=deadline:status='failed'
 r={'status':status,'error':locals().get('error'),'traceback':locals().get('failure_traceback'),'command':row,'elapsed_seconds':time.monotonic()-start,'freeze_sha256':H(P/'freeze.json'),'inputs_unchanged':unchanged,'cleanup_errors':cleanup,'pending_signals':pending,'peak_sampled_target_bytes':peak,'target_threshold_bytes':binding['monitored_target_bytes'],'threshold_is_monitor_not_hard_quota':True,'output_retained':True,'scientific_execution':False,'actual_progress_module':True,'full_root_ecbench_crate':False,'output_directory':str(out)}
 (out/'receipt.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r))
 if pending or time.monotonic()>=deadline:sys.exit(1)
 sys.exit(0 if status=='passed' else 1)
