from pathlib import Path
import json,hashlib,base64,tarfile,io,runpy
C=Path(__file__).resolve().parent;ROOT=C.parents[1];index=json.loads((C/'RAW_INDEX.json').read_text())
with tarfile.open(fileobj=io.BytesIO(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_bytes())),mode='r:xz')as t:
 for name,h in index.items():
  if not name.startswith('research/discriminant_roots_20261116_v106d/'):continue
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h;p=ROOT/name
  if p.exists():assert p.read_bytes()==b
  else:p.parent.mkdir(parents=True,exist_ok=True);p.open('xb').write(b)
runpy.run_path(str(C/'REPLAY.py'),run_name='__main__')
