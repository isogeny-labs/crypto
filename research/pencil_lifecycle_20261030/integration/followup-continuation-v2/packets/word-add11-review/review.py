from pathlib import Path
import hashlib,json,shutil,subprocess
p=Path(__file__).parent;s=Path('/var/tmp/pencil-add11-word-composition-20261031-v1');sha=lambda q:hashlib.sha256(q.read_bytes()).hexdigest()
assert sha(s/'manifest.json')=='548e842e15ab61ee11f508f4388306ae975210750b60bccceb6e9170b618a911'
for n,h in json.loads((s/'manifest.json').read_text())['files'].items():
 if isinstance(h,dict):h=h['sha256']
 assert sha(s/n)==h,n
for f,h in json.loads((s/'sources.json').read_text())['files'].items():assert sha(Path(f))==h
shutil.copytree(s,p/'producer',ignore=shutil.ignore_patterns('*.olean'))
d=p/'replay';d.mkdir()
for n in ['WordAdd11.lean','Controls.lean','Readback.lean','check.py']:shutil.copyfile(s/n,d/n)
rr=[]
for n in ['WordAdd11','Controls','Readback']:
 x=subprocess.run(['python3',str(d/'check.py'),n,n+'-check'],capture_output=True,text=True,timeout=130)
 rec=json.loads((d/(n+'-check')/'receipt.json').read_text());rr.append(rec);print(n,x.stdout,flush=True)
(p/'receipts.json').write_text(json.dumps(rr,indent=2)+'\n')
