"""Standard-library replay of complete natural-coordinate displacement ranks."""
from pathlib import Path
import json,hashlib
P=Path(__file__).resolve().parents[2]/'research/displacement_20261115_v27/raw.json';data=P.read_bytes();assert hashlib.sha256(data).hexdigest()=='dbefdcd0f029e8aae8a945e106556a2d9e237f58908f206b9260ddfe4f8b337b';raw=json.loads(data);assert raw['error']is None;a=raw['out']['actual']
def xor(vs):
 v=0
 for x in vs:v^=x
 return v
def rank(vs):
 piv={}
 for x in vs:
  while x:
   j=x.bit_length()-1
   if j not in piv:piv[j]=x;break
   x^=piv[j]
 return len(piv)
def mul_shift(u,j):
 v=u<<j
 while v.bit_length()>83:v^=a['modulus']<<(v.bit_length()-84)
 return v
def displacement(rows,images,blocks):
 D=[];start=0
 for length,step in blocks:
  for j in range(length):
   product=sum(((rows[start+j]&v).bit_count()%2)<<i for i,v in enumerate(images));D.append(product^(rows[start+j-step]if step and j>=step else 0))
  start+=length
 assert start==len(rows);return D
assert len(a['natural_base_rows'])==93 and len(a['natural_Gold4_rows'])==4
for f in a['families']:
 g=f['family']=='Gold69';front=a['natural_base_rows']+(a['natural_Gold4_rows']if g else[])
 for r in f['records']:
  powers=[mul_shift(r['u'],j)for j in range(83)]
  rows=[m for m,c in front]+[sum(((v>>j)&1)<<i for i,v in enumerate(powers))for j in range(52,83)]+[sum(((v>>j)&1)<<(83+i)for i,v in enumerate(powers))for j in range(69,83)]
  normal=displacement(rows,a['ordinary_column_images'],[(48,1),(31,1),(14,1)]+([(4,0)]if g else[])+[(31,1),(14,1)])
  mixed=displacement(rows,a['mixed_column_images'],[(48,2),(31,2),(14,4)]+([(4,0)]if g else[])+[(31,1),(14,2)])
  assert rank(rows)==r['system_rank'] and rank(normal)==r['ordinary_displacement_rank']
  assert rank(mixed)==r['mixed_displacement_rank'] and rank(mixed[48:])==r['mixed_outside_AS_rank']<=13
  assert rank(mixed)==(61 if g else 57) and rank(mixed)>8
print('VERIFIED: all64complete matrices, ordinary/mixed displacement ranks, outside-AS bounds; whole rank8 prediction refuted.')
