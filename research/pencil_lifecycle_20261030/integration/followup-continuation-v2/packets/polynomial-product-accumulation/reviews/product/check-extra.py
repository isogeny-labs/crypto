from pathlib import Path
import os,subprocess,json,hashlib,shutil
D=Path(__file__).parent;S=D/'snapshot';P=Path('/var/tmp/pencil-product-accumulation-20261101-v1')
for name in ['Controls','Readback']:shutil.copy2(P/(name+'.lean'),S/(name+'.lean'))
lean='/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean';env=os.environ.copy();env['LEAN_SYSROOT']=str(Path(lean).parent.parent);env['LEAN_PATH']=str(S)+':/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union'
results=[]
for name in ['Controls','Readback','IndependentControls']:
 c=[lean,'-j1','-M4096','-o',str(S/(name+'.olean')),str(S/(name+'.lean'))];r=subprocess.run(c,cwd=S,env=env,text=True,capture_output=True,timeout=120)
 (D/(name+'.stdout')).write_text(r.stdout);(D/(name+'.stderr')).write_text(r.stderr);results.append({'command':c,'exit_code':r.returncode,'source_sha256':hashlib.sha256((S/(name+'.lean')).read_bytes()).hexdigest()});(D/'extra-receipts.json').write_text(json.dumps(results,indent=2));print(name,r.returncode,flush=True)
 if r.returncode:break
