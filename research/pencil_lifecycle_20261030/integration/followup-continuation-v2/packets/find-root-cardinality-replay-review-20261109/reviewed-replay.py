from pathlib import Path
import os,sys,json,hashlib,time,signal,subprocess,resource,re,shutil
P=Path(__file__).resolve().parent
W=Path('/home/ec2-user/pencil-native-find-root-cardinality-20261109')
ABSOLUTE_DEADLINE=1794287499  # 2026-11-10T05:11:39Z
OWNED=False
progress={'phase':None,'stage':'initialization','checked_files':0,'checked_bytes':0,'current_path':None,'current_bytes_read':0,'declared_files':16736,'declared_bytes':3111892318}
last_progress=0.0
campaign_start=None
mode=sys.argv[1];assert mode in ('pre','compiler','post')
EXPECTED_FREEZE=sys.argv[2];assert len(EXPECTED_FREEZE)==64
# Reject repeat attempts before entering receipt-writing cleanup, preserving prior receipts.
if (mode=='pre' and W.exists()) or (W/(mode+'.started')).exists():
 raise SystemExit('existing workspace/phase attempt; no retry, original evidence preserved')
start=time.monotonic();limit=600 if mode=='compiler' else 900;work=limit-20
progress['phase']=mode
resource.setrlimit(resource.RLIMIT_AS,(8*1024**3,8*1024**3))
resource.setrlimit(resource.RLIMIT_STACK,(10*1024**2,10*1024**2))
resource.setrlimit(resource.RLIMIT_FSIZE,(16*1024**2,16*1024**2))
pending=[];children=[];rows=[];errors=[];status='failed'
def stop(s,f):
 pending.append(s)
 raise RuntimeError('signal '+str(s))
signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
def checkpoint(force=False):
 global last_progress
 now=time.monotonic()
 if OWNED and (force or now-last_progress>=5):
  progress.update(elapsed_seconds=now-start,recorded_utc_seconds=time.time())
  durable(W/(mode+'-progress.json'),progress)
  last_progress=now
def boundary():
 checkpoint()
 assert time.time()<ABSOLUTE_DEADLINE,'absolute deadline'
 assert campaign_start is None or time.monotonic()-campaign_start<2380,'campaign work deadline'
 assert not pending,'pending signal'
 assert time.monotonic()-start<work,'work deadline'
def sha(p):
 progress['current_path']=str(p);progress['current_bytes_read']=0
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  while True:
   boundary();b=f.read(1024*1024)
   if not b:break
   h.update(b);progress['current_bytes_read']+=len(b)
 return h.hexdigest()
def durable(p,j):
 t=p.with_suffix('.tmp')
 with t.open('w') as f:json.dump(j,f,sort_keys=True);f.write('\n');f.flush();os.fsync(f.fileno())
 os.replace(t,p);fd=os.open(p.parent,os.O_DIRECTORY)
 try:os.fsync(fd)
 finally:os.close(fd)
def group_members(pg):
 out=[]
 for p in Path('/proc').iterdir():
  if not p.name.isdigit():continue
  try:
   s=(p/'stat').read_text();a=s[s.rindex(')')+2:].split()
   if int(a[2])==pg:out.append({'pid':int(p.name),'state':a[0],'start_ticks':int(a[19])})
  except FileNotFoundError:pass
 return out
def audit(raw,names):
 found=re.findall(r"'([^']+)' depends on axioms:\s*\[([^\]]*)\]",raw)
 no=re.findall(r"'([^']+)' does not depend on any axioms",raw)
 got=[n for n,a in found]+no
 assert sorted(got)==sorted(names) and len(got)==len(set(got)),'named coverage'
 for n,a in found:assert set(filter(None,(x.strip() for x in a.split(','))))<={'propext','Classical.choice','Quot.sound'},'forbidden axiom'
 return got
def output_size():
 n=sum(p.stat().st_size for p in W.rglob('*') if p.is_file())
 assert n<=256*1024**2,'sampled aggregate output threshold'
 return n
