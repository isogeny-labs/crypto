from .codec import field
from research.twist_auxiliary_20261116_v86.algebra import s3

def solve(c):
 f=field(c);m=c['arity'];D=c['D'];assert f.q<=32,'table budget';stats={'S3_evaluations':0,'stored_transitions':0,'walk_edges':0};T={};last={}
 for u in range(f.q):
  last[u]=[]
  for x in D:
   stats['S3_evaluations']+=1
   if s3(u,x,c['z'],c['A'],c['B'],f)==0:last[u].append(x)
   for v in range(f.q):
    stats['S3_evaluations']+=1
    if s3(u,x,v,c['A'],c['B'],f)==0:T.setdefault(u,[]).append((x,v));stats['stored_transitions']+=1
 viable={m-1:{u for u,xs in last.items()if xs}}
 for j in range(m-2,1,-1):viable[j]={u for u in range(f.q)if any(v in viable[j+1]for x,v in T.get(u,[]))}
 out=set()
 def walk(j,u,xs,aux):
  if j==m-1:
   for x in last[u]:stats['walk_edges']+=1;out.add(xs+(x,)+aux)
  else:
   for x,v in T.get(u,[]):
    if v in viable[j+1]:stats['walk_edges']+=1;walk(j+1,v,xs+(x,),aux+(v,))
 for x in D:
  for y in D:
   for u in viable[2]:
    stats['S3_evaluations']+=1
    if s3(x,y,u,c['A'],c['B'],f)==0:walk(2,u,(x,y),(u,))
 return sorted(out),stats
