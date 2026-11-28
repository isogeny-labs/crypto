from pathlib import Path
import tarfile,json,hashlib,subprocess
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';a=s/'root-unit-qualification-360.tar.xz';root=Path('/workspace/root-unit-extracted-361');root.mkdir()
assert hashlib.sha256(a.read_bytes()).hexdigest()==(s/(a.name+'.sha256')).read_text().split()[0]
with tarfile.open(a,'r:xz') as t:t.extractall(root,filter='data')
rows=json.loads((root/'index.json').read_text())['files'];assert sorted(x['path'] for x in rows)==sorted(str(p.relative_to(root)) for p in root.rglob('*') if p.is_file() and p!=root/'index.json')
for r in rows:
 p=root/r['path'];assert p.stat().st_size==r['bytes'] and hashlib.sha256(p.read_bytes()).hexdigest()==r['sha256'],r['path']
base=Path('/workspace/root-flat-extracted-339/source')
for context in ['ROOT_UNIT_SOURCE_345.json','ROOT_UNIT_SOURCE_348.json']:
 for p,h in json.loads((root/'study'/context).read_text())['source_sha256'].items():
  q=root/'source-delta'/p
  if context.endswith('345.json') and p=='src/cryptanalysis/ic_framework/pkm_semaev.rs':q=root/'contexts/pkm-before-import-348.rs'
  elif not q.is_file():q=base/p
  assert hashlib.sha256(q.read_bytes()).hexdigest()==h,(context,p)
for p,h in json.loads((root/'study/ROOT_EXAMPLE_CONTEXT_357.json').read_text())['source_sha256'].items():assert hashlib.sha256((root/'source-delta'/p).read_bytes()).hexdigest()==h,p
ctx=json.loads((root/'study/FLAT_ROOT_AUDIT_SOURCE_331.json').read_text());assert hashlib.sha256((root/'bin/flat-root-audit').read_bytes()).hexdigest()==ctx['binary_sha256']
out=Path('/workspace/root-unit-proof-replay-361.json');r=subprocess.run(['taskset','-c','0',str(root/'bin/flat-root-audit'),str(root/'native-352'),str(root/'references-85'),str(root/'study/MATCHED_PINS_BEFORE_85.json'),str(out)],capture_output=True)
for e,b in [('stdout',r.stdout),('stderr',r.stderr)]:(s/f'development-01/root-unit-proof-replay-361.{e}').write_bytes(b)
assert r.returncode==0,(r.returncode,r.stderr);assert out.read_bytes()==(root/'study/FLAT_ROOT_AUDIT_331.json').read_bytes()
repack=Path('/workspace/root-unit-repacked-361.tar.xz')
with tarfile.open(repack,'x:xz',format=tarfile.GNU_FORMAT,preset=6) as t:
 for p in sorted(root.rglob('*')):
  if p.is_file():
   info=t.gettarinfo(str(p),str(p.relative_to(root)));info.mtime=1795392000;info.uid=info.gid=0;info.uname=info.gname='';info.mode=0o755 if p.stat().st_mode&0o111 else 0o644
   with p.open('rb') as f:t.addfile(info,f)
assert repack.read_bytes()==a.read_bytes()
r={'schema':'root-unit-qualified-extraction/v1','archive_sha256':hashlib.sha256(a.read_bytes()).hexdigest(),'indexed_files_byte_checked':len(rows),'all_source345_and348_pins_reconstructed_bound':1452,'example357_source_and_lock_pins_bound':True,'saved_native_proofs_replayed':36,'hash_consistent_semantic_canaries_reject':2,'replay_report_byte_equal_331':True,'deterministic_repack_byte_equal':True,'actual_unit_tests_reexecuted_by_extraction':0,'unit_rebuild_required':'Trusted parent95f6003d16/dependencies/submodules plus source delta and exact source/build recipes.557MiB executable not redistributed.','native_operation_counter_attestation':False,'signed_execution_verdict':False,'performance_comparison':False,'15x_admitted':False};(s/'ROOT_UNIT_EXTRACTION_361.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r))
