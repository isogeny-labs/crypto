from collections import defaultdict
from itertools import product
from .trellis import paths,fresh_stats

def solve(R,Q,m,add,neg,image,K):
 assert K[0]is None and len(K)==len(set(K));labels={U:i for i,U in enumerate(K)};table=[[labels[add(U,V)]for V in K]for U in K];stats=fresh_stats();fibers=defaultdict(list)
 for U in R:fibers[image(U)].append(U)
 info={}
 for u,vs in fibers.items():
  base=vs[0];allowed={k:add(base,V)for k,V in enumerate(K)if add(base,V)in vs};assert set(allowed.values())==set(vs);info[u]=(base,allowed)
 D=list(info);pairs=defaultdict(list)
 for u,v in product(D,repeat=2):pairs[add(u,v)].append((u,v))
 roots=set();image_matches=0
 for tail in product(D,repeat=m-2):
  S=None
  for U in tail:S=add(S,U)
  for head in pairs.get(add(image(Q),neg(S)),[]):
   image_matches+=1;us=head+tail;bases=[info[u][0]for u in us];allowed=[info[u][1]for u in us];S=None;prefix={}
   for j,U in enumerate(bases,1):
    S=add(S,U)
    if 2<=j<m:prefix[j]=[add(S,V)for V in K]
   delta=add(Q,neg(S));assert delta in labels
   forbidden={j:{k for k,U in enumerate(v)if U is None}for j,v in prefix.items()}
   for ks in paths([tuple(a)for a in allowed],table,labels[delta],forbidden,stats):
    s=0;aux=[]
    for j,k in enumerate(ks[:-1],1):
     s=table[s][k]
     if j>=2:aux.append(prefix[j][s][0])
    roots.add(tuple(allowed[j][k][0]for j,k in enumerate(ks))+tuple(aux))
 return roots,{'kernel_order':len(K),'image_matches':image_matches,**stats}
