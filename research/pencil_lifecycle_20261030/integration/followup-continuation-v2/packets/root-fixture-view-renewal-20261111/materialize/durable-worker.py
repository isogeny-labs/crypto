#!/usr/bin/env python3
import datetime, hashlib, json, math, os, re, resource, shutil, signal, stat, subprocess, sys, time, traceback
from pathlib import Path
P=Path(__file__).resolve().parent
from durable_io import atomic_json, flush_logs, load_admission, durable_limits
F=load_admission(P)
W=Path(F['workspace']); OUT=Path(F['durable_run'])/'worker'; SOURCE=Path(F['compiled_source']); STAGE=W/'scratch'
START=time.monotonic(); END=float(os.environ['PENCIL_CHECK_END_MONOTONIC']); ACTIVE=None; INTERRUPTED=False;OWNED_OUTPUT=False;PERMIT_REQUIRED=False;DISK_NEXT=0
R={'status':'failed','commands':[],'errors':[],'invocation_id':os.environ.get('INVOCATION_ID'),'source_scope':'exact fixture-view materialization or unchanged cached library suite'}
resource.setrlimit(resource.RLIMIT_AS,(16*1024**3,16*1024**3))
def checkpoint(phase):
 R['phase']=phase;R['elapsed_seconds']=time.monotonic()-START
 if OWNED_OUTPUT:
  atomic_json(OUT/'worker-receipt.json',R);flush_logs(OUT)
def interrupted(sig,frame):
 global INTERRUPTED
 INTERRUPTED=True
 raise RuntimeError('signal '+str(sig))
for sig in (signal.SIGTERM,signal.SIGINT):signal.signal(sig,interrupted)
def retain_permit_anomaly(snapshots, error=None):
 row={'snapshots':snapshots}
 if error is not None:row['error']=repr(error)
 R.setdefault('permit_anomalies',[]).append(row)
 R['permit_anomalies']=R['permit_anomalies'][-32:]
def read_permit_snapshot():
 meta={'read_start_unix':time.time(),'read_start_monotonic':time.monotonic()}
 try:
  with (Path(F['durable_run'])/'supervisor-permit.json').open('rb') as stream:raw=stream.read(4097)
  meta.update(bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest())
  assert len(raw)<=4096,'supervisor permit size'
  permit=json.loads(raw)
  meta['value']=permit
  assert isinstance(permit,dict),'supervisor permit object'
  assert permit['invocation_id']==R['invocation_id'] and permit['lease']==F['lease'],'supervisor permit identity'
  expiry=permit['expires_unix']
  assert isinstance(expiry,(int,float)) and not isinstance(expiry,bool) and math.isfinite(expiry),'supervisor permit expiry format'
  meta.update(read_end_unix=time.time(),read_end_monotonic=time.monotonic())
  return permit,meta
 except BaseException as error:
  meta.update(read_end_unix=time.time(),read_end_monotonic=time.monotonic())
  retain_permit_anomaly([meta],error)
  raise
def check_permit():
 first,one=read_permit_snapshot()
 if time.time()<first['expires_unix']:return
 # No grace period: only a newly read, identity-matched, unexpired permit can pass.
 retry_start=time.monotonic()
 try:
  second,two=read_permit_snapshot()
  assert time.monotonic()-retry_start<=0.25,'supervisor permit reread bound'
  assert not INTERRUPTED and time.monotonic()<END-40,'supervisor permit deadline/signal'
  assert time.time()<second['expires_unix'],'supervisor permit expired'
 except BaseException as error:
  retain_permit_anomaly([one]+([two] if 'two' in locals() else []),error)
  raise
 retain_permit_anomaly([one,two])
def guard():
 global DISK_NEXT
 if time.monotonic()>=DISK_NEXT:
  DISK_NEXT=time.monotonic()+1;size=0
  for base,dirs,names in os.walk(W,followlinks=False,onerror=lambda e: (_ for _ in ()).throw(e)):
   for name in names:
    f=Path(base)/name
    try:st=f.lstat()
    except FileNotFoundError:continue
    size+=st.st_size
  assert size<=1024**3,"private workspace 1GiB ceiling"
 if PERMIT_REQUIRED:check_permit()
 if OWNED_OUTPUT:durable_limits(Path(F['durable_run']),F['limits'])
 if INTERRUPTED or time.monotonic()>=END-40:raise RuntimeError('work deadline/signal')
 if datetime.datetime.now(datetime.timezone.utc)>=datetime.datetime.fromisoformat(F['absolute_deadline'].replace('Z','+00:00')):raise RuntimeError('absolute deadline')
 if shutil.disk_usage(W).free<6*1024**3:raise RuntimeError('free disk below6GiB')
