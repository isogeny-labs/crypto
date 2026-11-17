"""Byte-preserving raw evidence materialization; no arithmetic experiment."""
from pathlib import Path
import hashlib,json,sys,tarfile
p=Path(sys.argv[1]) if len(sys.argv)>1 else Path(__file__).parent
j=json.loads((p/'evidence/runs-inventory.json').read_text());a=p/j['archive']
assert hashlib.sha256(a.read_bytes()).hexdigest()==j['sha256']
with tarfile.open(a,'r:gz') as t:
 assert {m.name for m in t.getmembers()}=={x['path'] for x in j['files']}
 for x in j['files']:
  rel=Path(x['path']);assert rel.parts[:2]==('evidence','runs') and '..' not in rel.parts
  b=t.extractfile(str(rel)).read();assert len(b)==x['bytes'] and hashlib.sha256(b).hexdigest()==x['sha256']
  f=p/rel
  if f.exists():assert f.read_bytes()==b
  else:f.parent.mkdir(parents=True,exist_ok=True);f.write_bytes(b)
print('Raw evidence materialized byte-for-byte; no files overwritten.')
