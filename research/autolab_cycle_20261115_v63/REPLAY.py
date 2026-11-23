from pathlib import Path
import sys,json,hashlib,base64,gzip,itertools
R=Path(__file__).resolve().parents[2];P=Path(__file__).resolve().parent;sys.path.insert(0,str(R))
for name,digest in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==digest,name
b=json.loads((R/'research/quadratic_product_lift_20261115_v63/raw.json').read_text());o=json.loads((R/'research/odd_quadratic_lift_20261115_v63/raw.json').read_text());pilot=json.loads((R/'research/quadratic_baseline_pilot_20261115_v63/raw.json').read_text())
assert b['error'] is None and o['error'] is None and pilot['error'] is None
payload=gzip.decompress(base64.b64decode((R/'research/quadratic_product_lift_20261115_v63/neutral.json.gz.b64').read_bytes()));assert hashlib.sha256(payload).hexdigest()==b['out']['input_sha256'];s=json.loads(payload);assert s['n_vars']==272 and s['term_occurrences']==27558 and s['max_degree']==2
assert len(pilot['out']['records'])==3 and all(not r['complete_matching_roots'] for r in pilot['out']['records'])
if '--oracles' in sys.argv:
 from research.quadratic_product_lift_20261115_v63.encoder import encode,extend,masks,evaluate
 from research.quadratic_product_lift_20261115_v63.worker import literal
 for p in b['out']['profiles']:
  n=p['n'];d=p['d'];rows=masks(encode(n,p['modulus'],d,p['z'],p['B']));roots=[]
  for bits in range(1<<(3*d+n)):
   want=literal(bits,n,p['modulus'],d,p['z'],p['B']);assert evaluate(rows,extend(bits,n,d))==want
   if not want:roots.append(bits)
  assert roots==p['ordered_root_masks']
 from research.odd_quadratic_lift_20261115_v63.worker import F,original,lifted
 for p in o['out']['profiles']:
  f=F(p['p'],p['T_squared']);roots=[]
  for a,b,u in itertools.product(range(f.q),repeat=3):
   aux=[f.mul(a,b),f.mul(f.add(a,b),u),f.mul(f.sub(a,b),u)];want=original(f,a,b,u,p['A'],p['B']);assert want==lifted(f,a,b,u,p['A'],p['B'],*aux)
   if not want:roots.append([a,b,u])
  assert hashlib.sha256(json.dumps(roots,separators=(',',':')).encode()).hexdigest()==p['ordered_roots_sha256']
print('Hashes, lifted input, retained incomplete baselines and requested oracle replay verified. No qualified speedup claim.')
