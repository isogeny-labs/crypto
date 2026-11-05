"""Static custody verification, not a rerun of the construction."""
from pathlib import Path
import ast,hashlib,json
D=Path(__file__).resolve().parent
locks=json.loads((D/'check-lock.json').read_text())
for name,digest in locks.items():assert hashlib.sha256((D/name).read_bytes()).hexdigest()==digest,name
receipt=json.loads((D/'construction-receipt.json').read_text())
assert receipt['exit_code']==0 and receipt['attempts']==1 and receipt['output_within_limit']
assert hashlib.sha256((D/'construction.log').read_bytes()).hexdigest()==receipt['log_sha256']
summary=json.loads((D/'certificate/summary.json').read_text())
for name in ('program','factor'):
 assert hashlib.sha256((D/'certificate'/(name+'.json')).read_bytes()).hexdigest()==summary[name+'_sha256']
files=list(D.rglob('*.py'))
for p in files:ast.parse(p.read_text())
checker=ast.parse((D/'verify_program.py').read_text())
assert [(x.module,[a.name for a in x.names]) for x in checker.body if isinstance(x,ast.ImportFrom)]==[('math',['isqrt'])]
assert not any(isinstance(x,ast.Import) for x in checker.body)
print(json.dumps(dict(syntax_files=len(files),locked_inputs=len(locks),artifact_hashes=2,
 construction_exit_code=0,independent_checker_imports_verified=True,
 scope='static custody; exact computation and full-field checks separately receipted')))
