"""Pure predata schedule audit: no operator invocation or numerical trial."""
from pathlib import Path
import json,sys,hashlib
D=Path(__file__).resolve().parent;sys.path.insert(0,str(D.parent/'tools'))
from experiment_protocol import seeds_table
p=json.loads((D/'protocol.json').read_text());plan=json.loads((D/'plan.json').read_text())
table=seeds_table(p)
expected=[dict(arm=arm,instance=str(i),replicate=0,seed=z['seed']) for arm in ['candidate','aa'] for i,z in enumerate(plan)]
assert table==expected and len(table)==1698
out=dict(status='verified_before_data',cells=len(table),instances_per_arm=len(plan),table_sha256=hashlib.sha256(json.dumps(table,sort_keys=True).encode()).hexdigest(),plan_sha256=hashlib.sha256((D/'plan.json').read_bytes()).hexdigest(),scope='Metadata enumeration only; no operator or matrix trials')
(D/'seed-preflight.json').write_text(json.dumps(out,indent=2)+'\n');print(out)
