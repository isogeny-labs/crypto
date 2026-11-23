# Byte restoration/hash metadata only; no research verifier.
from pathlib import Path
import sys,tarfile,hashlib
h=Path(__file__).resolve().parent;o=Path(sys.argv[1]);o.mkdir(parents=True,exist_ok=False)
a=h/'runs.tar.xz';assert hashlib.sha256(a.read_bytes()).hexdigest()==(h/'runs.tar.xz.sha256').read_text().split()[0]
with tarfile.open(a) as t:t.extractall(o,filter='data')
for l in (h/'RUNS.sha256').read_text().splitlines():
 d,p=l.split('  ',1);assert hashlib.sha256((o/p).read_bytes()).hexdigest()==d,p
print('Every raw proof/source/log hash restored')
