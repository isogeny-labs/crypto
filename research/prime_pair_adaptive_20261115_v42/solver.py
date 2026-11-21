"""Adaptive positive-interval reciprocal pairs; caller proves prime p."""
from math import gcd,isqrt

def query(p,t,M,max_work=1<<20,max_output=1<<18):
 assert all(isinstance(v,int)and not isinstance(v,bool)for v in[p,t,M]);assert M>=1 and 0<=t<p
 if 4*M**3>=p:raise ValueError('unsupported rational uniqueness bound')
 def done(pairs,fraction=None,work=0,method='none'):
  return dict(status='complete',complete=True,pairs=sorted(pairs),exact_count=len(pairs),fraction=fraction,work=work,method=method)
 if not t:return done([])
 r0,r1=p,t;s0,s1=0,1
 while r1>2*M:
  q,r2=divmod(r0,r1);s2=s0-q*s1;r0,r1=r1,r2;s0,s1=s1,s2
 if s1<=0 or s1>M*M:return done([])
 N,D=r1,s1;g=gcd(N,D);N//=g;D//=g;assert 0<N<=2*M and 0<D<=M*M and(t*D-N)%p==0
 qlo=max(1,(4*D+N*N-1)//(N*N));qhi=min((2*M)//N,(M*M)//D)
 if M*N<=D:return done([],fraction=[N,D])
 alo=max(1,D//N+1,(M*D+M*N-D-1)//(M*N-D));ahi=min(M,(2*D)//N)
 qc=max(0,qhi-qlo+1);ac=max(0,ahi-alo+1)
 if not qc or not ac:return done([],fraction=[N,D])
 method='a'if ac<qc else'q';work=min(ac,qc)
 if work>max_work:return dict(status='inconclusive',complete=False,pairs=[],exact_count=None,lower_bound=0,fraction=[N,D],q_range=[qlo,qhi],a_range=[alo,ahi],reason='both work caps')
 pairs=[]
 def emit(a,b):
  assert 1<=a<=b<=M and(pow(a,-1,p)+pow(b,-1,p))%p==t;pairs.append((a,b))
  if a!=b:pairs.append((b,a))
 if method=='a':
  for a in range(alo,ahi+1):
   b,r=divmod(D*a,N*a-D)
   if not r and a<=b<=M:emit(a,b)
   if len(pairs)>max_output:break
 else:
  for q in range(qlo,qhi+1):
   S=q*N;delta=S*S-4*q*D;r=isqrt(delta)
   if r*r==delta and(S-r)%2==0:
    a=(S-r)//2;b=(S+r)//2
    if 1<=a<=b<=M:emit(a,b)
   if len(pairs)>max_output:break
 if len(pairs)>max_output:return dict(status='inconclusive',complete=False,pairs=pairs[:max_output],exact_count=None,lower_bound=max_output,reason='output cap')
 return done(pairs,fraction=[N,D],work=work,method=method)