def H(path):
 guard(); h=hashlib.sha256()
 with path.open('rb') as s:
  for b in iter(lambda:s.read(4*1024**2),b''):guard();h.update(b)
 return h.hexdigest()
def verify(rows):
 for path,row in rows.items():
  f=Path(path)
  if 'resolved' in row:assert str(f.resolve())==row['resolved'],path
  else:assert stat.S_ISREG(f.lstat().st_mode),path
  assert H(f)==row['sha256'],path
def traversal_error(error):
 raise error
def run(label,argv,cwd=None,env=None,require_success=True):
 global ACTIVE
 guard();row={'label':label,'argv':argv};R['commands'].append(row);t=time.monotonic()
 with (OUT/(label+'.stdout')).open('xb') as out,(OUT/(label+'.stderr')).open('xb') as err:
  ACTIVE=subprocess.Popen(argv,cwd=cwd or W,env=env or {'PATH':'/usr/bin:/bin','LANG':'C.UTF-8'},stdout=out,stderr=err,start_new_session=True)
  row['pid']=ACTIVE.pid;checkpoint('running-'+label)
  try:
   while ACTIVE.poll() is None:
    guard();time.sleep(.1)
   row['exit_code']=ACTIVE.returncode
  finally:
   if ACTIVE.poll() is None:os.killpg(ACTIVE.pid,signal.SIGKILL)
   row['reaped_exit_code']=ACTIVE.wait(timeout=3)
   try:os.killpg(ACTIVE.pid,signal.SIGKILL)
   except ProcessLookupError:pass
   ACTIVE=None;row['elapsed_seconds']=time.monotonic()-t;checkpoint('finished-'+label)
 if require_success:assert row['exit_code']==0,label
 return (OUT/(label+'.stdout')).read_bytes()
def snapshot_toolchain():
 # Complete standard include tree and GCC driver tree, plus conservative runtime/linker closure.
 import glob
 roots=['/usr/include','/usr/lib/gcc/x86_64-amazon-linux/11','/usr/libexec/gcc/x86_64-amazon-linux/11','/usr/lib64/ldscripts']
 paths=set(['/usr/bin/c++','/usr/bin/g++','/usr/bin/as','/usr/bin/ld','/usr/bin/readelf','/etc/ld.so.cache','/etc/ld.so.conf'])
 assert H(Path(F['elf_dependencies']))==F['elf_dependencies_sha256']
 deps=json.loads(Path(F['elf_dependencies']).read_text());assert not deps['unresolved']
 for row in deps['objects']:paths.add(row['path']);paths.add(row['resolved'])
 for root in roots:
  assert Path(root).is_dir(),root
  for base,dirs,names in os.walk(root,followlinks=False,onerror=traversal_error):
   guard()
   for n in names:paths.add(str(Path(base)/n))
   for n in dirs:
    p=Path(base)/n
    if p.is_symlink():raise RuntimeError('toolchain directory symlink requires explicit closure: '+str(p))
 for pat in ['libstdc++*','libgcc*','libc.*','libc-*','libc_nonshared*','libm.*','libm-*','libmpc*','libmpfr*','libgmp*','libisl*','libz.*','libzstd*','libbfd*','libctf*','libjansson*','libsframe*','libdl*','libpthread*','ld-linux*','crt*.o']:
  for root in ['/usr/lib64','/lib64']:paths.update(glob.glob(root+'/'+pat))
 rows={};total=0
 for p in sorted(paths):
  f=Path(p)
  if f.is_dir():continue
  resolved=f.resolve(strict=True);assert resolved.is_file()
  size=resolved.stat().st_size;total+=size;assert total<=1024**3,'toolchain inventory 1GiB read bound'
  rows[p]={'bytes':size,'sha256':H(resolved),'resolved':str(resolved)}
 return rows

