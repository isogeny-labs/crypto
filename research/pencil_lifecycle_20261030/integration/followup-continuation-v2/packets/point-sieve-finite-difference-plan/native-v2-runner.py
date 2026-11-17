from pathlib import Path
import hashlib,json,subprocess,time,os,signal,resource,sys,tempfile
P=Path(__file__).resolve().parent
resource.setrlimit(resource.RLIMIT_AS,(2*1024**3,2*1024**3))
start=time.monotonic();out=Path(tempfile.mkdtemp(prefix='point-sieve-native-',dir='/var/tmp'));(P/'native-v2-workspace.txt').write_text(str(out)+'\n');rows=[];status='failed';child=None
H=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
freeze=json.loads((P/'native-v2-freeze.json').read_text())
def signal_stop(s,f):raise RuntimeError('signal '+str(s))
signal.signal(signal.SIGTERM,signal_stop);signal.signal(signal.SIGINT,signal_stop)
try:
 assert all(H(Path(k))==v for k,v in freeze['files'].items())
 for name,cmd,cap in [('compile',[freeze['compiler'],'--edition=2021','-C','opt-level=1','-C','overflow-checks=yes',str(P/'native-v2/check.rs'),'-o',str(out/'check')],90),('check',[str(out/'check')],75)]:
  remaining=min(cap,165-(time.monotonic()-start));assert remaining>0
  row={'name':name,'argv':cmd,'reaped':False};rows.append(row)
  with (out/(name+'.stdout')).open('wb') as so,(out/(name+'.stderr')).open('wb') as se:
   child=subprocess.Popen(cmd,stdout=so,stderr=se,start_new_session=True)
   try:row['exit']=child.wait(timeout=remaining);row['reaped']=True
   except BaseException:
    os.killpg(child.pid,signal.SIGKILL);child.wait(timeout=2);row['reaped']=True;raise
   child=None
  assert row['exit']==0,row
 assert all(H(Path(k))==v for k,v in freeze['files'].items());status='passed'
except BaseException as e:error=repr(e)
finally:
 r={'status':status,'error':locals().get('error'),'commands':rows,'elapsed_seconds':time.monotonic()-start,'freeze_sha256':H(P/'native-v2-freeze.json'),'inputs_unchanged':all(H(Path(k))==v for k,v in freeze['files'].items()),'workspace':str(out),'scientific_execution':False,'full_crate_integration':False}
 (out/'receipt.json').write_text(json.dumps(r,indent=2)+'\n');(P/'native-v2-receipt.json').write_bytes((out/'receipt.json').read_bytes());print(json.dumps(r));sys.exit(0 if status=='passed' else 1)
