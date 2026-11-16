import os,sys,json,hashlib,time,resource,subprocess,signal
from pathlib import Path
D=Path(__file__).resolve().parent
W=Path('/var/tmp/retained-native-export-20261103-v1')
ROOT=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029')
resource.setrlimit(resource.RLIMIT_AS,(2*1024**3,2*1024**3))
resource.setrlimit(resource.RLIMIT_CORE,(0,0))
OUT=D/'attempt1';OUT.mkdir();start=time.monotonic();rows=[];pending=None;child=None
inputs=json.loads((D/'inputs.json').read_text())
def sha(f):
 h=hashlib.sha256()
 with Path(f).open('rb') as s:
  for b in iter(lambda:s.read(1024*1024),b''):h.update(b)
 return h.hexdigest()
def changed():return [f for f,v in inputs.items() if not Path(f).is_file() or sha(f)!=v['sha256']]
def stop(sig,frame):
 global pending
 pending=sig
 raise RuntimeError('received signal '+str(sig))
signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
def command(argv):
 global child
 remain=105-(time.monotonic()-start)
 assert remain>0
 row={'argv':argv,'reaped':False};rows.append(row)
 with (OUT/(str(len(rows))+'.stdout')).open('wb') as so,(OUT/(str(len(rows))+'.stderr')).open('wb') as se:
  child=subprocess.Popen(argv,cwd=W,stdout=so,stderr=se,start_new_session=True,env={'PATH':'/usr/bin:/bin','HOME':str(W),'CARGO_NET_OFFLINE':'true','CARGO_BUILD_JOBS':'1','RUST_TEST_THREADS':'1','TMPDIR':str(W),'LC_ALL':'C','RUST_MIN_STACK':'16777216'})
  row['pid']=child.pid
  try:row['exit']=child.wait(timeout=remain);row['reaped']=True
  except BaseException:
   os.killpg(child.pid,signal.SIGKILL)
   try:row['exit']=child.wait(timeout=2);row['reaped']=True
   except BaseException:row['reaped']=False
   raise
  child=None
 assert row['exit']==0,row
rustc='/home/ec2-user/.rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin/rustc'
deps=ROOT/'target/release/deps'
args=[rustc,'--edition=2021','--test','--crate-name','retained_native_export',str(W/'lib.rs'),'-o',str(W/'controls'),'-L','dependency='+str(deps),'-C','codegen-units=1','-C','debuginfo=0','-C','opt-level=0','-C','linker=/usr/bin/cc','-C','link-arg=-fuse-ld=bfd']
for name,file in [('crypto_lib','libcrypto_lib-daf9fdac4ff20898.rlib'),('serde_json','libserde_json-458049281485fb75.rlib'),('serde','libserde-0bf82e85196bd177.rlib'),('libc','liblibc-56ab79ec8afd5c44.rlib'),('sha2','libsha2-d9f2e7249be3be6b.rlib')]:args += ['--extern',name+'='+str(deps/file)]
status='failed';error=None
try:
 assert not changed(),'changed preregistered input'
 command([rustc,'-Vv'])
 command(args)
 command([str(W/'controls'),'--test-threads=1','--nocapture'])
 assert not changed(),'changed input after check'
 assert pending is None and time.monotonic()-start<110
 status='passed'
except BaseException as e:error=repr(e)
finally:
 receipt={'schema':'software.retained-export-check/v1','status':status,'error':error,'commands':rows,'elapsed_seconds':time.monotonic()-start,'pending_signal':pending,'changed_inputs':changed(),'cached_dependencies_rebuilt':False,'scientific_runs':0,'outer_ceiling_seconds':120,'hard_address_space_bytes_per_process':2*1024**3,'build_jobs':1,'offline':True}
 (OUT/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt));sys.exit(0 if status=='passed' else 1)
