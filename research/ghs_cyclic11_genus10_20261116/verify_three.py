#!/usr/bin/env sage -python
"""Direct eleventh-power recount for each genus-five trace cell."""
from sage.all import GF
from pathlib import Path
import hashlib
import json

here=Path(__file__).parent
input_path=here/'THREE_BRANCH.json'
rows=json.loads(input_path.read_text())['records']
chosen={}
for r in rows:chosen.setdefault(r['trace_t'],r)
assert set(chosen)=={-24,-9,24}
K=GF(243,name='u')
powers={v**11 for v in K if v}
checks=[]
for trace,row in sorted(chosen.items()):
    if row['case']=='three_rational':
        a,b=row['exponents']
        roots=3+sum(11 if x**a*(x-1)**b in powers else 0
                    for x in K if x not in (0,1))
    else:
        roots=1+sum(11 if x*x+1 in powers else 0 for x in K)
    assert roots==row['N5'] and (244-roots)//5==trace
    checks.append({'trace_t':trace,'case':row['case'],
                   'exponents':row['exponents'],'N5':roots})
print(json.dumps({'schema':'ghs.cyclic11-genus5-direct-replay/v1',
                  'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'input_sha256':hashlib.sha256(input_path.read_bytes()).hexdigest(),
                  'checked':len(checks),'records':checks},sort_keys=True))
