"""Metadata for completed checks; no research arithmetic."""
from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
v=p/'validation01';old=p.parent/'feature-native-signed-reciprocal'
records=json.loads((p/'generation.json').read_text())['copied']
for row in records:
 assert sha(p/row['source'])==row['sha256']==sha(old/row['source'])
deps=[Path(x).resolve() for x in Path('/tmp/t1899-check.d').read_text().splitlines()[0].split(': ',1)[1].split()]
ref=p.parent/'feature-native-planned-width/validation01/check.stdout'
assert (v/'inherited.stdout').read_bytes()==ref.read_bytes()
assert not (v/'check.stderr').read_bytes()
receipt={'scope':'Deterministic transported signed-image and public recovery software qualification',
 'build_exit':0,'focused_exit':0,'inherited_exit':0,'lean_exit':0,'compile_limit_seconds':90,'check_limit_seconds':60,'lean_limit_seconds':30,
 'commands':['rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info driver.rs -o /tmp/t1899-check','/tmp/t1899-check','/tmp/t1899-check --inherited','lean Transport.lean'],
 'binary_sha256':sha(Path('/tmp/t1899-check')),'source_hashes':{str(f):sha(f) for f in deps},
 'copied_sources':len(records),'logs':{f.name:sha(f) for f in v.iterdir() if f.suffix in ['.stdout','.stderr']},
 'inherited_byte_equal':True,'reference_sha256':sha(ref),'lean_source_sha256':sha(p/'Transport.lean'),
 'lean_version':'4.34.1','lean_commit':'5045d0056413266e57c625dcd7c365b10e377c52'}
(v/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print('copied_sources',len(records),'dependencies',len(deps))
