import os,sys,json,time,signal,resource,subprocess,hashlib,shutil,re,stat
from pathlib import Path
D=Path(__file__).resolve().parent;P=json.loads((D/'exporter2-command.json').read_text());S=Path(P['source_root']);O=Path(P['output']);A=D/'exporter2-attempt1';A.mkdir()
resource.setrlimit(resource.RLIMIT_AS,(8*1024**3,8*1024**3));resource.setrlimit(resource.RLIMIT_FSIZE,(512*1024**2,512*1024**2));resource.setrlimit(resource.RLIMIT_CORE,(0,0))
start=time.monotonic();pending=None;phase='preflight';child=None;fds=();observations=[]
R={'status':'failed','commands':[],'pre_changed_inputs':None,'post_changed_inputs':None,'launch_uncertain':False,'tests_passed':0,'scientific_runs':0,'hard_as_bytes':8*1024**3,'outer_seconds':120,'aggregate_disk_monitor_not_hard_quota':True}
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
def changed():
 result=[];src=json.loads((D/'exporter2-source-inputs.json').read_text());actual={str(p.relative_to(S)) for p in S.rglob('*') if p.is_file()}
 result.extend('source-membership:'+p for p in sorted(actual.symmetric_difference(src)))
 result.extend('source-symlink:'+str(p.relative_to(S)) for p in S.rglob('*') if p.is_symlink())
 for name,rows in [('source',src),('external',json.loads((D/'exporter2-external-inputs.json').read_text()))]:
  for n,v in rows.items():
   p=S/n if name=='source' else Path(n)
   if not p.is_file() or sha(p)!=v['sha256']:result.append(str(p))
 for n,h in json.loads((D/'exporter2-freeze.json').read_text()).items():
  if sha(D/n)!=h:result.append(n)
 return result
def stop(sig,frame):
 global pending
 pending=sig
 if phase=='exit':raise SystemExit(128+sig)
 if phase not in ('launching','cleanup','postcheck','receipt'):raise RuntimeError('signal '+str(sig))
signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
def usage():
 total=sum(p.stat().st_size for p in O.rglob('*') if p.is_file() and not p.is_symlink());free=shutil.disk_usage(O).free;logs=sum(p.stat().st_size for p in A.iterdir() if p.suffix in ('.stdout','.stderr'))
 observations.append({'elapsed':time.monotonic()-start,'output_bytes':total,'free_bytes':free,'log_bytes':logs});return total,free,logs
def boundary():
 assert pending is None and time.monotonic()-start<100,'signal or work deadline'
 size,free,logs=usage();assert size<=1024**3 and free>=4*1024**3 and logs<=16*1024**2,'disk/log threshold'
def groups(pgid):
 found=[]
 for p in Path('/proc').iterdir():
  if p.name.isdigit():
   try:
    fields=(p/'stat').read_text().rsplit(')',1)[1].split()
    if int(fields[2])==pgid:found.append(int(p.name))
   except (FileNotFoundError,ProcessLookupError):pass
 return found
def close_fds():
 global fds
 for fd in fds:
  try:os.close(fd)
  except OSError:pass
 fds=()
