from pathlib import Path
import os,subprocess,json,time,hashlib
p=Path(__file__).parent;d=p/'replay';lean='/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean';env=os.environ.copy();env['LEAN_SYSROOT']=str(Path(lean).parent.parent);env['LEAN_PATH']=str(d)+':/var/tmp/independent-same-support-replay-ow8hrtl9:/var/tmp/pencil-word-classifier-v2-20261031/dependency-union';receipts=[]
for n in ['CanonicalResidue','UInt64Residue','NatClassifier','WordClassifier','CrossScaling','WeightedSameSupport','WordErrorExclusion','Readback','Controls']:
 cmd=['timeout','-k','5s','120s',lean,'-j1','-M4096','-o',str(d/(n+'.olean')),str(d/(n+'.lean'))];r=subprocess.run(cmd,cwd=d,env=env,capture_output=True,text=True);(d/(n+'.stdout')).write_text(r.stdout);(d/(n+'.stderr')).write_text(r.stderr);receipts.append({'module':n,'exit_code':r.returncode,'command':cmd,'source_sha256':hashlib.sha256((d/(n+'.lean')).read_bytes()).hexdigest()});(p/'replay-receipts.json').write_text(json.dumps(receipts,indent=2));print(n,r.returncode,flush=True)
 if r.returncode:break