def verify_toolchain_membership(rows):
 roots=['/usr/include','/usr/lib/gcc/x86_64-amazon-linux/11','/usr/libexec/gcc/x86_64-amazon-linux/11','/usr/lib64/ldscripts']
 for root in roots:
  actual=set()
  for base,dirs,names in os.walk(root,followlinks=False,onerror=traversal_error):
   guard()
   for n in dirs:assert not (Path(base)/n).is_symlink(),'toolchain directory symlink'
   for n in names:actual.add(str(Path(base)/n))
  assert actual=={p for p in rows if p.startswith(root+'/')},'toolchain membership changed: '+root
 verify(rows)

def copy_bound(src,dst,digest,size):
 guard();assert src.is_file() and not src.is_symlink();assert src.stat().st_size==size
 dst.parent.mkdir(parents=True,exist_ok=True);h=hashlib.sha256()
 with src.open('rb') as inp,dst.open('xb') as out:
  while True:
   b=inp.read(1024**2)
   if not b:break
   guard();h.update(b);out.write(b)
  out.flush();os.fsync(out.fileno())
 assert h.hexdigest()==digest,str(src)

def manifest_membership(root,rows):
 actual=set()
 for base,dirs,names in os.walk(root,followlinks=False,onerror=traversal_error):
  guard()
  for n in dirs:assert not (Path(base)/n).is_symlink()
  for n in names:actual.add(str(Path(base)/n))
 assert actual==set(rows),'source exact membership'
 verify(rows)

