from pathlib import Path
import json,hashlib,shutil,tarfile,re,os,datetime
r=Path('/workspace/relation-supervisor-live');s=r/'research/relation_fusion_20261117';pkg=Path('/workspace/root-fixture-package-392');pkg.mkdir()
sha=lambda p:hashlib.file_digest(p.open('rb'),'sha256').hexdigest()
# All copies are immutable hardlinks for space; no source file is changed afterward.
def copy(p,n):
 p=Path(p);q=pkg/n;q.parent.mkdir(parents=True,exist_ok=True);os.link(p,q)
for p in s.iterdir():
 m=re.search(r'_(\d+)\.',p.name)
 if p.is_file() and m and 378<=int(m[1])<=395:copy(p,Path('study')/p.name)
for p in (s/'development-01').iterdir():
 m=re.search(r'-(\d+)\.',p.name)
 if p.is_file() and m and 380<=int(m[1])<=390:copy(p,Path('logs')/p.name)
for p in (s/'root-fixture-repair').rglob('*'):
 if p.is_file():copy(p,Path('source/research/relation_fusion_20261117/root-fixture-repair')/p.relative_to(s/'root-fixture-repair'))
for n in ['src/cryptanalysis/binary_radical_root.rs','src/cryptanalysis/curve_id.rs','src/cryptanalysis/curve_aliases.json','Cargo.toml','Cargo.lock']:
 copy(r/n,Path('source')/n)
for n in ['root-fixture-report-391.typ','root-fixture-report-391.pdf']:copy(s/n,Path('study')/n)
for p in (s/'visual-391').iterdir():copy(p,Path('study/visual-391')/p.name)
for n in ['reclaim-extraction-379.py','run-root-full-380.py','replay-root-failures-384.py','prepare-root-repair-385.py','prepare-geometric-controls-386.py','integrate-root-fixtures-387.py','run-root-modules-388.py','format-root-fixture-389.py','run-root-modules-390.py','render-root-fixture-391.py','polish-root-report-391.py']:
 copy(Path('/workspace')/n,Path('scripts')/n)
copy('/workspace/root-fixture-modules-389',Path('bin/root-fixture-modules'))
# Prototype executable deliberately omitted; source plus originalAPI/alias outputs retained.
for n in ['ROOT_UNIT_SOURCE_348.json','ROOT_RECOVERED_UNIT_CONTEXT_350.json','FLAT_MATCHED_REPORT_372.md']:
 copy(s/n,Path('references')/n)
copy(__file__,Path('package.py'))
rows=[{'path':str(p.relative_to(pkg)),'bytes':p.stat().st_size,'sha256':sha(p)} for p in sorted(pkg.rglob('*')) if p.is_file()]
index={'schema':'root-fixture-diagnostic-archive/v1','base_parent':'41f837ac6bb345f8efab9c7c41f66d45d515dc0f','scope':'Failed fullroot380/384 diagnostics, source fixtures and final7-module replay. Whole-root350ELF and prototype386ELF not embedded; their source/context/raw outputs retained.','signed_execution_verdict':False,'performance_comparison':False,'files':rows};(pkg/'index.json').write_text(json.dumps(index,indent=2)+'\n')
archive=s/'root-fixture-qualification-392.tar.xz'
with tarfile.open(archive,'x:xz',format=tarfile.GNU_FORMAT,preset=6) as t:
 for p in sorted(pkg.rglob('*')):
  if p.is_file():
   info=t.gettarinfo(str(p),str(p.relative_to(pkg)));info.type=tarfile.REGTYPE;info.linkname='';info.mtime=1795392000;info.uid=info.gid=0;info.uname=info.gname='';info.mode=0o755 if p.stat().st_mode&0o111 else 0o644
   with p.open('rb') as f:t.addfile(info,f)
h=sha(archive);(s/(archive.name+'.sha256')).write_text(f'{h}  {archive.name}\n');out={'schema':'root-fixture-diagnostic-archive-receipt/v1','archive':archive.name,'sha256':h,'bytes':archive.stat().st_size,'indexed_files':len(rows),'uncompressed_bytes':sum(v['bytes'] for v in rows),'source_contexts':[385,386,387,389],'final_module_tests':7,'full_root_suite_qualified':False,'signed_execution_verdict':False,'performance_comparison':False,'15x_admitted':False};(s/'ROOT_FIXTURE_ARCHIVE_392.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out))
