
from pathlib import Path
import datetime,json,os,subprocess,fcntl,hashlib,signal
root=Path('/home/ec2-user/crypto-icl-L4');base=root/'research/f4_early_stop_20261018';lock=(base/'.resume-validation.lock').open('a+');fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
for p in Path('/proc').iterdir():
 if not p.name.isdigit() or int(p.name)==os.getpid():continue
 try:
  if os.readlink(p/'cwd')==str(root):raise RuntimeError('active L4 process: '+p.name)
 except (FileNotFoundError,PermissionError,ProcessLookupError):pass
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
out=base/('resume-validation-'+datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ'));out.mkdir()
paths=['src/cryptanalysis/f4_fp.rs','src/cryptanalysis/jv_cover.rs','examples/f4_stop_study.rs','examples/jv_cover.rs','Cargo.toml','Cargo.lock']
def hashes():return {p:hashlib.sha256((root/p).read_bytes()).hexdigest() for p in paths}
state={'status':'validating','started_at':now(),'pid':os.getpid(),'out':str(out),'source_sha256':hashes(),'head':subprocess.check_output(['git','-C',str(root),'rev-parse','HEAD'],text=True).strip(),'steps':[],'conductor_task':'T-15','new_measurement_runs':0}
def save():
 for path in [out/'state.json',base/'latest-resume-validation.json']:
  temp=path.with_suffix('.tmp')
  with temp.open('w') as f:json.dump(state,f,indent=2);f.flush();os.fsync(f.fileno())
  os.replace(temp,path)
  fd=os.open(path.parent,os.O_RDONLY)
  try:os.fsync(fd)
  finally:os.close(fd)
save()
commands=[('f4-focused',['cargo','test','--release','--locked','--offline','--lib','f4_fp','-j','2','--','--test-threads=1'],900),('cover-focused',['cargo','test','--release','--locked','--offline','--lib','jv_cover','-j','2','--','--test-threads=1'],900),('examples',['cargo','test','--release','--locked','--offline','--example','jv_cover','--example','f4_stop_study','-j','2','--','--test-threads=1'],900),('full-library',['cargo','test','--release','--locked','--offline','--lib','-j','2','--','--test-threads=2'],1800)]
try:
 for name,cmd,bound in commands:
  if hashes()!=state['source_sha256']:raise RuntimeError('source changed before '+name)
  row={'name':name,'command':cmd,'started_at':now(),'timeout_seconds':bound};state['steps'].append(row)
  with (out/(name+'.log')).open('wb') as log:
   p=subprocess.Popen(['taskset','-c','24,25','nice','-n','10']+cmd,cwd=root,env=dict(os.environ,RAYON_NUM_THREADS='1',CARGO_BUILD_JOBS='2'),stdin=subprocess.DEVNULL,stdout=log,stderr=subprocess.STDOUT,start_new_session=True);row['pid']=p.pid;save()
   try:code=p.wait(timeout=bound)
   except subprocess.TimeoutExpired:
    os.killpg(p.pid,signal.SIGTERM)
    try:p.wait(timeout=10)
    except subprocess.TimeoutExpired:os.killpg(p.pid,signal.SIGKILL);p.wait()
    code=124;row['timed_out']=True
  row.update(exit_code=code,finished_at=now());save()
  if code:raise RuntimeError(name+' exited '+str(code))
  if hashes()!=state['source_sha256']:raise RuntimeError('source changed during '+name)
 state['status']='passed'
except Exception as e:state.update(status='needs_attention',error=str(e))
finally:state['finished_at']=now();save()
