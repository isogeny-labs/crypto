"""Read-only inverse source-map and shared-module checks; no kernels."""
from pathlib import Path
import hashlib,json
D=Path(__file__).resolve().parent
for arm in ('baseline','candidate'):
 rows=json.loads((D/f'instrumentation-{arm}.json').read_text())['modules']
 for row in rows:
  original=(D.parent/row['original']).read_bytes();generated=(D/f'generated-{arm}'/f"{row['module']}.rs").read_bytes()
  assert hashlib.sha256(original).hexdigest()==row['original_sha256']
  assert hashlib.sha256(generated).hexdigest()==row['generated_sha256']
  text=generated.decode()
  for edit in reversed(row['edits']):text=text.replace(edit['new'],edit['old'],1)if edit.get('first_only')else text.replace(edit['new'],edit['old'])
  assert text.encode()==original
for module in ('geometry','jets','value','polynomial','decoder','recovery'):
 assert (D/'generated-baseline'/f'{module}.rs').read_bytes()==(D/'generated-candidate'/f'{module}.rs').read_bytes()
print(json.dumps({'reverse_transforms':14,'unchanged_non_source_modules':6,'kernel_executions':0}))
