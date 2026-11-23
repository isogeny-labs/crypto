#!/usr/bin/env python3
"""Prospective bounded preparation; no compiler or Cargo command. Config is root-published."""
import datetime,hashlib,json,os,re,resource,shutil,signal,stat,subprocess,sys,tarfile,time,urllib.request,traceback
from pathlib import Path,PurePosixPath
P=Path(__file__).resolve().parent
F=json.loads((P/'preparation.json').read_text());START=time.monotonic();END=START+880
REV='29861c9a55f9ae5a33e1e09b3866aa397fbde605';W=Path(F['workspace']);OUT=Path(F['output']);REPO=Path(F['repository']);OWN=False;ACTIVE=None;LAST_HEARTBEAT=0
R={'status':'failed','revision':REV,'compiler_invoked':False,'errors':[],'files':0,'phase':'admission'};IMM={};CACHE={};SOURCE=W/'source';STAGE=W/'cache-stage-v2'
resource.setrlimit(resource.RLIMIT_AS,(4*1024**3,4*1024**3))
def stop(sig,frame):raise RuntimeError('signal '+str(sig))
for sig in [signal.SIGTERM,signal.SIGINT]:signal.signal(sig,stop)
def checkpoint():
 if not OWN:return
 R['elapsed_seconds']=time.monotonic()-START
 tmp=OUT/'receipt.tmp';tmp.write_text(json.dumps(R,indent=2)+'\n')
 with tmp.open('rb') as f:os.fsync(f.fileno())
 os.replace(tmp,OUT/'receipt.json');fd=os.open(OUT,os.O_RDONLY|os.O_DIRECTORY);os.fsync(fd);os.close(fd)
def guard():
 global LAST_HEARTBEAT
 if time.monotonic()-LAST_HEARTBEAT>=20:
  creds=json.loads(Path('/home/ec2-user/.conductor/credentials').read_text())
  req=urllib.request.Request(creds['endpoint']+'/v1/leases/heartbeat',data=json.dumps(F['lease']).encode(),headers={'Authorization':'Bearer '+creds['token'],'Content-Type':'application/json'})
  with urllib.request.urlopen(req,timeout=5) as response:assert response.status==200
  LAST_HEARTBEAT=time.monotonic()
 if time.monotonic()>=END:raise RuntimeError('880-second work limit')
 if datetime.datetime.now(datetime.timezone.utc)>=datetime.datetime.fromisoformat(F['absolute_deadline']):raise RuntimeError('campaign deadline')
 if W.exists() and shutil.disk_usage(W).free<6*1024**3:raise RuntimeError('free workspace below6GiB')
def H(path):
 h=hashlib.sha256()
 with Path(path).open('rb') as f:
  for b in iter(lambda:f.read(1024**2),b''):guard();h.update(b)
 return h.hexdigest()
def row(path,table):
 if Path(path).is_relative_to(W):
  with Path(path).open('rb') as f:os.fsync(f.fileno())
 table[str(path)]={'sha256':H(path),'bytes':path.stat().st_size};R['files']+=1
 if R['files']%20==0:checkpoint()
