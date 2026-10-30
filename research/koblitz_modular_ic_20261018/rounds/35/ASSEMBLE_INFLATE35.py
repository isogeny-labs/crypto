#!/usr/bin/env python3
"""Native source concatenation plus byte checks, no field computation."""
from pathlib import Path
import hashlib,json
h=Path(__file__).resolve().parent
source=(h/'MULTIPLY35.rs').read_text().split('fn main(){',1)[0]
(h/'INFLATE35.rs').write_text(source+(h/'INFLATE-MAIN35.rs').read_text())
prior=h.parent/'34';manifest={n:d for d,n in (line.split(maxsplit=1) for line in (prior/'runs.sha256').read_text().splitlines())}
out=h/'source-data/coupled';out.mkdir(exist_ok=True);pins={}
paths=sorted((prior/'runs/couple34-01').glob('coupled-*.sing'));assert len(paths)==10
for p in paths:
 n=str(p.relative_to(prior));d=hashlib.sha256(p.read_bytes()).hexdigest();assert d==manifest[n];(out/p.name).write_bytes(p.read_bytes());pins[n]=d
(h/'inflate-donors.json').write_text(json.dumps(pins,indent=2)+'\n')
print('Assembled native inflation source; ten immutable coupled donors byte-verified.')
