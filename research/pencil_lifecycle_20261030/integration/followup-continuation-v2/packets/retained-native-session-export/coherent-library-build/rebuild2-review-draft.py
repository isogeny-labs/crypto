import os,json,time,signal,resource,subprocess,hashlib,shutil
from pathlib import Path
D=Path(__file__).resolve().parent
P=json.loads((D/'rebuild2-command.json').read_text());S=Path(P['source_root']);O=Path(P['output'])
A=D/'rebuild2-attempt1';A.mkdir();start=time.monotonic();pending=None;phase='preflight';child=None;pipe_fds=();observations=[]
resource.setrlimit(resource.RLIMIT_AS,(8*1024**3,8*1024**3));resource.setrlimit(resource.RLIMIT_FSIZE,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CORE,(0,0))
R={'status':'failed','commands':[],'pre_changed_inputs':None,'post_changed_inputs':None,'reaped':False,'launch_uncertain':False,'tests_run':0,'scientific_runs':0,'hard_as_bytes':8*1024**3,'outer_seconds':300,'aggregate_disk_monitor_not_hard_quota':True}
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
def changed():
 out=[]
 for file in ['rebuild-source-inputs.json','rebuild2-reused-inputs.json']:
  for name,v in json.loads((D/file).read_text()).items():
   p=S/name if file=='rebuild-source-inputs.json' else Path(name)
   if not p.is_file() or sha(p)!=v['sha256']:out.append(str(p))
 if sha(Path(P['command_argv'][0][0]))!=P['rustc_sha256']:out.append('rustc')
 for name,digest in json.loads((D/'rebuild2-freeze.json').read_text()).items():
  if sha(D/name)!=digest:out.append(name)
 return out
def stop(sig,frame):
 global pending
 pending=sig
 if phase not in ('launching','cleanup','postcheck','receipt'):raise RuntimeError('received signal '+str(sig))
signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
def usage():
 total=sum(p.stat().st_size for p in O.rglob('*') if p.is_file() and not p.is_symlink());free=shutil.disk_usage(O).free
 logs=sum(p.stat().st_size for p in A.glob('*.stdout'))+sum(p.stat().st_size for p in A.glob('*.stderr'))
 observations.append({'elapsed':time.monotonic()-start,'output_bytes':total,'free_bytes':free,'log_bytes':logs})
 return total,free,logs
def boundary():
 assert pending is None,'pending signal'
 assert time.monotonic()-start<275,'work deadline'
 size,free,logs=usage();assert size<=4*1024**3 and free>=4*1024**3 and logs<=16*1024**2,'disk or log stop threshold'
def write(name,obj):
 path=A/name;tmp=A/(name+'.tmp')
 with tmp.open('w') as f:json.dump(obj,f,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
 os.replace(tmp,path)
try:
 assert not any(O.iterdir()),'nonempty new output'
 assert usage()[1]>=8*1024**3,'less than8GiB free'
 R['pre_changed_inputs']=changed();assert not R['pre_changed_inputs'],'changed input before compile'
 boundary()
 import stat
 pipe_fds=os.pipe();r,w=pipe_fds
 stats=[os.fstat(fd) for fd in pipe_fds];assert all(stat.S_ISFIFO(s.st_mode) for s in stats);assert stats[0].st_ino==stats[1].st_ino and stats[0].st_dev==stats[1].st_dev
 # No write is ever made to this new pipe: zero additional jobserver tokens.
 R['jobserver']={'read_fd':r,'write_fd':w,'additional_tokens':0,'implicit_slot':1,'inode':stats[0].st_ino,'device':stats[0].st_dev,'mode':stats[0].st_mode,'pass_fds':list(pipe_fds),'scope':'jobserver-managed work slots, not hard CPU or thread quota'}
 env=dict(P['environment']);env['MAKEFLAGS']=env['CARGO_MAKEFLAGS']=f'--jobserver-auth={r},{w} -j'
 R['actual_environment']=env;argv=P['command_argv'][0];row={'argv':argv};R['commands'].append(row)
 with (A/'1.stdout').open('wb') as so,(A/'1.stderr').open('wb') as se:
  boundary();phase='launching'
  try:child=subprocess.Popen(argv,cwd=S,env=env,stdout=so,stderr=se,start_new_session=True,pass_fds=pipe_fds)
  except BaseException:R['launch_uncertain']=True;raise
  row['pid']=child.pid;phase='running';boundary()
  while child.poll() is None:boundary();time.sleep(.2)
  row['exit']=child.wait(timeout=2);R['reaped']=True
 assert row['exit']==0,'compiler failed'
 assert 'jobserver' not in (A/'1.stderr').read_text(errors='replace').lower(),'jobserver warning/error'
 boundary();R['compile_passed']=True
except BaseException as e:R['error']=repr(e)
finally:
 phase='cleanup'
 if child is not None and not R['reaped']:
  try:os.killpg(child.pid,signal.SIGKILL)
  except ProcessLookupError:pass
  except BaseException as e:R['kill_error']=repr(e)
  try:R['commands'][-1]['exit']=child.wait(timeout=2);R['reaped']=True
  except BaseException as e:R['reap_error']=repr(e)
 if child is None or R['reaped']:
  for fd in pipe_fds:
   try:os.close(fd)
   except OSError:pass
  R['parent_jobserver_fds_closed']=True
 else:R['parent_jobserver_fds_closed']=False
 R['pending_signal']=pending;R['elapsed_seconds']=time.monotonic()-start
 write('cleanup-receipt.json',R)
 phase='postcheck'
 try:R['post_changed_inputs']=changed()
 except BaseException as e:R['postcheck_error']=repr(e)
 try:
  size,free,logs=usage();R['terminal_disk_limits_ok']=size<=4*1024**3 and free>=4*1024**3 and logs<=16*1024**2
  R['outputs']={str(p.relative_to(O)):{'sha256':sha(p),'bytes':p.stat().st_size} for p in O.rglob('*') if p.is_file()}
 except BaseException as e:R['output_readback_error']=repr(e)
 phase='receipt';R['pending_signal']=pending;R['elapsed_seconds']=time.monotonic()-start
 R['disk_observations']=observations
 R['maximum_observed_output_bytes']=max(x['output_bytes'] for x in observations)
 R['output_threshold_overshoot_bytes']=max(0,R['maximum_observed_output_bytes']-4*1024**3)
 R['minimum_observed_free_bytes']=min(x['free_bytes'] for x in observations)
 if R.get('terminal_disk_limits_ok') and R.get('compile_passed') and R['reaped'] and not R['launch_uncertain'] and R['post_changed_inputs']==[] and pending is None and R['elapsed_seconds']<290 and not any(k in R for k in ['kill_error','reap_error','postcheck_error','output_readback_error']):R['status']='passed'
 write('receipt.json',R)
print(json.dumps({k:v for k,v in R.items() if k!='disk_observations'}))
