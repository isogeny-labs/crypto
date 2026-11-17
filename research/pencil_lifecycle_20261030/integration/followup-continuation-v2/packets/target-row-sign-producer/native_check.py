from pathlib import Path
import subprocess,os,json,hashlib
D=Path(__file__).parent;R=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');out=D/'native-evidence';out.mkdir()
paths=list((R/'research/pencil_large_prime_20261029/src').rglob('*.rs'))+[R/'research/pencil_large_prime_20261029/Cargo.toml',R/'research/pencil_lifecycle_20261030/integration/five-followups/rank-information/src/lib.rs',R/'research/pencil_lifecycle_20261030/integration/five-followups/rank-information/Cargo.toml',D/'native/main.rs',D/'native/Cargo.toml',D/'native_check.py',D/'NATIVE-PLAN.md']
def bindings():return {str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
before=bindings();(out/'input-bindings-before.json').write_text(json.dumps(before,indent=2)+'\n');env=dict(os.environ,CARGO_TARGET_DIR=str(D/'target'),CARGO_NET_OFFLINE='true',CARGO_BUILD_JOBS='1')
commands=[['cargo','generate-lockfile','--offline','--manifest-path',str(D/'native/Cargo.toml')],['cargo','build','--offline','--locked','--manifest-path',str(D/'native/Cargo.toml')]]
for i,cmd in enumerate(commands):
 r=subprocess.run(['timeout','120s']+cmd,env=env,capture_output=True,text=True);(out/f'build{i}.stdout').write_text(r.stdout);(out/f'build{i}.stderr').write_text(r.stderr);(out/f'build{i}.receipt.json').write_text(json.dumps({'argv':cmd,'exit_code':r.returncode},indent=2)+'\n');assert r.returncode==0
inputs=[];expects=[]
for p,records in [(5,json.loads((D/'rows-p5.json').read_text())),(7,json.loads((D/'rows-p7.json').read_text())),(7,list(json.loads((D/'fixed-controls.json').read_text()).values()))]:
 for row in records:
  for key in ['legacy','normalized']:
   inputs.append(str(p)+'|'+';'.join(','.join(map(str,v)) for v in row[key]));ranks=row['ranks'];expects.append(','.join(map(str,ranks))+','+str(row['solutions_count']>0).lower()+','+str(len(row['target_values'])==1).lower())
(out/'inputs.txt').write_text('\n'.join(inputs)+'\n');binary=D/'target/debug/row-sign-check';r=subprocess.run(['timeout','120s',str(binary)],input='\n'.join(inputs)+'\n',capture_output=True,text=True,env=env);(out/'stdout').write_text(r.stdout);(out/'stderr').write_text(r.stderr);(out/'receipt.json').write_text(json.dumps({'argv':[str(binary)],'exit_code':r.returncode,'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest()},indent=2)+'\n');assert r.returncode==0 and r.stdout.splitlines()==expects
after=bindings();(out/'input-bindings-after.json').write_text(json.dumps(after,indent=2)+'\n');assert before==after
result={'native_rows_checked':len(inputs),'all_outputs_match':True,'input_bindings_unchanged':True,'independent_review':False};(out/'comparison.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
