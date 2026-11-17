"""Project complete accounting records; no cryptographic execution."""
from pathlib import Path
import json
D=Path(__file__).resolve().parent
raw=(D/'accounting-test.log').read_text();rows=[]
for line in raw.splitlines():
 if 'count\t' in line:
  _,label,stage,component,op,n=line[line.index('count\t'):].split('\t')
  rows.append(dict(label=label,stage=stage,component=component,op=op,value=int(n)))
assert len(rows)==raw.count('count\t')
assert len(rows)==len({(x['label'],x['stage'],x['component'],x['op'])for x in rows})
record=dict(schema='quartic-arithmetic-events/v1',source='accounting-test.log',rows=rows)
assert record==json.loads((D/'counts.json').read_text())
print(json.dumps(dict(projected_records=len(rows),all_raw_markers_included=True,cryptographic_executions=0)))
