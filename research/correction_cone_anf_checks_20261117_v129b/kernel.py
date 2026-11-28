class ExpansionLimit(RuntimeError):
 def __init__(self,reason,stats):
  super().__init__(reason);self.stats=dict(stats)

def expand(C,outputs,max_degree=8,max_monomials=50000,max_stored=1000000,max_work=2000000):
 if any(type(c)is not int or c<0 for c in [max_degree,max_monomials,max_stored,max_work]):raise ValueError('capsmustbenonnegativeintegers')
 needed=set();stack=list(outputs)
 while stack:
  i=stack.pop()
  if i in needed:continue
  needed.add(i);node=C.nodes[i]
  if node[0]in['and','xor','not']:stack.extend(node[1:])
 forms={0:frozenset(),1:frozenset([0])};stats={'work':0,'stored_monomials':1,'needed_nodes':len(needed),'expanded_nodes':0,'maximum_degree':0}
 def stop(reason):raise ExpansionLimit(reason,stats)
 if stats['stored_monomials']>max_stored:stop('initialstoredcap')
 for i in sorted(needed):
  if i<2:continue
  node=C.nodes[i];kind=node[0]
  if kind=='input':value={1<<node[1]}
  elif kind=='not':
   left=forms[node[1]];stats['work']+=len(left)+1
   if stats['work']>max_work:stop('workcap')
   value=left^frozenset([0])
  elif kind=='xor':
   left,right=forms[node[1]],forms[node[2]];stats['work']+=len(left)+len(right)
   if stats['work']>max_work:stop('workcap')
   value=left^right
  else:
   assert kind=='and';left,right=forms[node[1]],forms[node[2]];stats['work']+=len(left)*len(right)
   if stats['work']>max_work:stop('workcap')
   value=set()
   for a in left:
    for b in right:
     monomial=a|b
     if monomial.bit_count()>max_degree:stop('transientdegreecap')
     if monomial in value:value.remove(monomial)
     else:value.add(monomial)
     if len(value)>max_monomials:stop('monomialcap')
  degree=max((m.bit_count()for m in value),default=0)
  if degree>max_degree:stop('degreecap')
  if len(value)>max_monomials:stop('monomialcap')
  stats['stored_monomials']+=len(value)
  if stats['stored_monomials']>max_stored:stop('storedmonomialcap')
  forms[i]=frozenset(value);stats['expanded_nodes']+=1;stats['maximum_degree']=max(stats['maximum_degree'],degree)
 return [forms[i]for i in outputs],stats

def evaluate(polynomial,assignment):return sum((assignment&m)==m for m in polynomial)%2
