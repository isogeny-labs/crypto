import resource,subprocess,os,time,json,hashlib,signal
from pathlib import Path
D=Path(__file__).parent;S=D/'snapshot';O=D/'replay';O.mkdir();start=time.monotonic()
resource.setrlimit(resource.RLIMIT_AS,(2*1024**3,2*1024**3))
j=json.loads((D/'execution-freeze.json').read_text());result={'status':'failed'}
def hashes():return {n:hashlib.sha256((S/n).read_bytes()).hexdigest() for n in j['inputs']}
assert hashes()==j['inputs']
proc=None
try:
 with (O/'stdout').open('wb') as so,(O/'stderr').open('wb') as se:
  proc=subprocess.Popen(j['command'],cwd=S,stdout=so,stderr=se,start_new_session=True)
  try:result['exit']=proc.wait(timeout=105)
  except BaseException:
   os.killpg(proc.pid,signal.SIGKILL);proc.wait(timeout=3);raise
 result['after']=hashes();assert result['after']==j['inputs'] and result['exit']==0
 result['status']='passed'
except BaseException as e:result['error']=repr(e)
finally:
 result.update(elapsed_seconds=time.monotonic()-start,command=j['command'],runtime_executed=False,memory_bytes=j['memory_bytes'])
 (O/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
print(result['status']);raise SystemExit(0 if result['status']=='passed' else 1)