try:
 if mode=='pre':
  W.mkdir(mode=0o700);OWNED=True
  campaign_start=start
  durable(W/'campaign-start.json',{'monotonic':start,'absolute_deadline':ABSOLUTE_DEADLINE,'total_seconds':2400})
 assert W.is_dir() and not W.is_symlink()
 marker=W/(mode+'.started');fd=os.open(marker,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o600);os.close(fd)
 OWNED=True
 campaign_start=json.loads((W/'campaign-start.json').read_text())['monotonic']
 checkpoint(force=True)
 assert shutil.disk_usage(W).free>=1024**3
 assert sha(P/'freeze.json')==EXPECTED_FREEZE,'freeze pin mismatch'
 F=json.loads((P/'freeze.json').read_text())
 for n,h in F['files'].items():assert sha(P/n)==h,('frozen input',n)
 if mode in ('pre','post'):
  C=Path(F['closure']['path']);B=Path(F['baseline']['path'])
  assert sha(C)==F['closure']['sha256'];assert sha(B)==F['baseline']['sha256']
  cp=json.loads(C.read_text());baseline=json.loads(B.read_text())
  files={f['path']:f for v in cp['done'].values() for f in v['files']}
  assert set(files)==set(baseline) and len(files)==16736
  done={}
  progress['stage']='native_dependencies'
  for name,v in sorted(files.items()):
   boundary();p=Path(name);assert str(p.resolve())==v['realpath'];s=p.stat();h=sha(p);a=p.stat()
   assert (s.st_dev,s.st_ino,s.st_size,s.st_mtime_ns)==(a.st_dev,a.st_ino,a.st_size,a.st_mtime_ns)
   assert h==baseline[name]['sha256'] and a.st_size==v['bytes']
   done[name]={'sha256':h,'bytes':a.st_size,'device':a.st_dev,'inode':a.st_ino}
   progress['checked_files']+=1;progress['checked_bytes']+=a.st_size
   progress['last_checked_path']=name
   checkpoint()
  durable(W/(mode+'-hashes.json'),done)
  if mode=='post':assert done==json.loads((W/'pre-hashes.json').read_text())
  assert progress['checked_files']==16736 and progress['checked_bytes']==3111892318
  progress['stage']='runtime_support'
  # Native runtime support is independently bound on both sides as well.
  rt={k:sha(k) for k in F['runtime']};assert rt==F['runtime'];durable(W/(mode+'-runtime.json'),rt)
 else:
  progress['stage']='compiler'
  pre=json.loads((W/'pre-receipt.json').read_text());assert pre['status']=='passed'
  roots=F['lean_roots'];lean=F['lean']
  for n in ('GraphAlgebra', 'Find', 'Cycles', 'Union', 'Forest', 'Composition', 'UnionSize', 'FindRootCardinality', 'Controls', 'Readback', 'ExpectedFailure'):
   (W/(n+'.lean')).write_bytes((P/(n+'.lean.txt')).read_bytes())
  env={'PATH':'/usr/bin:/bin','LANG':'C.UTF-8','LEAN_SYSROOT':str(Path(lean).parent.parent),'LEAN_PATH':':'.join([str(W)]+roots),'TMPDIR':str(W)}
  assert {k:sha(k) for k in F['runtime']}==F['runtime']
  # Parser controls exercise missing, duplicated and forbidden-axiom records before Lean.
  for bad in ('',"'x' depends on axioms: [sorryAx]","'x' does not depend on any axioms\n'x' does not depend on any axioms"):
   try:audit(bad,['x'])
   except AssertionError:pass
   else:raise AssertionError('audit accepted malformed evidence')
  for name in ('GraphAlgebra', 'Find', 'Cycles', 'Union', 'Forest', 'Composition', 'UnionSize', 'FindRootCardinality', 'Controls', 'Readback', 'ExpectedFailure'):
   boundary();cmd=[lean,'-j1','-M3500','-s16384','-o',str(W/(name+'.olean')),str(W/(name+'.lean'))]
   row={'module':name,'argv':cmd,'expected_exit':1 if name=='ExpectedFailure' else 0};rows.append(row)
   with (W/(name+'.stdout')).open('xb') as so,(W/(name+'.stderr')).open('xb') as se:
    oldmask=signal.pthread_sigmask(signal.SIG_BLOCK,{signal.SIGTERM,signal.SIGINT})
    try:
     # The child restores the inherited mask before exec; parent registers ownership before unmasking.
     child=subprocess.Popen(cmd,cwd=W,env=env,stdout=so,stderr=se,start_new_session=True,
      preexec_fn=lambda:signal.pthread_sigmask(signal.SIG_SETMASK,oldmask))
     children.append(child)
     row.update(pid=child.pid,pgid=child.pid,reaped=False,start_seconds=time.monotonic()-start)
    finally:signal.pthread_sigmask(signal.SIG_SETMASK,oldmask)
    durable(W/'commands.json',rows)
    while True:
     boundary();output_size()
     try:row['exit']=child.wait(timeout=min(1,work-(time.monotonic()-start)));row['reaped']=True;break
     except subprocess.TimeoutExpired:pass
    row['end_seconds']=time.monotonic()-start;row['remaining_group']=group_members(child.pid);durable(W/'commands.json',rows)
    assert not row['remaining_group'],'descendants remain'
    assert row['exit']==row['expected_exit'],row
   raw=(W/(name+'.stdout')).read_text()+(W/(name+'.stderr')).read_text()
   if name!='ExpectedFailure':assert 'sorry' not in raw.lower(),'sorry warning'
  names=json.loads((P/'names.json').read_text());got=audit((W/'Readback.stdout').read_text(),names['producer']+names['reviewer'])
  neg=(W/'ExpectedFailure.stdout').read_text()
  errs=[line for line in neg.splitlines() if 'error:' in line]
  assert len(errs)==1 and 'unsolved goals' in errs[0], 'not the intended single mathematical rejection'
  assert re.search(r'⊢\s*False\b',neg), 'expected simplified false child-size goal absent'
  forbidden=['unknown identifier','unknown constant','unexpected token','failed to synthesize',
   'type mismatch','application type mismatch','maximum recursion','maximum number of heartbeats',
   'out of memory','failed to create thread','object file','invalid import','resource exhausted',
   'unknown module','cannot find','tactic failed']
  assert not any(x in neg.lower() for x in forbidden), 'negative syntax/elaboration/environment failure'
  assert not (W/'ExpectedFailure.stderr').read_text().strip(),'unexpected negative stderr'
  durable(W/'axioms.json',{'names':got,'allowed':['propext','Classical.choice','Quot.sound']})
  for n in ('GraphAlgebra', 'Find', 'Cycles', 'Union', 'Forest', 'Composition', 'UnionSize', 'FindRootCardinality', 'Controls', 'Readback', 'ExpectedFailure'):assert sha(W/(n+'.lean'))==F['files'][n+'.lean.txt']
  assert {k:sha(k) for k in F['runtime']}==F['runtime']
 for n,h in F['files'].items():assert sha(P/n)==h
 assert sha(P/'freeze.json')==EXPECTED_FREEZE,'final freeze pin mismatch'
 boundary();status='passed'