def write(name,obj):
 q=A/(name+'.tmp')
 with q.open('w') as f:json.dump(obj,f,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
 os.replace(q,A/name)
try:
 assert all(p.is_dir() for p in O.iterdir()) and not any((O/'scratch').iterdir()),'output/scratch not fresh'
 assert usage()[1]>=4*1024**3,'less than4GiB free before check'
 R['pre_changed_inputs']=changed();assert not R['pre_changed_inputs'],'changed input'
 for i,argv in enumerate(P['commands'],1):
  boundary();env=dict(P['environment']);row={'argv':argv,'reaped':False};R['commands'].append(row)
  if i==1:
   fds=os.pipe();r,w=fds;st=[os.fstat(fd) for fd in fds];assert all(stat.S_ISFIFO(x.st_mode) for x in st) and st[0].st_ino==st[1].st_ino and st[0].st_dev==st[1].st_dev
   env['MAKEFLAGS']=env['CARGO_MAKEFLAGS']=f'--jobserver-auth={r},{w} -j';row['jobserver']={'read_fd':r,'write_fd':w,'inode':st[0].st_ino,'device':st[0].st_dev,'additional_tokens':0,'implicit_slot':1,'scope':'jobserver-managed work slots, not hard CPU/thread quota'}
  else:
   assert sha(Path(argv[0]))==R['compiled_test_binary_sha256'],'compiled binary changed before tests'
  row['environment']=env
  with (A/f'{i}.stdout').open('wb') as so,(A/f'{i}.stderr').open('wb') as se:
   boundary();phase='launching'
   try:child=subprocess.Popen(argv,cwd=S,env=env,stdout=so,stderr=se,start_new_session=True,pass_fds=fds)
   except BaseException:R['launch_uncertain']=True;raise
   row['pid']=child.pid;phase='running';boundary()
   while child.poll() is None:boundary();time.sleep(.2)
   row['exit']=child.wait(timeout=2);row['reaped']=True;row['remaining_group_members']=groups(child.pid);assert not row['remaining_group_members'],'descendant group remains'
   child=None;close_fds();row['parent_jobserver_fds_closed']=True
  assert row['exit']==0,'command failed'
  if i==1:
   assert 'jobserver' not in (A/'1.stderr').read_text(errors='replace').lower(),'jobserver warning'
   R['compiled_test_binary_sha256']=sha(Path(P['commands'][1][0]))
  else:
   text=(A/'2.stdout').read_text(errors='replace');names=re.findall(r'^test (\S+) \.\.\. ok$',text,re.M);expected=json.loads((D/'exporter2-test-inventory.json').read_text())['expected_tests']
   assert len(names)==12 and len(set(names))==12 and set(names)==set(expected),'exact named12 test inventory not passed'
   assert re.search(r'test result: ok\. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;',text),'missing exact successful12 summary'
   assert sha(Path(argv[0]))==R['compiled_test_binary_sha256'],'test binary changed'
   R['tests_passed']=12;R['passed_test_names']=names
 boundary();R['sequence_passed']=True
except BaseException as e:R['error']=repr(e)
finally:
 phase='cleanup'
 if child is not None:
  row=R['commands'][-1]
  try:survivors=groups(child.pid)
  except BaseException as e:survivors=None;row['group_readback_error']=repr(e)
  if not row['reaped'] or survivors:
   try:os.killpg(child.pid,signal.SIGKILL)
   except ProcessLookupError:pass
   except BaseException as e:row['kill_error']=repr(e)
   try:row['exit']=child.wait(timeout=2);row['reaped']=True
   except BaseException as e:row['reap_error']=repr(e)
  try:
   drain_deadline=time.monotonic()+2
   row['remaining_group_members']=groups(child.pid)
   while row['remaining_group_members'] and time.monotonic()<drain_deadline:
    time.sleep(.05);row['remaining_group_members']=groups(child.pid)
  except BaseException as e:row['group_readback_error']=repr(e)
 if child is None or R['commands'][-1]['reaped']:close_fds()
 R['parent_fds_closed']=not fds;R['pending_signal']=pending;R['elapsed_seconds']=time.monotonic()-start;write('cleanup-receipt.json',R)
 phase='postcheck'
 try:R['post_changed_inputs']=changed()
 except BaseException as e:R['postcheck_error']=repr(e)
 try:
  size,free,logs=usage();R['terminal_disk_limits_ok']=size<=1024**3 and free>=4*1024**3 and logs<=16*1024**2
  R['outputs']={str(p.relative_to(O)):{'sha256':sha(p),'bytes':p.stat().st_size} for p in O.rglob('*') if p.is_file() and not p.is_symlink()}
 except BaseException as e:R['output_readback_error']=repr(e)
 phase='receipt';R['pending_signal']=pending;R['elapsed_seconds']=time.monotonic()-start;R['disk_observations']=observations
 R['maximum_observed_output_bytes']=max((x['output_bytes'] for x in observations),default=0);R['output_threshold_overshoot_bytes']=max(0,R['maximum_observed_output_bytes']-1024**3)
 R['minimum_observed_free_bytes']=min((x['free_bytes'] for x in observations),default=None)
 if R.get('sequence_passed') and R.get('terminal_disk_limits_ok') and all(x['reaped'] and x.get('remaining_group_members')==[] for x in R['commands']) and not R['launch_uncertain'] and R['parent_fds_closed'] and R['post_changed_inputs']==[] and pending is None and R['elapsed_seconds']<110 and not any(k in R for k in ['postcheck_error','output_readback_error']):R['status']='passed'
 R['completion_contract']='Requires passed receipt plus outer exit0; an interrupted receipt write alone is insufficient. Outputs retained under monitored limit.';write('receipt.json',R)
 if pending is not None or time.monotonic()-start>=110:R['status']='failed';R['terminal_error']='signal/deadline during receipt';write('receipt.json',R)
phase='exit'
if pending is not None or time.monotonic()-start>=110:R['status']='failed';R['terminal_error']='signal/deadline after receipt';write('receipt.json',R)
print(json.dumps({k:v for k,v in R.items() if k!='disk_observations'}));sys.exit(0 if R['status']=='passed' and pending is None and time.monotonic()-start<110 else 1)
