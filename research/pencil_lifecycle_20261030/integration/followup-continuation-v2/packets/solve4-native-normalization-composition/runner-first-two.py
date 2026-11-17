import os,sys,time,json,hashlib,signal,subprocess,resource,re
from pathlib import Path
D=Path(__file__).resolve().parent
resource.setrlimit(resource.RLIMIT_AS,(8*1024**3,8*1024**3))
resource.setrlimit(resource.RLIMIT_STACK,(10*1024**2,10*1024**2))
number=int(sys.argv[1]);assert number in (1,2)
WORK=Path('/var/tmp/pencil-native-normalization-composition-20261103-v1')
OUT=WORK/('attempt'+str(number));OUT.mkdir()
mods=['Narrow','Normalization','Composition','Controls','Readback','ExpectedFailure']
for n in mods:(OUT/(n+'.lean')).write_bytes((D/(n+'.lean.txt')).read_bytes())
start=time.monotonic();rows=[];child=None;pending=None
roots=[WORK/'imports',Path('/var/tmp/pencil-collector-composition-20261101-v1/reviews/graph/snapshot'),Path('/var/tmp/pencil-comparator-word-20261101/snapshot'),Path('/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union'),Path('/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union')]
lean=Path('/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean')
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
inputs={str(p):digest(p) for root in roots+[lean.parent.parent] for p in sorted(root.rglob('*')) if p.is_file()}
inputs.update({str(D/(n+'.lean.txt')):digest(D/(n+'.lean.txt')) for n in mods})
inputs.update({str(D/n):digest(D/n) for n in ['runner.py','check.sh','PLAN.md','names.json']})
(OUT/'inputs.json').write_text(json.dumps(inputs,indent=2)+'\n')
def stop(sig,frame):
 global pending
 pending=sig
 raise RuntimeError('signal '+str(sig))
signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
try:
 env=dict(os.environ,LEAN_SYSROOT=str(lean.parent.parent),LEAN_PATH=':'.join(map(str,[OUT]+roots)))
 for n in mods:
  remaining=80-(time.monotonic()-start)
  assert remaining>0
  cmd=[str(lean),'-j1','-M3500','-s16384','-o',str(OUT/(n+'.olean')),str(OUT/(n+'.lean'))]
  row={'module':n,'command':cmd,'expected_exit':1 if n=='ExpectedFailure' else 0,'reaped':False};rows.append(row)
  with (OUT/(n+'.stdout')).open('wb') as so,(OUT/(n+'.stderr')).open('wb') as se:
   child=subprocess.Popen(cmd,cwd=OUT,env=env,stdout=so,stderr=se,start_new_session=True)
   row['pid']=child.pid
   try:row['exit']=child.wait(timeout=remaining);row['reaped']=True
   except BaseException:
    os.killpg(child.pid,signal.SIGKILL)
    try:row['exit']=child.wait(timeout=2);row['reaped']=True
    except BaseException:row['reaped']=False
    raise
   child=None
  assert row['exit']==row['expected_exit'],row
 assert 'error:' in (OUT/'ExpectedFailure.stdout').read_text()
 audit=(OUT/'Readback.stdout').read_text()
 found=re.findall(r"'([^']+)' depends on axioms: \[([^\]]*)\]",audit)
 noaxioms=re.findall(r"'([^']+)' does not depend on any axioms",audit)
 names=[n for n,a in found]+noaxioms
 expected=json.loads((D/'names.json').read_text())
 assert sorted(names)==sorted(expected),(names,expected)
 for n,a in found:assert set(filter(None,(x.strip() for x in a.split(','))))<={'propext','Classical.choice','Quot.sound'}
 (OUT/'axioms.json').write_text(json.dumps({'names':names,'passed':True},indent=2)+'\n')
 changed=[p for p,h in inputs.items() if not Path(p).exists() or digest(Path(p))!=h]
 assert not changed,changed
 assert pending is None and time.monotonic()-start<90
 status='passed'
except BaseException as e:status='failed';error=repr(e)
finally:
 changed=[p for p,h in inputs.items() if not Path(p).exists() or digest(Path(p))!=h]
 (OUT/'receipt.json').write_text(json.dumps({'status':status,'error':locals().get('error'),'commands':rows,'elapsed_seconds':time.monotonic()-start,'inputs':len(inputs),'changed_inputs':locals().get('changed'),'scientific_runs':0,'pending_signal':pending,'limits':{'outer_seconds':95,'address_space_bytes':8*1024**3,'lean_heap_MiB':3500,'hard_OS_stack_bytes':10*1024**2,'lean_stack_KiB':16384,'workers':1}},indent=2)+'\n')
print(status)
sys.exit(0 if status=='passed' else 1)
