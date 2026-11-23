from pathlib import Path
import os,sys,json,hashlib,subprocess,resource,time,signal,tempfile,re,shutil
P=Path(__file__).resolve().parent;R=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');SRC=P;REV='T1538-compiler-phase7';start=time.monotonic();child=None
resource.setrlimit(resource.RLIMIT_AS,(8*1024**3,8*1024**3));resource.setrlimit(resource.RLIMIT_STACK,(10*1024**2,10*1024**2))
resource.setrlimit(resource.RLIMIT_FSIZE,(16*1024**2,16*1024**2))
assert shutil.disk_usage('/var/tmp').free>=1024**3,'startup free space below1GiB'
O=Path(tempfile.mkdtemp(prefix='nonbranch-norm-lean-',dir='/var/tmp'));(P/'lean-workspace.txt').write_text(str(O)+'\n')
H=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
roots=[Path(x) for x in ['/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal/.lake/build/lib/lean', '/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal/.lake/packages/LeanSearchClient/.lake/build/lib/lean', '/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal/.lake/packages/Qq/.lake/build/lib/lean', '/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal/.lake/packages/aesop/.lake/build/lib/lean', '/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal/.lake/packages/batteries/.lake/build/lib/lean', '/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal/.lake/packages/importGraph/.lake/build/lib/lean', '/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal/.lake/packages/mathlib/.lake/build/lib/lean', '/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal/.lake/packages/plausible/.lake/build/lib/lean', '/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal/.lake/packages/proofwidgets/.lake/build/lib/lean']];lean=Path('/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1/bin/lean')
runtime_pins=json.loads((P/'runtime-pins.json').read_text())
system_paths=[Path('/lib64')/n for n in ['libc.so.6','libpthread.so.0','libdl.so.2','libm.so.6','librt.so.1','ld-linux-x86-64.so.2']]
def inventory():return {str(f):H(f) for f in list(map(Path,runtime_pins))+system_paths}
def interrupted(s,f):raise RuntimeError('signal '+str(s))
signal.signal(signal.SIGTERM,interrupted);signal.signal(signal.SIGINT,interrupted);rows=[];status='failed';cleanup_errors=[];telemetry_count=0
def durable(path,obj):
 tmp=path.with_suffix('.tmp')
 with tmp.open('w') as f:json.dump(obj,f,sort_keys=True);f.write('\n');f.flush();os.fsync(f.fileno())
 os.replace(tmp,path);fd=os.open(path.parent,os.O_DIRECTORY);os.fsync(fd);os.close(fd)
def sample(pid):
 out={'pid':pid,'monotonic_seconds':time.monotonic()-start}
 try:
  proc=Path('/proc')/str(pid);raw=(proc/'stat').read_text();tail=raw[raw.rindex(')')+2:].split();out.update(state=tail[0],start_ticks=int(tail[19]),ppid=int(tail[1]),pgrp=int(tail[2]))
  out['io']=(proc/'io').read_text()[:4096];out['status']=(proc/'status').read_text()[:16384]
 except (OSError,ValueError,IndexError) as e:out['observation_error']=repr(e)
 return out
def save_sample(pid):
 global telemetry_count
 assert telemetry_count<600,'telemetry sample ceiling'
 with (O/'process-samples.jsonl').open('a') as f:f.write(json.dumps(sample(pid),sort_keys=True)+'\n');f.flush()
 telemetry_count+=1

def check_output_cap():
 total=sum(f.stat().st_size for f in O.rglob('*') if f.is_file());assert total<=256*1024**2,'aggregate output cap';return total
