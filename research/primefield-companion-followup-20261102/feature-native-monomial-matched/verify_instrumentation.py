"""Read-only provenance and corruption controls; no kernel execution."""
from pathlib import Path
import hashlib,json,sys
D=Path(__file__).resolve().parent
arm=sys.argv[1];assert arm in ('baseline','candidate')
sha=lambda b:hashlib.sha256(b).hexdigest()
def validate(original,generated,row):
 assert sha(original)==row['original_sha256'],'original hash'
 assert sha(generated)==row['generated_sha256'],'generated hash'
 forward=original.decode()
 for edit in row['edits']:
  assert forward.count(edit['old'])>0,'missing original fragment'
  if not edit['first_only']:assert forward.count(edit['old'])==edit['occurrences'],'occurrences'
  forward=forward.replace(edit['old'],edit['new'],1)if edit['first_only']else forward.replace(edit['old'],edit['new'])
 assert forward.encode()==generated,'forward transform'
 reverse=generated.decode()
 for edit in reversed(row['edits']):reverse=reverse.replace(edit['new'],edit['old'],1)if edit['first_only']else reverse.replace(edit['new'],edit['old'])
 assert reverse.encode()==original,'reverse transform'
rows=json.loads((D/('instrumentation-'+arm+'.json')).read_text())['modules'];rejections=0
for row in rows:
 original=(D.parent/row['original']).read_bytes();generated=(D/('generated-'+arm)/f"{row['module']}.rs").read_bytes();validate(original,generated,row)
 for a,b in [(original+b' ',generated),(original,generated+b' ')]:
  try:validate(a,b,row)
  except AssertionError:rejections+=1
  else:raise AssertionError('corruption accepted')
assert rejections==2*len(rows)
print(json.dumps(dict(original_modules=len(rows),forward_transforms=len(rows),reverse_transforms=len(rows),corruption_rejections=rejections,kernel_executions=0)))
