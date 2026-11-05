from pathlib import Path
import resource,subprocess,os,signal,time,json,hashlib
P=Path(__file__).resolve().parent;S=P/'snapshot';O=P/'native-evidence';O.mkdir();start=time.monotonic();rows=[];status='failed';child=None
resource.setrlimit(resource.RLIMIT_AS,(2*1024**3,2*1024**3))
def h(p):return hashlib.sha256(p.read_bytes()).hexdigest()
freeze=json.loads((P/'review-freeze.json').read_text())
def stop(s,f):raise RuntimeError('interrupted')
signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
try:
 assert all(h(Path(k))==v for k,v in freeze['files'].items())
 compiler=json.loads((S/'native-v2-freeze.json').read_text())['compiler']
 for name,cmd in [('compile',[compiler,'--edition=2021','-C','opt-level=1','-C','overflow-checks=yes',str(S/'native-v2/check.rs'),'-o',str(O/'check')]),('replay',[str(O/'check')])]:
  remain=108-(time.monotonic()-start);assert remain>0
  row={'name':name,'argv':cmd};rows.append(row)
  with (O/(name+'.stdout')).open('wb') as so,(O/(name+'.stderr')).open('wb') as se:
   child=subprocess.Popen(cmd,stdout=so,stderr=se,start_new_session=True)
   try:row['exit']=child.wait(timeout=remain)
   except BaseException:os.killpg(child.pid,signal.SIGKILL);child.wait(timeout=2);raise
   child=None
  assert row['exit']==0
 assert all(h(Path(k))==v for k,v in freeze['files'].items());status='passed'
except BaseException as e:error=repr(e)
finally:
 (O/'receipt.json').write_text(json.dumps({'status':status,'error':locals().get('error'),'commands':rows,'elapsed_seconds':time.monotonic()-start,'inputs_unchanged':all(h(Path(k))==v for k,v in freeze['files'].items()),'scientific_execution':False},indent=2)+'\n')
 print((O/'receipt.json').read_text())
 raise SystemExit(0 if status=='passed' else 1)
