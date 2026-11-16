import os,json,time,signal,resource,subprocess,hashlib
from pathlib import Path
D=Path(__file__).resolve().parent;O=D/'compat-attempt1';O.mkdir()
resource.setrlimit(resource.RLIMIT_AS,(2147483648,2147483648));resource.setrlimit(resource.RLIMIT_FSIZE,(2796202,2796202));resource.setrlimit(resource.RLIMIT_CORE,(0,0))
start=time.monotonic();p=None;rows=[];pending=None
def sha(p):
 h=hashlib.sha256()
 with Path(p).open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
inputs=json.loads((D/'compat-inputs.json').read_text())
def changed():return [f for f,v in inputs.items() if sha(f)!=v['sha256']]
def stop(sig,frame):
 global pending
 pending=sig;raise RuntimeError('signal '+str(sig))
signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
status='no_match';error=None
try:
 assert not changed(),'changed inputs'
 for candidate in json.loads((D/'compat-candidates.json').read_text()):
  remaining=48-(time.monotonic()-start);assert remaining>0,'shared deadline'
  name=candidate['name'];argv=['/home/ec2-user/.rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin/rustc','--edition=2021','--crate-name','metadata_probe','--crate-type=lib','--emit=metadata',str(D/'metadata-probe.rs'),'-o',str(O/(name+'.rmeta')),'-L','dependency='+candidate['dependency_dir'],'--extern','crypto_lib=/home/ec2-user/crypto-pencil-signed-graph-20261029/target/release/deps/libcrypto_lib-daf9fdac4ff20898.rlib']
  row={'candidate':name,'argv':argv,'reaped':False};rows.append(row)
  env={'PATH':'/usr/bin:/bin','HOME':'/var/tmp/retained-native-export-compat-20261103-v1','TMPDIR':'/var/tmp/retained-native-export-compat-20261103-v1','LC_ALL':'C','RUSTC_LOG':'rustc_metadata::locator=info','CARGO_NET_OFFLINE':'true','CARGO_BUILD_JOBS':'1','RUST_MIN_STACK':'16777216'}
  with (O/(name+'.stdout')).open('wb') as so,(O/(name+'.stderr')).open('wb') as se:
   p=subprocess.Popen(argv,env=env,cwd=D,stdout=so,stderr=se,start_new_session=True);row['pid']=p.pid
   try:row['exit']=p.wait(timeout=remaining);row['reaped']=True
   except BaseException:
    try:os.killpg(p.pid,signal.SIGKILL)
    except ProcessLookupError:pass
    try:row['exit']=p.wait(timeout=2);row['reaped']=True
    except BaseException as x:row['reap_error']=repr(x)
    raise
   p=None
  if row['exit']==0:status='match';break
 assert not changed(),'changed inputs after probe'
 assert pending is None and time.monotonic()-start<53,'terminal deadline/signal'
except BaseException as e:status='failed';error=repr(e)
receipt={'status':status,'error':error,'commands':rows,'elapsed_seconds':time.monotonic()-start,'pending_signal':pending,'tests_run':0,'scientific_runs':0,'hard_as_bytes':2147483648,'outer_seconds':60,'outputs':{f.name:{'sha256':sha(f),'bytes':f.stat().st_size} for f in O.iterdir() if f.is_file()}}
(O/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(receipt))