try:
 OUT.mkdir(mode=0o700,exist_ok=False);OWNED_OUTPUT=True;checkpoint('worker-start')
 assert os.getuid()==F['uid'] and os.getgid()==F['gid']
 assert R['invocation_id'] and re.fullmatch('[a-f0-9]{32}',R['invocation_id'])
 cgline=Path('/proc/self/cgroup').read_text().strip();assert cgline.startswith('0::/') and '\n' not in cgline
 cg=Path('/sys/fs/cgroup')/cgline[3:].lstrip('/');assert cg.name==F['unit']
 controls={n:(cg/n).read_text().strip() for n in ['cpu.max','memory.max','memory.swap.max','pids.max']}
 quota,period=controls['cpu.max'].split();assert quota!='max' and int(quota)<=2*int(period)
 assert controls['memory.max']==str(16*1024**3) and controls['memory.swap.max']=='0' and controls['pids.max']=='512'
 R['cgroup']={'path':str(cg),'inode':cg.stat().st_ino,'controls':controls}
 wait_end=min(END-40,time.monotonic()+10)
 while True:
  try:check_permit();break
  except (FileNotFoundError,AssertionError):
   if time.monotonic()>=wait_end:raise
   time.sleep(.1)
 PERMIT_REQUIRED=True
 assert H(Path(F['initial_manifest']))==F['initial_manifest_sha256']
 initial=json.loads(Path(F['initial_manifest']).read_text())['immutable']
 source_rows={p:r for p,r in initial.items() if p.startswith(F['compiled_source']+'/')}
 assert H(Path(F['binary']['path']))==F['binary']['sha256']
 assert H(Path(F['test_inventory']))==F['test_inventory_sha256']
 expected_tests=json.loads(Path(F['test_inventory']).read_text());assert len(expected_tests)==6179 and len(set(expected_tests))==6179
 if F['phase']=='materialize':
  verify(source_rows);view=W/'source';view.mkdir();rows={}
  for path,row in source_rows.items():
   dst=view/Path(path).relative_to(SOURCE);copy_bound(Path(path),dst,row['sha256'],row['bytes']);rows[str(dst)]={'bytes':row['bytes'],'sha256':row['sha256']}
  assert H(Path(F['additions_manifest']))==F['additions_manifest_sha256']
  additions=json.loads(Path(F['additions_manifest']).read_text());assert additions['git_commit']==F['git_commit'] and additions['total_bytes']==41117354
  for i,row in enumerate(additions['files']):
   rel=Path(row['path']);assert not rel.is_absolute() and '..' not in rel.parts
   dst=view/rel;assert not dst.exists(),'additive only'
   b=run('blob-'+str(i),['/usr/bin/git','-C',F['git_repository'],'cat-file','blob',row['git_blob']])
   assert len(b)==row['bytes'] and hashlib.sha256(b).hexdigest()==row['sha256'];dst.parent.mkdir(parents=True,exist_ok=True)
   with dst.open('xb') as out:out.write(b);out.flush();os.fsync(out.fileno())
   rows[str(dst)]={'bytes':row['bytes'],'sha256':row['sha256']}
  assert sum(r['bytes'] for r in rows.values())<=512*1024**2
  manifest_membership(view,rows);tools=snapshot_toolchain();verify_toolchain_membership(tools);verify(source_rows)
  ignored_output=run('ignored-inventory',[F['binary']['path'],'--list','--ignored']).decode()
  ignored_names=[line[:-6] for line in ignored_output.splitlines() if line.endswith(': test')]
  assert len(ignored_names)==len(set(ignored_names)) and set(ignored_names)<=set(expected_tests)
  result={'schema':'fixture-view-materialization/v1','initial_manifest_sha256':F['initial_manifest_sha256'],'additions_manifest_sha256':F['additions_manifest_sha256'],'source_rows':rows,'toolchain_rows':tools,'binary':F['binary'],'compiled_source':F['compiled_source'],'view_source':str(view),'expected_inventory_count':6179,'allowed_ignored_names':ignored_names}
  atomic_json(OUT/'materialized-view.json',result);R['materialized_view_sha256']=H(OUT/'materialized-view.json')
 else:
  meta=Path(F['materialized_view_manifest']);assert H(meta)==F['materialized_view_sha256'];view=json.loads(meta.read_text())
  assert view['binary']==F['binary'] and view['compiled_source']==F['compiled_source']
  # Verify mounted destination, not merely the unmounted original or view directory.
  rows={str(SOURCE/Path(p).relative_to(view['view_source'])):r for p,r in view['source_rows'].items()}
  manifest_membership(SOURCE,rows);verify_toolchain_membership(view['toolchain_rows'])
  assert any(str(SOURCE) in line and ' ro,' in line for line in Path('/proc/self/mountinfo').read_text().splitlines()),'read-only fixture mount required'
  tmp=W/'tmp';tmp.mkdir()
  env={'PATH':'/usr/bin:/bin','LANG':'C.UTF-8','LC_ALL':'C.UTF-8','TMPDIR':str(tmp),'CXX':'/usr/bin/c++','RUST_TEST_THREADS':'1','RUST_BACKTRACE':'0'}
  output=run('full-suite',[F['binary']['path'],'--test-threads=1','--nocapture'],cwd=SOURCE,env=env,require_success=False)
  result=R['commands'][-1];assert result['exit_code']==0,'cached full suite failed'
  summaries=re.findall(rb'test result: ok\. ([0-9]+) passed; ([0-9]+) failed; ([0-9]+) ignored; ([0-9]+) measured; ([0-9]+) filtered out;',output)
  assert len(summaries)==1,'one full suite summary required'
  passed,failed,ignored,measured,filtered=map(int,summaries[0])
  assert passed+ignored==6179 and failed==measured==filtered==0 and ignored==len(view['allowed_ignored_names']),'full6179 inventory counts'
  ignored_rows=re.findall(rb'^test (.*?) \.\.\. ignored(?:,.*?)?$',output,re.M)
  assert sorted(x.decode() for x in ignored_rows)==sorted(view['allowed_ignored_names']),'exact allowed ignored tests'
  R['summary']={'passed':passed,'failed':failed,'ignored':ignored,'measured':measured,'filtered':filtered,'allowed_ignored_names':view['allowed_ignored_names']}
  manifest_membership(SOURCE,rows);verify_toolchain_membership(view['toolchain_rows'])
  assert H(Path(F['binary']['path']))==F['binary']['sha256']
 R['post_inputs_verified']=True;R['status']='passed'
except BaseException as error:
 R['errors'].append({'error':repr(error),'traceback':traceback.format_exc()})
finally:
 if ACTIVE is not None:
  try:os.killpg(ACTIVE.pid,signal.SIGKILL);ACTIVE.wait(timeout=3)
  except BaseException as error:R['errors'].append({'cleanup':repr(error)})
 checkpoint('terminal')
sys.exit(0 if R['status']=='passed' else 1)
