from pathlib import Path
import json,hashlib,os,sys,resource,subprocess,time,signal,re
D=Path(Path('/var/tmp/normalization-review-location').read_text());S=D/'source';O=D/'replay';O.mkdir();resource.setrlimit(resource.RLIMIT_AS,(8*1024**3,8*1024**3));resource.setrlimit(resource.RLIMIT_STACK,(10*1024**2,10*1024**2));start=time.monotonic();rows=[];child=None
roots=[S,*[D/f'imports-{i}' for i in range(3)],Path('/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union'),Path('/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union')];tool=Path('/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain');lean=tool/'bin/lean';generated={S/(n+'.olean') for n in ['Normalization','Controls','ReviewerControls','Readback','ExpectedFailure']}
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def inventory():return {str(p):digest(p) for root in [*roots,tool] for p in sorted(root.rglob('*')) if p.is_file() and p not in generated}
before=inventory();(O/'before.json').write_text(json.dumps(before,indent=2)+'\n');env=dict(os.environ,LEAN_SYSROOT=str(tool),LEAN_PATH=':'.join(map(str,roots)))
try:
 for n in ['Normalization','Controls','ReviewerControls','Readback','ExpectedFailure']:
  remaining=285-(time.monotonic()-start);assert remaining>0
  cmd=[str(lean),'-j1','-M3500','-s16384','-o',str(S/(n+'.olean')),str(S/(n+'.lean'))];r={'module':n,'argv':cmd};rows.append(r)
  with (O/(n+'.stdout')).open('wb') as so,(O/(n+'.stderr')).open('wb') as se:
   child=subprocess.Popen(cmd,cwd=D,env=env,stdout=so,stderr=se);r['exit']=child.wait(timeout=min(90,remaining));child=None
  assert r['exit']==(1 if n=='ExpectedFailure' else 0),(n,r)
 after=inventory();(O/'after.json').write_text(json.dumps(after,indent=2)+'\n');assert before==after,'input mutation'
 expected=json.loads((D/'expected-names.json').read_text());raw=(O/'Readback.stdout').read_text();found=re.findall(r"'([^']+)' depends on axioms: \[([^\]]*)\]",raw);none=re.findall(r"'([^']+)' does not depend on any axioms",raw);names=[n for n,_ in found]+none;assert len(names)==len(expected) and set(names)==set(expected)
 for n,a in found:assert set(filter(None,map(str.strip,a.split(',')))) <= {'propext','Classical.choice','Quot.sound'},(n,a)
 assert 'is false' in (O/'ExpectedFailure.stdout').read_text()
 assert time.monotonic()-start<295
 (O/'axiom-audit.json').write_text(json.dumps({'names':expected,'allowed':['propext','Classical.choice','Quot.sound'],'accepted':True},indent=2)+'\n');status='passed'
except BaseException as e:status='failed';error=repr(e)
finally:
 if child is not None and child.poll() is None:child.kill();child.wait(timeout=2)
 (O/'receipt.json').write_text(json.dumps({'status':status,'error':locals().get('error'),'commands':rows,'elapsed_seconds':time.monotonic()-start,'input_files':len(before),'unchanged':locals().get('after')==before,'outer_seconds':300,'scientific_runs':0},indent=2)+'\n')
print(status);sys.exit(0 if status=='passed' else 1)
