from pathlib import Path
import json,hashlib,importlib.util
from flint import fmpz_mat
from sympy import sqrt_mod
root=Path(__file__).resolve().parent;s=importlib.util.spec_from_file_location('ec',root.parent/'round18-p192/cached_orbit.py');ec=importlib.util.module_from_spec(s);s.loader.exec_module(ec);p,r=ec.p,ec.r;t=p+1-r
assert p+1-t==r
secrets=[1,17,20261107]+[int.from_bytes(hashlib.sha256(('round50-public-'+str(i)).encode()).digest(),'big')%r for i in range(2)]
rows=[]
for x in secrets:
 basis=[[1,0,x],[0,1,x*x%r],[0,0,r]];red=fmpz_mat(basis).lll();vectors=[[int(red[i,j]) for j in range(3)] for i in range(3)]
 u,b,c=min(vectors,key=lambda v:(max(map(abs,v)),tuple(v)));a=u//2;d=a-u;M=[[a,b],[c,d]]
 assert (b*x*x+u*x-c)%r==0 and any(z%r for z in [u,b,c])
 if b%r:
  delta=(u*u+4*b*c)%r;roots=sorted({(-u+int(z))*pow(2*b%r,-1,r)%r for z in sqrt_mod(delta,r,all_roots=True)})
 else:roots=[c*pow(u%r,-1,r)%r]
 Q=ec.mul(x,ec.G);valid=[z for z in roots if ec.mul(z,ec.G)==Q];assert valid==[x]
 mul=lambda n,P:ec.mul(n%r,P)
 image=[ec.add(mul(a,ec.G),mul(b,Q)),ec.add(mul(c,ec.G),mul(d,Q))]
 assert image[1]==mul(x,image[0])
 # alpha=pi-1. On rational points alpha=0, hence F_x(G,Q)=(0,Q-xG).
 assert ec.add(Q,ec.neg(mul(x,ec.G))) is None
 rows.append({'public_secret':x,'Q':Q,'lattice_basis':basis,'reduced_basis':vectors,'u_b_c':[u,b,c],'matrix':M,'max_entry_bits':max(abs(z).bit_length() for row in M for z in row),'roots':roots,'checked_root':x,'image':image})
q=11;counts=[]
for x in range(q):
 n=0;invertible=0
 for a in range(q):
  for b in range(q):
   for c in range(q):
    for d in range(q):
     if (b*x*x+(a-d)*x-c)%q==0:
      n+=1;invertible+=(a*d-b*c)%q!=0
 assert n==q**3 and invertible==q*(q-1)**2
 counts.append({'x':x,'stabilizers':n,'invertible':invertible})
out={'all_checks_pass':True,'p':p,'r':r,'trace':t,'alpha_norm':p+1-t,'fixtures':rows,'toy_prime':q,'toy_counts':counts,'scope':'known-secret construction and marked-action extraction; unmarked quotient isomorphism is a written theorem'}
(root/'certificate.json').write_text(json.dumps(out,indent=2)+'\n');print('POINT_ACTION_AND_ROOT_CONTROLS',len(rows));print('ENTRY_BITS',[z['max_entry_bits'] for z in rows]);print('TOY_COUNTS',counts[0])
