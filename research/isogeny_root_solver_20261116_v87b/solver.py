from itertools import product
from collections import defaultdict
import json,time
from research.twist_three_isogeny_20261116_v86b.curve import add,neg,on,target,phi,scale

def solve(case,method):
 p,A,B=case['p'],case['A'],case['B'];R=[tuple(x)for x in case['R']];Q=tuple(case['Q']);T=tuple(case['T']);m=case['arity'];assert m in[3,5]and len(set(R))==len(R)
 assert case['group_order']%2==1 and on(T,p,A,B)and scale(3,T,p,A)is None
 assert on(Q,p,A,B)and all(U is not None and on(U,p,A,B)for U in R);assert all(neg(U,p)in R for U in R);start=time.monotonic();stats={'pair_steps':0,'tail_steps':0,'lift_tuples':0}
 if method=='reference':D=R;Z=Q;AA=A;fibers=None
 elif method=='auxiliary':
  AA,BB=target(T,p,A,B);fibers=defaultdict(list)
  for U in R:fibers[phi(U,T,p,A)].append(U)
  D=list(fibers);Z=phi(Q,T,p,A)
 else:raise ValueError('method')
 pairs=defaultdict(list)
 for U,V in product(D,repeat=2):
  S=add(U,V,p,AA);stats['pair_steps']+=1
  if method=='reference'and S is None:continue
  pairs[S].append((U,V))
 roots=set()
 def retain(xs):
  S=xs[0];aux=[]
  for i,U in enumerate(xs[1:],1):
   S=add(S,U,p,A)
   if i<m-1:
    if S is None:return
    aux.append(S[0])
  if S==Q:roots.add(tuple(U[0]for U in xs)+tuple(aux))
 for tail in product(D,repeat=m-2):
  S=tail[0]
  for U in tail[1:]:S=add(S,U,p,AA)
  need=add(Z,neg(S,p),p,AA);stats['tail_steps']+=1
  for head in pairs.get(need,[]):
   xs=head+tail
   if method=='reference':retain(xs)
   else:
    for us in product(*(fibers[U]for U in xs)):stats['lift_tuples']+=1;retain(us)
 out={'complete':True,'roots':sorted(roots),'exact_root_count':len(roots),'stats':stats,'kernel_setup_and_search_seconds':time.monotonic()-start};trial=json.dumps(out,separators=(',',':'))
 if time.monotonic()-start>=9:out['complete']=False;out['exact_root_count']=None
 return out
