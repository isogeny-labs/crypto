from pathlib import Path
import json,hashlib,os,time,signal,tempfile,sys
P=Path(__file__).resolve().parent;C=Path('/var/tmp/nonbranch-import-v3-z0qu6o6i/checkpoint.json');expected='6c164aea372b13ba53638675e22f87aba7c138a51cbfd24e3700f087f6f552c8';H=lambda b:hashlib.sha256(b).hexdigest();assert H(C.read_bytes())==expected;j=json.loads(C.read_text());files={f['path']:f for v in j['done'].values() for f in v['files']};assert len(files)==16736;assert sum(v['bytes'] for v in files.values())==3111892318
mode='post';prior=json.loads((P/'custody-complete-receipt.json').read_text());assert prior['status']=='complete';assert H(Path(prior['hashes_path']).read_bytes())==prior['hashes_sha256']
W=Path(tempfile.mkdtemp(prefix='nonbranch-custody-'+mode+'-',dir='/var/tmp'));done={};start=time.monotonic();status='incomplete';error=None

def atomic(path,data):
 tmp=path.with_suffix('.tmp')
 with tmp.open('w') as f:json.dump(data,f,sort_keys=True);f.write('\n');f.flush();os.fsync(f.fileno())
 os.replace(tmp,path);fd=os.open(path.parent,os.O_DIRECTORY);os.fsync(fd);os.close(fd)
def save():atomic(W/'hashes.json',done)
def stop(s,f):raise RuntimeError('signal '+str(s))
signal.signal(signal.SIGTERM,stop);signal.signal(signal.SIGINT,stop)
try:
 for name,v in sorted(files.items()):
  if time.monotonic()-start>=290:raise RuntimeError('custody work cutoff')
  f=Path(name);assert str(f.resolve())==v['realpath'];before=f.stat();h=hashlib.sha256()
  with f.open('rb') as r:
   while True:
    if time.monotonic()-start>=290:raise RuntimeError('custody work cutoff')
    b=r.read(1024*1024)
    if not b:break
    h.update(b)
  after=f.stat();assert (before.st_dev,before.st_ino,before.st_size,before.st_mtime_ns)==(after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns);assert after.st_size==v['bytes']
  sha=h.hexdigest()
  if v['sha256'] is not None:assert sha==v['sha256'],'source hash changed '+name
  done[name]={'sha256':sha,'bytes':after.st_size,'device':after.st_dev,'inode':after.st_ino}
  if len(done)%200==0:save()
 if mode=='post':assert done==json.loads(Path(prior['hashes_path']).read_text()),'pre/post hash or identity mismatch'
 status='complete'
except BaseException as e:error=repr(e)
finally:
 save();r={'mode':mode,'status':status,'error':error,'hashes_path':str(W/'hashes.json'),'hashes_sha256':H((W/'hashes.json').read_bytes()),'files_hashed':len(done),'bytes_hashed':sum(v['bytes'] for v in done.values()),'expected_files':16736,'expected_bytes':3111892318,'elapsed_seconds':time.monotonic()-start,'compiler_invocations':0,'closure_sha256':expected};atomic(W/'receipt.json',r);(P/('custody-'+mode+'-receipt.json')).write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r));sys.exit(0 if status=='complete' else 1)
