from research.correction_cone_zdd_20261117_v129c.kernel import Manager,ZDDLimit

class LiveManager(Manager):
 def __init__(self,n,max_total_nodes=500000,max_gc_visits=5000000,**caps):
  if type(max_total_nodes)is not int or max_total_nodes<2 or type(max_gc_visits)is not int or max_gc_visits<0:raise ValueError('invalidlivenesslimits')
  self.total_created=2;self.peak_nodes=2;self.collections=0;self.reclaimed=0;self.gc_visits=0;self.gate_retries=0;self.frontier_count=0;self.max_total_nodes=max_total_nodes;self.max_gc_visits=max_gc_visits;super().__init__(n,**caps)
 def stats(self):return {**super().stats(),'total_created':self.total_created,'peak_nodes':self.peak_nodes,'collections':self.collections,'reclaimed':self.reclaimed,'gc_visits':self.gc_visits,'gate_retries':self.gate_retries,'frontier_count':self.frontier_count}
 def make(self,v,lo,hi):
  fresh=hi!=0 and(v,lo,hi)not in self.unique
  if fresh and self.total_created>=self.max_total_nodes:raise ZDDLimit('totalnodecap',self.stats())
  before=len(self.nodes);value=super().make(v,lo,hi);self.total_created+=len(self.nodes)-before;self.peak_nodes=max(self.peak_nodes,len(self.nodes));return value
 def collect(self,forms):
  keep={0,1};stack=list(forms.values())
  while stack:
   i=stack.pop()
   if i in keep:continue
   self.gc_visits+=2
   if self.gc_visits>self.max_gc_visits:raise ZDDLimit('gcworkcap',self.stats())
   keep.add(i);v,lo,hi=self.nodes[i];stack.extend([lo,hi])
  mapped={0:0,1:1};nodes=self.nodes[:2];unique={}
  for i in sorted(keep):
   if i<2:continue
   v,lo,hi=self.nodes[i];key=(v,mapped[lo],mapped[hi]);assert key not in unique;mapped[i]=len(nodes);unique[key]=len(nodes);nodes.append(key)
  remapped={i:mapped[value]for i,value in forms.items()};self.reclaimed+=len(self.nodes)-len(nodes);self.nodes=nodes;self.unique=unique;self.xcache.clear();self.mcache.clear();forms.clear();forms.update(remapped);self.collections+=1;self.frontier_count=len(forms)

def compile_live(C,outputs,collect_every=128,**caps):
 if type(collect_every)is not int or collect_every<1:raise ValueError('invalidcollectioninterval')
 M=LiveManager(C.n,**caps);needed=set();stack=list(outputs)
 while stack:
  i=stack.pop()
  if i in needed:continue
  needed.add(i);node=C.nodes[i]
  if node[0]in['and','xor','not']:stack.extend(node[1:])
 uses={i:0 for i in needed}
 for i in needed:
  node=C.nodes[i]
  if node[0]in['and','xor','not']:
   for parent in node[1:]:uses[parent]+=1
 for i in outputs:uses[i]+=1
 forms={0:0,1:1};done=0
 for i in sorted(needed):
  if i<2:continue
  kind,*args=C.nodes[i];M.frontier_count=len(forms)
  def operation():
   if kind=='input':return M.make(args[0],0,1)
   if kind=='xor':return M.xor(forms[args[0]],forms[args[1]])
   if kind=='and':return M.mul(forms[args[0]],forms[args[1]])
   assert kind=='not';return M.xor(forms[args[0]],1)
  try:value=operation()
  except ZDDLimit as e:
   if str(e)!='nodecap':raise
   M.collect(forms);M.gate_retries+=1;value=operation()
  forms[i]=value
  if kind in['and','xor','not']:
   for parent in args:
    uses[parent]-=1
    if uses[parent]==0 and parent>=2:del forms[parent]
  done+=1;M.frontier_count=len(forms)
  if done%collect_every==0:M.collect(forms)
 M.collect(forms);return M,[forms[i]for i in outputs]
