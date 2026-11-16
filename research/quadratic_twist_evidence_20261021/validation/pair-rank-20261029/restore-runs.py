#!/usr/bin/env python3
# Exact bytes/metadata only. Does not run research arithmetic or a verifier.
from pathlib import Path
import sys,json,hashlib,tarfile,base64
here=Path(__file__).resolve().parent;root=here.parents[3];native=here.parents[1]/'native/pair-rank-20261029';out=Path(sys.argv[1]);out.mkdir(parents=True,exist_ok=False)
def sha(b):return hashlib.sha256(b).hexdigest()
archive=here/'runs.tar.xz';expected=(here/'runs.tar.xz.sha256').read_text().split()[0];assert sha(archive.read_bytes())==expected
with tarfile.open(archive) as t:t.extractall(out,filter='data')
m=json.loads((native/'cases-manifest.json').read_text());cases=b''
for p in m['parts']:
 b=(native/p['path']).read_bytes();assert sha(b)==p['sha256'];cases+=b
assert sha(cases)==m['sha256']
pending=json.loads((here/'DEHYDRATED.json').read_text())
while pending:
 progress=False
 for rel,d in list(pending.items()):
  if d['kind']=='canonical':b=cases if d['path']=='@cases.tsv' else (root/d['path']).read_bytes()
  elif d['kind']=='patch-cases':b=cases[:d['prefix_bytes']]+base64.b64decode(d['insert_base64'])+(cases[-d['suffix_bytes']:] if d['suffix_bytes'] else b'')
  else:
   p=out/d['path']
   if not p.exists():continue
   b=p.read_bytes()
  assert sha(b)==d['sha256'];p=out/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b);del pending[rel];progress=True
 assert progress,'unresolved duplicate bindings'
for line in (here/'RUNS.sha256').read_text().splitlines():
 h,rel=line.split('  ',1);assert sha((out/rel).read_bytes())==h,rel
print('Every original raw hash restored, including all corrupted payloads')
