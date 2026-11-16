from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent
for name,digest in json.loads((p/'EVIDENCE_SHA256.json').read_text()).items():
 assert hashlib.sha256(Path(name).read_bytes()).hexdigest()==digest,name
small=json.loads(Path('research/prime_orbit_roots_20261116_v71/raw.json').read_text())
large=json.loads(Path('research/nist_orbit_correctness_20261116_v71/raw.json').read_text())
assert small['error'] is None and large['error'] is None
assert small['out']['systems']==189 and small['out']['literal_shift_aux_checks']==310149
assert len(large['out']['fields'])==5
for r in large['out']['fields']:
 assert r['prime_proof']['proven_prime'] and r['standard_generator_on_curve']
 assert r['known_solution'] in r['roots']
 assert r['factor_seconds_development']>r['finite_difference_seconds_development']
print('C71 evidence hashes and recorded outcomes verified; no qualified speedup claim')
