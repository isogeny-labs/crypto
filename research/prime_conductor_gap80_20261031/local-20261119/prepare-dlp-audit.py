#!/usr/bin/env python3
"""Metadata serialization only; PARI independently verifies actual recoveries."""
from pathlib import Path
import json
s=Path(__file__).resolve().parent;lines=[]
for name in ['pilot-l0','pilot96-l0']:
 for line in (s/'results'/name/'records.jsonl').read_text().splitlines():
  r=json.loads(line);w=r['workload'];c=w['curve_spec']
  assert r['outcome']['status']=='verified'
  v=[c['p'],c['a'],c['b'],c['r'],w['curve']['generator'],w['target'],r['outcome']['recovered'],r['record_id']]
  lines.append(json.dumps(v,separators=(',',':')))
(s/'dlp-audit.gpvec').write_text('\n'.join(lines)+'\n')
