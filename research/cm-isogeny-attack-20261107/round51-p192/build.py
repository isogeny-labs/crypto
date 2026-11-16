from pathlib import Path
from math import isqrt
import json,importlib.util
from sympy import sqrt_mod
root=Path(__file__).resolve().parent;old=json.loads((root.parent/'round50-p192/certificate.json').read_text());p,r,t=old['p'],old['r'],old['trace'];assert p%r not in [0,1]
def squares(n):
 for z in sqrt_mod(-1,n,all_roots=True):
  a,b=n,int(z)
  while b*b>n:a,b=b,a%b
  y=isqrt(n-b*b)
  if b*b+y*y==n:return sorted([b,y])
 raise ValueError('no sum of two squares')
def bar(a):return [a[0]+t*a[1],-a[1]]
def mmul(a,b):return [a[0]*b[0]-p*a[1]*b[1],a[0]*b[1]+a[1]*b[0]+t*a[1]*b[1]]
def norm(a):return mmul(a,bar(a))[0]
def chi(a,eigen):return (a[0]+a[1]*eigen)%r
splits=[]
a,b=squares(r)
for sign in [1,-1]:
 u=[a,0];v=[sign*b,0];x=-chi(u,1)*pow(chi(v,1),-1,r)%r;splits.append({'kind':'integer','u':u,'v':v,'x':x})
N=t-2;sq=squares(N);aa,bb=sq
for n in [aa*aa,bb*bb]:
 for sign in [1,-1]:
  u=[-(n+1),1];v=[sign*aa*bb,0];x=-chi(u,1)*pow(chi(v,1),-1,r)%r;splits.append({'kind':'CM','u':u,'v':v,'x':x})
s=importlib.util.spec_from_file_location('ec',root.parent/'round18-p192/cached_orbit.py');ec=importlib.util.module_from_spec(s);s.loader.exec_module(ec)
for z in splits:
 u,v,x=z['u'],z['v'],z['x'];assert norm(u)+norm(v)==r
 # Matrix [u,v; -bar(v),bar(u)] kills both formal kernel generators.
 for e,P in [(1,[1,x]),(p%r,[-x%r,1])]:
  assert (chi(u,e)*P[0]+chi(v,e)*P[1])%r==0
  assert (-chi(bar(v),e)*P[0]+chi(bar(u),e)*P[1])%r==0
 Q=ec.mul(x,ec.G);z['Q']=Q
 assert ec.add(ec.mul(chi(u,1),ec.G),ec.mul(chi(v,1),Q)) is None
 assert ec.add(ec.mul(-chi(bar(v),1)%r,ec.G),ec.mul(chi(bar(u),1),Q)) is None
assert len({z['x'] for z in splits})==6
# Formal symplectic-space controls; gamma=(1,x,0,0), partner=(0,0,-x,1).
formal=[]
for z in old['fixtures']:
 x=z['public_secret'];formal.append({'x':x,'gamma':[1,x,0,0],'rational_completion':[[1,0,0,0],[0,1,0,0]],'mixed_completion':[[1,x,0,0],[0,0,-x%r,1]],'anti_isometry_diagonal':[x,-pow(x,-1,r)%r]})
# Minimal extension-degree exclusions only; no exact order claim.
v=1
for k in range(1,1001):v=v*(p%r)%r;assert v!=1
out={'all_checks_pass':True,'p':p,'r':r,'trace':t,'sum_squares_r':[a,b],'sum_squares_trace_minus_two':sq,'formal_controls':formal,'product_completions':splits,'extension_degree_exclusions':{'checked_k_max':1000,'last_residue':v,'exact_degree':'not computed'},'scope':'formal geometric torsion and explicit product-return controls; no general quotient model constructed'}
(root/'certificate.json').write_text(json.dumps(out,indent=2)+'\n');print('FORMAL_KERNEL_CONTROLS',len(formal),'EXPLICIT_PRODUCT_COMPLETIONS',len(splits));print('SUM_SQUARES_R',a,b);print('SUM_SQUARES_T_MINUS_2',aa,bb)
