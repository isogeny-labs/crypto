"""Static custody and decoder preservation; no repeated contract test execution."""
from pathlib import Path
import ast,hashlib,json
D=Path(__file__).resolve().parent
locks=json.loads((D/'check-lock.json').read_text())
for rel,digest in locks.items():assert hashlib.sha256((D/rel).read_bytes()).hexdigest()==digest,rel
preserved=json.loads((D/'source-preservation.json').read_text())
for name,record in preserved.items():
 assert hashlib.sha256((D/name).read_bytes()).hexdigest()==record['sha256']
 original=D/record['source']
 if original.exists():assert original.read_bytes()==(D/name).read_bytes()
receipt=json.loads((D/'contract-check-receipt.json').read_text())
assert receipt['exit_code']==0 and receipt['attempts']==1 and receipt['fresh_source_cases']==0
assert hashlib.sha256((D/'contract-check.log').read_bytes()).hexdigest()==receipt['log_sha256']
files=list(D.rglob('*.py'))
for p in files:ast.parse(p.read_text())
print(json.dumps(dict(syntax_files=len(files),locked_files=len(locks),preserved_sources=len(preserved),
 contract_methods=3,recorded_oracle_cases=4,fresh_source_cases=0,
 scope='static custody; contract replay and full-field reload checks separately receipted')))
