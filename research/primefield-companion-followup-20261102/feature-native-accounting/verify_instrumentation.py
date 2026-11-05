"""Read-only verification of generated source mapping; executes no kernels."""
from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent
sha=lambda b:hashlib.sha256(b).hexdigest()
rows=json.loads((D/'instrumentation.json').read_text())['modules']
for row in rows:
 original=(D.parent/row['original']).read_bytes();generated=(D/'generated'/f"{row['module']}.rs").read_bytes()
 assert sha(original)==row['original_sha256']
 assert sha(generated)==row['generated_sha256']
 text=generated.decode()
 for edit in reversed(row['edits']):
  text=text.replace(edit['new'],edit['old'],1)if edit.get('first_only')else text.replace(edit['new'],edit['old'])
 assert text.encode()==original
print(json.dumps({'original_modules':len(rows),'generated_hashes_verified':len(rows),'exact_reverse_transforms':len(rows),'kernel_executions':0}))
