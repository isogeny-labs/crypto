from research.correction_cone_zdd_20261117_v129c.kernel import Manager

class OrderedManager(Manager):
 def __init__(self,n,order,**caps):
  if sorted(order)!=list(range(n)):raise ValueError('order must be a permutation')
  super().__init__(n,**caps);self.order=list(order);self.rank={original:rank for rank,original in enumerate(order)}
 def evaluate(self,roots,assignment):
  ranked=sum(((assignment>>original)&1)<<rank for rank,original in enumerate(self.order));return super().evaluate(roots,ranked)
 def original_monomials(self,root):
  return frozenset(sum(((m>>rank)&1)<<original for rank,original in enumerate(self.order))for m in super().monomials(root))

def compile_cones(C,outputs,order,**caps):
 M=OrderedManager(C.n,order,**caps);needed=set();stack=list(outputs)
 while stack:
  i=stack.pop()
  if i in needed:continue
  needed.add(i);node=C.nodes[i]
  if node[0]in['and','xor','not']:stack.extend(node[1:])
 forms={0:0,1:1}
 for i in sorted(needed):
  if i<2:continue
  kind,*args=C.nodes[i]
  if kind=='input':value=M.make(M.rank[args[0]],0,1)
  elif kind=='xor':value=M.xor(forms[args[0]],forms[args[1]])
  elif kind=='and':value=M.mul(forms[args[0]],forms[args[1]])
  else:assert kind=='not';value=M.xor(forms[args[0]],1)
  forms[i]=value
 return M,[forms[i]for i in outputs]

def fixed_orders(C,outputs):
 needed=set();stack=list(outputs)
 while stack:
  i=stack.pop()
  if i in needed:continue
  needed.add(i);node=C.nodes[i]
  if node[0]in['and','xor','not']:stack.extend(node[1:])
 support={0:0,1:0};frequency=[0]*C.n
 for i in sorted(needed):
  if i<2:continue
  kind,*args=C.nodes[i];mask=1<<args[0]if kind=='input'else support[args[0]]|(support[args[1]]if len(args)>1 else 0);support[i]=mask
  for j in range(C.n):frequency[j]+=(mask>>j)&1
 natural=list(range(C.n));return {'natural':natural,'reverse':natural[::-1],'frequency':sorted(natural,key=lambda j:(-frequency[j],j))}
