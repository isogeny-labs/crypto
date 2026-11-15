from pathlib import Path
import json,hashlib,shutil,subprocess,time
p=Path(__file__).parent;s=Path('/var/tmp/pencil-kummer-target-rank-20261031');sha=lambda q:hashlib.sha256(q.read_bytes()).hexdigest()
m=json.loads((s/'manifest.json').read_text())
for n,h in m['files'].items():assert sha(s/n)==h
for n,h in json.loads((s/'pre-execution-manifest.json').read_text()).items():assert sha(s/n)==h
shutil.copytree(s,p/'producer');t=time.monotonic();r=subprocess.run(['timeout','120s','python3',str(p/'producer/check.py')],capture_output=True,cwd=p)
(p/'replay.stdout').write_bytes(r.stdout);(p/'replay.stderr').write_bytes(r.stderr)
assert r.returncode==0 and r.stdout==(s/'result.json').read_bytes()
(p/'receipt.json').write_text(json.dumps({'exit_code':r.returncode,'elapsed_seconds':time.monotonic()-t,'command':['timeout','120s','python3',str(p/'producer/check.py')],'byte_equal_to_producer_result':True},indent=2)+'\n')
src=Path('/var/tmp/pencil-kummer-original-gstep-20261031/src/pruned.rs');shutil.copyfile(src,p/'pruned.rs')
(p/'status.json').write_text(json.dumps({'scope':'independent algebra and source-placement review','producer_manifest_sha256':sha(s/'manifest.json'),'source_path':str(src),'source_sha256':sha(src),'fixed_cases':12,'replay_exit_code':0,'result_byte_equal':True,'blocking_discrepancy':False,'implementation_status':'proposal only','generic_input_validator_tested':False,'original_114_edge_forest_recomputed':False,'scientific_execution':False},indent=2)+'\n')
