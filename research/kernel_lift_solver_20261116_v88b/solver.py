from itertools import product
from collections import defaultdict
import json,time
from research.isogeny_root_solver_20261116_v87b.solver import solve as prior
from research.twist_three_isogeny_20261116_v86b.curve import add,neg,on,target,phi,scale

def solve(case,method):
 if method!='kernel':return prior(case,method)
 p,A,B=case['p'],case['A'],case['B'];R=[tuple(U)for U in case['R']];Q=tuple(case['Q']);T=tuple(case['T']);m=case['arity'];assert m in[3,5]and len(set(R))==len(R);assert case['group_order']%2==1 and on(T,p,A,B)and scale(3,T,p,A)is None;assert on(Q,p,A,B)and all(U is not None and on(U,p,A,B)for U in R);assert all(neg(U,p)in R for U in R)
 start=time.monotonic();AA,BB=target(T,p,A,B);fibers=defaultdict(list)
 for U in R:fibers[phi(U,T,p,A)].append(U)
 K=[None,T,neg(T,p)];labels={U:i for i,U in enumerate(K)};info={};stats={'pair_steps':0,'tail_steps':0,'lift_tuples':0,'kernel_prefix_proposals':0,'prefix_shift_additions':0,'fiber_label_additions':0}
 for u,values in fibers.items():
  base=values[0];allowed={}
  for k,V in enumerate(K):
   W=add(base,V,p,A);stats['fiber_label_additions']+=1
   if W in values:allowed[k]=W
  assert set(allowed.values())==set(values);info[u]=(base,allowed)
 D=list(fibers);Z=phi(Q,T,p,A);pairs=defaultdict(list)
 for U,V in product(D,repeat=2):pairs[add(U,V,p,AA)].append((U,V));stats['pair_steps']+=1
 roots=set()
 def lift(us):
  bases=[info[u][0]for u in us];allowed=[info[u][1]for u in us];S=bases[0];prefix={}
  for j in range(1,m):
   S=add(S,bases[j],p,A)
   if j<m-1:
    prefix[j+1]=[add(S,K[k],p,A)for k in range(3)];stats['prefix_shift_additions']+=3
  delta=add(Q,neg(S,p),p,A);assert delta in labels;target_label=labels[delta]
  for ks in product(*(v.keys()for v in allowed[:-1])):
   stats['kernel_prefix_proposals']+=1;last=(target_label-sum(ks))%3
   if last not in allowed[-1]:continue
   full=ks+(last,);stats['lift_tuples']+=1;aux=[];run=0
   for j,k in enumerate(full[:-1],1):
    run=(run+k)%3
    if j>=2:
     U=prefix[j][run]
     if U is None:break
     aux.append(U[0])
   else:roots.add(tuple(allowed[j][k][0]for j,k in enumerate(full))+tuple(aux))
 for tail in product(D,repeat=m-2):
  S=tail[0]
  for U in tail[1:]:S=add(S,U,p,AA)
  need=add(Z,neg(S,p),p,AA);stats['tail_steps']+=1
  for head in pairs.get(need,[]):lift(head+tail)
 out={'complete':True,'roots':sorted(roots),'exact_root_count':len(roots),'stats':stats,'kernel_setup_and_search_seconds':time.monotonic()-start};trial=json.dumps(out,separators=(',',':'))
 if time.monotonic()-start>=9:out['complete']=False;out['exact_root_count']=None
 return out
