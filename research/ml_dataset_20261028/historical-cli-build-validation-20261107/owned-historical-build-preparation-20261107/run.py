"""One authorized local link attempt; no solver execution or cloud submission."""
from pathlib import Path
import datetime,fcntl,hashlib,json,os,signal,subprocess,time
P=Path(__file__).resolve().parent
R=Path('/home/ec2-user/crypto-ml-checkpoint-integration-20261107')
TARGET=Path('/var/tmp/crypto-ml-search-recovery-target-20261107')
def save(name,value):(P/name).write_text(json.dumps(value,indent=2)+'\n')
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
    a=json.loads((P/'authorization.json').read_text())
    if a.get('approved') is not True or a.get('attempt_limit')!=1:raise SystemExit('explicit new local link authorization required')
    assert not (P/'started.json').exists(),'reconcile existing attempt; never retry implicitly'
    lock=open('/var/tmp/spot-host-heavy-build.lock','a')
    try:fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
    except BlockingIOError:raise SystemExit(75)
    expected=json.loads((P/'SOURCE_HASHES.json').read_text())
    def verify():
        for path,h in expected.items():assert sha(R/path)==h,path
    verify()
    head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip()
    # A later evidence-only commit may advance HEAD; compiled inputs must remain
    # exactly those published in the authorized source revision's manifest.
    published=json.loads(subprocess.check_output(['git','show',a['source_commit']+':research/ml_dataset_20261028/owned-historical-route-integration-20261107/ASSEMBLED_SOURCE_HASHES.json'],cwd=R,text=True))
    assert published==expected,'authorized published source manifest changed'
    record={'started_at':now(),'source_commit':a['source_commit'],'checkout_commit':head,'source_manifest_sha256':sha(P/'SOURCE_HASHES.json'),'seconds':1200,'jobs':2,'attempts':1,'cloud_jobs':0,'solver_executions':0,'commands':[]}
    with (P/'started.json').open('x') as f:json.dump(record,f)
    deadline=time.monotonic()+1200
    env=dict(os.environ,CARGO_TARGET_DIR=str(TARGET),CARGO_BUILD_JOBS='2',RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1')
    def run(argv,name):
        remaining=deadline-time.monotonic()
        if remaining<=0:return 124
        with (P/name).open('xb') as log:
            child=subprocess.Popen(argv,cwd=R,env=env,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
            record['pid']=child.pid;record['commands'].append(argv);save('started.json',record)
            try:return child.wait(timeout=remaining)
            except subprocess.TimeoutExpired:
                os.killpg(child.pid,signal.SIGKILL);child.wait();return 124
    try:
        record['build_exit']=run(['cargo','+1.98','build','--release','--bin','crypto','--locked','--offline'],'build.log')
        if record['build_exit']==0:
            record['binary_sha256']=sha(TARGET/'release/crypto')
            record['help_exit']=run([str(TARGET/'release/crypto'),'dataset','--help'],'help.log')
        verify();record['source_unchanged']=True
    except Exception as e:record['error']=repr(e)
    record['finished_at']=now();record['passed']=record.get('build_exit')==0 and record.get('help_exit')==0 and record.get('source_unchanged') is True
    save('receipt.json',record)
    return 0 if record['passed'] else 1
if __name__=='__main__':raise SystemExit(main())
