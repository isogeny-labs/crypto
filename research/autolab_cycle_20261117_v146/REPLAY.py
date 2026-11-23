from pathlib import Path
import json,hashlib,tarfile,io,base64
C=Path(__file__).resolve().parent;R=C.parents[1]
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for n,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():assert hashlib.sha256(t.extractfile(n).read()).hexdigest()==h,n
for n,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
print('C146source/raw frozen hashes replayed;scientific status retained including any refutation;noqualifiedspeedclaim.')
