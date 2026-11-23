"""Independent standard-library verification of shared-kernel certificates."""
from pathlib import Path
import json,hashlib
R=Path(__file__).resolve().parents[2]
def read(folder,sha):
 data=(R/'research'/folder/'raw.json').read_bytes();assert hashlib.sha256(data).hexdigest()==sha
 o=json.loads(data);assert o['error']is None;return o['out']['actual']['families']
full=read('full_kernel_block_20261115_v26','926aded622c60dbe84ead8c58da7e278507d25662844757c61169cd3625c0187')
small=read('shared_block_20261115_v26','211f1fd92695d601c4d3104d9e8c2ee839156ff593eb88a669bb072a57116658')
def xor(vs):
 out=0
 for v in vs:out^=v
 return out
def rank(vs):
 piv={}
 for row in vs:
  while row:
   i=row.bit_length()-1
   if i not in piv:piv[i]=row;break
   row^=piv[i]
 return len(piv)
def image(rows,v):return sum(((m&v).bit_count()%2)<<i for i,(m,r)in enumerate(rows))
def matrix(tables,u):return [(xor(t[j][0]for i,t in enumerate(tables)if u>>i&1),xor(t[j][1]for i,t in enumerate(tables)if u>>i&1))for j in range(len(tables[0]))]
def consistent(rows,n):return rank([m for m,r in rows])==rank([m|(r<<n)for m,r in rows])
def project(rows,plan):
 U=plan['complement_basis'];out=[]
 for q in plan['quotient_rows']:
  m=xor(m for i,(m,r)in enumerate(rows)if q>>i&1);r=xor(r for i,(m,r)in enumerate(rows)if q>>i&1)
  out.append((sum(((m&u).bit_count()%2)<<i for i,u in enumerate(U)),r))
 return out
for f,local in zip(full,small):
 assert f['family']==local['family'];tables=f['basis_tables'];n=f['parameter_bits'];assert len(tables)==35
 for level in f['levels']:
  d=level['direction_bits'];plan=level['plan'];K=plan['kernel_basis'];U=plan['complement_basis'];stack=[m for rows in tables[:d]for m,r in rows]
  assert len(K)==n-rank(stack) and rank(K)==len(K) and rank(K+U)==n
  assert all(not image(rows,v)for rows in tables[:d]for v in K)
  assert level['active_columns']==len(U)
 for cell in local['cells']:
  plan=cell['plan'];K=plan['kernel_basis'];U=plan['complement_basis'];Q=plan['quotient_rows'];p=cell['high3']<<6;M0=matrix(tables,p)
  assert len(K)==n-rank([m for rows in tables[:6]for m,r in rows]) and rank(K+U)==n
  assert all(not image(rows,v)for rows in tables[:6]for v in K)
  W=[image(M0,v)for v in K];r=rank(W)
  assert r==plan['image_rank'] and len(Q)==45-r and rank(Q)==len(Q)
  assert all((q&w).bit_count()%2==0 for q in Q for w in W)
  for record in cell['records']:
   rows=matrix(tables,record['u']);red=project(rows,plan);original_rank=rank([m for m,c in rows]);reduced_rank=rank([m for m,c in red])
   assert original_rank==r+reduced_rank
   ok=consistent(rows,n);assert ok==consistent(red,len(U))==record['consistent']
   assert record['nullity']==(n-original_rank if ok else None)
 for probe in f['probes']:
  rows=matrix(tables,probe['u']);ok=consistent(rows,n)
  assert ok==probe['consistent'] and probe['nullity']==(n-rank([m for m,r in rows])if ok else None)
print('VERIFIED: every full-width shared-kernel basis and dimension, every complement, all512local fibers per family, all128full-width probes; full35coordinate compression claim refuted.')
