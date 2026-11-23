from pathlib import Path
import os,subprocess,json,hashlib
D=Path(__file__).parent;S=D/'snapshot';mods=['Comparator','ComparatorControls','ComparatorReadback','IndependentControls2'];lean='/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean';env=os.environ.copy();env['LEAN_SYSROOT']=str(Path(lean).parent.parent);env['LEAN_PATH']=str(S)+':/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union';paths=[p for p in S.iterdir() if p.suffix in ['.lean','.olean'] and not(p.suffix=='.olean' and p.stem in mods)]+[Path(__file__),Path(lean),D/'review-plan.md'];before={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths};(D/'final-before.json').write_text(json.dumps(before,indent=2));rr=[]
for n in mods:
 c=[lean,'-j1','-M4096','-o',str(S/(n+'.olean')),str(S/(n+'.lean'))];r=subprocess.run(c,cwd=S,env=env,capture_output=True,text=True,timeout=120);(D/('final-'+n+'.stdout')).write_text(r.stdout);(D/('final-'+n+'.stderr')).write_text(r.stderr);rr.append({'module':n,'command':c,'exit_code':r.returncode});print(n,r.returncode,flush=True)
 if r.returncode:break
(D/'final-receipts.json').write_text(json.dumps(rr,indent=2));after={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths};(D/'final-after.json').write_text(json.dumps(after,indent=2));assert before==after
