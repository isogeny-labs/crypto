import time
START=time.monotonic()
import itertools,json,math
from .common import P,finish
from .kernel import frontier,frontier_count,solve
out={'structural_cases':0,'complete_root_cases':0,'tuple_work':0,'signed_cases':0,'refusals':0,'all_registered_contexts_complete':False,'original_complete':False};error=None
try:
 data=json.loads((P/'DATA.json').read_bytes());cap=data['candidate_cap']
 for p in data['small_primes']:
  for n in data['dimensions']:
   for upper in itertools.product(range(p),repeat=n):
    widths=[m+1 for m in upper];domain=list(itertools.product(*map(range,widths)))
    for v in itertools.product(data['weight_values'],repeat=n):
     if not any(v):continue
     if time.monotonic()-START>=70:raise RuntimeError('internaltimecap')
     pivot=next(i for i,w in enumerate(v)if w);A=list(frontier(widths,v));canonical=set()
     for z in domain:
      b=min(a//w for a,w in zip(z,v)if w);canonical.add(tuple(a-b*w for a,w in zip(z,v)))
     out['tuple_work']+=len(domain)+len(A);assert set(A)==canonical and len(A)==len(set(A))==frontier_count(widths,v);out['structural_cases']+=1
     for sign in [0,1,-1]:
      def c(z,p=p,v=v,pivot=pivot,sign=sign):
       I=[(v[pivot]*a-w*z[pivot])%p for a,w in zip(z,v)];return (sign*z[pivot]+sum(a*a for a in I)+I[0]*I[-1])%p
      k=sign*v[pivot]%p;expected=sorted(z for z in domain if c(z)==0);got=solve(p,upper,v,k,c,cap=cap);assert got['complete']and got['roots']==expected;out['complete_root_cases']+=1;out['tuple_work']+=len(domain)+len(A)
     assert out['tuple_work']<=data['tuple_work_cap'],'tupleworkcap'
 for p in data['small_primes']:
  for n in [2,3]:
   upper=[min(2,p-1)]*n;domain=list(itertools.product(range(upper[0]+1),repeat=n))
   for v in itertools.product(range(-2,3),repeat=n):
    if not any(v):continue
    pivot=next(i for i,w in enumerate(v)if w)
    for sign in [0,1]:
     c=lambda z,v=v,p=p,pivot=pivot,sign=sign:(sign*z[pivot]+sum((v[pivot]*a-w*z[pivot])**2 for a,w in zip(z,v)))%p
     got=solve(p,upper,v,sign*v[pivot],c,cap=cap);expected=sorted(z for z in domain if c(z)==0);assert got['complete']and got['roots']==expected;out['signed_cases']+=1
 assert solve(5,[-1,2],[1,1],1,lambda z:0)['roots']==[]
 assert solve(5,[1,1],[0,0],0,lambda z:sum(z)%5)['roots']==[(0,0)]
 for q in [solve(5,[3,3],[1,1],1,lambda z:z[0],cap=0),solve(5,[1,1],[0,0],0,lambda z:0,cap=0),solve(5,[1,1],[1,1],0,lambda z:0,output_cap=0)]:assert not q['complete']and q['roots']is None;out['refusals']+=1
 out['all_registered_contexts_complete']=True;out['total_seconds']=time.monotonic()-START
except Exception as exc:error=repr(exc)
if error:out['all_registered_contexts_complete']=False
finish(out,error,START,'Weighted reflected integer-fiber exact roots with independent exhaustive original-domain and fiber oracles','If complete,profile actual certified directions and charged integration;otherwise freeze a changed bounded panel or repair the failed assumption without replacing old evidence.')
