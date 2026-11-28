"""Unexecuted positive-interval reciprocal-pair prototype; caller proves prime p."""
from math import gcd,isqrt

def query(p,t,M,max_work=1<<20,max_output=1<<18):
 assert all(isinstance(v,int)and not isinstance(v,bool)for v in[p,t,M])
 assert M>=1 and 0<=t<p
 if 4*M**3>=p:raise ValueError('unsupported rational uniqueness bound')
 def done(pairs,fraction=None,work=0):
  return dict(status='complete',complete=True,pairs=pairs,exact_count=len(pairs),fraction=fraction,work=work)
 if not t:return done([])
 r0,r1=p,t;s0,s1=0,1
 while r1>2*M:
  q,r2=divmod(r0,r1);s2=s0-q*s1;r0,r1=r1,r2;s0,s1=s1,s2
 if s1<=0 or s1>M*M:return done([])
 N,D=r1,s1;g=gcd(N,D);N//=g;D//=g
 assert 0<N<=2*M and 0<D<=M*M and(t*D-N)%p==0
 low=max(1,(4*D+N*N-1)//(N*N));high=min((2*M)//N,(M*M)//D)
 if high<low:return done([],fraction=[N,D])
 if high-low+1>max_work:return dict(status='inconclusive',complete=False,pairs=[],exact_count=None,lower_bound=0,fraction=[N,D],q_range=[low,high],reason='q-work cap')
 pairs=[]
 for q in range(low,high+1):
  S=q*N;delta=S*S-4*q*D;r=isqrt(delta)
  if r*r!=delta or(S-r)%2:continue
  a=(S-r)//2;b=(S+r)//2
  if 1<=a<=b<=M:
   assert(pow(a,-1,p)+pow(b,-1,p))%p==t;pairs.append((a,b))
   if a!=b:pairs.append((b,a))
  if len(pairs)>max_output:return dict(status='inconclusive',complete=False,pairs=pairs[:max_output],exact_count=None,lower_bound=max_output,reason='output cap')
 return done(sorted(pairs),fraction=[N,D],work=high-low+1)
