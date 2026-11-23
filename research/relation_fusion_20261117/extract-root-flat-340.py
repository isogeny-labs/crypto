from pathlib import Path
import hashlib,json,tarfile,subprocess,os
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';archive=Path('/workspace/root-flat-recovery-337.tar.xz');root=Path('/workspace/root-flat-extracted-339');assert root.is_dir() # Continue after full safe extraction and2087 byte checks; no prior math execution
expected=(s/(archive.name+'.sha256')).read_text().split()[0];assert hashlib.sha256(archive.read_bytes()).hexdigest()==expected
# Preserve original extraction, adapter-only continuation340.
files=json.loads((root/'index.json').read_text())['files'];assert sorted(x['path'] for x in files)==sorted(str(p.relative_to(root)) for p in root.rglob('*') if p.is_file() and p!=root/'index.json')
for x in files:
 p=root/x['path'];assert p.stat().st_size==x['bytes'] and hashlib.sha256(p.read_bytes()).hexdigest()==x['sha256'],x['path']
for n in ['FLAT_ROOT_SOURCE_322.json','ROOT_CACHE_SOURCE_330.json','FLAT_ROOT_AUDIT_SOURCE_331.json']:
 for p,h in json.loads((root/'study'/n).read_text())['source_sha256'].items():
  q=root/'source'/p
  if not q.is_file() and p in ['research/relation_fusion_20261117/Cargo.toml','research/relation_fusion_20261117/Cargo.lock']:q=root/'study'/Path(p).name
  assert hashlib.sha256(q.read_bytes()).hexdigest()==h,(n,p)
audit=Path('/workspace/flat-root-extracted-audit-340.json')
r=subprocess.run(['taskset','-c','0',str(root/'bin/flat-root-audit'),str(root/'native-322'),str(root/'references-85'),str(root/'study/MATCHED_PINS_BEFORE_85.json'),str(audit)],capture_output=True)
for e,b in [('stdout',r.stdout),('stderr',r.stderr)]:(s/f'development-01/flat-root-extracted-audit-340.{e}').write_bytes(b)
assert r.returncode==0,(r.returncode,r.stderr)
assert audit.read_bytes()==(root/'study/FLAT_ROOT_AUDIT_331.json').read_bytes()
out=Path('/workspace/root-cache-extracted-340')
env=dict(os.environ,ROOT_CACHE_QUALIFICATION_OUT=str(out),RAYON_NUM_THREADS='1',OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',RUST_TEST_THREADS='1')
r=subprocess.run(['taskset','-c','0',str(root/'bin/cache-controls-330'),'--test-threads=1'],env=env,capture_output=True)
for e,b in [('stdout',r.stdout),('stderr',r.stderr)]:(s/f'development-01/root-cache-extracted-340.{e}').write_bytes(b)
assert r.returncode==0,(r.returncode,r.stderr)
def without_clocks(x):
 if isinstance(x,dict):return {k:without_clocks(v) for k,v in x.items() if not k.endswith('_ns')}
 if isinstance(x,list):return [without_clocks(v) for v in x]
 return x
n=0
for p in sorted((root/'cache-330').glob('*.json')):
 a=json.loads(p.read_text());b=json.loads((out/p.name).read_text());assert without_clocks(a)==without_clocks(b),p.name;n+=1
assert n==99
r={'schema':'root-flat-extracted-replay/v1','archive_sha256':expected,'indexed_files_byte_checked':len(files),'all322_runtime_source_pins_bound':1452,'cache330_and_checker331_source_pins_bound':True,'saved_complete_proofs_replayed':36,'saved_report_byte_equal_331':True,'semantic_canaries_reject':2,'actual_cache_calls_reexecuted':98,'all_99_cache_files_equal_except_diagnostic_ns_clocks':True,'post_run_frozen_cache_binary':True,'native_operation_counts_independently_reexecuted':False,'signed_execution_verdict':False,'comparative_timing':False,'15x_admitted':False}
(s/'ROOT_FLAT_EXTRACTION_340.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r))