try:
 recipe=json.loads((P/'phase7-recipe.json').read_text());assert all(H(P/n)==v for n,v in recipe['source_hashes'].items()),'frozen source hash mismatch'
 custody=json.loads((P/'custody-phase7-pre-receipt.json').read_text());assert custody['status']=='complete' and custody['files_hashed']==16736;assert H(Path(custody['hashes_path']))==custody['hashes_sha256'],'custody manifest changed'
 before=inventory();assert all(before[k]==v for k,v in runtime_pins.items());(O/'inputs-before.json').write_text(json.dumps(before,sort_keys=True)+'\n')
 retained=recipe['retained_olean'];assert H(Path(retained['path']))==retained['sha256'];shutil.copyfile(retained['path'],O/'Nonbranch.olean');assert H(O/'Nonbranch.olean')==retained['sha256']
 assert (P/'Controls.lean.txt').read_bytes()==(P/'original-controls.lean.txt').read_bytes()+(P/'Readback.lean.txt').read_bytes().split(b'\n',1)[1],'exact concatenation mismatch'
 private=[]
 for n in ['Nonbranch','Controls','Readback','ExpectedFailure']:(O/(n+'.lean')).write_bytes((P/(n+'.lean.txt')).read_bytes())
 sourcepins={str(f):H(f) for f in sorted(O.rglob('*')) if f.is_file() and f.name!='inputs-before.json'};(O/'private-inputs.json').write_text(json.dumps(sourcepins,sort_keys=True)+'\n')
 env=dict(os.environ,LEAN_SYSROOT=str(lean.parent.parent),LEAN_PATH=':'.join(map(str,[O]+roots)))
 for n in ['Controls','ExpectedFailure']:
  remaining=570-(time.monotonic()-start);assert remaining>0
  cmd=[str(lean),'-j1','-M3500','-s16384','-o',str(O/(n+'.olean')),str(O/(n+'.lean'))];row={'module':n,'argv':cmd,'expected_exit':1 if n=='ExpectedFailure' else 0};rows.append(row)
  with (O/(n+'.stdout')).open('wb') as so,(O/(n+'.stderr')).open('wb') as se:
   child=subprocess.Popen(cmd,cwd=O,env=env,stdout=so,stderr=se,start_new_session=True)
   row['pid']=child.pid;row['process_group']=child.pid;row['reaped']=False;row['start_seconds']=time.monotonic()-start;row['initial_process']=sample(child.pid);durable(O/'owned-child.json',row);durable(O/'commands.json',rows)
   try:
    while True:
     save_sample(child.pid);check_output_cap();remaining=570-(time.monotonic()-start);assert remaining>0,'compiler work cutoff'
     try:
      row['exit']=child.wait(timeout=min(1,remaining));row['reaped']=True;row['end_seconds']=time.monotonic()-start;durable(O/'commands.json',rows);break
     except subprocess.TimeoutExpired:pass
   except BaseException:
    row['primary_failure_seconds']=time.monotonic()-start;raise
   child=None
  assert row['exit']==row['expected_exit'],row
 def audit(raw,names):
  found=re.findall(r"'([^']+)' depends on axioms: \[([^\]]*)\]",raw);no=re.findall(r"'([^']+)' does not depend on any axioms",raw);got=[n for n,a in found]+no
  assert sorted(got)==sorted(names) and len(got)==len(set(got)),'declaration coverage'
  for n,a in found:assert set(filter(None,(x.strip() for x in a.split(','))))<={'propext','Classical.choice','Quot.sound'},'forbidden axiom'
  return got
 expected=json.loads((SRC/'names.json').read_text());names=audit((O/'Controls.stdout').read_text(),expected)
 own=[]
 negatives=[]
 for raw in ['',"'x' depends on axioms: [sorryAx]","'x' does not depend on any axioms\n'x' does not depend on any axioms"]:
  try:audit(raw,['x'])
  except AssertionError:negatives.append(True)
  else:raise AssertionError('audit accepted bad evidence')
 assert 'decide' in (O/'ExpectedFailure.stdout').read_text() and 'false' in (O/'ExpectedFailure.stdout').read_text()
 after=inventory();assert after==before;assert all(H(Path(k))==v for k,v in sourcepins.items());(O/'inputs-after.json').write_text(json.dumps(after,sort_keys=True)+'\n');(O/'axioms.json').write_text(json.dumps({'producer_names':names,'own_names':own,'negative_audit_controls':negatives,'allowed':['propext','Classical.choice','Quot.sound']},indent=2)+'\n');status='passed'
except BaseException as e:error=repr(e)
finally:
 # Outer timeout still enforces final KILL; avoid SIGTERM interrupting receipt cleanup.
 signal.signal(signal.SIGTERM,signal.SIG_IGN)
 if child is not None:
  try:
   if child.poll() is None:os.killpg(child.pid,signal.SIGKILL)
  except BaseException as e:cleanup_errors.append({'step':'killpg','error':repr(e)})
  try:
   child.wait(timeout=min(10,max(.1,590-(time.monotonic()-start))))
   if rows:rows[-1]['reaped']=True;rows[-1]['exit']=child.returncode
  except BaseException as e:
   cleanup_errors.append({'step':'wait','error':repr(e),'unresolved_owned_pid':child.pid})
   if rows:rows[-1]['reaped']=False;rows[-1]['unresolved_owned_pid']=child.pid
 try:output_bytes=check_output_cap()
 except BaseException as e:output_bytes=None;cleanup_errors.append({'step':'output_cap','error':repr(e)})
 remaining_groups=[]
 try:
  owned={r['process_group'] for r in rows if 'process_group' in r}
  for proc in Path('/proc').iterdir():
   if not proc.name.isdigit():continue
   try:
    raw=(proc/'stat').read_text();tail=raw[raw.rindex(')')+2:].split()
    if int(tail[2]) in owned:remaining_groups.append({'pid':int(proc.name),'pgrp':int(tail[2]),'state':tail[0],'start_ticks':int(tail[19])})
   except FileNotFoundError:pass
  if remaining_groups:cleanup_errors.append({'step':'process_group_remaining','processes':remaining_groups})
 except BaseException as e:cleanup_errors.append({'step':'process_group_observation','error':repr(e)})
 if cleanup_errors:status='failed'
 result={'status':status,'primary_error':locals().get('error'),'cleanup_errors':cleanup_errors,'commands':rows,'elapsed_seconds':time.monotonic()-start,'runtime_file_count':len(locals().get('before',{})),'dependency_files_bound_by_pre_custody':16736,'dependency_post_custody_pending':True,'all_started_children_reaped':all(r.get('reaped',False) for r in rows if 'pid' in r),'output_bytes':output_bytes,'output_aggregate_limit':256*1024**2,'per_file_limit':16*1024**2,'startup_free_threshold':1024**3,'telemetry_samples':telemetry_count,'source_revision':REV,'workspace':str(O),'runtime_execution':False,'independent_binary_kernel_rechecker':False,'trust':'Pinned precursor and shared mathlib/toolchain caches; pre/post inventories do not exclude transient concurrent changes.'}
 for target in [O/'receipt.json',P/'lean-phase7-receipt.json']:
  try:durable(target,result)
  except BaseException as e:
   status='failed';result['status']='failed';result.setdefault('receipt_write_errors',[]).append({'path':str(target),'error':repr(e)});print('RECEIPT_WRITE_FAILED '+str(target)+' '+repr(e),file=sys.stderr)
 print(json.dumps(result));sys.exit(0 if status=='passed' else 1)
