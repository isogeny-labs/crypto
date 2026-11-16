from pathlib import Path
import os,shutil,subprocess,json,hashlib
D=Path(__file__).parent;S=D/'snapshot';S.mkdir(exist_ok=True);P=Path('/var/tmp/pencil-comparator-word-20261101/snapshot');H=Path('/var/tmp/independent-product-review-20261101/snapshot');bound={}
for root in [P,H]:
 for f in root.iterdir():
  if f.suffix not in ['.lean','.olean','.txt']:continue
  if not(S/f.name).exists():shutil.copy2(f,S/f.name);bound[str(f)]=hashlib.sha256(f.read_bytes()).hexdigest()
(D/'producer-inputs.json').write_text(json.dumps(bound,indent=2))
lean='/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean';env=os.environ.copy();env['LEAN_SYSROOT']=str(Path(lean).parent.parent);env['LEAN_PATH']=str(S)+':/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union';mods=['Comparator','WordComparator','WordControls','WordReadback'];inputs=[p for p in S.iterdir() if p.suffix in ['.lean','.olean','.txt'] and not(p.suffix=='.olean' and p.stem in mods)]+[Path(__file__),D/'review-plan.md',Path(lean)];before={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs};(D/'before.json').write_text(json.dumps(before,indent=2));out=[]
for n in mods:
 c=[lean,'-j1','-M4096','-o',str(S/(n+'.olean')),str(S/(n+'.lean'))];r=subprocess.run(c,env=env,cwd=S,capture_output=True,text=True,timeout=120);(D/(n+'.stdout')).write_text(r.stdout);(D/(n+'.stderr')).write_text(r.stderr);out.append({'module':n,'command':c,'exit_code':r.returncode});print(n,r.returncode,flush=True)
 if r.returncode:break
(D/'receipts.json').write_text(json.dumps(out,indent=2));after={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs};(D/'after.json').write_text(json.dumps(after,indent=2));assert before==after
