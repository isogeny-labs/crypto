from pathlib import Path
import json,hashlib,tarfile,io,base64
C=Path(__file__).resolve().parent;R=C.parents[1];receipt=json.loads((C/'ARCHIVE_TRANSPORT.json').read_bytes());raw=(C/'RAW_EVIDENCE.tar.xz').read_bytes()if(C/'RAW_EVIDENCE.tar.xz').exists()else base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes());assert hashlib.sha256(raw).hexdigest()==receipt['binary_sha256']
with tarfile.open(fileobj=io.BytesIO(raw),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():assert hashlib.sha256(t.extractfile(name).read()).hexdigest()==h,name
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
print('C147all source/raw/graph/child-ledger hashes replayed;native UNKNOWNand block cost refutation retained;noqualifiedspeedclaim.')
