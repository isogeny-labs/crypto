from pathlib import Path
import os,sys,json,hashlib,subprocess,resource,time,signal,tempfile,re,shutil
P=Path(__file__).resolve().parent;R=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');SRC=P;REV='T1538-compiler-phase2';start=time.monotonic();child=None
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
signal.signal(signal.SIGTERM,interrupted);signal.signal(signal.SIGINT,interrupted);rows=[];status='failed'
def check_output_cap():
 total=sum(f.stat().st_size for f in O.rglob('*') if f.is_file());assert total<=256*1024**2,'aggregate output cap';return total
try:
 recipe=json.loads((P/'phase2-recipe.json').read_text());assert all(H(P/n)==v for n,v in recipe['source_hashes'].items()),'frozen source hash mismatch'
 custody=json.loads((P/'custody-complete-receipt.json').read_text());assert custody['status']=='complete' and custody['files_hashed']==16736;assert H(Path(custody['hashes_path']))==custody['hashes_sha256'],'custody manifest changed'
 before=inventory();assert all(before[k]==v for k,v in runtime_pins.items());(O/'inputs-before.json').write_text(json.dumps(before,sort_keys=True)+'\n')
 private=[]
 for n in ['Nonbranch','Controls','Readback','ExpectedFailure']:(O/(n+'.lean')).write_bytes((P/(n+'.lean.txt')).read_bytes())
 sourcepins={str(f):H(f) for f in sorted(O.rglob('*')) if f.is_file() and f.name!='inputs-before.json'};(O/'private-inputs.json').write_text(json.dumps(sourcepins,sort_keys=True)+'\n')
 env=dict(os.environ,LEAN_SYSROOT=str(lean.parent.parent),LEAN_PATH=':'.join(map(str,[O]+roots)))
 for n in ['Nonbranch','Controls','Readback','ExpectedFailure']:
  remaining=110-(time.monotonic()-start);assert remaining>0
  cmd=[str(lean),'-j1','-M3500','-s16384','-o',str(O/(n+'.olean')),str(O/(n+'.lean'))];row={'module':n,'argv':cmd,'expected_exit':1 if n=='ExpectedFailure' else 0};rows.append(row)
  with (O/(n+'.stdout')).open('wb') as so,(O/(n+'.stderr')).open('wb') as se:
   child=subprocess.Popen(cmd,cwd=O,env=env,stdout=so,stderr=se,start_new_session=True)
   row['pid']=child.pid;row['process_group']=child.pid;row['reaped']=False
   try:
    while True:
     check_output_cap();remaining=110-(time.monotonic()-start);assert remaining>0,'compiler work cutoff'
     try:
      row['exit']=child.wait(timeout=min(1,remaining));row['reaped']=True;break
     except subprocess.TimeoutExpired:pass
   except BaseException:
    if child.poll() is None:os.killpg(child.pid,signal.SIGKILL)
    child.wait(timeout=2);row['reaped']=True;row['killed_on_exception']=True;raise
   child=None
  assert row['exit']==row['expected_exit'],row
 def audit(raw,names):
  found=re.findall(r"'([^']+)' depends on axioms: \[([^\]]*)\]",raw);no=re.findall(r"'([^']+)' does not depend on any axioms",raw);got=[n for n,a in found]+no
  assert sorted(got)==sorted(names) and len(got)==len(set(got)),'declaration coverage'
  for n,a in found:assert set(filter(None,(x.strip() for x in a.split(','))))<={'propext','Classical.choice','Quot.sound'},'forbidden axiom'
  return got
 expected=json.loads((SRC/'names.json').read_text());names=audit((O/'Readback.stdout').read_text(),expected)
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
 if child is not None:
  if child.poll() is None:os.killpg(child.pid,signal.SIGKILL)
  child.wait(timeout=2)
  if rows:rows[-1]['reaped']=True
 try:output_bytes=check_output_cap()
 except BaseException as cap_error:output_bytes=None;status='failed';error=repr(cap_error)
 result={'status':status,'error':locals().get('error'),'commands':rows,'elapsed_seconds':time.monotonic()-start,'runtime_file_count':len(locals().get('before',{})),'dependency_files_bound_by_pre_custody':16736,'dependency_post_custody_pending':True,'all_started_children_reaped':all(r.get('reaped',False) for r in rows if 'pid' in r),'output_bytes':output_bytes,'output_aggregate_limit':256*1024**2,'per_file_limit':16*1024**2,'startup_free_threshold':1024**3,'source_revision':REV,'workspace':str(O),'runtime_execution':False,'independent_binary_kernel_rechecker':False,'trust':'Pinned precursor and shared mathlib/toolchain caches; pre/post inventories do not exclude transient concurrent changes.'};(O/'receipt.json').write_text(json.dumps(result,indent=2)+'\n');(P/'lean-receipt.json').write_bytes((O/'receipt.json').read_bytes());print(json.dumps(result));sys.exit(0 if status=='passed' else 1)
