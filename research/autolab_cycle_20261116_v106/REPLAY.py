from pathlib import Path
import json,hashlib,base64,tarfile,io
c=Path(__file__).resolve().parent;root=c.parents[1]
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((c/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in json.loads((c/'RAW_INDEX.json').read_text()).items():assert hashlib.sha256(t.extractfile(name).read()).hexdigest()==h
for name,h in json.loads((c/'SOURCE_SHA256.json').read_text()).items():assert hashlib.sha256((root/name).read_bytes()).hexdigest()==h
H=[0,0,1,1,1,1]
assert [x for x in range(3)if sum(a*pow(x,i,3)for i,a in enumerate(H))%3==0]==[0,2]
print('C106 raw/source hashes and literal GF3 counterexample verified. Proposed GCD repair remains unexecuted; no timing/root-solver promotion.')
