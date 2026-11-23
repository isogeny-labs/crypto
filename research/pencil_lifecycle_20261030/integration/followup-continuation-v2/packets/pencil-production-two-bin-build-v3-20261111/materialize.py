"""Prospective charged source/cache staging. No compiler or test is invoked here."""
from pathlib import Path
import hashlib,json,os,stat,time,datetime,resource,tarfile
P=Path(__file__).resolve().parent
A=json.loads((P/'admission.json').read_text())
assert A['released'] is True and A['prior_v4_terminal_confirmed'] is True
assert A['build_phase_seconds']==1800 and A['total_seconds']==2400
assert A['automatic_retry'] is False and A['attempts']==1
END=float(os.environ['SIGNED_BUILD_END_MONOTONIC'])
assert time.monotonic()<END and END-time.monotonic()<=1800
ABS=datetime.datetime.fromisoformat(A['absolute_deadline'].replace('Z','+00:00')).timestamp()
W=Path(A['workspace']);DONOR=Path(A['donor_workspace']);SOURCE=W/'source';CACHE=W/'cache'
assert str(W)=='/var/tmp/pencil-pencil-production-two-bin-build-v3-20261111'
assert str(DONOR)=='/var/tmp/pencil-root-materialize-t1953-20261109'
assert W.is_dir() and not W.is_symlink()
assert {x.name for x in W.iterdir()}=={'workspace-owner.json'},'existing staged attempt; do not overwrite'
assert json.loads((W/'workspace-owner.json').read_text())=={'unit':A['unit'],'lease':A['lease']}
resource.setrlimit(resource.RLIMIT_AS,(16*1024**3,16*1024**3))
def guard():
 assert time.monotonic()<END-20 and time.time()<ABS,'charged build deadline'
 assert os.statvfs(W.parent).f_bavail*os.statvfs(W.parent).f_frsize>=6*1024**3,'free disk below6GiB'
def digest(path):
 h=hashlib.sha256()
 with Path(path).open('rb') as f:
  for b in iter(lambda:f.read(1024**2),b''):guard();h.update(b)
 return h.hexdigest()
def checked(path,row,allow_resolved=False):
 path=Path(path);st=path.lstat()
 if allow_resolved and 'resolved' in row:assert str(path.resolve())==row['resolved']
 else:assert stat.S_ISREG(st.st_mode),str(path)
 assert path.stat().st_size==row['bytes'] and digest(path)==row['sha256'],str(path)
 if 'mode' in row:assert stat.S_IMODE(st.st_mode)==row['mode'],str(path)+' mode differs from frozen inventory'
def copy(src,dst,row):
 checked(src,row);dst.parent.mkdir(parents=True,exist_ok=True)
 with Path(src).open('rb') as f,dst.open('xb') as g:
  if 'mode' in row:assert stat.S_IMODE(os.fstat(f.fileno()).st_mode)==row['mode'],'source fd mode changed'
  for b in iter(lambda:f.read(1024**2),b''):guard();g.write(b)
  if 'mode' in row:
   assert type(row['mode']) is int and 0<=row['mode']<=0o777,'special mode bits disallowed'
   os.fchmod(g.fileno(),row['mode'])
   assert stat.S_IMODE(os.fstat(g.fileno()).st_mode)==row['mode'],'destination fd mode differs'
   assert stat.S_IMODE(os.fstat(f.fileno()).st_mode)==row['mode'],'source fd mode changed during copy'
  g.flush();os.fsync(g.fileno())
 checked(dst,row);checked(src,row)
def bound(key):
 b=A[key];assert digest(b['path'])==b['sha256'],key
 return json.loads(Path(b['path']).read_text())
for row in A['release_receipts'].values():assert digest(row['path'])==row['sha256']
for name,h in A['packet_files'].items():assert digest(P/name)==h,name
M=bound('immutable_manifest');C=bound('fresh_cache_manifest')
assert C['schema']=='root-current-cache/v1'
mode_file=P/'cache-mode-inventory.json'
assert digest(mode_file)=='c432da2dff731eaf5cd58593ef3267500c64bc13e53f25e57f22184b990bf241'
mode_inventory=json.loads(mode_file.read_text())
assert mode_inventory['schema']=='pencil.cache-mode-inventory/v1'
assert mode_inventory['cache_manifest']['sha256']==A['fresh_cache_manifest']['sha256']
assert mode_inventory['donor_roots']==C['roots']
assert mode_inventory['path_order']=='lexicographic_unicode_codepoint_absolute_paths'
assert mode_inventory['file_types_allowed']==['regular'] and mode_inventory['special_permission_bits_allowed'] is False
paths=sorted(C['files']);indices=mode_inventory['mode_index_nibbles'];table=mode_inventory['mode_table']
assert len(paths)==len(indices)==mode_inventory['file_count']==2165
assert table==['0600','0644','0700','0755'] and all(c in '0123' for c in indices)
cache_modes={path:int(table[int(code,16)],8) for path,code in zip(paths,indices)}
assert {mode:sum(cache_modes[path]==int(mode,8) for path in paths) for mode in table}==mode_inventory['mode_counts']
assert [i for i,path in enumerate(paths) if cache_modes[path]&0o111]==mode_inventory['executable_file_indices']
assert sum(bool(mode&0o111) for mode in cache_modes.values())==mode_inventory['executable_file_count']==82

