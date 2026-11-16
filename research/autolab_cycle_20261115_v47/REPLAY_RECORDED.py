from pathlib import Path
import json,hashlib
from research.root_constant_image_20261115_v47.worker import run
P=Path(__file__).resolve().parent;R=P.parents[1]
for name,digest in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():
 raw=(R/name).read_bytes();assert hashlib.sha256(raw).hexdigest()==digest;data=json.loads(raw);assert data['error']is None
 assert json.loads(json.dumps(run()))==data['out']
print('Verified complete small root cells, sampled actual root fibers, back substitution and projected Gray increments.')
