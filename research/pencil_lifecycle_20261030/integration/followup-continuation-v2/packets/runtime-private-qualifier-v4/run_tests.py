import hashlib,json,os,resource,signal,subprocess,time
from pathlib import Path
P=Path(__file__).resolve().parent
resource.setrlimit(resource.RLIMIT_AS,(2*1024**3,2*1024**3))
OUT=P/'attempt2';OUT.mkdir();start=time.monotonic();pending=None;proc=None;receipt={'status':'failed','runtime_executed':False}
files=['supervise.py','private_contract.py','head_contract.py','gate.c','binding.template.json','test_private.py','test_inspection.py','test_release_fd.py','run_tests.py','check.sh','PLAN.md','AMENDMENT.md']
def hashes():return {n:hashlib.sha256((P/n).read_bytes()).hexdigest() for n in files}
before=hashes();(OUT/'inputs.json').write_text(json.dumps(before,indent=2)+'\n')
def stop(sig,frame):
 global pending
 pending=sig
 raise RuntimeError('signal '+str(sig))
signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
try:
 with (OUT/'stdout').open('wb') as so,(OUT/'stderr').open('wb') as se:
  argv=['python3','-B','-m','unittest','-v','test_private','test_inspection','test_release_fd'];receipt['command']=argv
  proc=subprocess.Popen(argv,cwd=P,stdout=so,stderr=se,start_new_session=True);receipt['pid']=proc.pid
  try:receipt['exit']=proc.wait(timeout=105)
  except BaseException:
   os.killpg(proc.pid,signal.SIGKILL)
   try:receipt['exit']=proc.wait(timeout=2);receipt['reaped']=True
   except BaseException:receipt['reaped']=False
   raise
 receipt['reaped']=True;receipt['changed_inputs']=[n for n,h in hashes().items() if before[n]!=h]
 assert receipt['exit']==0 and not receipt['changed_inputs'] and pending is None and time.monotonic()-start<110
 receipt['status']='passed'
except BaseException as e:receipt['error']=repr(e)
finally:
 receipt.update(elapsed_seconds=time.monotonic()-start,pending_signal=pending,memory_bytes=2*1024**3,outer_seconds=120)
 (OUT/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt));raise SystemExit(0 if receipt['status']=='passed' else 1)
