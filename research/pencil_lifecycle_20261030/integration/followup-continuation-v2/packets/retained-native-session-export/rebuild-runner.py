import os,json,time,signal,resource,subprocess,hashlib,shutil
from pathlib import Path
D=Path(__file__).resolve().parent
P=json.loads((D/'rebuild-command.json').read_text());S=Path(P['source_root']);O=Path(P['output'])
A=D/'rebuild-attempt1';A.mkdir();start=time.monotonic();rows=[];pending=None;child=None;peak=0;minimum=None;changes=[]
resource.setrlimit(resource.RLIMIT_AS,(8*1024**3,8*1024**3));resource.setrlimit(resource.RLIMIT_FSIZE,(1024**3,1024**3));resource.setrlimit(resource.RLIMIT_CORE,(0,0))
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
def changed():
 out=[]
 for file in ['rebuild-source-inputs.json','rebuild-reused-inputs.json']:
  for name,v in json.loads((D/file).read_text()).items():
   p=S/name if file=='rebuild-source-inputs.json' else Path(name)
   if not p.is_file() or sha(p)!=v['sha256']:out.append(str(p))
 if sha(Path(P['command_argv'][0][0]))!=P['rustc_sha256']:out.append('rustc')
 return out
def stop(sig,frame):
 global pending
 pending=sig;raise RuntimeError('received signal '+str(sig))
signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
def usage():
 global peak,minimum
 total=sum(p.stat().st_size for p in O.rglob('*') if p.is_file() and not p.is_symlink());free=shutil.disk_usage(O).free
 peak=max(peak,total);minimum=free if minimum is None else min(minimum,free)
 logs=sum(p.stat().st_size for p in A.glob('*.stdout'))+sum(p.stat().st_size for p in A.glob('*.stderr'))
 return total,free,logs
status='failed';error=None
try:
 assert not any(O.iterdir()),'output nonempty'
 assert usage()[1]>=8*1024**3,'less than8GiB free before launch'
 changes=changed();assert not changes,'changed frozen inputs'
 for i,argv in enumerate(P['command_argv'],1):
  assert time.monotonic()-start<280,'shared deadline'
  row={'argv':argv,'reaped':False};rows.append(row)
  with (A/f'{i}.stdout').open('wb') as so,(A/f'{i}.stderr').open('wb') as se:
   child=subprocess.Popen(argv,cwd=S,env=P['environment'],stdout=so,stderr=se,start_new_session=True);row['pid']=child.pid
   try:
    while child.poll() is None:
     size,free,logs=usage()
     assert size<=4*1024**3,'output threshold crossed'
     assert free>=4*1024**3,'free space threshold crossed'
     assert logs<=16*1024**2,'log cap crossed'
     assert time.monotonic()-start<280,'shared deadline'
     time.sleep(.2)
    row['exit']=child.wait(timeout=2);row['reaped']=True
   except BaseException:
    try:os.killpg(child.pid,signal.SIGKILL)
    except ProcessLookupError:pass
    try:row['exit']=child.wait(timeout=2);row['reaped']=True
    except BaseException as x:row['reap_error']=repr(x)
    raise
   child=None
  assert row['exit']==0,'compiler failed'
 size,free,logs=usage();assert size<=4*1024**3 and free>=4*1024**3 and logs<=16*1024**2,'terminal disk/log check'
 changes=changed();assert not changes,'changed post inputs'
 assert pending is None and time.monotonic()-start<290,'terminal deadline/signal'
 status='passed'
except BaseException as e:error=repr(e)
finally:
 usage()
 receipt={'status':status,'error':error,'elapsed_seconds':time.monotonic()-start,'commands':rows,'pending_signal':pending,'changed_inputs':changes,'maximum_observed_output_bytes':peak,'minimum_observed_free_bytes':minimum,'aggregate_disk_monitor_not_hard_quota':True,'source_commit':P['source_commit'],'hard_as_bytes':8*1024**3,'hard_perfile_bytes':1024**3,'outer_seconds':300,'tests_run':0,'scientific_runs':0,'outputs':{str(p.relative_to(O)):{'sha256':sha(p),'bytes':p.stat().st_size} for p in O.rglob('*') if p.is_file()}}
 (A/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt))
