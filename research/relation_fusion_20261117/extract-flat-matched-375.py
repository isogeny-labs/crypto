from pathlib import Path
import tarfile,json,hashlib,subprocess,shutil,os
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';a=s/'flat-matched-qualification-373.tar.xz';root=Path('/workspace/flat-matched-extracted-375');root.mkdir()
assert hashlib.sha256(a.read_bytes()).hexdigest()==(s/(a.name+'.sha256')).read_text().split()[0]
with tarfile.open(a,'r:xz') as t:t.extractall(root,filter='data')
idx=json.loads((root/'index.json').read_text());rows=idx['files'];assert sorted(x['path'] for x in rows)==sorted(str(p.relative_to(root)) for p in root.rglob('*') if p.is_file() and p!=root/'index.json')
for r in rows:
 p=root/r['path'];assert p.stat().st_size==r['bytes'] and hashlib.sha256(p.read_bytes()).hexdigest()==r['sha256'],r['path']
for name in ['FLAT_MATCHED_SYNTHETIC_SOURCE_366.json','SUPERVISED_SYNTHETIC_SOURCE_253.json']:
 d=json.loads((root/'study'/name).read_text())
 for p,h in d['source_sha256'].items():assert hashlib.sha256((root/'source'/p).read_bytes()).hexdigest()==h,(name,p)
 if name.startswith('FLAT_'):
  for p,h in d['tool_sha256'].items():assert hashlib.sha256((root/'bin'/Path(p).name).read_bytes()).hexdigest()==h,p
for number,mode,stored in [(367,'neutral','NEUTRAL'),(368,'hypothetical20','POSITIVE')]:
 raw=root/f'{mode}-{number}';out=Path(f'/workspace/flat-matched-extracted-analysis-{number}-375')
 r=subprocess.run(['taskset','-c','0',str(root/'bin/flat-supervised-analyze'),str(raw),str(raw/'monitor.jsonl'),str(raw/'prospective-pins.json'),str(out)],capture_output=True)
 for e,b in [('stdout',r.stdout),('stderr',r.stderr)]:(s/f'development-01/flat-matched-extracted-{number}-375.{e}').write_bytes(b)
 assert r.returncode==0,(number,r.returncode,r.stderr)
 assert (out/'analysis.json').read_bytes()==(root/f'study/FLAT_MATCHED_SYNTHETIC_{stored}_{number}.json').read_bytes()
source=root/'neutral-367';source_before={str(p.relative_to(source)):hashlib.sha256(p.read_bytes()).hexdigest() for p in source.rglob('*') if p.is_file()}
for mode,reason in [('clock',b'complete phase/boundary binding'),('duplicate',b'missing/duplicate cell')]:
 view=Path(f'/workspace/flat-matched-extracted-{mode}-375');shutil.copytree(source,view,copy_function=os.link)
 delta=root/'canary-deltas'/mode;roster=json.loads((delta/'delta-roster.json').read_text())
 for rel in roster['changed_files']:
  p=view/rel;tmp=p.with_name(p.name+'.replacement375');tmp.write_bytes((delta/rel).read_bytes());tmp.replace(p)
 out=Path(f'/workspace/flat-matched-extracted-{mode}-analysis-375')
 r=subprocess.run(['taskset','-c','0',str(root/'bin/flat-supervised-analyze'),str(view),str(view/'monitor.jsonl'),str(view/'prospective-pins.json'),str(out)],capture_output=True)
 for e,b in [('stdout',r.stdout),('stderr',r.stderr)]:(s/f'development-01/flat-matched-extracted-canary-{mode}-375.{e}').write_bytes(b)
 assert r.returncode==1 and reason in r.stderr and not (out/'analysis.json').exists(),(mode,r.returncode,r.stderr)
assert {str(p.relative_to(source)):hashlib.sha256(p.read_bytes()).hexdigest() for p in source.rglob('*') if p.is_file()}==source_before
repack=Path('/workspace/flat-matched-repacked-375.tar.xz')
with tarfile.open(repack,'x:xz',format=tarfile.GNU_FORMAT,preset=6) as t:
 for p in sorted(root.rglob('*')):
  if p.is_file():
   info=t.gettarinfo(str(p),str(p.relative_to(root)));info.mtime=1795392000;info.uid=info.gid=0;info.uname=info.gname='';info.mode=0o755 if p.stat().st_mode&0o111 else 0o644
   if info.islnk():t.addfile(info)
   else:
    with p.open('rb') as f:t.addfile(info,f)
assert repack.read_bytes()==a.read_bytes()
r={'schema':'flat-matched-synthetic-extraction/v1','archive_sha256':hashlib.sha256(a.read_bytes()).hexdigest(),'indexed_files_byte_checked':len(rows),'frozen253_andnew366_source_pins_bound':True,'all366_tool_pins_bound':True,'complete_whole_proof_replays':7200,'both_analytic_reports_byte_equal_367_368':True,'both_reconstructed_canaries_reject_exit1':True,'entire_neutral_source_byte_map_unchanged':len(source_before),'deterministic_repack_byte_equal':True,'performance_observations_collected':0,'signed_execution_verdict':False,'15x_admitted':False};(s/'FLAT_MATCHED_EXTRACTION_375.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r))
