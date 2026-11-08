"""Capture completed software qualification; no research arithmetic."""
from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;v=p/'validation02';sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
old=p.parent/'feature-character-phase';rows=json.loads((p/'generation.json').read_text())['copied']
for row in rows:assert sha(p/row['source'])==row['sha256']==sha(old/row['source'])
deps=[(p/x).resolve() for x in Path('/tmp/t1921-v2-check.d').read_text().splitlines()[0].split(': ',1)[1].split()]
ref=p.parent/'feature-native-planned-width/validation01/check.stdout'
assert (v/'inherited.stdout').read_bytes()==ref.read_bytes()
assert not(v/'check.stderr').read_bytes() and not(v/'inherited.stderr').read_bytes()
assert 'odd_L_public_cases=228' in (v/'check.stdout').read_text()
receipt={'scope':'Deterministic correctness checks for character-certified initialization and endpoint-correct lift without phase powers; no timing or superiority claim','build_exit':0,'check_exit':0,'inherited_exit':0,'limits_seconds':{'compile':90,'focused':60,'inherited':60},'commands':['rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info driver.rs -o /tmp/t1921-v2-check','/tmp/t1921-v2-check','/tmp/t1921-v2-check --inherited'],'binary_sha256':sha(Path('/tmp/t1921-v2-check')),'source_hashes':{str(f):sha(f) for f in deps},'copied_sources':len(rows),'logs':{f.name:sha(f) for f in v.iterdir() if f.suffix in ['.stdout','.stderr']},'inherited_byte_equal':True,'reference_sha256':sha(ref)}
(v/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print('copied_sources',len(rows),'compiler_dependencies',len(deps))