def cmd(args):
 global ACTIVE
 guard();ACTIVE=subprocess.Popen(args,cwd=REPO,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
 try:
  while True:
   guard()
   try:out,err=ACTIVE.communicate(timeout=min(1,max(.1,END-time.monotonic())));break
   except subprocess.TimeoutExpired:continue
  assert ACTIVE.returncode==0,err.decode(errors='replace')[:2000];return out
 finally:
  if ACTIVE.poll() is None:os.killpg(ACTIVE.pid,signal.SIGKILL)
  ACTIVE.wait();ACTIVE=None
try:
 assert F['revision']==REV and F['absolute_deadline']=='2026-11-06T04:00:00+00:00'
 assert F['published_github_commit'] and F['reviewed'] is True
 assert set(F['lease'])=={'task_id','attempt_id','lease_id','fencing_epoch'}
 assert F['campaign_remaining_worker_seconds']>=900 and type(F['campaign_remaining_worker_seconds']) is int
 assert F['campaign_remaining_worker_seconds']<=46500
 assert W.is_absolute() and OUT.is_absolute() and not W.is_relative_to(OUT) and not OUT.is_relative_to(W)
 assert str(OUT).startswith('/home/ec2-user/pencil-validation-receipts/')
 OUT.mkdir(mode=0o700,exist_ok=False);OWN=True;checkpoint()
 cgline=Path('/proc/self/cgroup').read_text().strip();assert cgline.startswith('0::/') and '\n' not in cgline
 cg=Path('/sys/fs/cgroup')/cgline[3:].lstrip('/');assert cg.name==F['unit']
 controls={n:(cg/n).read_text().strip() for n in ['cpu.max','memory.max','memory.swap.max','pids.max']}
 quota,period=controls['cpu.max'].split();assert quota!='max' and int(quota)<=2*int(period)
 assert controls['memory.max']==str(4*1024**3) and controls['memory.swap.max']=='0' and controls['pids.max']=='512'
 R['cgroup']={'path':str(cg),'controls':controls};checkpoint()
 R['workspace_mount']=cmd(['findmnt','-T',str(W.parent),'-J','-o','TARGET,SOURCE,FSTYPE,OPTIONS']).decode();R['output_mount']=cmd(['findmnt','-T',str(OUT),'-J','-o','TARGET,SOURCE,FSTYPE,OPTIONS']).decode();checkpoint()
 required={F[k] for k in ('historical_source_manifest','concat_addendum','candidate','registry_archives','tool_manifest')}|{str(Path(__file__).resolve())}
 assert required<=set(F['input_files'])
 for file,h in F['input_files'].items():assert H(Path(file))==h,file
 assert shutil.disk_usage(W.parent).free>=12*1024**3
 W.mkdir(mode=0o700,exist_ok=False);SOURCE.mkdir();STAGE.mkdir()
 # Complete pinned directory blobs plus historical include seeds; no checkout mutation.
 tree={}
 for line in cmd(['git','ls-tree','-r','-l','-z',REV]).split(b'\0'):
  if not line:continue
  meta,name=line.split(b'\t',1);mode,kind,oid,size=meta.decode().split();tree[name.decode()]=(mode,kind,oid,size)
 seeds=json.loads(Path(F['historical_source_manifest']).read_text())['inputs']
 wanted={k for k in tree if k.startswith(('src/','crates/','examples/','benches/','.cargo/')) or '/' not in k and (k.startswith(('Cargo.','rust-toolchain')) or k=='build.rs')}
 wanted.update(seeds);wanted.update(json.loads(Path(F['concat_addendum']).read_text())['files'])
 done=set();dependency_roots={str(PurePosixPath(k).parent) for k in wanted if k.endswith('/Cargo.toml') and not k.startswith('research/')};total=0;unresolved=[];R['phase']='source';checkpoint()
 while wanted-done:
  name=sorted(wanted-done)[0];guard();rel=PurePosixPath(name);assert not rel.is_absolute() and '..' not in rel.parts
  mode,kind,oid,size=tree[name];assert int(size)<=256*1024**2;assert kind=='blob' and mode in ('100644','100755'),name
  raw=cmd(['git','cat-file','blob',oid]);total+=len(raw);assert total<=2*1024**3 and len(done)<100000
  dest=SOURCE/name;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(raw);dest.chmod(0o755 if mode=='100755' else 0o644);row(dest,IMM);done.add(name)
  if name.endswith('Cargo.toml'):
   # Conservative literal path expansion, including dev/build/target path dependencies.
   # A nonliteral dependency path requires explicit disposition instead of omission.
   doc=raw.decode()
   for match in re.finditer(r'\bpath\s*=\s*"([^"\n]+)"',doc):
    root=os.path.normpath(str(PurePosixPath(name).parent/match.group(1)));assert not root.startswith('../') and not root.startswith('/')
    if root in tree:wanted.add(root)
    else:
     entries={k for k in tree if k.startswith(root+'/')};assert entries,'missing path dependency '+root;wanted.update(entries);dependency_roots.add(root)
  # Binary/example files are copied but are not root --lib compilation inputs.
  compiled_source=name.startswith('src/') or name=='build.rs' or any(name.startswith(d+'/src/') or name==d+'/build.rs' for d in dependency_roots)
  if compiled_source and name.endswith('.rs') and '/bin/' not in name and '/examples/' not in name and not name.startswith(('examples/','benches/')):
   text=raw.decode();found=[]
   patterns=[(r'include(?:_str|_bytes)?!\s*\(\s*"([^"\n]+)"\s*,?\s*\)',False),(r'include(?:_str|_bytes)?!\s*\(\s*concat!\s*\(\s*env!\s*\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"\n]+)"\s*,?\s*\)\s*,?\s*\)',True)]
   for pattern,isroot in patterns:
    for m in re.finditer(pattern,text):
     found.append((m.start(),m.end()));v=m.group(1)
     if isroot:
      parent=PurePosixPath(name).parent
      while str(parent)!='.' and str(parent/'Cargo.toml') not in tree:parent=parent.parent
      target=os.path.normpath(str(parent)+v)
     else:target=os.path.normpath(str(PurePosixPath(name).parent/v))
     assert target in tree,'include missing '+target;wanted.add(target)
   for m in re.finditer(r'include(?:_str|_bytes)?!\s*\(',text):
    prefix=text[text.rfind('\n',0,m.start())+1:m.start()]
    if prefix.lstrip().startswith('//'):continue
    if not any(a<=m.start()<b for a,b in found):unresolved.append({'source':name,'offset':m.start(),'line':text.count('\n',0,m.start())+1})
 R['unresolved_includes']=unresolved;checkpoint();assert not unresolved,'unresolved include expressions require source disposition'
 pencil=SOURCE/'src/cryptanalysis/ecbench/pencil.rs';assert H(pencil)=='481f94cc389a1e79880fc168c3a98b5bb37e9411694d424632d4f8f389e5ed00'
 candidate=Path(F['candidate']);assert H(candidate)=='555a5c62339a927579599d46c7e94265e52497dc56bacf0b13bb53c2dcd133c4';pencil.write_bytes(candidate.read_bytes());row(pencil,IMM)
 assert H(SOURCE/'Cargo.lock')=='cf805b650c4e42ce27a9621749abbde2d337266a6856e39aa0b15cf012f444e6'
 assert H(SOURCE/'Cargo.toml')=='f81e15d7ca2cfbba419165b272a15cd736db548f1c90e3454d32fbd3ded373e3'
 lock=(SOURCE/'Cargo.lock').read_text();assert not re.search(r'^source = "git\+',lock,re.M),'unbound git dependency'
 pkgs={}
 for block in lock.split('[[package]]')[1:]:
  fields=dict(re.findall(r'^([a-z_]+) = "([^"\n]+)"$',block,re.M))
  if fields.get('source','').startswith('registry+'):
   key=fields['name']+'-'+fields['version']+'.crate';assert key not in pkgs;pkgs[key]=fields['checksum']
 archives=json.loads(Path(F['registry_archives']).read_text())['archives'];assert {Path(r['path']).name:r['expected'] for r in archives}==pkgs
 vendor=STAGE/'vendor';vendor.mkdir();expanded=0;R['phase']='vendor';checkpoint()
 for r in archives:
  archive=Path(r['path']);assert H(archive)==r['expected'];prefix=archive.name.removesuffix('.crate');root=vendor/prefix;root.mkdir();checks={};seen=set()
  with tarfile.open(archive,'r:gz') as tar:
   for m in tar:
    guard();rel=PurePosixPath(m.name);assert not rel.is_absolute() and '..' not in rel.parts and rel.parts[0]==prefix
    assert m.isdir() or m.isfile();assert m.name not in seen;seen.add(m.name);tail=rel.parts[1:]
    if not tail:assert m.isdir();continue
    assert tail[-1]!='.cargo-checksum.json';dst=root.joinpath(*tail)
    if m.isdir():dst.mkdir(parents=True,exist_ok=True);continue
    expanded+=m.size;assert expanded<=2*1024**3 and R['files']<100000;dst.parent.mkdir(parents=True,exist_ok=True)
    with tar.extractfile(m) as stream,dst.open('xb') as out:
     count=0
     while True:
      guard();b=stream.read(1024**2)
      if not b:break
      count+=len(b);assert count<=m.size;out.write(b)
    assert count==m.size;dst.chmod(0o755 if m.mode&0o111 else 0o644);row(dst,IMM);checks['/'.join(tail)]=IMM[str(dst)]['sha256']
  ck=root/'.cargo-checksum.json';ck.write_text(json.dumps({'files':checks,'package':r['expected']},sort_keys=True)+'\n');row(ck,IMM);assert H(archive)==r['expected']
 cache=Path(F['cache_source']);target=STAGE/'target/release';target.mkdir(parents=True);copied=0;R['phase']='cache';checkpoint()
 for directory in ('deps','build','.fingerprint'):
  assert (cache/directory).is_dir()
  for src in sorted((cache/directory).rglob('*')):
   guard();st=src.lstat()
   if stat.S_ISDIR(st.st_mode):continue
   assert stat.S_ISREG(st.st_mode);copied+=st.st_size;assert copied<=3*1024**3 and R['files']<100000
   dst=target/src.relative_to(cache);dst.parent.mkdir(parents=True,exist_ok=True)
   with os.fdopen(os.open(src,os.O_RDONLY|os.O_NOFOLLOW),'rb') as inp,dst.open('xb') as out:
    before=os.fstat(inp.fileno());count=0
    while True:
     guard();b=inp.read(1024**2)
     if not b:break
     count+=len(b);assert count<=before.st_size;out.write(b)
    after=os.fstat(inp.fileno());assert (before.st_size,before.st_mtime_ns,before.st_ctime_ns)==(after.st_size,after.st_mtime_ns,after.st_ctime_ns) and count==before.st_size
   assert (src.stat().st_dev,src.stat().st_ino)==(before.st_dev,before.st_ino);dst.chmod(stat.S_IMODE(before.st_mode)&0o777);os.utime(dst,ns=(before.st_atime_ns,before.st_mtime_ns));row(dst,CACHE)
 home=STAGE/'cargo-home';home.mkdir();config=home/'config.toml';config.write_text('[source.crates-io]\nreplace-with="vendored-sources"\n[source.vendored-sources]\ndirectory='+json.dumps(str(vendor))+'\n[net]\noffline=true\n');row(config,IMM);(STAGE/'tmp').mkdir()
 R['phase']='tool-and-final-inputs';checkpoint()
 for path,old in json.loads(Path(F['tool_manifest']).read_text())['files'].items():
  f=Path(path);assert H(f)==old['sha256'];IMM[path]={'sha256':old['sha256'],'bytes':f.stat().st_size,'resolved':str(f.resolve())}
 for path,r in {**IMM,**CACHE}.items():assert H(path)==r['sha256'],path
 (OUT/'materialized-inputs.json').write_text(json.dumps({'immutable':IMM,'initial_cache':CACHE},indent=2)+'\n')
 with (OUT/'materialized-inputs.json').open('rb') as stream:os.fsync(stream.fileno())
 R.update(status='passed',source_bytes=total,vendor_bytes=expanded,cache_bytes=copied,input_manifest_sha256=H(OUT/'materialized-inputs.json'));guard()
except BaseException as e:R['status']='failed';R['errors'].append({'repr':repr(e),'traceback':traceback.format_exc()})
finally:
 if ACTIVE is not None:
  if ACTIVE.poll() is None:os.killpg(ACTIVE.pid,signal.SIGKILL)
  ACTIVE.wait();R['status']='failed'
 if time.monotonic()>=END:R['status']='failed'
 checkpoint()
raise SystemExit(0 if R['status']=='passed' else 1)
