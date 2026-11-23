from pathlib import Path
import json,hashlib,runpy
C=Path(__file__).resolve().parent;d=json.loads((C/'ARCHIVE_PARTS.json').read_text());raw=b''
for r in d['parts']:
 b=(C/r['path']).read_bytes();assert len(b)==r['bytes']and hashlib.sha256(b).hexdigest()==r['sha256'];raw+=b
assert hashlib.sha256(raw).hexdigest()==d['aggregate_sha256'];p=C/'RAW_EVIDENCE.tar.xz.b64'
if p.exists():assert p.read_bytes()==raw
else:p.open('xb').write(raw)
runpy.run_path(str(C/'REPLAY.py'),run_name='__main__')
