"""Exact bounded reciprocal-pair query with verified Berlekamp factorization."""
from collections import Counter
from research.reciprocal_rational_20261115_v38 import prepared as p
from research.rational_fraction_oracles_20261115_v39 import worker as lin

def gcd(a,b):
 while b:a,b=b,p.divide(a,b)[1]
 return a

def irreducible(poly):
 n=p.degree(poly)
 if n<1:return False
 x=p.divide(2,poly)[1];q=x
 for i in range(1,n+1):
  q=p.divide(p.multiply(q,q),poly)[1]
  if i<=n//2 and gcd(q^x,poly)!=1:return False
 return q==x

def factors(poly):
 n=p.degree(poly)
 if n<=0:return[]
 if n==1:return[poly]
 der=sum(1<<(i-1)for i in range(1,poly.bit_length(),2)if poly>>i&1)
 if not der:
  root=sum(1<<i for i in range((poly.bit_length()+1)//2)if poly>>(2*i)&1)
  assert p.multiply(root,root)==poly
  return factors(root)*2
 g=gcd(poly,der)
 if g!=1:
  q,r=p.divide(poly,g);assert not r;return factors(g)+factors(q)
 cols=[p.divide(1<<(2*j),poly)[1]^(1<<j)for j in range(n)];rows=[sum(((v>>j)&1)<<i for i,v in enumerate(cols))for j in range(n)];K=lin.kernel(rows,n)
 if len(K)==1:assert irreducible(poly);return[poly]
 for z in K:
  if z in(0,1):continue
  g=gcd(poly,z)
  if g not in(1,poly):q,r=p.divide(poly,g);assert not r;return factors(g)+factors(q)
 raise AssertionError('no Berlekamp split')

def query(f,t,d,max_divisors=1<<20,max_pairs=1<<18):
 fraction=p.reconstruct(f.modulus,t,d);limit=1<<d
 if fraction['status']=='no_bounded_fraction':return dict(status='complete',complete=True,pairs=[],fraction=None,divisors=0,exact_count=0)
 N=fraction['numerator'];D=fraction['denominator']
 if not N:
  if limit-1>max_pairs:return dict(status='inconclusive',complete=False,pairs=[],exact_count=None,lower_bound=0)
  return dict(status='complete',complete=True,pairs=[(x,x)for x in range(1,limit)],fraction=[N,D],divisors=0,exact_count=limit-1)
 fs=factors(D);product=1
 for v in fs:assert irreducible(v);product=p.multiply(product,v)
 assert product==D
 if any(p.degree(v)>=d for v in fs):return dict(status='complete',complete=True,pairs=[],fraction=[N,D],divisors=0,factors=sorted(Counter(fs).items()),exact_count=0)
 multiplicities=sorted(Counter(fs).items());total=1
 for _,e in multiplicities:total*=2*e+1
 if total>max_divisors:return dict(status='inconclusive',complete=False,pairs=[],exact_count=None,lower_bound=0,divisor_bound=total)
 D2=p.multiply(D,D);out=[];seen=0
 def visit(i,U):
  nonlocal seen
  if i<len(multiplicities):
   factor,e=multiplicities[i];v=U
   for j in range(2*e+1):visit(i+1,v);v=p.multiply(v,factor)
   return
  seen+=1;V,r=p.divide(D2,U);assert not r
  aa,ra=p.divide(U^D,N);bb,rb=p.divide(V^D,N)
  if not ra and not rb and 0<aa<limit and 0<bb<limit:out.append((aa,bb))
 visit(0,1);assert seen==total and len(out)==len(set(out));out.sort()
 if len(out)>max_pairs:return dict(status='inconclusive',complete=False,pairs=out[:max_pairs],exact_count=None,lower_bound=max_pairs)
 return dict(status='complete',complete=True,pairs=out,fraction=[N,D],factors=multiplicities,divisors=total,exact_count=len(out))
