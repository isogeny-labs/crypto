from pathlib import Path
import os,subprocess,json,hashlib,sys,tempfile,shutil
D=Path(__file__).resolve().parent
R=Path(tempfile.mkdtemp(prefix="invocation-",dir=D));S=R/"snapshot";S.mkdir()
lean=Path('/var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean')
H=Path('/var/tmp/independent-graph-review-20261101/snapshot')
for root in [D,H]:
 for p in root.iterdir():
  if p.suffix in ['.lean','.olean'] and not(S/p.name).exists() and (root!=D or p.suffix=='.lean'):shutil.copy2(p,S/p.name)
mods=sys.argv[1:] or ['Find','Controls','Readback']
env=os.environ.copy();env['LEAN_SYSROOT']=str(lean.parent.parent);env['LEAN_PATH']=str(S)+':/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union'
inputs=[*S.glob('*.lean'),*[p for p in S.glob('*.olean') if p.stem not in mods],D/'gate.py',D/'PLAN.md',lean]
def hashes():return {str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs}
before=hashes();(R/'before.json').write_text(json.dumps(before,indent=2));rs=[];gate=0
try:
 for n in mods:
  c=[str(lean),'-j1','-M4096','-o',str(S/(n+'.olean')),str(S/(n+'.lean'))]
  try:
   r=subprocess.run(c,cwd=S,env=env,capture_output=True,text=True,timeout=120);code=r.returncode;stdout=r.stdout;stderr=r.stderr
  except subprocess.TimeoutExpired as e:code=124;stdout=str(e.stdout);stderr=str(e.stderr)
  (R/(n+'.stdout')).write_text(stdout);(R/(n+'.stderr')).write_text(stderr);rs.append({'module':n,'command':c,'native_exit':code});print(n,code,flush=True)
  if code:gate=code if code>0 else 128-code;break
except Exception as e:
 gate=125;(R/'exception.txt').write_text(repr(e))
after=hashes();(R/'after.json').write_text(json.dumps(after,indent=2))
if before!=after:gate=126
receipt={'modules':rs,'gate_exit':gate,'bindings_unchanged':before==after,'directory':str(R)}
(R/'receipt.tmp').write_text(json.dumps(receipt,indent=2));(R/'receipt.tmp').rename(R/'receipt.json')
print(str(R),flush=True);sys.exit(gate)
