from pathlib import Path
from math import comb
import json,importlib.util
root=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('ec',root.parent/'round18-p192/cached_orbit.py');ec=importlib.util.module_from_spec(s);s.loader.exec_module(ec);r=ec.r
# Sparse scalar-field polynomials, keys (a exponent,b exponent).
def deriv(H,k):return {(i-(k==0),j-(k==1)):v*(i if k==0 else j)%r for (i,j),v in H.items() if (i if k==0 else j) and v%r}
def extract(H):
 A=deriv(H,0);B=deriv(H,1)
 if not A:return None
 key=next(iter(A));x=B.get(key,0)*pow(A[key],-1,r)%r
 return x if all(B.get(k,0)==x*A.get(k,0)%r for k in A.keys()|B.keys()) else None
def val(H,a,b):return sum(v*pow(a,i,r)*pow(b,j,r) for (i,j),v in H.items())%r
def encode(H):return [[i,j,v] for (i,j),v in sorted(H.items())]
rows=[]
for x in [0,1,2,17,20261107]:
 for ds in [[5],[10],[15],[5,10,15]]:
  H={(0,0):7}
  for d in ds:
   for j in range(d+1):
    v=(d+1)*comb(d,j)*pow(x,j,r)%r
    if v:H[d-j,j]=v
  recovered=extract(H);assert recovered==x
  for a,b,t in [(1,2,3),(17,29,31),(0,0,1)]:
   assert val(H,a,b)==val(H,(a-x*t)%r,(b+t)%r)
   assert val(H,a,b)==val(H,ec.mu*a%r,ec.mu*b%r)
  assert ec.mul(recovered,ec.G)==ec.mul(x,ec.G)
  rows.append({'public_secret':x,'degrees':ds,'polynomial':encode(H),'recovered':recovered})
negative=[]
for H in [{(0,0):7},{(0,5):1},{(5,0):1,(0,5):1}]:negative.append({'polynomial':encode(H),'extractor':extract(H)})
assert all(z['extractor'] is None for z in negative)
# a^5 is orbit-invariant, but fails point descent for x=17.
a5={(5,0):1};assert val(a5,1,0)==val(a5,ec.mu,0)
assert 1==(1-17)+17
assert val(a5,1,0)!=val(a5,1-17,1)
# Coefficient identity for the x-coordinate addition recurrence.
# Expand the numerator in ascending powers of X with formal parameters u,A,B.
import sympy as sp
X,u,A,B=sp.symbols('X u A B')
lhs=2*(X**3+A*X+B+u**3+A*u+B)-2*(X+u)*(X-u)**2
rhs=2*(u*X**2+(A+u**2)*X+A*u+2*B)
assert sp.expand(lhs-rhs)==0
out={'all_checks_pass':True,'r':r,'p':ec.p,'positive_cases':rows,'negative_cases':negative,'label_collision_counterexample':{'secret':17,'labels':[[1,0],[-16,1]],'values':[val(a5,1,0),val(a5,-16,1)],'same_point':True},'coordinate_polynomial_bound':{'assumptions':'P-192 r<p, f in F_p[T], f(k)=x([k]G) for integers 1<=k<r','valid_recurrence_arguments':r-3,'minimum_degree':(r-3+2)//3,'symbolic_numerator_identity':str(rhs),'status':'derived; symbolic identity checked, universal degree argument in report'}}
(root/'certificate.json').write_text(json.dumps(out,indent=2)+'\n');print('LABEL_EXTRACTOR_PASS',len(rows),'NEGATIVE_CONTROLS',len(negative));print('COORDINATE_DEGREE_FLOOR',(r-1)//3)
