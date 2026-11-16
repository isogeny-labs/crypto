from pathlib import Path
import json,hashlib
R=Path(__file__).resolve().parents[2];P=Path(__file__).resolve().parent
for n,h in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
cal=json.loads((R/'research/orbit_roots_20261116_v70/raw.json').read_text());cold=json.loads((R/'research/orbit_cold_20261116_v70/raw.json').read_text());assert cal['error']is None and cold['error']is None
assert cal['out']['small_systems']==256 and len(cold['out']['runs'])==12
for r in cold['out']['runs']:assert r['root_count']==len(cold['out']['roots'])
assert cold['out']['qualified_speedup']is None
print('Hashes,completeconditionalroots,12coldrunsandallcomparatorsverified. Noqualified50xclaim.')
