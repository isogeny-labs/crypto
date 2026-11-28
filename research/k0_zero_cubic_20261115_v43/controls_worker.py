from pathlib import Path
import json,sys,hashlib
from research.k0_zero_cubic_20261115_v43.worker import controls
p=Path(__file__).parent
o=controls()
b=json.dumps(o,sort_keys=True).encode()
with(p/'raw-controls.json').open('xb')as f:f.write(b)
print(json.dumps(dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status='passed',summary='Independent tiny commonX/conjugation bridge oracle only; no newnativeexecution',evidence=dict(controls=o,raw_sha256=hashlib.sha256(b).hexdigest(),qualified_speedup=None,complete_global_root_search=False),next_actions=['Reuse audited historicalcubic certificate or run newlyprepared oppositeordinate nativeworker nextsession'])))
