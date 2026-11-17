"""Prospective charged source/cache staging. No compiler or test is invoked here."""
from pathlib import Path
import hashlib,json,os,stat,time,datetime,resource
P=Path(__file__).resolve().parent
A=json.loads((P/'admission.json').read_text())
assert A['released'] is True and A['prior_v4_terminal_confirmed'] is True
assert A['build_phase_seconds']==1800 and A['total_seconds']==2400
assert A['automatic_retry'] is False and A['attempts']==1
END=float(os.environ['SIGNED_BUILD_END_MONOTONIC'])
assert time.monotonic()<END and END-time.monotonic()<=1800
ABS=datetime.datetime.fromisoformat(A['absolute_deadline'].replace('Z','+00:00')).timestamp()
W=Path(A['workspace']);DONOR=Path(A['donor_workspace']);SOURCE=W/'source';CACHE=W/'cache'
assert str(W)=='/var/tmp/pencil-signed-kummer-combined-v5-20261111'
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
def copy(src,dst,row):
 checked(src,row);dst.parent.mkdir(parents=True,exist_ok=True)
 with Path(src).open('rb') as f,dst.open('xb') as g:
  for b in iter(lambda:f.read(1024**2),b''):guard();g.write(b)
  g.flush();os.fsync(g.fileno())
 checked(dst,row);checked(src,row)
def bound(key):
 b=A[key];assert digest(b['path'])==b['sha256'],key
 return json.loads(Path(b['path']).read_text())
for row in A['release_receipts'].values():assert digest(row['path'])==row['sha256']
for name,h in A['packet_files'].items():assert digest(P/name)==h,name
M=bound('immutable_manifest');C=bound('fresh_cache_manifest')
assert C['schema']=='root-current-cache/v1'
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
assert sum(row['bytes'] for row in C['files'].values())+187457976<8*1024**3,'private workspace8GiB ceiling'
SOURCE.mkdir();CACHE.mkdir()
for path,row in M['immutable'].items():
 f=Path(path)
 if f.is_relative_to(DONOR/'source'):copy(f,SOURCE/f.relative_to(DONOR/'source'),row)
# Private regular copies: never share mutable cache hardlinks with the failed/current donor.
for directory in C['directories']:
 d=Path(directory);assert d.is_relative_to(DONOR/'cache-stage-v2');(CACHE/d.relative_to(DONOR/'cache-stage-v2')).mkdir(parents=True,exist_ok=True)
for path,row in C['files'].items():
 f=Path(path);assert f.is_relative_to(DONOR/'cache-stage-v2');copy(f,CACHE/f.relative_to(DONOR/'cache-stage-v2'),row)
for row in json.loads((P/'overlays.json').read_text()):
 dst=SOURCE/row['path'];src=P/row['overlay_path'];expected={'sha256':row['post_sha256'],'bytes':row['post_bytes']};checked(src,expected)
 if row['in_donor_source']:
  assert digest(dst)==row['pre_sha256'];dst.unlink()
 else:assert not dst.exists()
 copy(src,dst,expected)
expected=json.loads((P/'expected-source-manifest.json').read_text());actual=set()
def traversal_error(e):raise e
for base,dirs,names in os.walk(SOURCE,followlinks=False,onerror=traversal_error):
 for n in dirs:assert stat.S_ISDIR((Path(base)/n).lstat().st_mode)
 for n in names:
  f=Path(base)/n;name=str(f.relative_to(SOURCE));assert name in expected;checked(f,expected[name]);actual.add(name)
assert actual==set(expected)
# Cargo config retains exact immutable donor vendor path and offline mode.
config=CACHE/'cargo-home/config.toml';assert digest(config)==M['immutable'][str(DONOR/'cache-stage-v2/cargo-home/config.toml')]['sha256']
receipt={'status':'passed','source_files':len(actual),'source_manifest_sha256':digest(P/'expected-source-manifest.json'),'fresh_cache_manifest_sha256':A['fresh_cache_manifest']['sha256'],'workspace':str(W),'build_end_monotonic':END,'absolute_deadline':A['absolute_deadline'],'compiler_started':False,'donor_modified':False}
target=W/'materialization.json'
with target.open('x') as f:json.dump(receipt,f,indent=2);f.flush();os.fsync(f.fileno())
fd=os.open(W,os.O_DIRECTORY);os.fsync(fd);os.close(fd)
print(json.dumps(receipt))
