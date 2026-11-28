from pathlib import Path
import json,hashlib,shutil,subprocess
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';out=Path('/workspace/relation-native-qualifiers/flat-supervised-366');out.mkdir()
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
for n in ['compare','analyze','fixture']:
 p=out/f'flat-supervised-{n}';shutil.copy2(Path('/workspace/supervised-study-target-230/release')/p.name,p);p.chmod(0o555)
files=[s/'src/flat_supervised_roster.rs',s/'src/packed_roster.rs',*[s/f'src/bin/flat-supervised-{n}.rs' for n in ['compare','analyze','fixture']],s/'Cargo.toml',s/'Cargo.lock',s/'native-supervisor/src/lib.rs',s/'native-supervisor/Cargo.toml',s/'native-supervisor/Cargo.lock']
for d in ['crates/certified-relations','crates/relation-replay-checker']:
 files+=list((repo/d/'src').rglob('*.rs'));files+=[repo/d/'Cargo.toml']
ctx={'schema':'flat-matched-draft-synthetic-source/v1','scope':'Sources/tools before synthetic fixture and analysis invocation; binaries built and tests completed; not prospective performance pins','source_parent_commit':'4e6364822b5cc30412f2b58a4d788bdba063d9ed','source_sha256':{str(p.relative_to(repo) if p.is_absolute() else p):sha(p) for p in files},'tool_sha256':{str(p):sha(p) for p in out.iterdir()},'performance_observations_collected':0,'canonical_protocol_validation':'pending','15x_admitted':False};(s/'FLAT_MATCHED_SYNTHETIC_SOURCE_366.json').write_text(json.dumps(ctx,indent=2)+'\n')
pins=json.loads((s/'PACKED_PINS_200.json').read_text());pins['worker_sha256'][2]=sha('/workspace/relation-native-qualifiers/flat-274/fused-native');pins['scope']='SYNTHETIC FIXTURE INPUT IDENTITIES ONLY; no performance pins/admission. Current worker identity updated to actual flat274; original200 unchanged.';pins['synthetic_fixture']=True;(s/'FLAT_MATCHED_SYNTHETIC_PINS_366.json').write_text(json.dumps(pins,indent=2)+'\n')
r=subprocess.run(['taskset','-c','0',str(out/'flat-supervised-compare'),'--print-roster'],capture_output=True);assert r.returncode==0,r.stderr
(s/'FLAT_MATCHED_DRAFT_366.json').write_bytes(r.stdout);d=json.loads(r.stdout);assert d['planned_calls']==3600 and d['primary']=='flat-dag-487-native' and d['performance_observations_collected']==0
print(json.dumps({'frozen_tools':3,'planned_synthetic_cells':3600,'primary':d['primary'],'real_performance_calls':0}))
