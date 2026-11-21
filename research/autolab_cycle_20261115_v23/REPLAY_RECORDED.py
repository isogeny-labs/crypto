"""Standalone verification of all1022 recorded pencil systems."""
from pathlib import Path
import hashlib,json
R=Path(__file__).resolve().parents[2];raw=(R/'research/kernel_class_20261115_v23/raw.json').read_bytes();assert hashlib.sha256(raw).hexdigest()=='8d52c93409bc65b5030aea2ad5985984723ffd75da207b5f639591ed2980e9d7'
data=json.loads(raw)['out']['actual']
def xor(values):
 out=0
 for v in values:out^=v
 return out
def rank(values):
 piv={}
 for v in values:
  while v:
   p=v.bit_length()-1
   if p in piv:v^=piv[p]
   else:piv[p]=v;break
 return len(piv)
sets={};count=0
for family in data['families']:
 tables=family['basis_tables'];n=family['parameter_bits'];m=len(tables[0]);assert m==45 and len(tables)==9;seen=set();good=set()
 for record in family['records']:
  u=record['u'];assert 1<=u<512 and u not in seen;seen.add(u)
  rows=[(xor(t[j][0]for i,t in enumerate(tables)if u>>i&1),xor(t[j][1]for i,t in enumerate(tables)if u>>i&1))for j in range(m)]
  assert rank(mask for mask,rhs in rows)==record['rank']
  if record['consistent']:
   v=record['particular'];assert all((mask&v).bit_count()%2==rhs for mask,rhs in rows);good.add(u)
  else:
   lam=record['contradiction'];assert xor(mask for i,(mask,rhs)in enumerate(rows)if lam>>i&1)==0;assert xor(rhs for i,(mask,rhs)in enumerate(rows)if lam>>i&1)==1
  count+=1
 assert seen==set(range(1,512))and len(good)==371
 columns=[sum(((mask>>j)&1)<<i for i,(mask,rhs)in enumerate(table))for table in tables for j in range(n)]
 assert rank(columns)==45 and family['common_dual']['dimension']==0
 sets[family['family']]=good
assert sets['pre-Gold73']==sets['Gold69']and min(sets['Gold69'])==10
print(json.dumps(dict(systems_replayed=count,consistent_per_family=371,inconsistent_per_family=140,same_direction_sets=True,smallest_counterexample_u=10,constant_common_dual_space_zero=True,all_original_source_roots_complete=False)))
