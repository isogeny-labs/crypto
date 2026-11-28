"""Record completed deterministic checks; no research arithmetic."""
from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent
sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
v=p/'validation03'
deps=[Path(x).resolve() for x in Path('/tmp/t1904-v3-check.d').read_text().splitlines()[0].split(': ',1)[1].split()]
assert not (v/'check.stderr').read_bytes()
assert 'large_projection_equalities=56' in (v/'check.stdout').read_text()
receipt={'scope':'Exact block recurrence and residue-section algebra checks; no timed experiment',
 'build_exit':0,'check_exit':0,'lean_exit':0,'compile_limit_seconds':90,'check_limit_seconds':60,'lean_limit_seconds':30,
 'commands':['rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info main.rs -o /tmp/t1904-v3-check','/tmp/t1904-v3-check','lean Algebra.lean'],
 'binary_sha256':sha(Path('/tmp/t1904-v3-check')),'source_hashes':{str(f):sha(f) for f in deps},
 'logs':{f.name:sha(f) for f in v.iterdir() if f.suffix in ['.stdout','.stderr']},
 'lean_source_sha256':sha(p/'Algebra.lean'),'lean_version':'4.34.1','lean_commit':'5045d0056413266e57c625dcd7c365b10e377c52'}
(v/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
v=p/'validation01'
(v/'receipt.json').write_text(json.dumps({'scope':'Block-only audit before section implementation','build_exit':0,'check_exit':0,'binary_sha256':sha(Path('/tmp/t1904-check')),'sources':{f.name:sha(f) for f in v.glob('*.snapshot')},'stdout_sha256':sha(v/'check.stdout')},indent=2)+'\n')
print('compiler_dependencies',len(deps))
