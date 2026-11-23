from pathlib import Path
import tarfile,hashlib,json,subprocess
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';src=s/'flat-qualification-301.tar.xz';root=Path('/workspace/flat-qualification-extracted-302');root.mkdir()
assert hashlib.sha256(src.read_bytes()).hexdigest()==(s/(src.name+'.sha256')).read_text().split()[0]
with tarfile.open(src,'r:xz') as tar:
 for m in tar.getmembers():assert m.isfile() and not Path(m.name).is_absolute() and '..' not in Path(m.name).parts
 tar.extractall(root,filter='data')
idx=json.loads((root/'index.json').read_text());assert idx['performance_comparison_collected'] is False
files=idx['files'];assert sorted(r['path'] for r in files)==sorted(str(p.relative_to(root)) for p in root.rglob('*') if p.is_file() and p!=root/'index.json')
for r in files:
 b=(root/r['path']).read_bytes();assert len(b)==r['bytes'] and hashlib.sha256(b).hexdigest()==r['sha256'],r['path']
for n in ['FLAT_SUITES_273.json','FLAT_QUALIFIER_PINS_274.json','FLAT_SOURCE_CLOSURE_278.json','FLAT_WORKER_SOURCE_280.json','FLAT_ARTIFACT_AUDIT_SOURCE_296.json']:
 d=json.loads((root/'study'/n).read_text())
 for key in ['source_sha256','tool_sha256']:
  for path,digest in d.get(key,{}).items():
   if key=='source_sha256':p=root/'source'/path
   else:p=root/'bin'/Path(path).parent.name/Path(path).name
   assert hashlib.sha256(p.read_bytes()).hexdigest()==digest,(n,path)
report=Path('/workspace/flat-extracted-audit-302.json')
r=subprocess.run(['taskset','-c','0',str(root/'bin/flat-audit-296/flat-artifact-audit'),str(root/'direct-277'),str(root/'worker-281'),str(root/'study/FLAT_QUALIFIER_PINS_274.json'),str(report)],capture_output=True)
for ext,data in [('stdout',r.stdout),('stderr',r.stderr)]:(s/f'development-01/flat-extracted-audit-302.{ext}').write_bytes(data)
assert r.returncode==0,(r.returncode,r.stderr)
assert report.read_bytes()==(s/'FLAT_ARTIFACT_AUDIT_297.json').read_bytes()
for mode,base,msg in [('missing-mutation','direct-277',b'complete mutation roster'),('control-summary','worker-281',b'negative control summary binding')]:
 dst=Path('/workspace')/f'flat-extracted-canary-302-{mode}';subprocess.run(['cp','-al',str(root/base),str(dst)],check=True)
 delta=root/'canary-deltas'/mode;roster=json.loads((delta/'delta-roster.json').read_text())
 for rel in roster['changed_files']:
  p=dst/rel;temp=p.with_name(p.name+'.replace302');temp.write_bytes((delta/rel).read_bytes());temp.replace(p)
 a=dst if base=='direct-277' else root/'direct-277';b=dst if base=='worker-281' else root/'worker-281'
 q=subprocess.run(['taskset','-c','0',str(root/'bin/flat-audit-296/flat-artifact-audit'),str(a),str(b),str(root/'study/FLAT_QUALIFIER_PINS_274.json'),f'/workspace/flat-extracted-canary-302-{mode}.json'],capture_output=True)
 for ext,data in [('stdout',q.stdout),('stderr',q.stderr)]:(s/f'development-01/flat-extracted-canary-302-{mode}.{ext}').write_bytes(data)
 assert q.returncode==1 and msg in q.stderr,(mode,q.returncode,q.stderr)
r={'schema':'flat-qualification-extraction/v1','archive_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'indexed_files_byte_checked':len(files),'source_tool_contexts_bound':[273,274,278,280,296],'full_saved_replays':180,'replayed_report_byte_equal_297':True,'both_reconstructed_canaries_reject_exit1':True,'performance_comparison_collected':False,'native_matrix_executed_cases':0,'signed_execution_verdict':False,'15x_admitted':False}
(s/'FLAT_EXTRACTION_302.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r))
