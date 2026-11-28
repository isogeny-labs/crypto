from pathlib import Path
import json,hashlib,subprocess,os,signal,datetime
P=Path(__file__).resolve().parent
R=P.parent/'owned-search-continuation-delivery-20261107/assembled'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def save(n,d):(P/n).write_text(json.dumps(d,indent=2)+'\n')
a=json.loads((P/'authorization.json').read_text())
assert a.get('approved') is True and a.get('attempt_limit')==1,'explicit new bounded attempt authorization required'
assert not (P/'started.json').exists(),'reconcile existing attempt; no retry'
expected=json.loads((P/'SOURCE_HASHES.json').read_text())
def verify():
 for rel,h in expected.items():
  assert sha(R/rel)==h,rel
verify()
args=['cargo','+1.98','check','--release','--lib','--bin','crypto','--locked','--offline']
r={'started_at':now(),'argv':args,'source_commit':a['source_commit'],'source_manifest_sha256':sha(P/'SOURCE_HASHES.json'),'jobs':2,'timeout_seconds':1200,'retries':0,'scientific_jobs':0,'native_solver_executed':False}
# Exclusive creation consumes the sole attempt before process creation.
with (P/'started.json').open('x') as f:json.dump(r,f)
try:
 env=dict(os.environ,CARGO_TARGET_DIR='/var/tmp/crypto-ml-search-recovery-target-20261107',CARGO_BUILD_JOBS='2',RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1')
 with (P/'compile.log').open('xb') as f:
  child=subprocess.Popen(args,cwd=R,env=env,stdout=f,stderr=subprocess.STDOUT,start_new_session=True)
  r['pid']=child.pid;save('started.json',r)
  try:r['exit_code']=child.wait(timeout=1200)
  except subprocess.TimeoutExpired:
   os.killpg(child.pid,signal.SIGKILL);child.wait();r['exit_code']=124
 verify();r['source_unchanged']=True;r['log_sha256']=sha(P/'compile.log')
except Exception as e:r['exception']=repr(e)
r['finished_at']=now();r['passed']=r.get('exit_code')==0 and r.get('source_unchanged') is True
save('receipt.json',r);print(json.dumps(r));raise SystemExit(0 if r['passed'] else 1)
