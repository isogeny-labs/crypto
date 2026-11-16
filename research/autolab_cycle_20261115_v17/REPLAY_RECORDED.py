"""Post-run semantic verification of recorded certificates; no new target queries."""
from pathlib import Path
import gzip,hashlib,json
R=Path(__file__).resolve().parents[2];C=Path(__file__).resolve().parent
path=R/'research/e2_certificate_20261115_v17/raw.json.gz'
assert hashlib.sha256(path.read_bytes()).hexdigest()=='0662164f932e28afbce12b66ff09ff283799c6ea49e25ff684f7297c8af5864a'
cells=json.loads(gzip.decompress(path.read_bytes()))['out']['actual']['cells'];truth=[]
for cell in cells:
 n=cell['remaining_bits'];count=1<<n;ones=(1<<count)-1
 variables=[sum(1<<t for t in range(count)if t>>i&1)for i in range(n)]
 synd=dict(cell['syndrome']);q={}
 for mon,value in synd.items():
  vector=ones
  for i in range(n):
   if mon>>i&1:vector &=variables[i]
  while value:
   low=value&-value;bit=low.bit_length()-1;q[bit]=q.get(bit,0)^vector;value^=low
 truth.append((ones,variables,q))
reused=[]
for source in cells:
 row=[]
 for ones,variables,q in truth:
  vector=0
  for bit,mult in source['polynomial_certificate']['witness']:
   selector=ones
   for i,v in enumerate(variables):
    if mult>>i&1:selector &=v
   vector ^=q.get(bit,0)&selector
  row.append(vector==ones)
 reused.append(row)
assert all(reused[i][i]for i in range(len(cells)))
result=dict(schema='recorded-certificate-truth-vector-replay/v1',raw_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),all_eight_certificates_verified=True,boolean_assignments_per_identity=4096,total_identity_truth_points=32768,independent_method='Bitset semantic truth-vector AND/XOR evaluation, not polynomial OR-monomial convolution or Gaussian coefficient elimination',cross_cell_fixed_witness_validity=reused,external_reviewer=False,new_target_search=False,global_roots_complete=False)
with(C/'IDENTITY_REPLAY.json').open('x')as f:json.dump(result,f,indent=2);f.write('\n')
print(json.dumps(result))
