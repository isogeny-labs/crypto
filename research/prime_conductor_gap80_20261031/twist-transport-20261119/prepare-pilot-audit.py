#!/usr/bin/env python3
"""Serialize observed records for independent PARI point checks."""
import json
from pathlib import Path
HERE=Path(__file__).resolve().parent
rows=[]
for line in (HERE/'results/pilot-l0/records.jsonl').read_text().splitlines():
    r=json.loads(line); c=r['workload']['curve_spec']; target=[int(x,16) for x in r['workload']['target']]
    recovered=r['outcome']['recovered']
    rows.append([c['p'],c['a'],c['b'],c['r'],[c['gx'],c['gy']],target,r['workload']['planted'],int(recovered) if recovered is not None else -1])
(HERE/'pilot-audit.gpvec').write_text(json.dumps(rows)+'\n')
