from pathlib import Path
import hashlib,json,shutil,subprocess,os
p=Path(__file__).parent;s=Path('/var/tmp/pencil-add11-weighted-invariant-20261031-v1');sha=lambda q:hashlib.sha256(q.read_bytes()).hexdigest()
assert sha(s/'manifest.json')=='e7c4941c44f5f9b21f59080250ae9e8ec83a603e804e002745828172318b1488'
m=json.loads((s/'manifest.json').read_text());print(type(m),list(m)[:3])
for n,v in m['files'].items():assert sha(s/n)==v['sha256'],n
sources=json.loads((s/'sources.json').read_text())
for group in ['rust','lean_dependencies']:
 for path,h in sources[group].items():assert sha(Path(path))==h
shutil.copytree(s,p/'producer',ignore=shutil.ignore_patterns('*.olean'))
d=p/'replay';d.mkdir()
for n in ['WeightedAdd11.lean','Controls.lean','Readback.lean','check.py']:shutil.copyfile(s/n,d/n)
receipts=[]
for name in ['WeightedAdd11','Controls','Readback']:
 r=subprocess.run(['python3',str(d/'check.py'),name,name+'-check'],cwd=d,capture_output=True,text=True,timeout=130)
 receipts.append({'module':name,'wrapper_exit':r.returncode,'native_receipt':json.loads((d/(name+'-check')/'receipt.json').read_text())});print(name,r.stdout,r.stderr,flush=True)
(p/'replay-receipts.json').write_text(json.dumps(receipts,indent=2)+'\n')
