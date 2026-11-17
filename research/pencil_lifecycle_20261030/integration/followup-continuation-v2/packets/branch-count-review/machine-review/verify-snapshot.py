from pathlib import Path
import json,hashlib
r=Path('/var/tmp/pencil-branch-count-replay-20261102'); state=json.loads((r/'reconciliation.json').read_text());rows=[]
for i in [10,11]:
 root=Path(state['dependency_roots'][i]);copy=r/'dependencies'/str(i)
 for f in sorted(copy.rglob('*')):
  if not f.is_file():continue
  src=root/f.relative_to(copy)
  a=hashlib.sha256(src.read_bytes()).hexdigest();b=hashlib.sha256(f.read_bytes()).hexdigest()
  assert a==b,(src,f)
  rows.append({'source':str(src),'copy':str(f),'sha256':b})
(r/'expanded-dependency-ledger.json').write_text(json.dumps(rows,separators=(',',':'))+'\n')
print(len(rows),'expanded cache objects match sources')
