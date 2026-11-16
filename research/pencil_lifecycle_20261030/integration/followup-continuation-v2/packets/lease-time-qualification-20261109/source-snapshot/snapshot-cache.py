#!/usr/bin/env python3
"""One bounded read-only snapshot; no cache writes or compiler invocation."""
import argparse, datetime, hashlib, json, os, resource, stat, time
from pathlib import Path
from durable_io import atomic_json
A=argparse.ArgumentParser();A.add_argument('--execute',action='store_true');A.add_argument('--output',required=True);a=A.parse_args()
assert a.execute
START=time.monotonic();DEADLINE=datetime.datetime(2026,11,10,5,11,39,tzinfo=datetime.timezone.utc)
resource.setrlimit(resource.RLIMIT_AS,(1024**3,1024**3))
P=Path(__file__).resolve().parent;OUT=Path(a.output);OUT.mkdir(mode=0o700,parents=False,exist_ok=False)
STAGE=Path('/var/tmp/pencil-root-materialize-t1953-20261109/cache-stage-v2');ROOTS=[STAGE/n for n in ['target','cargo-home','tmp']]
R={'status':'failed','worker_reservation_seconds':60,'cpu':1,'memory_bytes':1024**3,'deadline':DEADLINE.isoformat(),'compiler_invoked':False}
def guard():
 assert time.monotonic()-START<55,'snapshot work deadline'
 assert datetime.datetime.now(datetime.timezone.utc)<DEADLINE,'absolute deadline'
def inventory():
 files={};directories=[]
 for root in ROOTS:
  assert stat.S_ISDIR(root.lstat().st_mode),str(root)
  for base,dirs,names in os.walk(root,followlinks=False):
   guard();directories.append(base)
   for n in dirs:assert stat.S_ISDIR((Path(base)/n).lstat().st_mode),str(Path(base)/n)
   for n in names:
    f=Path(base)/n;s=f.lstat();assert stat.S_ISREG(s.st_mode),str(f)
    files[str(f)]=(s.st_dev,s.st_ino,s.st_size,s.st_mtime_ns,s.st_ctime_ns)
 assert sum(row[2] for row in files.values())<=8*1024**3,'snapshot byte ceiling'
 return files,sorted(directories)
def digest(path):
 guard();fd=os.open(path,os.O_RDONLY|os.O_NOFOLLOW);h=hashlib.sha256()
 with os.fdopen(fd,'rb') as f:
  before=os.fstat(f.fileno());assert stat.S_ISREG(before.st_mode)
  for b in iter(lambda:f.read(4*1024**2),b''):guard();h.update(b)
  after=os.fstat(f.fileno());assert (before.st_dev,before.st_ino,before.st_size,before.st_mtime_ns,before.st_ctime_ns)==(after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns,after.st_ctime_ns),'file changed while hashing'
 return h.hexdigest()
try:
 initial=Path('/home/ec2-user/pencil-validation-receipts/t1953-materializer-20261109/worker/materialized-inputs.json')
 assert digest(initial)=='c5178d2138d79f1ea8cac1fc96cb8d968913ba403d68ee66fa36afdd60fdf3f2'
 old=json.loads(initial.read_text())['initial_cache'];before,dirs=inventory()
 rows={n:{'sha256':digest(n),'bytes':v[2]} for n,v in sorted(before.items())}
 after,afterdirs=inventory();assert before==after and dirs==afterdirs,'cache changed during snapshot'
 snapshot={'schema':'root-current-cache/v1','roots':[str(x) for x in ROOTS],'directories':dirs,'files':rows,'initial_manifest_sha256':'c5178d2138d79f1ea8cac1fc96cb8d968913ba403d68ee66fa36afdd60fdf3f2','elapsed_seconds':time.monotonic()-START}
 atomic_json(OUT/'current-cache.json',snapshot)
 changes={'added':sorted(set(rows)-set(old)),'missing':sorted(set(old)-set(rows)),'changed':sorted(n for n in set(rows)&set(old) if rows[n]!=old[n]),'unchanged_count':sum(rows[n]==old[n] for n in set(rows)&set(old)),'attribution':'Comparison to initial target-only manifest; additions in other mutable roots are not attributed solely to the failed compiler.'}
 atomic_json(OUT/'changes.json',changes)
 R.update(status='passed',cache_sha256=digest(OUT/'current-cache.json'),file_count=len(rows),bytes=sum(x['bytes'] for x in rows.values()),changes_sha256=digest(OUT/'changes.json'))
except BaseException as e:
 R['error']={'type':type(e).__name__,'message':str(e)}
 raise
finally:
 R['elapsed_seconds']=time.monotonic()-START;atomic_json(OUT/'result.json',R)