assert C['roots']==[str(DONOR/'cache-stage-v2'/n) for n in ['target','cargo-home','tmp']]
assert A['immutable_manifest']['sha256']=='c5178d2138d79f1ea8cac1fc96cb8d968913ba403d68ee66fa36afdd60fdf3f2'
# A complete post-v4 snapshot must still match exact cache membership.
actual_files=set();actual_dirs=[]
def traversal_error(error):raise error
for root in C['roots']:
 assert stat.S_ISDIR(Path(root).lstat().st_mode)
 for base,dirs,names in os.walk(root,followlinks=False,onerror=traversal_error):
  guard();actual_dirs.append(base)
  for name in dirs:assert stat.S_ISDIR((Path(base)/name).lstat().st_mode)
  for name in names:actual_files.add(str(Path(base)/name))
assert sorted(actual_dirs)==C['directories'] and actual_files==set(C['files']),'fresh cache inventory changed'
# Verify immutable donor/vendor/toolchain before cloning; vendor stays read-only.
for path,row in M['immutable'].items():checked(path,row,True)
assert sum(row['bytes'] for row in C['files'].values())+sum(row['bytes'] for row in json.loads((P/'expected-source-manifest.json').read_text()).values())<8*1024**3,'private workspace8GiB ceiling'
SOURCE.mkdir();CACHE.mkdir()
for path,row in M['immutable'].items():
 f=Path(path)
 if f.is_relative_to(DONOR/'source'):copy(f,SOURCE/f.relative_to(DONOR/'source'),row)
# Private regular copies: never share mutable cache hardlinks with the failed/current donor.
for directory in C['directories']:
 d=Path(directory);assert d.is_relative_to(DONOR/'cache-stage-v2');(CACHE/d.relative_to(DONOR/'cache-stage-v2')).mkdir(parents=True,exist_ok=True)
for path,row in C['files'].items():
 f=Path(path);assert f.is_relative_to(DONOR/'cache-stage-v2');copy(f,CACHE/f.relative_to(DONOR/'cache-stage-v2'),dict(row,mode=cache_modes[path]))
archive=json.loads((P/'overlay-archive.json').read_text());checked(P/archive['path'],archive)
rows=json.loads((P/'overlays.json').read_text());mapped={row['overlay_path']:row for row in rows};seen=set()
with tarfile.open(P/archive['path'],'r:gz') as tar:
 for member in tar:
  guard();assert member.isfile() and member.name in mapped and member.name not in seen
  row=mapped[member.name];assert member.size==row['post_bytes'];seen.add(member.name)
  dst=SOURCE/row['path'];assert not Path(row['path']).is_absolute() and '..' not in Path(row['path']).parts
  if row['in_donor_source']:assert digest(dst)==row['pre_sha256'];dst.unlink()
  else:assert not dst.exists()
  dst.parent.mkdir(parents=True,exist_ok=True);stream=tar.extractfile(member)
  with dst.open('xb') as target:
   for chunk in iter(lambda:stream.read(1024**2),b''):guard();target.write(chunk)
   target.flush();os.fsync(target.fileno())
  checked(dst,{'sha256':row['post_sha256'],'bytes':row['post_bytes']})
assert seen==set(mapped)
expected=json.loads((P/'expected-source-manifest.json').read_text());actual=set()
def traversal_error(e):raise e
for base,dirs,names in os.walk(SOURCE,followlinks=False,onerror=traversal_error):
 for n in dirs:assert stat.S_ISDIR((Path(base)/n).lstat().st_mode)
 for n in names:
  f=Path(base)/n;name=str(f.relative_to(SOURCE));assert name in expected;checked(f,expected[name]);actual.add(name)
assert actual==set(expected)
# Cargo config retains exact immutable donor vendor path and offline mode.
config=CACHE/'cargo-home/config.toml';assert digest(config)==M['immutable'][str(DONOR/'cache-stage-v2/cargo-home/config.toml')]['sha256']
# Reconcile every copied cache mode again at the completed staging boundary.
for path,mode in cache_modes.items():
 guard();source=Path(path);target=CACHE/source.relative_to(DONOR/'cache-stage-v2')
 for file in [source,target]:
  info=file.lstat();assert stat.S_ISREG(info.st_mode) and stat.S_IMODE(info.st_mode)==mode,str(file)+' final mode mismatch'
receipt={'cache_mode_inventory_sha256':digest(mode_file),'cache_modes_verified':len(cache_modes),'cache_executable_modes_verified':82,'status':'passed','source_files':len(actual),'source_manifest_sha256':digest(P/'expected-source-manifest.json'),'fresh_cache_manifest_sha256':A['fresh_cache_manifest']['sha256'],'workspace':str(W),'build_end_monotonic':END,'absolute_deadline':A['absolute_deadline'],'compiler_started':False,'donor_modified':False}
target=W/'materialization.json'
with target.open('x') as f:json.dump(receipt,f,indent=2);f.flush();os.fsync(f.fileno())
fd=os.open(W,os.O_DIRECTORY);os.fsync(fd);os.close(fd)
print(json.dumps(receipt))