except BaseException as e:errors.append(repr(e))
finally:
 signal.signal(signal.SIGTERM,lambda s,f:pending.append(s));signal.signal(signal.SIGINT,lambda s,f:pending.append(s))
 for child in children:
  try:
   if child.poll() is None or group_members(child.pid):os.killpg(child.pid,signal.SIGKILL)
  except ProcessLookupError:pass
  except BaseException as e:errors.append('kill '+repr(e))
  try:
   child.wait(timeout=min(2,max(.1,limit-5-(time.monotonic()-start))))
   next(r for r in rows if r.get('pid')==child.pid).update(reaped=True,exit=child.returncode)
   remains=group_members(child.pid)
   if remains:errors.append('remaining group '+repr(remains))
  except BaseException as e:errors.append('cleanup '+repr(e))
 if errors or pending or time.monotonic()-start>=limit-5 or time.time()>=ABSOLUTE_DEADLINE or (campaign_start is not None and time.monotonic()-campaign_start>=2400):status='failed'
 result={'phase':mode,'status':status,'errors':errors,'pending_signals':pending,'elapsed_seconds':time.monotonic()-start,'commands':rows,'compiler_invocations':len(rows),'workspace':str(W),'dependency_files_declared':16736,'dependency_bytes_declared':3111892318,'checked_dependency_files':progress['checked_files'],'checked_dependency_bytes':progress['checked_bytes'],'hash_progress':dict(progress),'machine_review_complete':False,'post_custody_required':True,'scientific_execution':False,'independent_binary_kernel_rechecker':False,'trust':'Pinned shared caches; pre/post checks do not exclude transient concurrent changes.'}
 try:
  if not OWNED:raise RuntimeError('workspace/phase not owned; no receipt overwrite')
  checkpoint(force=True)
  durable(W/(mode+'-receipt.json'),result)
  if pending or time.monotonic()-start>=limit-5 or time.time()>=ABSOLUTE_DEADLINE or (campaign_start is not None and time.monotonic()-campaign_start>=2400):
   status='failed';result['status']='failed';result['late_boundary_failed']=True;durable(W/(mode+'-receipt.json'),result)
 except BaseException as e:status='failed';print('receipt failure '+repr(e),file=sys.stderr)
 print(json.dumps(result));sys.exit(0 if status=='passed' else 1)
