from pathlib import Path
import hashlib,json,shutil,subprocess
p=Path(__file__).parent;s=Path('/var/tmp/pencil-add11-word-validity-20261031-v1');sha=lambda q:hashlib.sha256(q.read_bytes()).hexdigest()
m=json.loads((s/'manifest.json').read_text())
for n,h in m['files'].items():
 if isinstance(h,dict):h=h['sha256']
 assert sha(s/n)==h,n
shutil.copytree(s,p/'producer',ignore=shutil.ignore_patterns('*.olean'))
d=p/'replay';d.mkdir()
for n in ['WordAdd11Validity.lean','Controls.lean','Readback.lean','check.py']:shutil.copyfile(s/n,d/n)
r=[]
for n in ['WordAdd11Validity','Controls','Readback']:
 x=subprocess.run(['python3',str(d/'check.py'),n,n+'-check'],capture_output=True,text=True,timeout=130);r.append(json.loads((d/(n+'-check')/'receipt.json').read_text()));print(n,x.stdout,flush=True)
(p/'receipts.json').write_text(json.dumps(r,indent=2)+'\n')
