from pathlib import Path
import shutil,hashlib,json,tarfile,subprocess
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';pkg=Path('/workspace/root-flat-package-337');pkg.mkdir()
def copy(src,dst):
 src=Path(src);p=pkg/dst;p.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(src,p)
def tree(src,dst):shutil.copytree(Path('/workspace')/src,pkg/dst)
for a,b in [('flat-root-matrix-322','native-322'),('root-cache-controls-329','failed-cache-329'),('root-cache-controls-330','cache-330')]:tree(a,b)
for d in ['root-tail-recovery-307','root-trace-union-310','root-quartic-union-312','root-semaev-union-314','root-trace-proposal-315','root-name-recovery-318','root-api-recovery-321','root-algebra-before-326']:tree(d,Path('contexts')/d)
for pattern in ['recover-root-tails-308*.py','recover-quartic-312.py','recover-semaev-314.py','prepare-trace-union-315*.py','apply-trace-315.py','root-genus-*-309.rs','cache-controls-source*-failed.rs','f4-solvers-da11-321.rs','sparse-f66-334.rs']:
 for p in Path('/workspace').glob(pattern):copy(p,Path('contexts')/p.name)
# Every declared runtime pin322 is present as complete source, not an inferred checkout.
for p,h in json.loads((s/'FLAT_ROOT_SOURCE_322.json').read_text())['source_sha256'].items():
 assert hashlib.sha256((repo/p).read_bytes()).hexdigest()==h,p
 copy(repo/p,Path('source')/p)
for d in ['src','root-algebra-tests']:
 for p in (s/d).rglob('*'):
  if p.is_file() and not any(x in p.parts for x in ['target','.git']):copy(p,Path('source/research/relation_fusion_20261117')/p.relative_to(s))
for n in ['Cargo.toml','Cargo.lock','MATCHED_PINS_BEFORE_85.json','ROOT_FLAT_REPORT_334.md','root-flat-report-334.typ','root-flat-report-334.pdf','ROOT_FLAT_REPLAY_336.md']:
 copy(s/n,Path('study')/n)
for p in (s/'visual-334').glob('*'):copy(p,Path('study/visual-334')/p.name)
for p in s.glob('*.json'):
 import re
 m=re.search(r'_(\d+)\.json$',p.name)
 if m and 306<=int(m[1])<=334:copy(p,Path('study')/p.name)
for p in (s/'development-01').glob('*'):
 import re
 m=re.search(r'-(\d+)\.',p.name)
 if m and 308<=int(m[1])<=333:copy(p,Path('logs')/p.name)
for prime in [2147483647,9007199254740881]:
 for n in [3,4,6]:
  rel=Path(f'p{prime}-n{n}-frontier/original.json');copy(Path('/workspace/fusion-qualification-66')/rel,Path('references-85')/rel)
copy('/workspace/relation-native-qualifiers/flat-root-audit-331/flat-root-audit',Path('bin/flat-root-audit'))
# Post-run freeze of correctness executable; never represented as a prospective performance pin.
cache=Path('/workspace/crypto-root-unoptimized-181/release/build/relation-root-algebra-tests/fb79b62f0fa34915/out/cache_controls-fb79b62f0fa34915')
copy(cache,Path('bin/cache-controls-330'))
for p in [pkg/'bin/flat-root-audit',pkg/'bin/cache-controls-330']:p.chmod(0o555)
(pkg/'post-run-tools.json').write_text(json.dumps({'purpose':'Saved native proof audit and exact correctness-only test reexecution; no timing admission','post_run_cache_executable_freeze':True,'signed_execution_attestation':False,'tools':{str(p.relative_to(pkg)):hashlib.sha256(p.read_bytes()).hexdigest() for p in (pkg/'bin').iterdir()}},indent=2)+'\n')
copy(__file__,Path('package.py'))
files=[{'path':str(p.relative_to(pkg)),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(pkg.rglob('*')) if p.is_file()]
(pkg/'index.json').write_text(json.dumps({'schema':'root-flat-recovery-archive/v1','signed_execution_verdict':False,'comparative_timing':False,'files':files},indent=2)+'\n')
archive=s/'root-flat-recovery-337.tar.xz'
with tarfile.open(archive,'x:xz',format=tarfile.GNU_FORMAT,preset=6) as tar:
 for p in sorted(pkg.rglob('*')):
  if p.is_file():
   info=tar.gettarinfo(str(p),str(p.relative_to(pkg)));info.mtime=1795392000;info.uid=info.gid=0;info.uname=info.gname='';info.mode=0o755 if p.stat().st_mode&0o111 else 0o644
   with p.open('rb') as f:tar.addfile(info,f)
digest=hashlib.sha256(archive.read_bytes()).hexdigest();(s/(archive.name+'.sha256')).write_text(f'{digest}  {archive.name}\n')
r={'schema':'root-flat-recovery-archive-receipt/v1','archive':archive.name,'sha256':digest,'bytes':archive.stat().st_size,'indexed_files':len(files),'uncompressed_bytes':sum(x['bytes'] for x in files),'complete_native_cases':36,'complete_cache_calls':98,'failed_cache329_calls':96,'unit332_compile_errors':18,'native322_source_pins':1452,'signed_execution_verdict':False,'performance_comparison':False,'15x_admitted':False,'policy':'UserLocalCPU0, one numerical thread, zero cloud, persistent checkpoint/S3 waiver. Deterministic GNU tar, xz single-thread preset6.'}
(s/'ROOT_FLAT_ARCHIVE_337.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r))
