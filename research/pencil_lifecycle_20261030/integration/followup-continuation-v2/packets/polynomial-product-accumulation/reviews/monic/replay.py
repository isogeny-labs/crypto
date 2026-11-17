from pathlib import Path
import re, shutil, json, hashlib, subprocess, os
D=Path(__file__).parent; S=D/'snapshot';S.mkdir(exist_ok=True)
Z=Path('/var/tmp/pencil-monic-remainder-zero-test-20261031-v1');T=Path('/var/tmp/pencil-monic-remainder-termination-20261031-v1')
txt=(Z/'check.py').read_text(); roots=[Z]+[Path(p) for p in re.search("e\['LEAN_PATH'\]=str\(D\)\+'([^']+)'",txt).group(1).split(':') if p]
bind={}
for root in roots:
 if root.name=='dependency-union':continue
 for pat in ['*.lean','*.olean']:
  for f in root.glob(pat):
   out=S/f.name
   if not out.exists():shutil.copy2(f,out);bind[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
for name,root in [('TerminationControls',T),('ZeroControls',Z)]:
 shutil.copy2(root/'Controls.lean',S/(name+'.lean'))
 rb=(root/'Readback.lean').read_text().replace('import Controls','import '+name)
 (S/(name+'Readback.lean')).write_text(rb)
(D/'inputs.json').write_text(json.dumps(bind,indent=2))
lean='/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean';env=os.environ.copy();env['LEAN_SYSROOT']=str(Path(lean).parent.parent);env['LEAN_PATH']=':'.join([str(S)]+[str(r) for r in roots if r.name=='dependency-union'])
results=[]
for mod in ['Descent','Fuel','Normalized','ZeroTest','TerminationControls','ZeroControls','TerminationControlsReadback','ZeroControlsReadback']:
 c=[lean,'-j1','-M4096','-o',str(S/(mod+'.olean')),str(S/(mod+'.lean'))]
 r=subprocess.run(c,env=env,cwd=S,capture_output=True,text=True,timeout=120)
 (D/(mod+'.stdout')).write_text(r.stdout);(D/(mod+'.stderr')).write_text(r.stderr)
 results.append({'module':mod,'exit_code':r.returncode,'command':c});(D/'receipts.json').write_text(json.dumps(results,indent=2));print(mod,r.returncode,flush=True)
 if r.returncode:break
