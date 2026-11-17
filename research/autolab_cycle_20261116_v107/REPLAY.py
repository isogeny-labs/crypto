from pathlib import Path
import json,hashlib,base64,tarfile,io,sys,ast,itertools
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT))
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_text()).items():assert hashlib.sha256(t.extractfile(name).read()).hexdigest()==h
for name,h in json.loads((C/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h
for item in json.loads((C/'PREPARED_HARNESS_SYNTAX.json').read_text())['checks']:
 p=ROOT/item['path'];assert hashlib.sha256(p.read_bytes()).hexdigest()==item['sha256'];ast.parse(p.read_text())
from research.discriminant_roots_20261116_v106d.kernel import Field,solve,gcd
f=Field(3,[0,1]);H=[0,0,1,1,1,1]
assert gcd([0,1,1],[0,0],f)==[0,1,1]
for method in ['direct','radical']:assert solve(H,f,method,1061611)['roots']==[[0,2],[2,1]]
assert (ROOT/'research/discriminant_roots_20261116_v107b/PANEL.json').read_bytes()==(ROOT/'research/discriminant_roots_20261116_v106d/PANEL.json').read_bytes()
print('C107 raw/source hashes, GCD regression, corrected harness syntax and frozen panel equality passed; NO cold timing rows exist.')
