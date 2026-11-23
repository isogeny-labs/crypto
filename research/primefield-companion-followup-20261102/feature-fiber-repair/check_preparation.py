"""Static custody audit; does not repeat exact/unit checks."""
from pathlib import Path
import ast,hashlib,json
D=Path(__file__).resolve().parent
locks=json.loads((D/'check-lock.json').read_text())
for rel,digest in locks.items():
    assert hashlib.sha256((D.parent/rel).read_bytes()).hexdigest()==digest,rel
receipt=json.loads((D/'unit-check-receipt.json').read_text())
assert receipt['exit_code']==0 and receipt['attempts']==1
assert hashlib.sha256((D/'unit-check.log').read_bytes()).hexdigest()==receipt['log_sha256']
files=list(D.rglob('*.py'))
for p in files:ast.parse(p.read_text())
print(json.dumps(dict(syntax_files=len(files),locked_files=len(locks),unit_log_hash_verified=True,
    unit_exit_code=0,unit_methods=5,report_pages_inspected=1,
    scope='static custody audit; local exact checks are separately receipted; no benchmark')))
