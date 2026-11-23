from pathlib import Path
import json,hashlib
R=Path(__file__).resolve().parents[2];P=Path(__file__).resolve().parent
for n,h in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
for d in ['packed_cubic_20261115_v65','packed_cubic_trials_20261115_v65','packed_cubic_cold_20261115_v65']:assert json.loads((R/'research'/d/'raw.json').read_text())['error']is None
v=json.loads((R/'research/packed_cubic_trials_20261115_v65/raw.json').read_text())['out']['records'][0];assert v['native_stats']['verified_packed_calls']==v['native_stats']['packed_calls']==262144 and v['exact_root_count']==6
c=json.loads((R/'research/packed_cubic_cold_20261115_v65/raw.json').read_text())['out'];assert len(c['runs'])==12 and all(x['exact_root_count']==6 for x in c['runs']);assert c['qualified_speedup']is None
print('Immutable evidence,262144exact fiber comparisons and12complete cold runs verified. Fiftyfold gain is unestablished.')
