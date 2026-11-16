import os,sys,json,time,signal,resource,subprocess,hashlib
from pathlib import Path
D=Path(__file__).resolve().parent
O=D/'metadata-attempt1';O.mkdir()
resource.setrlimit(resource.RLIMIT_AS,(2147483648,2147483648))
resource.setrlimit(resource.RLIMIT_FSIZE,(8388608,8388608))
resource.setrlimit(resource.RLIMIT_CORE,(0,0))
R=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029/target/release/deps')
argv=['/home/ec2-user/.rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/bin/rustc','--edition=2021','--crate-name','metadata_probe','--crate-type=lib','--emit=metadata',str(D/'metadata-probe.rs'),'-o',str(O/'probe.rmeta'),'-L','dependency='+str(R),'--extern','crypto_lib='+str(R/'libcrypto_lib-daf9fdac4ff20898.rlib')]
env={'PATH':'/usr/bin:/bin','HOME':'/var/tmp/retained-native-export-20261103-v1','TMPDIR':'/var/tmp/retained-native-export-20261103-v1','LC_ALL':'C','RUSTC_LOG':'rustc_metadata::locator=info','CARGO_NET_OFFLINE':'true','CARGO_BUILD_JOBS':'1','RUST_MIN_STACK':'16777216'}
start=time.monotonic();p=None;receipt={'argv':argv,'env':env,'reaped':False,'pending_signal':None,'scientific_runs':0,'tests_run':0,'hard_as_bytes':2147483648,'outer_seconds':60}
def stop(sig,frame):
 receipt['pending_signal']=sig
 raise RuntimeError('signal '+str(sig))
signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
try:
 with (O/'stdout').open('wb') as so,(O/'stderr').open('wb') as se:
  p=subprocess.Popen(argv,env=env,cwd=D,stdout=so,stderr=se,start_new_session=True)
  receipt['pid']=p.pid
  receipt['exit']=p.wait(timeout=50);receipt['reaped']=True
except BaseException as e:
 receipt['error']=repr(e)
 if p is not None and not receipt['reaped']:
  try:os.killpg(p.pid,signal.SIGKILL)
  except ProcessLookupError:pass
  try:receipt['exit']=p.wait(timeout=2);receipt['reaped']=True
  except BaseException as x:receipt['reap_error']=repr(x)
finally:
 receipt['elapsed_seconds']=time.monotonic()-start
 receipt['outputs']={f.name:{'bytes':f.stat().st_size,'sha256':hashlib.sha256(f.read_bytes()).hexdigest()} for f in O.iterdir() if f.is_file()}
 (O/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt))
