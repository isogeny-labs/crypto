from pathlib import Path
import json,hashlib,shutil,tarfile,re,subprocess
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';pkg=Path('/workspace/root-unit-package-360');pkg.mkdir()
def copy(p,n):
 p=Path(p);q=pkg/n;q.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(p,q)
# Seven-file algorithm/API delta, plus complete harness contexts.
files=list(json.loads((s/'ROOT_UNIT_RECOVERY_344.json').read_text())['files'])
for n in files:copy(repo/n,Path('source-delta')/n);copy(Path('/workspace/root-unit-before-344')/n,Path('contexts/before-344')/n)
for d in ['root-integration','root-example-tests']:
 for p in (s/d).rglob('*'):
  if p.is_file() and 'target' not in p.parts:copy(p,Path('source-delta/research/relation_fusion_20261117')/p.relative_to(s))
copy(repo/'examples/prime_pair_table_check.rs',Path('source-delta/examples/prime_pair_table_check.rs'))
for n in ['pkm-before-import-348.rs','repair-root-unit-344-initial.py','repair-root-unit-344.py','run-root-recovered-350.py','run-native-root-352.py','run-root-api-354-initial.py','run-root-api-356.py','run-root-example-357.py','render-root-unit-358.py']:
 copy(Path('/workspace')/n,Path('contexts')/n)
for p in s.iterdir():
 m=re.search(r'_(\d+)\.',p.name)
 if p.is_file() and m and 343<=int(m[1])<=359:copy(p,Path('study')/p.name)
for n in ['root-unit-recovery-348.patch','root-unit-report-358.typ','root-unit-report-358.pdf','FLAT_ROOT_SOURCE_322.json','FLAT_ROOT_AUDIT_331.json','FLAT_ROOT_AUDIT_SOURCE_331.json','MATCHED_PINS_BEFORE_85.json']:
 copy(s/n,Path('study')/n)
for p in (s/'visual-358').iterdir():copy(p,Path('study/visual-358')/p.name)
for p in (s/'development-01').iterdir():
 m=re.search(r'-(\d+)\.',p.name)
 if p.is_file() and m and 345<=int(m[1])<=357:copy(p,Path('logs')/p.name)
shutil.copytree('/workspace/flat-root-matrix-352',pkg/'native-352')
base=Path('/workspace/root-flat-extracted-339');shutil.copytree(base/'references-85',pkg/'references-85');copy(base/'bin/flat-root-audit',Path('bin/flat-root-audit'));(pkg/'bin/flat-root-audit').chmod(0o555)
copy(__file__,Path('package.py'))
rows=[{'path':str(p.relative_to(pkg)),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(pkg.rglob('*')) if p.is_file()]
(pkg/'index.json').write_text(json.dumps({'schema':'root-unit-qualification-archive/v1','base_parent':'95f6003d167c91f6e40e82980f00a67b5adf75b2','base_archive337_sha256':'0be6533b7ebb1c68bfecc855b718c940537d350b2e0198b3ecab32f371e6352a','signed_execution_verdict':False,'performance_comparison':False,'files':rows},indent=2)+'\n')
archive=s/'root-unit-qualification-360.tar.xz'
with tarfile.open(archive,'x:xz',format=tarfile.GNU_FORMAT,preset=6) as t:
 for p in sorted(pkg.rglob('*')):
  if p.is_file():
   info=t.gettarinfo(str(p),str(p.relative_to(pkg)));info.mtime=1795392000;info.uid=info.gid=0;info.uname=info.gname='';info.mode=0o755 if p.stat().st_mode&0o111 else 0o644
   with p.open('rb') as f:t.addfile(info,f)
h=hashlib.sha256(archive.read_bytes()).hexdigest();(s/(archive.name+'.sha256')).write_text(f'{h}  {archive.name}\n');out={'schema':'root-unit-qualification-archive-receipt/v1','archive':archive.name,'sha256':h,'bytes':archive.stat().st_size,'indexed_files':len(rows),'uncompressed_bytes':sum(r['bytes'] for r in rows),'source_delta_files':len(files),'recovered_tests_passed':338,'api_tests_passed':16,'native_cases':36,'example_calls':2,'signed_execution_verdict':False,'performance_comparison':False,'15x_admitted':False};(s/'ROOT_UNIT_ARCHIVE_360.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out))
