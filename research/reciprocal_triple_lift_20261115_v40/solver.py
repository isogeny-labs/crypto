"""Exact reciprocal triples from a bounded product-polynomial linear lift."""
from collections import Counter
from math import comb
from research.reciprocal_rational_20261115_v38 import prepared as p
from research.rational_fraction_oracles_20261115_v39 import worker as lin
from research.rational_pair_query_20261115_v39 import solver as factor

def solve(f,t,d,max_dimension=12,max_allocations=1<<20,max_output=1<<18):
 A=2*d-2;B=3*d-3;assert d>=1 and B<f.degree and 0<=t<f.order
 cols=[f.mul(t,1<<j)>>(A+1)for j in range(B+1)];rows=[sum(((v>>j)&1)<<i for i,v in enumerate(cols))for j in range(f.degree-A-1)];K=lin.kernel(rows,B+1)
 evidence=dict(n=f.degree,modulus=f.modulus,t=t,d=d,numerator_degree=A,product_degree=B,kernel_basis=K,kernel_dimension=len(K),columns=cols,products=[],allocation_upper_sum=0)
 roots=[]
 def finish(complete,why=None):return dict(status='complete'if complete else'inconclusive',complete=complete,triples=sorted(roots),exact_count=len(roots)if complete else None,lower_bound=len(roots),reason=why,evidence=evidence)
 if len(K)>max_dimension:return finish(False,'lift dimension cap')
 D=0
 for i in range(1,1<<len(K)):
  D^=K[(i&-i).bit_length()-1];assert D;N=f.mul(t,D);assert p.degree(N)<=A
  fs=factor.factors(D);product=1
  for v in fs:assert factor.irreducible(v);product=p.multiply(product,v)
  assert product==D;counts=sorted(Counter(fs).items());record=dict(D=D,N=N,factors=counts,large_factor=None,allocation_upper=0,checked_leaves=0,triples=[]);evidence['products'].append(record)
  large=next((v for v in fs if p.degree(v)>=d),None)
  if large is not None:record['large_factor']=large;continue
  upper=1
  for _,e in counts:upper*=comb(e+2,2)
  record['allocation_upper']=upper;evidence['allocation_upper_sum']+=upper
  if evidence['allocation_upper_sum']>max_allocations:return finish(False,'factor allocation cap')
  powers=[]
  for v,e in counts:
   values=[1]
   for _ in range(e):values.append(p.multiply(values[-1],v))
   powers.append(values)
  def visit(j,a,b,c):
   if j==len(counts):
    record['checked_leaves']+=1
    if p.multiply(a,b)^p.multiply(a,c)^p.multiply(b,c)==N:
     assert f.inv(a)^f.inv(b)^f.inv(c)==t;roots.append((a,b,c));record['triples'].append((a,b,c))
    return
   v,e=counts[j];degree=p.degree(v)
   for ea in range(e+1):
    if p.degree(a)+ea*degree>=d:continue
    for eb in range(e-ea+1):
     ec=e-ea-eb
     if p.degree(b)+eb*degree>=d or p.degree(c)+ec*degree>=d:continue
     visit(j+1,p.multiply(a,powers[j][ea]),p.multiply(b,powers[j][eb]),p.multiply(c,powers[j][ec]))
  visit(0,1,1,1)
  if len(roots)>max_output:return finish(False,'output cap')
 assert len(set(roots))==len(roots)
 return finish(True)
