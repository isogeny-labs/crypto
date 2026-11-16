from pathlib import Path
import os,subprocess,json,hashlib
D=Path(__file__).parent;lean='/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean';env=os.environ.copy();env['LEAN_SYSROOT']=str(Path(lean).parent.parent);env['LEAN_PATH']=str(D)+':/var/tmp/independent-graph-review-20261101/snapshot:/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union'
inputs=[*D.glob('*.lean'),D/'PLAN.md',Path(lean),Path(__file__),*Path('/var/tmp/independent-graph-review-20261101/snapshot').glob('*.lean'),*Path('/var/tmp/independent-graph-review-20261101/snapshot').glob('*.olean')];before={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs};(D/'bindings-before.json').write_text(json.dumps(before,indent=2));rs=[]
for n in ['Cycles','Controls','Readback']:
 c=[lean,'-j1','-M4096','-o',str(D/(n+'.olean')),str(D/(n+'.lean'))];r=subprocess.run(c,cwd=D,env=env,capture_output=True,text=True,timeout=120);(D/(n+'.stdout')).write_text(r.stdout);(D/(n+'.stderr')).write_text(r.stderr);rs.append({'module':n,'command':c,'exit_code':r.returncode});print(n,r.returncode)
 if r.returncode:break
(D/'receipts.json').write_text(json.dumps(rs,indent=2));after={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs};(D/'bindings-after.json').write_text(json.dumps(after,indent=2));assert before==after
