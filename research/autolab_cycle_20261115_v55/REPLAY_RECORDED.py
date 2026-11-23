"""Run from any directory; verify public evidence without native binaries."""
import hashlib,json,pathlib,statistics
R=pathlib.Path(__file__).resolve().parents[2];P=R/'research/autolab_cycle_20261115_v55'
for name,h in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
D=R/'research/original_chain_lift_20261115_v55';data=json.loads((D/'DATA.json').read_text());expected={}
def mul(a,b,m):
 v=0
 while b:
  if b&1:v^=a
  b>>=1;a<<=1
 while v.bit_length()>=m.bit_length():v^=m<<(v.bit_length()-m.bit_length())
 return v
for case in data['actual']:
 rows=json.loads((D/(case['name']+'.json')).read_text())['tuples'];expected[case['name']]=rows
 for row in rows:
  a,b,c,d,e,u,v,w=row
  for x,y,z in[(a,b,u),(u,c,v),(v,d,w),(w,e,case['target'])]:
   m=case['modulus'];q=mul(x,y,m)^mul(x,z,m)^mul(y,z,m);assert mul(q,q,m)^mul(mul(x,y,m),z,m)^case['B']==0
cold=json.loads((R/'research/original_chain_cold_20261115_v55/raw.json').read_text());assert cold['error']is None;out=cold['out']
for record in out['records']:
 raw=R/record['raw_path'];assert hashlib.sha256(raw.read_bytes()).hexdigest()==record['raw_sha256'];body=json.loads(raw.read_text());child=json.loads(body['stdout']);assert body['exit_code']==0
 for case in child['cases']:assert case['complete']and case['original_tuples']==expected[case['name']]
for m in ['reference','compressed']:
 times=[r['cold_wall_seconds']for r in out['records']if r['method']==m];assert len(times)==3;assert statistics.median(times)==out['medians'][m];assert max(times)/min(times)-1==out['AA_spread'][m]
assert out['conditional_ratio']==out['medians']['reference']/out['medians']['compressed']
print('Hashes, all conditional original tuples, literal residuals and cold timing arithmetic verified. Timing noise fails10%; global genuineGB gate remains open.')
