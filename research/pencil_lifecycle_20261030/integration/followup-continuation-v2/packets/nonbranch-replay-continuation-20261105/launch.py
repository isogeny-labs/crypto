#!/usr/bin/python3
"""Single prospective continuation. Invoke only after publication and independent review."""
from pathlib import Path
import argparse,hashlib,json,os,subprocess,time
P=Path(__file__).resolve().parent
W=Path('/home/ec2-user/pencil-native-nonbranch-t1628-20261105')
O=Path(str(W)+'.outer')
DEADLINE=1793937600
ap=argparse.ArgumentParser();ap.add_argument('--execute',action='store_true');ap.add_argument('--freeze-sha256',required=True)
a=ap.parse_args()
assert a.execute,'Preparation only; explicit execution required after independent review and publication'
assert hashlib.sha256((P/'freeze.json').read_bytes()).hexdigest()==a.freeze_sha256,'freeze mismatch'
F=json.loads((P/'freeze.json').read_text())
for name,digest in F['files'].items():assert hashlib.sha256((P/name).read_bytes()).hexdigest()==digest,('frozen input',name)
assert time.time()+2400<DEADLINE,'insufficient absolute deadline allowance'
assert not W.exists(),'existing native workspace; no retries'
O.mkdir(mode=0o700)  # exclusive admission: existing outer directory also rejects repeats
start=time.monotonic()
def durable(name,obj):
 t=O/(name+'.tmp');d=O/name
 with t.open('x') as f:json.dump(obj,f,sort_keys=True);f.write('\n');f.flush();os.fsync(f.fileno())
 os.replace(t,d);fd=os.open(O,os.O_DIRECTORY)
 try:os.fsync(fd)
 finally:os.close(fd)
results=[]
try:
 durable('admission.json',{'freeze_sha256':a.freeze_sha256,'start_utc_seconds':time.time(),'campaign_seconds':46800,'prior_debit_seconds':300,'continuation_ceiling_seconds':2400,'deadline':DEADLINE,'retries':0})
 for phase,limit in [('pre',900),('compiler',600),('post',900)]:
  if phase=='compiler' and not results[-1]['passed']:break
  # Post-custody remains required after any compiler attempt, including failed attempts.
  assert time.time()+limit<DEADLINE and time.monotonic()-start+limit<=2400,'insufficient remaining phase allowance'
  unit='pencil-nonbranch-t1628-'+phase
  cmd=['sudo','-n','systemd-run','--unit='+unit,'--wait',
   '--property=User=ec2-user','--property=WorkingDirectory='+str(P),
   '--property=CPUQuota=100%','--property=MemoryMax=8G','--property=MemorySwapMax=0',
   '--property=TasksMax=128','--property=KillMode=control-group','--property=OOMPolicy=stop',
   '--property=RuntimeMaxSec='+str(limit),
   '--property=StandardOutput=file:'+str(O/(phase+'.stdout')),
   '--property=StandardError=file:'+str(O/(phase+'.stderr')),
   '/usr/bin/timeout','--signal=TERM','--kill-after=5s',str(limit-5)+'s',
   '/usr/bin/python3',str(P/'replay.py'),phase,a.freeze_sha256]
  durable(phase+'-command.json',{'argv':cmd,'limit_seconds':limit,'submitted':False})
  phase_start=time.monotonic()
  r=subprocess.run(cmd,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
  native=json.loads((W/(phase+'-receipt.json')).read_text()) if (W/(phase+'-receipt.json')).exists() else None
  passed=r.returncode==0 and native is not None and native['status']=='passed' and time.monotonic()-phase_start<limit and time.time()<DEADLINE
  row={'phase':phase,'argv':cmd,'returncode':r.returncode,'stdout':r.stdout,'stderr':r.stderr,'elapsed_seconds':time.monotonic()-phase_start,'native_receipt_present':native is not None,'passed':passed}
  durable(phase+'-outer-exit.json',row);results.append(row)
  # No automatic retry. A failed pre ends here; compiler failure still gets post custody.
  if phase=='pre' and not passed:break
 passed=len(results)==3 and all(r['passed'] for r in results) and time.monotonic()-start<2400 and time.time()<DEADLINE
 durable('completion.json',{'status':'passed' if passed else 'failed','phases':results,'elapsed_seconds':time.monotonic()-start,'machine_review_complete':passed,'costs_are_worker_wall_not_cpu':True})
 raise SystemExit(0 if passed else 1)
except Exception as e:
 durable('launcher-failure.json',{'error':repr(e),'phases':results,'elapsed_seconds':time.monotonic()-start,'status':'failed'})
 raise
