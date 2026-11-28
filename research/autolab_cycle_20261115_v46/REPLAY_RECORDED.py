from pathlib import Path
import hashlib,json,time
from research.three_point_line_20261115_v46 import worker as binary
P=Path(__file__).resolve().parent;R=P.parents[1];start=time.monotonic();raws={}
for name,digest in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():
 data=(R/name).read_bytes();assert hashlib.sha256(data).hexdigest()==digest;x=json.loads(data);assert x['error']is None;raws[name]=x['out']
old=raws['research/three_point_line_20261115_v46/raw.json'];assert binary.controls()==old['controls'];assert binary.actual()==old['actual']
cases=0
for profile in raws['research/odd_quadratic_constraints_20261115_v46/raw.json']['profiles']:
 p=profile['p'];q=p*p
 def mul(a,b):
  A=[a%p,a//p];B=[b%p,b//p];C=[0,0,0]
  for i in range(2):
   for j in range(2):C[i+j]+=A[i]*B[j]
  return (C[0]+2*C[2])%p+p*(C[1]%p)
 def add(a,b):return (a%p+b%p)%p+p*((a//p+b//p)%p)
 for row in profile['cases']:
  roots=[];poly=row['result']['polynomial']
  for t in range(q):
   values=[add(a,add(mul(b,t),mul(c,mul(t,t))))for a,b,c in row['maps']];want=all(v<p for v in values);v=0
   if poly is not None:
    for c in reversed(poly):v=add(mul(v,t),c)
   assert want==(poly is None or v==0)
   if want:roots.append(t)
  assert roots==row['roots'];cases+=1
print(json.dumps(dict(binary_literal_states=old['controls']['literal_coefficient_states'],actual_binary_cells=len(old['actual']['records']),odd_complete_maps=cases,seconds=time.monotonic()-start)))
