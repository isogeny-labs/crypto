from pathlib import Path
import json,hashlib,runpy
C=Path(__file__).resolve().parent
r=json.loads((C/'ARCHIVE_PARTS.json').read_bytes());b=b''.join((C/n).read_bytes()for n in r['parts']);assert hashlib.sha256(b).hexdigest()==r['sha256'];p=C/'RAW_EVIDENCE.tar.xz.b64'
if p.exists():assert p.read_bytes()==b
else:
 with p.open('xb')as f:f.write(b)
runpy.run_path(str(C/'REPLAY.py'),run_name='__main__')
