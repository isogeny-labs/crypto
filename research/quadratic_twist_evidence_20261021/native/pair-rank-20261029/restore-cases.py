#!/usr/bin/env python3
# Exact file I/O and SHA metadata only, no solver/field/LA/research algorithm.
from pathlib import Path
import sys,json,hashlib,shutil
src=Path(__file__).resolve().parent;out=Path(sys.argv[1]);out.mkdir(parents=True,exist_ok=False)
m=json.loads((src/'cases-manifest.json').read_text());all_bytes=b''
for p in m['parts']:
 b=(src/p['path']).read_bytes();assert len(b)==p['bytes'] and hashlib.sha256(b).hexdigest()==p['sha256'];all_bytes+=b
assert len(all_bytes)==m['bytes'] and hashlib.sha256(all_bytes).hexdigest()==m['sha256']
(out/'cases.tsv').write_bytes(all_bytes)
for f in ['summary.json','bindings.tsv','domain-controls.tsv']:shutil.copy2(src/f,out/f)
print('Exact native case files restored; no arithmetic replay claimed')
