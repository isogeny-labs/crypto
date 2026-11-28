from pathlib import Path
import json,hashlib,base64,tarfile,io
c=Path(__file__).resolve().parent
r=c.parents[1]
e=json.loads((c/'EVIDENCE_INDEX.json').read_text())
a=(c/'RAW_EVIDENCE.tar.xz.b64').read_bytes()
assert hashlib.sha256(a).hexdigest()==e['archive_sha256']
with tarfile.open(fileobj=io.BytesIO(base64.b64decode(a)),mode='r:xz') as t:
 for name,h in e.items():
  if name in ['archive_sha256','omitted_redundant_artifact']:continue
  assert hashlib.sha256(t.extractfile(name).read()).hexdigest()==h,name
from research.guarded_local_search_20261116_v101.kernel import source_product,valid_guard,prefix_remainder
for k in [1,3,5]:
 xs=[(1<<17)+i for i in range(k)]+[0]*(5-k)
 H=source_product(xs);q,H0=__import__('research.guarded_local_search_20261116_v101.kernel',fromlist=['divrem']).divrem(H[0],(1<<83)|(1<<45)|7)
 assert valid_guard(H,q)
assert prefix_remainder(source_product([0,1,1,2,3]),83,[int(i in (0,1,2,45,83)) for i in range(84)],18)==[]
assert prefix_remainder([0,0,0,1,1,1],83,[int(i in (0,1,2,45,83)) for i in range(84)],18)!=[]
print('C101 archive hashes and independent guard/support replay passed; no all-roots or performance claim')
