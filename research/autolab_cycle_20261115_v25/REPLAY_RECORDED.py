"""Verify numeric guard identities and complete exposed cover; standard library."""
from pathlib import Path
import json,hashlib
R=Path(__file__).resolve().parents[2]
def load(name,sha):
 p=R/'research'/name/'raw.json';data=p.read_bytes();assert hashlib.sha256(data).hexdigest()==sha
 value=json.loads(data);assert value['error']is None;return value['out']['actual']
original=load('kernel_class_20261115_v23','8d52c93409bc65b5030aea2ad5985984723ffd75da207b5f639591ed2980e9d7')['families']
refined=load('dual_refinement_20261115_v24','36889137a71fb5c5e1d6b88e45eca2b31c19b9715fbee2a4a3902351dc44a024')['families']
syzygies=load('dual_syzygy_20261115_v25','c52cb373487ad042f4bf6e7f9824d7204e53215721b1d7d2b7cb13d331ca32ef')['families']
covered=load('dual_cover_20261115_v25','b5823d6fefe80e13f12d4eae4c24649ad1f83b7aa5f18f11a972820c40c2a26f')['families']
def xor(vs):
 v=0
 for a in vs:v^=a
 return v
def points(p,B):return [p^xor(v for i,v in enumerate(B)if t>>i&1)for t in range(1<<len(B))]
def assemble(tables,u):return [(xor(row[j][0]for i,row in enumerate(tables)if u>>i&1),xor(row[j][1]for i,row in enumerate(tables)if u>>i&1))for j in range(len(tables[0]))]
def feasible(rows):
 piv={}
 for m,r in rows:
  while m:
   i=m.bit_length()-1
   if i not in piv:piv[i]=(m,r);break
   v,s=piv[i];m^=v;r^=s
  if not m and r:return False
 return True

def constant_check(tables,report,p,B):
 for log in report['logs']:
  assert log['parameter0']==p and log['parameter_basis']==B
  coeff=[assemble(tables,p)]+[assemble(tables,v)for v in B];rows=[]
  for dual in log['duals']['vectors']:
   lam=dual['multiplier'];syndrome=0
   for i,mat in enumerate(coeff):
    assert xor(m for j,(m,r)in enumerate(mat)if lam>>j&1)==0
    syndrome|=xor(r for j,(m,r)in enumerate(mat)if lam>>j&1)<<i
   assert syndrome==dual['syndrome'];rows.append((syndrome>>1,syndrome&1))
  assert rows==[tuple(v)for v in log['rows']]
  accepted={t for t in range(1<<len(B))if all((m&t).bit_count()%2==r for m,r in rows)};sol=log['solution']
  if not sol['consistent']:
   assert not accepted and report['empty'] and not report['points'];return
  assert accepted==set(points(sol['particular'],sol['basis']))
  p=p^xor(v for i,v in enumerate(B)if sol['particular']>>i&1);B=[xor(v for i,v in enumerate(B)if t>>i&1)for t in sol['basis']]
 assert not report['empty'] and p==report['parameter0'] and B==report['parameter_basis']
 assert set(report['points'])==set(points(p,B))

def syzygy_check(tables,p,B,result):
 coeff=[assemble(tables,p)]+[assemble(tables,v)for v in B];terms=[0]+[1<<i for i in range(len(B))];mon=result['monomials']
 assert mon==sorted({a|b for a in terms for b in terms})
 for guard in result['guards']:
  acc={q:[0,0]for q in mon}
  for i,a in enumerate(terms):
   lam=guard['multipliers'][i]
   for j,c in enumerate(terms):
    for r,(mask,rhs)in enumerate(coeff[j]):
     if lam>>r&1:acc[a|c][0]^=mask;acc[a|c][1]^=rhs
  assert all(v[0]==0 for v in acc.values())
  assert guard['polynomial']==sum(acc[q][1]<<i for i,q in enumerate(mon))
 ps=points(p,B)
 return {u for t,u in enumerate(ps)if all(xor(1 for i,q in enumerate(mon)if guard['polynomial']>>i&1 and t&q==q)==0 for guard in result['guards'])}
for f,ref,syz,cov in zip(original,refined,syzygies,covered):
 assert f['family']==ref['family']==syz['family']==cov['family'];tables=f['basis_tables']
 truth={u for u in range(512)if feasible(assemble(tables,u))};assert len(truth)==372
 constant_check(tables,ref['prefix'],0,[1<<i for i in range(6)])
 assert ref['prefix']['dimension_path']==[6,4] and len(ref['prefix']['points'])==16
 s=syz['result'];kept=syzygy_check(tables,s['parameter0'],s['parameter_basis'],s)
 assert kept==set(s['candidate_directions'])==truth&set(range(64))=={0,10,20,30,35,41,55,61}
 assert s['guards'][0]['polynomial']==134
 assert all((u&42).bit_count()%2==0 for u in kept)
 seen=set();allkept=set();constant_total=0
 for cell in cov['cells']:
  h=cell['high3'];domain=set(range(h<<6,(h+1)<<6));assert not seen&domain;seen|=domain
  cf=cell['constant_filter'];constant_check(tables,cf,h<<6,[1<<i for i in range(6)]);constant_total+=len(cf['points'])
  found=set()if cf['empty']else syzygy_check(tables,cf['parameter0'],cf['parameter_basis'],cell['syzygy'])
  assert found==set(cell['points']) and truth&domain<=found<=domain
  assert set(cell['actual_directions'])==truth&domain;assert cell['exact']==(found==truth&domain)
  bad=len(set(cf['points'])-truth);d=len(cf['parameter_basis']);bound=1<<max(0,d-2)
  assert cell['incompatible_after_constants']==bad and cell['nonzero_quadratic_minimum_weight']==bound
  obstruction=not cf['empty']and bad<bound;assert obstruction==cell['quadratic_guard_impossible_from_density']
  if obstruction:assert not cell['syzygy']['guards']
  allkept|=found
 assert seen==set(range(512)) and constant_total==432 and len(allkept)==384 and len(allkept-truth)==12
print('VERIFIED: constant refinement stalls; variable dual identities recover exact low6; complete low9 cover retains372feasible+12unresolved per family, with checked density obstructions.')
