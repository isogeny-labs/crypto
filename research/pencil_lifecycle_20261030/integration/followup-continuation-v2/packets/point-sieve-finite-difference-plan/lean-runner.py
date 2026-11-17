from pathlib import Path
import os,sys,json,hashlib,subprocess,resource,time,signal,tempfile,re,shutil
P=Path(__file__).resolve().parent;R=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');SRC=P;REV='source-only-finite-difference-first-attempt';start=time.monotonic();child=None
resource.setrlimit(resource.RLIMIT_AS,(8*1024**3,8*1024**3));resource.setrlimit(resource.RLIMIT_STACK,(10*1024**2,10*1024**2))
O=Path(tempfile.mkdtemp(prefix='point-sieve-lean-',dir='/var/tmp'));(P/'lean-workspace.txt').write_text(str(O)+'\n')
H=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
roots=[Path('/var/tmp/pencil-native-normalization-composition-20261103-v1/imports'),Path('/var/tmp/pencil-collector-composition-20261101-v1/reviews/graph/snapshot'),Path('/var/tmp/pencil-comparator-word-20261101/snapshot'),Path('/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union'),Path('/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union')];lean=Path('/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean')
def inventory():return {str(f):H(f) for root in roots+[lean.parent.parent] for f in sorted(root.rglob('*')) if f.is_file()}
def interrupted(s,f):raise RuntimeError('signal '+str(s))
signal.signal(signal.SIGTERM,interrupted);signal.signal(signal.SIGINT,interrupted);rows=[];status='failed'
try:
 before=inventory();(O/'inputs-before.json').write_text(json.dumps(before,sort_keys=True)+'\n')
 private=[]
 for n in ['Difference','Readback','ExpectedFailure']:(O/(n+'.lean')).write_bytes((P/(n+'.lean.txt')).read_bytes())
 sourcepins={str(f):H(f) for f in sorted(O.rglob('*')) if f.is_file() and f.name!='inputs-before.json'};(O/'private-inputs.json').write_text(json.dumps(sourcepins,sort_keys=True)+'\n')
 env=dict(os.environ,LEAN_SYSROOT=str(lean.parent.parent),LEAN_PATH=':'.join(map(str,[O]+roots)))
 for n in ['Difference','Readback','ExpectedFailure']:
  remaining=270-(time.monotonic()-start);assert remaining>0
  cmd=[str(lean),'-j1','-M3500','-s16384','-o',str(O/(n+'.olean')),str(O/(n+'.lean'))];row={'module':n,'argv':cmd,'expected_exit':1 if n=='ExpectedFailure' else 0};rows.append(row)
  with (O/(n+'.stdout')).open('wb') as so,(O/(n+'.stderr')).open('wb') as se:
   child=subprocess.Popen(cmd,cwd=O,env=env,stdout=so,stderr=se,start_new_session=True)
   try:row['exit']=child.wait(timeout=remaining)
   except BaseException:
    os.killpg(child.pid,signal.SIGKILL);child.wait(timeout=2);raise
   child=None
  assert row['exit']==row['expected_exit'],row
 def audit(raw,names):
  found=re.findall(r"'([^']+)' depends on axioms: \[([^\]]*)\]",raw);no=re.findall(r"'([^']+)' does not depend on any axioms",raw);got=[n for n,a in found]+no
  assert sorted(got)==sorted(names) and len(got)==len(set(got)),'declaration coverage'
  for n,a in found:assert set(filter(None,(x.strip() for x in a.split(','))))<={'propext','Classical.choice','Quot.sound'},'forbidden axiom'
  return got
 expected=json.loads((SRC/'lean-names.json').read_text());names=audit((O/'Readback.stdout').read_text(),expected)
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
 result={'status':status,'error':locals().get('error'),'commands':rows,'elapsed_seconds':time.monotonic()-start,'dependency_count':len(locals().get('before',{})),'source_revision':REV,'workspace':str(O),'runtime_execution':False,'independent_binary_kernel_rechecker':False,'trust':'Pinned precursor and shared mathlib/toolchain caches; pre/post inventories do not exclude transient concurrent changes.'};(O/'receipt.json').write_text(json.dumps(result,indent=2)+'\n');(P/'lean-receipt.json').write_bytes((O/'receipt.json').read_bytes());print(json.dumps(result));sys.exit(0 if status=='passed' else 1)
