from pathlib import Path
import re,shutil,json,hashlib
r=Path('/var/tmp/pencil-branch-count-replay-20261102')
s=r/'research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/branch-count'
p=r/'target';p.mkdir(exist_ok=True)
for x in s.glob('*.lean.txt'):shutil.copy2(x,p/x.name.removesuffix('.txt'))
for n in ['readback-names.json','audit-readback.py']:shutil.copy2(s/n,p/n)
line=next(x for x in (s/'check.sh').read_text().splitlines() if x.startswith('export LEAN_PATH='))
prefix=line.split('$D:',1)[1].split('$(cat',1)[0].rstrip(':')
roots=prefix.split(':')+(s/'dependencies.txt').read_text().strip().split(':')
paths=[];ledger=[]
for i,root in enumerate(roots):
 src=Path(root); dst=r/'dependencies'/str(i);dst.mkdir(parents=True,exist_ok=True)
 if not src.is_dir():raise RuntimeError('missing root '+root)
 paths.append(str(dst))
 for x in sorted(src.iterdir()):
  if x.is_file() and x.suffix in ('.lean','.olean'):
   before=hashlib.sha256(x.read_bytes()).hexdigest();shutil.copy2(x,dst/x.name)
   after=hashlib.sha256(x.read_bytes()).hexdigest();copy=hashlib.sha256((dst/x.name).read_bytes()).hexdigest()
   assert before==after==copy
   ledger.append({'source':str(x),'copy':str(dst/x.name),'sha256':copy})
(r/'dependency-path.txt').write_text(':'.join(paths))
(r/'dependency-ledger.json').write_text(json.dumps(ledger,indent=2)+'\n')
(r/'reconciliation.json').write_text(json.dumps({'dependency_roots':roots,'dependency_files':len(ledger),'producer_manifest_checked':True,'live_compilers_at_start':False,'private_snapshot':True},indent=2)+'\n')
print(len(ledger),'dependency files copied')
