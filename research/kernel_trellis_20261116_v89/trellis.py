def paths(allowed,table,target,forbidden,stats):
 """All label paths; forbidden[j] applies to the state after j labels."""
 m=len(allowed);n=len(table);back=[set()for _ in range(m+1)];back[m]={target};count=[{}for _ in range(m+1)];count[m][target]=1
 for j in range(m-1,-1,-1):
  for s in range(n):
   total=0
   for k in allowed[j]:
    t=table[s][k];stats['dp_transitions']+=1
    if t in forbidden.get(j+1,()):stats['dp_forbidden_edges']+=1;continue
    total+=count[j+1].get(t,0)
   if total:back[j].add(s);count[j][s]=total
 stats['dp_feasible_paths']+=count[0].get(0,0)
 def walk(j,s,prefix):
  if j==m:
   yield prefix;return
  for k in allowed[j]:
   t=table[s][k]
   if t in back[j+1]and t not in forbidden.get(j+1,()):
    stats['dp_walk_edges']+=1
    yield from walk(j+1,t,prefix+(k,))
 if 0 in back[0]:yield from walk(0,0,())

def fresh_stats():return {k:0 for k in ['dp_transitions','dp_forbidden_edges','dp_feasible_paths','dp_walk_edges']}
